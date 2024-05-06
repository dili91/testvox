use crate::{TestResult, TestStatus};
use anyhow::Result;
use core::f32;
use roxmltree::{Document, Node};
use std::{fs, path::PathBuf};

use super::TestParser;

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
            test_suite
                .children()
                .filter(|n| n.has_tag_name("testcase"))
                .for_each(|n| {
                    let mut test_result_builder = TestResult::builder().with_name(
                        n.attribute("name")
                            .unwrap_or("⚠️ missing test name")
                            .to_string(),
                    );

                    if let Some(suite_name) = test_suite.attribute("name").map(|s| s.to_string()) {
                        test_result_builder =
                            test_result_builder.clone().with_suite_name(suite_name);
                    }

                    if let Some(failure) = n.children().find(|n| n.has_tag_name("failure")) {
                        test_result_builder = test_result_builder
                            .clone()
                            .with_status(TestStatus::Failed)
                            .with_failure_message(
                                failure
                                    .attribute("message")
                                    .unwrap_or("⚠️ missing test name")
                                    .to_string(),
                            );
                    } else if n.children().any(|n| n.has_tag_name("skipped")) {
                        test_result_builder =
                            test_result_builder.clone().with_status(TestStatus::Skipped);
                    } else {
                        test_result_builder =
                            test_result_builder.clone().with_status(TestStatus::Passed);
                    };

                    if let Some(execution_time) = n.attribute("time") {
                        let _ = execution_time.parse::<f32>().map(|t| {
                            test_result_builder = test_result_builder.clone().with_execution_time(t)
                        });
                    }

                    test_results.push(test_result_builder.build())
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
