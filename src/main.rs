use anyhow::Result;
use clap::Parser;
use glob::glob;
use reportly::{
    parsers::{junit::JunitTestParser, parsers::TestParser},
    reporters::slack::SlackReport,
    TestResult,
};

#[derive(Parser)]
struct Args {
    /// The title of the test report
    #[arg(short, long, default_value_t = String::from("Test report"))]
    report_title: String,
    /// The test report pattern to look for
    #[arg(short, long)]
    test_reports_pattern: String,
}

//TODO: remove code that panics
fn main() {
    let args = Args::parse();

    let junit_test_results =
        parse_test_results(&args.test_reports_pattern).expect("Unable to create test report");

    let report = SlackReport::builder()
        .with_title(args.report_title)
        .with_test_blocks(junit_test_results)
        .build();

    println!(
        "{}",
        serde_json::to_string_pretty(&report).expect("unable to serialize to JSON string")
    )
}

// TODO: use a builder
fn parse_test_results(junit_reports_file_pattern: &str) -> Result<Vec<TestResult>> {
    let mut test_results: Vec<TestResult> = vec![];
    for test_file in glob(junit_reports_file_pattern).expect("something went wrong") {
        match test_file {
            Ok(path) => {
                let junit_parser = JunitTestParser {
                    file_name: path.to_str().unwrap().to_string(),
                };
                let mut individual_test_results = junit_parser.parse()?;
                test_results.append(&mut individual_test_results);
            }
            Err(e) => println!("{:?}", e),
        }
    }

    Ok(test_results)
}
