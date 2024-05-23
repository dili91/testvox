use crate::models::{
    test_report::{PrettyPrint, ReportBuilder},
    test_result::TestResult,
};
use serde::Serialize;
use url::Url;

/// Struct that defines a Slack report
#[derive(Serialize)]
pub struct SlackReport {
    pub blocks: Vec<Block>,
}

/// Enum that defines the variant of Block objects in Slack
#[derive(Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Block {
    Header { text: PlainText },
    Section { text: MarkdownText },
    Actions { elements: Vec<Element> },
    Divider,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Element {
    Button { text: PlainText, url: Url },
}

/// Struct that represents Slack's block JSON types
#[derive(Serialize)]
#[serde(tag = "type", rename = "plain_text")]
pub struct PlainText {
    pub text: String,
    pub emoji: bool,
}

/// Defines a markdown test object following Slack syntax
#[derive(Serialize)]
#[serde(tag = "type", rename = "mrkdwn")]
pub struct MarkdownText {
    pub text: String,
}

/// Pretty print implementation for the Slack report type
impl PrettyPrint for SlackReport {
    fn to_string_pretty(&self) -> String {
        serde_json::to_string_pretty(&self).expect("unable to serialize report to JSON")
    }
}

/// Turns test results into a Slack report
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

        if let Some(link) = value.link {
            let mut link_blocks = vec![
                Block::Divider,
                Block::Actions {
                    elements: vec![Element::Button {
                        text: PlainText {
                            text: ":link:  View details".to_string(),
                            emoji: true,
                        },
                        url: link,
                    }],
                },
            ];
            blocks.append(&mut link_blocks);
        }

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
    use crate::models::{
        test_report::{PrettyPrint, ReportBuilder},
        test_result::TestResult,
        test_status::TestStatus,
    };
    use assert_json::assert_json;
    use url::Url;

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
        let link = Url::parse("http://localhost/run/123").expect("unable to parse url");

        let report: SlackReport = ReportBuilder::new()
            .with_title(title.to_string())
            .include_passed()
            .include_skipped()
            .with_test_results(vec![
                test_failed.clone(),
                test_passed.clone(),
                test_skipped.clone(),
            ])
            .with_link(link)
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
