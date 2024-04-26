use anyhow::Result;
use glob::glob;
use std::{fs, path::Path};
use roxmltree::Document;
use junit_to_slack_reporter::{slack::{self, MarkdownText, PlainText}, MarkdownMessage, TestReport, TestResult, TestStatus};

fn main() {
    let test_report =
        build_report("/junit-reports/**/*.xml").expect("Unable to create test report");

    let header_block = slack::Block::Header {
        text: PlainText {
            text:
                ":java::fire: Acceptance tests are failing in Sandbox on the Java backend library!"
                    .to_string(),
            emoji: true,
        },
    };

    let mut section_blocks: Vec<slack::Block> = test_report
        .test_results
        .into_iter()
        .filter(|t| t.status != TestStatus::Passed)
        .map(|t| {
            vec![
                slack::Block::Divider,
                slack::Block::Section {
                    text: MarkdownText {
                        text: t.to_string(),
                    },
                },
            ]
        })
        .flatten()
        .collect();

    let mut blocks = vec![header_block];
    blocks.append(&mut section_blocks);

    let slack_report = slack::Report { blocks };

    println!(
        "{}",
        serde_json::to_string_pretty(&slack_report).expect("unable to serialize to JSON string")
    )
}

fn build_report(junit_reports_file_pattern: &str) -> Result<TestReport> {
    let mut test_results: Vec<TestResult> = vec![];
    for test_file in glob(junit_reports_file_pattern)? {
        match test_file {
            Ok(path) => {
                let mut individual_test_results = parse_test_file(path.as_path())?;
                test_results.append(&mut individual_test_results)
            }
            Err(e) => println!("{:?}", e),
        }
    }

    Ok(TestReport { test_results })
}

fn parse_test_file(test_file: &Path) -> Result<Vec<TestResult>> {
    let report = fs::read_to_string(test_file)?;
    let doc = Document::parse(report.as_str())?;

    let mut test_results: Vec<TestResult> = vec![];

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
                test_results.push(TestResult {
                    suite: test_suite.attribute("name").unwrap().to_string(),
                    name: test_name.to_string(),
                    status: TestStatus::Failed,
                    failure: Some(failure.attribute("message").unwrap().to_string()),
                })
            } else if n.children().any(|n| n.has_tag_name("skipped")) {
                test_results.push(TestResult {
                    suite: test_suite.attribute("name").unwrap().to_string(),
                    name: test_name.to_string(),
                    status: TestStatus::Skipped,
                    failure: None,
                })
            } else {
                test_results.push(TestResult {
                    suite: test_suite.attribute("name").unwrap().to_string(),
                    name: test_name.to_string(),
                    status: TestStatus::Passed,
                    failure: None,
                })
            };
        });

    Ok(test_results)
}
