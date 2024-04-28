use crate::{TestReport, TestResult, TestStatus};
use anyhow::Result;
use roxmltree::{Document, Node};
use std::fs;

use super::parsers::TestParser;

pub struct JunitTestParser {
    pub file_name: String,
}

impl TestParser for JunitTestParser {
    fn parse(&self) -> Result<TestReport> {
        let junit_report = fs::read_to_string(&self.file_name)?;

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

        Ok(TestReport {
            file_name: self.file_name.clone(),
            results: test_results,
        })
    }
}
