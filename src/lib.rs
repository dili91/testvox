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
