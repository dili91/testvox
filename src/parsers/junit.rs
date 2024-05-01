use crate::{TestResult, TestStatus};
use anyhow::Result;
use roxmltree::{Document, Node};
use std::{fs, path::PathBuf};

use super::parsers::TestParser;

pub struct JunitTestParser {
    pub test_file: PathBuf,
}

impl JunitTestParser {
    pub fn new(test_file: PathBuf) -> Self {
        Self { test_file }
    }
}

impl TestParser for JunitTestParser {
    fn parse(&self) -> Result<Vec<TestResult>> {
        let junit_report = fs::read_to_string(&self.test_file)?;

        let doc = Document::parse(junit_report.as_str())?;

        let mut test_results: Vec<TestResult> = vec![];

        let test_suites: Vec<Node> = doc
            .descendants()
            .filter(|n| n.has_tag_name("testsuite"))
            .collect();

        for test_suite in test_suites {
            let suite_name = test_suite.attribute("name").map(|s| s.to_string());
            test_suite
                .children()
                .filter(|n| n.has_tag_name("testcase"))
                .for_each(|n| {
                    let test_name = n.attribute("name").unwrap();

                    // TODO: make below more compact
                    if let Some(failure) = n.children().find(|n| n.has_tag_name("failure")) {
                        test_results.push(TestResult {
                            name: test_name.to_string(),
                            suite_name: suite_name.clone(),
                            execution_time: n.attribute("time").unwrap().parse().unwrap(),
                            status: TestStatus::Failed,
                            failure: Some(failure.attribute("message").unwrap().to_string()),
                        })
                    } else if n.children().any(|n| n.has_tag_name("skipped")) {
                        test_results.push(TestResult {
                            name: test_name.to_string(),
                            suite_name: suite_name.clone(),
                            execution_time: n.attribute("time").unwrap().parse().unwrap(),
                            status: TestStatus::Skipped,
                            failure: None,
                        })
                    } else {
                        test_results.push(TestResult {
                            name: test_name.to_string(),
                            suite_name: suite_name.clone(),
                            execution_time: n.attribute("time").unwrap().parse().unwrap(),
                            status: TestStatus::Passed,
                            failure: None,
                        })
                    };
                });
        }

        Ok(test_results)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_parse_junit_test_report() {
        todo!()
    }
}
