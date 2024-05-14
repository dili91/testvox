use std::collections::HashSet;

use crate::models::{test_result::TestResult, test_status::TestStatus};

pub mod slack;

#[derive(Default)]
pub struct ReportBuilder {
    title: String,
    test_results: Vec<TestResult>,
    reportable_statuses: HashSet<TestStatus>,
}

pub trait PrettyPrint {
    fn to_string_pretty(&self) -> String;
}

impl ReportBuilder {
    pub fn new() -> ReportBuilder {
        let mut rb = ReportBuilder::default();
        rb.reportable_statuses.insert(TestStatus::Failed);
        rb
    }

    pub fn with_title(mut self, title: String) -> ReportBuilder {
        self.title = title;
        self
    }

    pub fn with_test_results(mut self, test_results: Vec<TestResult>) -> ReportBuilder {
        self.test_results = test_results;
        self
    }

    pub fn include_passed(mut self) -> ReportBuilder {
        self.reportable_statuses.insert(TestStatus::Passed);
        self
    }

    pub fn include_skipped(mut self) -> ReportBuilder {
        self.reportable_statuses.insert(TestStatus::Skipped);
        self
    }

    pub fn build<T>(mut self) -> T
    where
        T: From<ReportBuilder> + PrettyPrint,
    {
        self.test_results
            .retain(|t| self.reportable_statuses.contains(&t.status));
        self.test_results.sort_by(|a, b| a.status.cmp(&b.status));

        Into::into(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{test_result::TestResult, test_status::TestStatus};

    use super::{PrettyPrint, ReportBuilder};
    use serde::Serialize;

    #[test]
    fn should_build_a_default_report_builder() {
        let rb = ReportBuilder::new();

        assert_eq!(rb.title, "");
        assert!(rb.test_results.is_empty());
        assert_eq!(rb.reportable_statuses.len(), 1);
        assert!(rb.reportable_statuses.contains(&TestStatus::Failed))
    }

    #[test]
    fn should_build_a_report_with_given_title_and_test_results() {
        let test_results: Vec<TestResult> = vec![
            TestResult::builder()
                .with_name("a-test-passed".to_string())
                .with_status(TestStatus::Passed)
                .build(),
            TestResult::builder()
                .with_name("a-test-failed".to_string())
                .with_status(TestStatus::Failed)
                .build(),
            TestResult::builder()
                .with_name("a-test-skipped".to_string())
                .with_status(TestStatus::Skipped)
                .build(),
        ];
        let report = ReportBuilder::new()
            .with_title("a-report".to_string())
            .with_test_results(test_results)
            .include_passed()
            .include_skipped()
            .build::<CustomReport>();

        assert_eq!(
            report.to_string_pretty(),
            "a-report=[(\"a-test-failed\", \"Failed\"), (\"a-test-skipped\", \"Skipped\"), (\"a-test-passed\", \"Passed\")]");
    }

    #[derive(Serialize)]
    struct CustomReport {
        report_title: String,
        test_results: Vec<(String, String)>,
    }

    impl From<ReportBuilder> for CustomReport {
        fn from(value: ReportBuilder) -> Self {
            let test_results: Vec<(String, String)> = value
                .test_results
                .into_iter()
                .map(|tr| (tr.name, tr.status.to_string()))
                .collect();

            Self {
                report_title: value.title,
                test_results,
            }
        }
    }

    impl PrettyPrint for CustomReport {
        fn to_string_pretty(&self) -> String {
            format!("{}={:?}", self.report_title, self.test_results)
        }
    }
}
