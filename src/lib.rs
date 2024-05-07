pub mod parsers;
pub mod reporters;

pub struct TestResult {
    pub name: String,
    pub suite_name: Option<String>,
    pub execution_time: f32,
    pub status: TestStatus,
    pub failure_message: Option<String>,
}

impl TestResult {
    pub fn builder() -> TestResultBuilder {
        TestResultBuilder::default()
    }
}

#[derive(PartialEq, Default, Clone)]
pub enum TestStatus {
    #[default]
    Failed,
    Passed,
    Skipped,
}

#[derive(Default, Clone)]
pub struct TestResultBuilder {
    name: String,
    suite_name: Option<String>,
    pub execution_time: f32,
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
        self.execution_time = execution_time;
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
                format!("✅ _{}_ *passed* (`{}s`)", self.name, self.execution_time)
            }
            TestStatus::Failed => format!(
                "❌ _{}_ *failed* (`{}s`): ```{}```",
                self.name,
                self.execution_time,
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
    use crate::{MarkdownTestResult, TestResult};

    #[test]
    fn should_convert_passed_test_result_to_markdown_test_message() {
        let test_result = TestResult {
            name: "SomeTest".to_string(),
            suite_name: Some("A test suite".to_string()),
            execution_time: 2.4,
            status: crate::TestStatus::Passed,
            failure_message: None,
        };

        let markdown_message = test_result.to_string();

        assert_eq!(markdown_message, "✅ _SomeTest_ *passed* (`2.4s`)");
    }

    #[test]
    fn should_convert_skipped_test_result_to_markdown_test_message() {
        let test_result = TestResult {
            name: "SomeTest".to_string(),
            suite_name: Some("A test suite".to_string()),
            execution_time: 2.4,
            status: crate::TestStatus::Skipped,
            failure_message: None,
        };

        let markdown_message = test_result.to_string();

        assert_eq!(markdown_message, "⏭️ _SomeTest_ was *skipped*");
    }

    #[test]
    fn should_convert_failed_test_result_to_markdown_test_message() {
        let test_result = TestResult {
            name: "SomeTest".to_string(),
            suite_name: Some("A test suite".to_string()),
            execution_time: 2.4,
            status: crate::TestStatus::Failed,
            failure_message: Some("A timeout occurred".to_string()),
        };

        let markdown_message = test_result.to_string();

        assert_eq!(
            markdown_message,
            "❌ _SomeTest_ *failed* (`2.4s`): ```A timeout occurred```"
        );
    }
}
