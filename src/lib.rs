//!
//! Testvox helps you turning test reports into human readable messages, ready to be shared on common messaging apps.

use anyhow::Result;
use models::{
    test_report::{PrettyPrint, ReportBuilder},
    test_result::TestResult, // TestResult is already here
};
// use parsers::{junit::JunitTestParser, TestParser}; // These are no longer directly used here
use url::Url;

// basic models of the library
pub mod models;
/// generic and custom parsers types
pub mod parsers;
/// generic and custom reporter types
pub mod reporters;

/// Utility to create a test report of the desired format. The generic type `T` must implement traits
/// that hold the logic of how the test results in specific formats should be formatted, and pretty printed.
pub fn create_test_report<T>(request: CreateTestReportRequest) -> T
where
    T: From<ReportBuilder> + PrettyPrint,
{
    // Parsing logic has moved to main.rs.
    // create_test_report now directly uses the test_results from the request.
    let mut report_builder = ReportBuilder::new()
        .with_title(request.title)
        .with_test_results(request.test_results); // Use request.test_results directly

    if request.include_passed {
        report_builder = report_builder.include_passed();
    }

    if request.include_skipped {
        report_builder = report_builder.include_skipped();
    }

    if let Some(link) = request.link {
        report_builder = report_builder.with_link(link);
    }

    report_builder.build::<T>()
}

// detect_parser function removed as parsing logic is now in main.rs

/// A struct that describe the request for creating a report
#[derive(Default)]
pub struct CreateTestReportRequest {
    /// the title that the generated report should have
    pub title: String,
    /// the parsed test results
    pub test_results: Vec<TestResult>, // Changed from reports_contents
    /// whether to include passed tests in the generated reports
    pub include_passed: bool,
    /// whether to include skipped tests in the generated reports
    pub include_skipped: bool,
    /// optional url to external system, usually a CI pipeline
    pub link: Option<Url>,
}

#[cfg(test)]
mod tests {
    use assert_json::assert_json;
    // use std::fs; // fs is no longer needed for this test directly
    use url::Url;
    use crate::models::test_status::TestStatus; // Added for constructing TestResult

    use crate::{
        create_test_report, models::test_report::PrettyPrint, reporters::slack::SlackReport,
        CreateTestReportRequest, TestResult, // Added TestResult here for direct use
    };

    #[test]
    fn should_create_a_slack_report_from_parsed_test_results() { // Renamed test
        // Manually create TestResult instances, similar to what junit.xml would produce
        let test_results_data = vec![
            TestResult::builder()
                .with_name("It should update the password".to_string())
                .with_status(TestStatus::Failed)
                .with_execution_time(0.982)
                .with_failure_message("bad credentials".to_string())
                .build(),
            TestResult::builder()
                .with_name("It should login the user".to_string())
                .with_status(TestStatus::Skipped)
                .build(),
            TestResult::builder()
                .with_name("It should create a new user".to_string())
                .with_status(TestStatus::Passed)
                .with_execution_time(2.113871)
                .build(),
            TestResult::builder()
                .with_name("It should fail due to user already existing".to_string())
                .with_status(TestStatus::Passed)
                .with_execution_time(1.051)
                .build(),
            TestResult::builder()
                .with_name("It should fail due to bad credentials".to_string())
                .with_status(TestStatus::Passed)
                .with_execution_time(0.781)
                .build(),
        ];

        let req = CreateTestReportRequest {
            title: "My cool test report".to_string(),
            test_results: test_results_data, // Use pre-parsed results
            include_passed: true,
            include_skipped: true,
            link: Some(Url::parse("http://localhost/run/123").expect("unable to parse link")),
        };

        let report: SlackReport = create_test_report(req);

        assert_json!(report.to_string_pretty().as_str(), {
            "blocks": [
                {
                    "type": "header",
                    "text": {
                        "emoji": true,
                        "text": "My cool test report",
                        "type": "plain_text"
                    }
                },
                {
                    "type":"divider"
                },
                {
                    "type":"section",
                    "text": {
                        "text": "❌ _It should update the password_ *failed* (`0.982s`): ```bad credentials```",
                        "type": "mrkdwn"
                    }
                },
                {
                    "type":"divider"
                },
                {
                    "type":"section",
                    "text": {
                        "text": "⏭️ _It should login the user_ was *skipped*",
                        "type": "mrkdwn"
                    }
                },
                {
                    "type":"divider"
                },
                {
                    "type":"section",
                    "text": {
                        "text": "✅ _It should create a new user_ *passed* (`2.113871s`)",
                        "type": "mrkdwn"
                    }
                },
                {
                    "type":"divider"
                },
                {
                    "type":"section",
                    "text": {
                        "text": "✅ _It should fail due to user already existing_ *passed* (`1.051s`)",
                        "type": "mrkdwn"
                    }
                },
                {
                    "type":"divider"
                },
                {
                    "type":"section",
                    "text": {
                        "text": "✅ _It should fail due to bad credentials_ *passed* (`0.781s`)",
                        "type": "mrkdwn"
                    }
                },
                {
                    "type":"divider"
                },
                {
                    "type":"actions",
                    "elements": [
                        {
                            "type": "button",
                            "text": {
                                "type": "plain_text",
                                "text": ":link:  View details",
                                "emoji": true
                            },
                            "url": "http://localhost/run/123"
                        }
                    ]
                }
            ]
        }
        );
    }
}
