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
    use crate::parsers::TestParser;
    use indoc::indoc;

    #[test]
    fn should_parse_junit_test_report_into_test_results() {
        // Arrange
        let junit_test_results_contents = indoc! {"
            <?xml version=\"1.0\" encoding=\"UTF-8\"?>
            <testsuites time=\"15.682687\">
                <testsuite name=\"Tests.Registration\" time=\"6.605871\">
                    <testcase name=\"testCase1\" classname=\"Tests.Registration\" time=\"2.113871\" />
                    <testcase name=\"testCase2\" classname=\"Tests.Registration\" time=\"1.051\" />
                    <testcase name=\"testCase3\" classname=\"Tests.Registration\" time=\"3.441\" />
                </testsuite>
                <testsuite name=\"Tests.Authentication\" time=\"9.076816\">
                    <testsuite name=\"Tests.Authentication.Login\" time=\"4.356\">
                        <testcase name=\"testCase4\" classname=\"Tests.Authentication.Login\" time=\"2.244\" />
                        <testcase name=\"testCase5\" classname=\"Tests.Authentication.Login\" time=\"0.781\" />
                        <testcase name=\"testCase6\" classname=\"Tests.Authentication.Login\" time=\"1.331\" />
                    </testsuite>
                    <testcase name=\"testCase7\" classname=\"Tests.Authentication\" time=\"2.508\" />
                    <testcase name=\"testCase8\" classname=\"Tests.Authentication\" time=\"1.230816\" />
                    <testcase name=\"testCase9\" classname=\"Tests.Authentication\" time=\"0.982\">
                        <failure message=\"Assertion error message\" type=\"AssertionError\">
                            <!-- Call stack printed here -->
                        </failure>            
                    </testcase>
                </testsuite>
            </testsuites>"};

        let junit_parser = JunitTestParser::from(junit_test_results_contents.to_string());

        // Act
        let test_results = junit_parser
            .parse()
            .expect("Unable to parse test results content");

        // Assert
        assert_eq!(test_results.len(), 9);
        todo!("to complete")
    }
}
