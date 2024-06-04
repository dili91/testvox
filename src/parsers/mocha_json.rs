use serde_json::Value;

use crate::models::{test_result::TestResult, test_status::TestStatus};

use super::TestParser;

/// Struct that defines the Junit test parser
pub struct MochaJsonTestParser {
    pub content: String,
}

impl MochaJsonTestParser {
    pub fn from(content: String) -> Self {
        Self { content }
    }
}

/// Logic that converts Mocha test results into a list of `TestResult` domain instances
impl TestParser for MochaJsonTestParser {
    fn parse(&self) -> anyhow::Result<Vec<crate::models::test_result::TestResult>> {
        let mut test_results: Vec<TestResult> = vec![];
        let json_report: Value = serde_json::from_str(&self.content)?;

        json_report.as_object().map(|json_report| {
            json_report.get("tests").map(|tests| {
                tests.as_array().into_iter().flatten().map(|tr| {
                    tr.as_object().map(|tr| {
                        let name = tr
                            .get("name")
                            .and_then(|n| n.as_str())
                            .unwrap_or("⚠️ missing test name");

                        //TODO: understand how to expect status...not clear if state is included
                        let status = tr
                            .get("status")
                            .and_then(|n| n.as_str())
                            .map(|s| match s {
                                "failed" => TestStatus::Failed,
                                "pending" => TestStatus::Skipped,
                                "passed" => TestStatus::Passed,
                                _ => TestStatus::Failed,
                            })
                            .unwrap_or(TestStatus::Failed);

                        test_results.push(
                            TestResult::builder()
                                .with_name(name.to_string())
                                .with_status(status)
                                .build(),
                        )
                    })
                })
            })
        });

        Ok(test_results)
    }
}

#[cfg(test)]
mod tests {
    use super::MochaJsonTestParser;
    use crate::{models::test_status::TestStatus, parsers::TestParser};
    use indoc::indoc;

    #[test]
    fn should_parse_mocha_test_report_into_test_results() {
        let junit_test_results_contents = indoc! {r#"
        {
            "stats": {
                "suites": 1,
                "tests": 3,
                "passes": 1,
                "pending": 1,
                "failures": 1,
                "start": "2024-06-04T12:00:00.000Z",
                "end": "2024-06-04T12:00:10.000Z",
                "duration": 10000
            },
            "tests": [
                {
                    "title": "should pass this test",
                    "fullTitle": "My Suite should pass this test",
                    "file": "test/sampleTest.js",
                    "duration": 10,
                    "currentRetry": 0,
                    "err": {},
                    "state": "passed"
                },
                {
                    "title": "should fail this test",
                    "fullTitle": "My Suite should fail this test",
                    "file": "test/sampleTest.js",
                    "duration": 5,
                    "currentRetry": 0,
                    "err": {
                    "message": "Expected true to be false",
                    "stack": "AssertionError: Expected true to be false\n    at Context.<anonymous> (test/sampleTest.js:10:15)"
                    },
                    "state": "failed"
                },
                {
                    "title": "should be a pending test",
                    "fullTitle": "My Suite should be a pending test",
                    "file": "test/sampleTest.js",
                    "duration": 0,
                    "currentRetry": 0,
                    "state": "pending"
                }
            ],
            "pending": [
                {
                    "title": "should be a pending test",
                    "fullTitle": "My Suite should be a pending test",
                    "file": "test/sampleTest.js",
                    "duration": 0,
                    "currentRetry": 0,
                    "state": "pending"
                }
            ],
            "failures": [
                {
                    "title": "should fail this test",
                    "fullTitle": "My Suite should fail this test",
                    "file": "test/sampleTest.js",
                    "duration": 5,
                    "currentRetry": 0,
                    "err": {
                    "message": "Expected true to be false",
                    "stack": "AssertionError: Expected true to be false\n    at Context.<anonymous> (test/sampleTest.js:10:15)"
                    },
                    "state": "failed"
                }
            ],
            "passes": [
                {
                    "title": "should pass this test",
                    "fullTitle": "My Suite should pass this test",
                    "file": "test/sampleTest.js",
                    "duration": 10,
                    "currentRetry": 0,
                    "err": {},
                    "state": "passed"
                }
            ]
        }"#};

        let mocha_json_parser = MochaJsonTestParser::from(junit_test_results_contents.to_string());

        let test_results = mocha_json_parser
            .parse()
            .expect("Unable to parse test results content");

        assert_eq!(test_results.len(), 3);
        assert_eq!(
            test_results
                .iter()
                .filter(|t| t.status == TestStatus::Passed)
                .count(),
            1
        );
        assert_eq!(
            test_results
                .iter()
                .filter(|t| t.status == TestStatus::Skipped)
                .count(),
            1
        );
        assert_eq!(
            test_results
                .iter()
                .filter(|t| t.status == TestStatus::Failed)
                .count(),
            1
        );

        // tests are ordered by status: Failed, Skipped, Passed
        let first = test_results
            .iter()
            .find(|t| t.name == "should fail this test")
            .expect("Failed to find expected test");
        assert_eq!(first.suite_name, Some("My Suite ".to_string()));
        assert!(matches!(first.status, TestStatus::Failed,));
        assert_eq!(
            first.failure_message,
            Some("Expected true to be false".to_string())
        );
        assert_eq!(first.execution_time, Some(0.982));

        let second = test_results
            .iter()
            .find(|t| t.name == "should be a pending test")
            .expect("Failed to find expected test");
        assert_eq!(second.suite_name, Some("My Suite ".to_string()));
        assert!(matches!(second.status, TestStatus::Skipped,));
        assert!(second.failure_message.is_none());
        assert!(second.execution_time.is_none());

        let third = test_results
            .iter()
            .find(|t| t.name == "testCase1")
            .expect("Failed to find expected test");
        assert!(third.suite_name.is_none());
        assert!(matches!(third.status, TestStatus::Passed,));
        assert!(third.failure_message.is_none());
        assert_eq!(third.execution_time, Some(2.113871));
    }
}
