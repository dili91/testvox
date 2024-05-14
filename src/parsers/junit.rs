use crate::{TestResult, TestStatus};
use anyhow::Result;
use core::f32;
use roxmltree::{Document, Node};

use super::TestParser;

pub struct JunitTestParser {
    pub content: String,
}

impl JunitTestParser {
    pub fn from(content: String) -> Self {
        Self { content }
    }
}

impl TestParser for JunitTestParser {
    fn parse(&self) -> Result<Vec<TestResult>> {
        let doc = Document::parse(self.content.as_str())?;

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
    use super::JunitTestParser;
    use crate::{parsers::TestParser, TestStatus};
    use indoc::indoc;

    #[test]
    fn should_parse_junit_test_report_into_test_results() {
        let junit_test_results_contents = indoc! {"
            <?xml version=\"1.0\" encoding=\"UTF-8\"?>
            <testsuites time=\"15.682687\">
                <testsuite name=\"Tests.Registration\" time=\"6.605871\">
                    <testcase name=\"testCase1\" classname=\"Tests.Registration\" time=\"2.113871\" />
                </testsuite>
                <testsuite name=\"Tests.Authentication\">
                    <testsuite name=\"Tests.Authentication.Login\">
                        <testcase name=\"testCase4\" classname=\"Tests.Authentication.Login\" >
                            <skipped/>
                        </testcase>
                    </testsuite>
                    <testcase name=\"testCase9\" classname=\"Tests.Authentication\" time=\"0.982\">
                        <failure message=\"Assertion error message\" type=\"AssertionError\">
                            <!-- Call stack printed here -->
                        </failure>            
                    </testcase>
                </testsuite>
            </testsuites>"};
        let junit_parser = JunitTestParser::from(junit_test_results_contents.to_string());

        let test_results = junit_parser
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
            .find(|t| t.name == "testCase9")
            .expect("Failed to find expected test");
        assert_eq!(first.suite_name, Some("Tests.Authentication".to_string()));
        assert!(matches!(first.status, TestStatus::Failed,));
        assert_eq!(
            first.failure_message,
            Some("Assertion error message".to_string())
        );
        assert_eq!(first.execution_time, Some(0.982));

        let second = test_results
            .iter()
            .find(|t| t.name == "testCase4")
            .expect("Failed to find expected test");
        assert_eq!(
            second.suite_name,
            Some("Tests.Authentication.Login".to_string())
        );
        assert!(matches!(second.status, TestStatus::Skipped,));
        assert!(second.failure_message.is_none());
        assert!(second.execution_time.is_none());

        let third = test_results
            .iter()
            .find(|t| t.name == "testCase1")
            .expect("Failed to find expected test");
        assert_eq!(third.suite_name, Some("Tests.Registration".to_string()));
        assert!(matches!(third.status, TestStatus::Passed,));
        assert!(third.failure_message.is_none());
        assert_eq!(third.execution_time, Some(2.113871));
    }
}
