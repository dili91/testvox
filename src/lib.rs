pub mod slack;

pub struct JunitTestReport {
    pub test_results: Vec<JunitTestResult>,
}
pub struct JunitTestResult {
    pub suite: String,
    pub name: String,
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

impl MarkdownTestResult for JunitTestResult {
    fn to_string(&self) -> String {
        match self.status {
            TestStatus::Passed => format!("✅ {}", self.name),
            TestStatus::Failed => format!(
                "❌ _{}_ *failed* with reason: ```{}```",
                self.name,
                self.failure.clone().expect("missing failure message")
            ),
            TestStatus::Skipped => format!("⏭️ _{}_ was *skipped*", self.name),
        }
    }
}
