pub mod parsers;
pub mod reporters;

pub struct TestReport {
    pub file_name: String,
    pub results: Vec<TestResult>,
}
pub struct TestResult {
    pub name: String,
    pub suite_name: Option<String>,
    pub execution_time: f32,
    pub status: TestStatus,
    pub failure: Option<String>,
}

#[derive(PartialEq)]
pub enum TestStatus {
    Passed,
    Failed,
    Skipped,
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
                self.failure.clone().expect("missing failure message")
            ),
            TestStatus::Skipped => format!("⏭️ _{}_ was *skipped*", self.name),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{MarkdownTestResult, TestResult};

    #[test]
    fn should_convert_to_markdown_test_message() {
        let test_result = TestResult{ 
            name: "A test name".to_string(), 
            suite_name: Some("A test suite".to_string()), 
            execution_time: 2.4, 
            status: crate::TestStatus::Passed, 
            failure: None };

        let markdown_message = test_result.to_string();

        assert_eq!(markdown_message,"✅ _A test name_ *passed* (`2.4s`)");
    }
}