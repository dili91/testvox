use anyhow::Result;
use glob::glob;
use reportly::{slack::SlackReport, JunitTestReport, JunitTestResult, TestStatus};
use roxmltree::Document;
use std::{fs, path::Path};

fn main() {
    let slack_report: SlackReport = build_report("/junit-reports/**/*.xml")
        .expect("Unable to create test report")
        .into();

    println!(
        "{}",
        serde_json::to_string_pretty(&slack_report).expect("unable to serialize to JSON string")
    )
}

// TODO: use a builder
fn build_report(junit_reports_file_pattern: &str) -> Result<JunitTestReport> {
    let mut test_results: Vec<JunitTestResult> = vec![];
    for test_file in glob(junit_reports_file_pattern)? {
        match test_file {
            Ok(path) => {
                let mut individual_test_results = parse_test_file(path.as_path())?;
                test_results.append(&mut individual_test_results)
            }
            Err(e) => println!("{:?}", e),
        }
    }

    Ok(JunitTestReport { test_results })
}

fn parse_test_file(test_file: &Path) -> Result<Vec<JunitTestResult>> {
    let report = fs::read_to_string(test_file)?;
    let doc = Document::parse(report.as_str())?;

    let mut test_results: Vec<JunitTestResult> = vec![];

    let test_suite = doc
        .descendants()
        .find(|n| n.has_tag_name("testsuite"))
        .unwrap();

    test_suite
        .children()
        .filter(|n| n.has_tag_name("testcase"))
        .for_each(|n| {
            let test_name = n.attribute("name").unwrap();

            if let Some(failure) = n.children().find(|n| n.has_tag_name("failure")) {
                test_results.push(JunitTestResult {
                    suite: test_suite.attribute("name").unwrap().to_string(),
                    name: test_name.to_string(),
                    status: TestStatus::Failed,
                    failure: Some(failure.attribute("message").unwrap().to_string()),
                })
            } else if n.children().any(|n| n.has_tag_name("skipped")) {
                test_results.push(JunitTestResult {
                    suite: test_suite.attribute("name").unwrap().to_string(),
                    name: test_name.to_string(),
                    status: TestStatus::Skipped,
                    failure: None,
                })
            } else {
                test_results.push(JunitTestResult {
                    suite: test_suite.attribute("name").unwrap().to_string(),
                    name: test_name.to_string(),
                    status: TestStatus::Passed,
                    failure: None,
                })
            };
        });

    Ok(test_results)
}
