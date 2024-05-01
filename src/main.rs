use clap::Parser;
use glob::glob;
use reportly::{
    parsers::{junit::JunitTestParser, parsers::TestParser},
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

    // Parse test results
    let junit_test_results: Vec<TestResult> = glob(&args.test_reports_pattern)
        .expect("Unable to use given file pattern")
        .into_iter()
        .filter_map(|test_file| test_file.ok())
        .map(|test_file| JunitTestParser::new(test_file).parse())
        .filter_map(|test_results| test_results.ok())
        .flatten()
        .collect();

    // Build the final report
    let report = SlackReport::builder()
        .with_title(args.report_title)
        .with_test_blocks(junit_test_results)
        .build();

    println!(
        "{}",
        serde_json::to_string_pretty(&report).expect("unable to serialize to JSON string")
    )
}
