use crate::TestResult;

pub mod slack;

#[derive(Default)]
pub struct ReportBuilder {
    title: String,
    test_results: Vec<TestResult>,
}

pub trait PrettyPrint {
    fn to_string_pretty(&self) -> String;
}

impl ReportBuilder {
    pub fn new() -> ReportBuilder {
        ReportBuilder::default()
    }

    pub fn with_title(mut self, title: String) -> ReportBuilder {
        self.title = title;
        self
    }

    pub fn with_test_results(mut self, test_results: Vec<TestResult>) -> ReportBuilder {
        self.test_results = test_results;
        self.test_results.sort_by(|a, b| a.status.cmp(&b.status));
        self
    }

    pub fn build<T>(self) -> T
    where
        T: From<ReportBuilder> + PrettyPrint,
    {
        Into::into(self)
    }
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use crate::TestResult;

    use super::{PrettyPrint, ReportBuilder};

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

    #[test]
    fn should_build_a_default_report_builder() {
        let rb = ReportBuilder::new();

        assert_eq!(rb.title, "");
        assert!(rb.test_results.is_empty());
    }

    #[test]
    fn should_build_a_report_with_given_title_and_test_results() {
        let test_results: Vec<TestResult> = vec![
            TestResult::builder()
                .with_name("a-test-passed".to_string())
                .with_status(crate::TestStatus::Passed)
                .build(),
            TestResult::builder()
                .with_name("a-test-failed".to_string())
                .with_status(crate::TestStatus::Failed)
                .build(),
            TestResult::builder()
                .with_name("a-test-skipped".to_string())
                .with_status(crate::TestStatus::Skipped)
                .build(),
        ];
        let report = ReportBuilder::new()
            .with_title("a-report".to_string())
            .with_test_results(test_results)
            .build::<CustomReport>();

        assert_eq!(
            report.to_string_pretty(),
            "a-report=[(\"a-test-failed\", \"Failed\"), (\"a-test-skipped\", \"Skipped\"), (\"a-test-passed\", \"Passed\")]");
    }
}
