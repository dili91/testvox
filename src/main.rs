use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use glob::glob;
use testvox::{
    parsers::{junit::JunitTestParser, TestParser},
    reporters::{slack::SlackReport, PrettyPrint},
    TestResult, TestStatus,
};

#[derive(Parser)]
struct CliArgs {
    /// The title of the test report
    #[arg(short, long, default_value_t = String::from("Test report"))]
    report_title: String,
    #[arg(long, default_value_t = false)]
    include_skipped: bool,
    #[arg(long, default_value_t = false)]
    include_passed: bool,
    /// The test report pattern to look for
    #[arg(
        num_args(1..),
        value_delimiter= ',',
        default_value = "./build/test-results/**/*.xml,./app/build/test-results/**/*.xml")
    ]
    test_reports_pattern: Vec<String>,
}

fn main() {
    // Parse CLI arguments
    let cli_args = CliArgs::parse();

    let test_results_files: Vec<PathBuf> = cli_args
        .test_reports_pattern
        .into_iter()
        .flat_map(|pattern| {
            glob(&pattern)
                .expect("Unable to use given file pattern")
                .filter_map(|test_file| test_file.ok())
                .collect::<Vec<PathBuf>>()
        })
        .collect();

    if test_results_files.is_empty() {
        eprintln!("Cannot find test results file");
        return;
    }

    // Automatically detect test parser and flatten all results into a single array of results
    let test_results: Vec<TestResult> = test_results_files
        .into_iter()
        .map(|test_file| {
            detect_parser(test_file)
                .expect("Unable to detect test parser")
                .parse()
        })
        .filter_map(|test_results| test_results.ok())
        .flatten()
        .filter(|test_result| {
            is_reportable(
                test_result,
                cli_args.include_skipped,
                cli_args.include_passed,
            )
        })
        .collect();

    // Build and print the final report
    let report = SlackReport::builder()
        .with_title(cli_args.report_title)
        .with_test_results(test_results)
        .build();

    println!("{}", report.to_string_pretty())
}

fn detect_parser(test_file: PathBuf) -> Result<Box<dyn TestParser>> {
    let content = fs::read_to_string(test_file)?;
    Ok(Box::new(JunitTestParser::from(content)))
}

fn is_reportable(test_result: &TestResult, include_skipped: bool, include_passed: bool) -> bool {
    test_result.status == TestStatus::Failed
        || (test_result.status == TestStatus::Skipped && include_skipped)
        || (test_result.status == TestStatus::Passed && include_passed)
}
