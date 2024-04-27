use anyhow::Result;
use glob::glob;
use reportly::{readers::junit::parse_test_file, writers::slack::SlackReport, TestReport};

//TODO: remove code that panics
fn main() {
    let slack_report: SlackReport = build_reports("./test-reports/**/*.xml")
        .expect("Unable to create test report")
        .into();

    println!(
        "{}",
        serde_json::to_string_pretty(&slack_report).expect("unable to serialize to JSON string")
    )
}

// TODO: use a builder
fn build_reports(junit_reports_file_pattern: &str) -> Result<Vec<TestReport>> {
    let mut reports: Vec<TestReport> = vec![];
    for test_file in glob(junit_reports_file_pattern)? {
        match test_file {
            Ok(path) => {
                let report: TestReport = parse_test_file(path.as_path())?;
                reports.push(report)
            }
            Err(e) => println!("{:?}", e),
        }
    }

    Ok(reports)
}
