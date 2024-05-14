use crate::models::test_result::TestResult;
use serde::Serialize;

use super::{PrettyPrint, ReportBuilder};

#[derive(Serialize)]
pub struct SlackReport {
    pub blocks: Vec<Block>,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Block {
    Header { text: PlainText },
    Section { text: MarkdownText },
    Divider,
}

#[derive(Serialize)]
#[serde(tag = "type", rename = "plain_text")]
pub struct PlainText {
    pub text: String,
    pub emoji: bool,
}

#[derive(Serialize)]
#[serde(tag = "type", rename = "mrkdwn")]
pub struct MarkdownText {
    pub text: String,
}

impl PrettyPrint for SlackReport {
    fn to_string_pretty(&self) -> String {
        serde_json::to_string_pretty(&self).expect("unable to serialize report to JSON")
    }
}

impl From<ReportBuilder> for SlackReport {
    fn from(value: ReportBuilder) -> Self {
        let header_block = Block::Header {
            text: PlainText {
                text: value.title,
                emoji: true,
            },
        };

        let mut section_blocks: Vec<Block> = if value.test_results.is_empty() {
            vec![
                Block::Divider,
                Block::Section {
                    text: MarkdownText {
                        text: "⚠️ unable to find test results".to_string(),
                    },
                },
            ]
        } else {
            value
                .test_results
                .into_iter()
                .flat_map(|t| {
                    vec![
                        Block::Divider,
                        Block::Section {
                            text: MarkdownText {
                                text: t.to_markdown_string(),
                            },
                        },
                    ]
                })
                .collect()
        };

        let mut blocks = vec![header_block];
        blocks.append(&mut section_blocks);

        SlackReport { blocks }
    }
}

impl From<TestResult> for Vec<Block> {
    fn from(test_result: TestResult) -> Self {
        vec![
            Block::Divider,
            Block::Section {
                text: MarkdownText {
                    text: test_result.to_markdown_string(),
                },
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        models::{test_result::TestResult, test_status::TestStatus},
        reporters::{PrettyPrint, ReportBuilder},
    };
    use assert_json::assert_json;

    use super::SlackReport;

    #[test]
    fn should_create_report_in_slack_format_empty() {
        let title = "An empty Slack report";
        let report = ReportBuilder::new()
            .with_title(title.to_string())
            .build::<SlackReport>();

        assert_json!(report.to_string_pretty().as_str(), {
                "blocks": [
                    {
                        "type": "header",
                        "text": {
                            "emoji": true,
                            "text": title,
                            "type": "plain_text"
                        }
                    },
                    {
                        "type":"divider"
                    },
                    {
                        "type":"section",
                        "text": {
                            "text": "⚠️ unable to find test results",
                            "type": "mrkdwn"
                        }
                    },
                ]
            }
        );
    }

    #[test]
    fn should_create_report_in_slack_format() {
        let title = "A Slack report";
        let test_failed = TestResult::builder()
            .with_name("a test failed".to_string())
            .with_status(TestStatus::Failed)
            .with_failure_message("A failure".to_string())
            .with_execution_time(1.2)
            .build();
        let test_skipped = TestResult::builder()
            .with_name("a test skipped".to_string())
            .with_status(TestStatus::Skipped)
            .build();
        let test_passed = TestResult::builder()
            .with_name("a test passed".to_string())
            .with_status(TestStatus::Passed)
            .with_execution_time(3.3)
            .build();

        let report: SlackReport = ReportBuilder::new()
            .with_title(title.to_string())
            .include_passed()
            .include_skipped()
            .with_test_results(vec![
                test_failed.clone(),
                test_passed.clone(),
                test_skipped.clone(),
            ])
            .build();

        assert_json!(report.to_string_pretty().as_str(), {
                "blocks": [
                    {
                        "type": "header",
                        "text": {
                            "emoji": true,
                            "text": title,
                            "type": "plain_text"
                        }
                    },
                    {
                        "type":"divider"
                    },
                    {
                        "type":"section",
                        "text": {
                            "text": test_failed.to_markdown_string(),
                            "type": "mrkdwn"
                        }
                    },
                    {
                        "type":"divider"
                    },
                    {
                        "type":"section",
                        "text": {
                            "text": test_skipped.to_markdown_string(),
                            "type": "mrkdwn"
                        }
                    },
                    {
                        "type":"divider"
                    },
                    {
                        "type":"section",
                        "text": {
                            "text": test_passed.to_markdown_string(),
                            "type": "mrkdwn"
                        }
                    }
                ]
            }
        );
    }
}
