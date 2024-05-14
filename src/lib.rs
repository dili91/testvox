//TODO: documentation

use anyhow::Result;
use models::{
    test_report::{PrettyPrint, ReportBuilder},
    test_result::TestResult,
};
use parsers::{junit::JunitTestParser, TestParser};

pub mod models;
pub mod parsers;
pub mod reporters;

pub fn create_test_report<T>(request: CreateTestReportRequest) -> T
where
    T: From<ReportBuilder> + PrettyPrint,
{
    // Automatically detect test parser and flatten all results into a single array of results
    let test_results: Vec<TestResult> = request
        .reports_contents
        .into_iter()
        .map(|test_file| {
            detect_parser(test_file)
                .expect("Unable to detect test parser")
                .parse()
        })
        .filter_map(|test_results| test_results.ok())
        .flatten()
        .collect();

    let mut report_builder = ReportBuilder::new()
        .with_title(request.title)
        .with_test_results(test_results);

    if request.include_passed {
        report_builder = report_builder.include_passed();
    }

    if request.include_skipped {
        report_builder = report_builder.include_skipped();
    }

    report_builder.build::<T>()
}

fn detect_parser(report_content: String) -> Result<Box<dyn TestParser>> {
    //TODO: implement real test detector
    Ok(Box::new(JunitTestParser::from(report_content)))
}

#[derive(Default)]
pub struct CreateTestReportRequest {
    pub title: String,
    pub reports_contents: Vec<String>,
    pub include_passed: bool,
    pub include_skipped: bool,
}

#[cfg(test)]
mod tests {
    use assert_json::assert_json;
    use std::fs;

    use crate::{
        create_test_report, models::test_report::PrettyPrint, reporters::slack::SlackReport,
        CreateTestReportRequest,
    };

    #[test]
    fn should_create_a_slack_report_from_junit_results() {
        let req = CreateTestReportRequest {
            title: "My cool test report".to_string(),
            reports_contents: vec![
                fs::read_to_string("./test-results/basic.xml").expect("Unable to read file")
            ],
            include_passed: true,
            include_skipped: true,
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
                }
            ]
        }
        );
    }
}
