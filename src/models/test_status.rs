use core::fmt;
use std::{cmp::Ordering, fmt::Formatter};

#[derive(PartialEq, Eq, Hash, Default, Clone, Debug)]
pub enum TestStatus {
    #[default]
    Failed,
    Passed,
    Skipped,
}

impl std::fmt::Display for TestStatus {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
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

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::models::{test_result::TestResult, test_status::TestStatus};

    #[test]
    fn list_should_be_ordered_based_on_status() {
        let mut test_results = vec![
            TestResult::builder()
                .with_status(TestStatus::Skipped)
                .build(),
            TestResult::builder()
                .with_status(TestStatus::Passed)
                .build(),
            TestResult::builder()
                .with_status(TestStatus::Failed)
                .build(),
        ];

        test_results.sort_by(|a, b| a.status.cmp(&b.status));

        assert!(matches!(
            test_results.first().unwrap().status,
            TestStatus::Failed
        ),);
        assert!(matches!(
            test_results.get(1).unwrap().status,
            TestStatus::Skipped
        ),);
        assert!(matches!(
            test_results.get(2).unwrap().status,
            TestStatus::Passed
        ),);
    }

    #[test_case(TestStatus::Failed, "Failed")]
    #[test_case(TestStatus::Skipped, "Skipped")]
    #[test_case(TestStatus::Passed, "Passed")]
    fn test_status_should_yield_string_representation(
        test_status: TestStatus,
        expected_string: &str,
    ) {
        assert_eq!(test_status.to_string(), expected_string)
    }
}
