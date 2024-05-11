use std::cmp::Ordering;

pub mod parsers;
pub mod reporters;

pub struct TestResult {
    pub name: String,
    pub suite_name: Option<String>,
    pub execution_time: Option<f32>,
    pub status: TestStatus,
    pub failure_message: Option<String>,
}

impl TestResult {
    pub fn builder() -> TestResultBuilder {
        TestResultBuilder::default()
    }
}

#[derive(PartialEq, Eq, Default, Clone)]
pub enum TestStatus {
    #[default]
    Failed,
    Passed,
    Skipped,
}

impl Ord for TestStatus {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (TestStatus::Failed, TestStatus::Failed) => Ordering::Equal,
            (TestStatus::Failed, _) => Ordering::Less,
            (_, TestStatus::Failed) => Ordering::Greater,
            (TestStatus::Skipped, TestStatus::Skipped) => Ordering::Equal,
            (TestStatus::Skipped, _) => Ordering::Less,
            (_, TestStatus::Skipped) => Ordering::Greater,
            (TestStatus::Passed, TestStatus::Passed) => Ordering::Equal,
        }
    }
}

impl PartialOrd for TestStatus {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default, Clone)]
pub struct TestResultBuilder {
    name: String,
    suite_name: Option<String>,
    pub execution_time: Option<f32>,
    pub status: TestStatus,
    pub failure_message: Option<String>,
}

impl TestResultBuilder {
    pub fn with_name(mut self, name: String) -> TestResultBuilder {
        self.name = name;
        self
    }

    pub fn with_suite_name(mut self, suite_name: String) -> TestResultBuilder {
        self.suite_name = Some(suite_name);
        self
    }

    pub fn with_execution_time(mut self, execution_time: f32) -> TestResultBuilder {
        self.execution_time = Some(execution_time);
        self
    }

    pub fn with_status(mut self, status: TestStatus) -> TestResultBuilder {
        self.status = status;
        self
    }

    pub fn with_failure_message(mut self, failure_message: String) -> TestResultBuilder {
        self.failure_message = Some(failure_message);
        self
    }

    pub fn build(self) -> TestResult {
        TestResult {
            name: self.name,
            suite_name: self.suite_name,
            execution_time: self.execution_time,
            status: self.status,
            failure_message: self.failure_message,
        }
    }
}

pub trait MarkdownTestResult {
    fn to_string(&self) -> String;
}

impl MarkdownTestResult for TestResult {
    fn to_string(&self) -> String {
        match self.status {
            TestStatus::Passed => {
                format!(
                    "✅ _{}_ *passed* (`{}s`)",
                    self.name,
                    self.execution_time.unwrap_or(0.0)
                )
            }
            TestStatus::Failed => format!(
                "❌ _{}_ *failed* (`{}s`): ```{}```",
                self.name,
                self.execution_time.unwrap_or(0.0),
                self.failure_message
                    .clone()
                    .unwrap_or("⚠️ missing failure message".to_string())
            ),
            TestStatus::Skipped => format!("⏭️ _{}_ was *skipped*", self.name),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{MarkdownTestResult, TestResult, TestStatus};
    use test_case::test_case;

    #[test_case(TestResult {
        name: "SomeTest".to_string(),
        suite_name: Some("A test suite".to_string()),
        execution_time: Some(2.4),
        status: TestStatus::Failed,
        failure_message: Some("A timeout occurred".to_string()),
    }, "❌ _SomeTest_ *failed* (`2.4s`): ```A timeout occurred```" ; "test failed")]
    #[test_case(TestResult {
        name: "AnotherTest".to_string(),
        suite_name: Some("A test suite".to_string()),
        execution_time: None,
        status: TestStatus::Skipped,
        failure_message: None,
    }, "⏭️ _AnotherTest_ was *skipped*"; "test skipped")]
    #[test_case(TestResult {
        name: "PassedTest".to_string(),
        suite_name: Some("A test suite".to_string()),
        execution_time: Some(2.4),
        status: TestStatus::Passed,
        failure_message: None,
    }, "✅ _PassedTest_ *passed* (`2.4s`)" ; "test passed")]
    fn trait_should_convert_to_markdown_test_message(
        test_result: TestResult,
        expected_markdown_message: &str,
    ) {
        let actual_markdown_message = test_result.to_string();

        assert_eq!(actual_markdown_message, expected_markdown_message);
    }

    #[test]
    fn builder_should_build_a_test_result() {
        let t = TestResult::builder()
            .with_name("a test name".to_string())
            .with_execution_time(1.2)
            .with_status(TestStatus::Failed)
            .with_failure_message("something bad happened".to_string())
            .with_suite_name("a suite name".to_string())
            .build();

        assert_eq!(t.name, "a test name");
        assert_eq!(t.execution_time, Some(1.2));
        assert!(matches!(t.status, TestStatus::Failed,));
        assert_eq!(
            t.failure_message,
            Some("something bad happened".to_string())
        );
        assert_eq!(t.suite_name, Some("a suite name".to_string()));
    }

    #[test]
    fn list_should_be_ordered_based_on_status() {
        todo!()
    }
}
