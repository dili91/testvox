use std::path::PathBuf;

use clap::Parser;
use glob::glob;
use reportly::{
    parsers::{junit::JunitTestParser, TestParser},
    reporters::slack::SlackReport,
    TestResult,
};

#[derive(Parser)]
struct CliArgs {
    /// The title of the test report
    #[arg(short, long, default_value_t = String::from("Test report"))]
    report_title: String,
    /// The test report pattern to look for
    #[arg(short, long)]
    test_reports_pattern: String,
}

fn main() {
    // Parse CLI arguments
    let args = CliArgs::parse();

    let test_results_files = glob(&args.test_reports_pattern)
        .expect("Unable to use given file pattern")
        .filter_map(|test_file| test_file.ok())
        .collect::<Vec<PathBuf>>();

    if test_results_files.is_empty() {
        eprintln!("Cannot find test results file");
        return;
    }

    let test_results: Vec<TestResult> = test_results_files
        .into_iter()
        .map(|test_file| JunitTestParser::new(test_file).parse())
        .filter_map(|test_results| test_results.ok())
        .flatten()
        .collect();

    // Build and print the final report
    let report = SlackReport::builder()
        .with_title(args.report_title)
        .with_test_blocks(test_results)
        .build();

    println!(
        "{}",
        serde_json::to_string_pretty(&report).expect("unable to serialize to JSON")
    )
}
