use clap::Parser;
use glob::glob;
use std::{fs, path::PathBuf};
use url::Url;

use testvox::{
    create_test_report, models::test_report::PrettyPrint, reporters::slack::SlackReport,
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
}

impl From<CliArgs> for CreateTestReportRequest {
    fn from(value: CliArgs) -> Self {
        let reports_contents: Vec<String> = value
            .reports_pattern
            .into_iter()
            .flat_map(|pattern| {
                glob(&pattern)
                    .expect("Unable to use given file pattern")
                    .filter_map(|test_file| test_file.ok())
                    .collect::<Vec<PathBuf>>()
            })
            .filter_map(|path| fs::read_to_string(path).ok())
            .collect();

        if reports_contents.is_empty() {
            panic!("Cannot find test results file");
        }

        Self {
            title: value.title,
            reports_contents,
            include_passed: value.include_passed,
            include_skipped: value.include_skipped,
            link: value.link,
        }
    }
}

fn main() {
    let cli_args = CliArgs::parse();

    let report: SlackReport = create_test_report(cli_args.into());

    println!("{}", report.to_string_pretty())
}
