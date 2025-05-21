use clap::Parser;
use glob::glob;
use std::{fs, path::PathBuf, io::stderr, io::Write};
use url::Url;

use testvox::{
    create_test_report,
    models::test_report::PrettyPrint,
    models::test_result::TestResult, // Added
    parsers::junit::JunitTestParser, // Added
    parsers::mocha::MochaTestParser, // Added
    parsers::TestParser,             // Added
    reporters::slack::SlackReport,
    CreateTestReportRequest,
};

/// Turns test reports into human readable summaries, to be shared on common messaging apps.
#[derive(Parser)]
#[command(arg_required_else_help(true))]
struct CliArgs {
    /// The title of the test report
    #[arg(short, long, required = true)]
    title: String,
    #[arg(short = 's', long, default_value_t = false)]
    /// Whether to include skipped tests in the report
    include_skipped: bool,
    /// Whether to include passed tests in the report
    #[arg(short = 'p', long, default_value_t = false)]
    include_passed: bool,
    /// The test report pattern to look for
    #[arg(
        short,
        long,
        required = true,
        num_args(1..),
        value_delimiter= ',',
        default_value = "./build/test-results/**/*.xml,./app/build/test-results/**/*.xml")
    ]
    reports_pattern: Vec<String>,
    /// Optional link to view more details related to the report, usually a CI workflow
    #[arg(short, long, default_value = None)]
    link: Option<Url>,
    /// The type of report to parse (e.g. junit, mocha)
    #[arg(short = 't', long, default_value = None)]
    report_type: Option<String>,
}

impl From<CliArgs> for CreateTestReportRequest {
    fn from(value: CliArgs) -> Self {
        let mut test_results: Vec<TestResult> = Vec::new();
        let report_type_str = value.report_type.as_deref().unwrap_or("junit");

        let mut selected_parser_type = "junit"; // Default

        if report_type_str.eq_ignore_ascii_case("mocha") {
            selected_parser_type = "mocha";
        } else if !report_type_str.eq_ignore_ascii_case("junit") {
            // Only warn if a report_type was specified AND it's not junit or mocha
            if value.report_type.is_some() {
                 let _ = writeln!(
                    stderr(),
                    "Warning: Unknown report_type '{}' specified. Defaulting to 'junit'.",
                    report_type_str
                );
            }
            // selected_parser_type remains "junit"
        }

        for pattern in value.reports_pattern {
            match glob(&pattern) {
                Ok(paths) => {
                    for entry in paths {
                        match entry {
                            Ok(path) => {
                                if let Ok(file_content) = fs::read_to_string(&path) {
                                    let parser: Box<dyn TestParser> =
                                        if selected_parser_type == "mocha" {
                                            Box::new(MochaTestParser::from(file_content))
                                        } else {
                                            Box::new(JunitTestParser::from(file_content))
                                        };

                                    match parser.parse() {
                                        Ok(mut parsed_results) => {
                                            test_results.append(&mut parsed_results)
                                        }
                                        Err(e) => {
                                            let _ = writeln!(
                                                stderr(),
                                                "Warning: Failed to parse file {:?}: {}",
                                                path, e
                                            );
                                        }
                                    }
                                } else {
                                     let _ = writeln!(stderr(), "Warning: Failed to read file {:?}", path);
                                }
                            }
                            Err(e) => {
                                let _ = writeln!(stderr(), "Warning: Glob entry error: {}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                     let _ = writeln!(stderr(), "Warning: Invalid glob pattern '{}': {}", pattern, e);
                }
            }
        }

        if test_results.is_empty() {
            // This behavior is slightly different from before.
            // Previously, it panicked if reports_contents was empty (meaning no files found/read).
            // Now, it means no tests were successfully parsed.
            // Depending on desired behavior, one might panic or allow empty reports.
            // For now, let it proceed, an empty report will be generated.
            let _ = writeln!(stderr(), "Warning: No test results found or parsed. Generating an empty report.");
        }

        Self {
            title: value.title,
            test_results, // Field name changed from reports_contents
            include_passed: value.include_passed,
            include_skipped: value.include_skipped,
            link: value.link,
        }
    }
}

fn main() {
    let cli_args = CliArgs::parse();
    // Temporary print removed

    let report_request: CreateTestReportRequest = cli_args.into();
    
    // Check if test_results is empty and if we should exit early
    // This depends on how create_test_report handles an empty test_results vector.
    // If it panics or errors, we might want to handle it here.
    // For now, assume create_test_report can handle it or produce an empty report.

    let report: SlackReport = create_test_report(report_request);

    println!("{}", report.to_string_pretty())
}
