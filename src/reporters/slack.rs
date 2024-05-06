use serde::Serialize;

use crate::{MarkdownTestResult, TestResult};

#[derive(Serialize)]
pub struct SlackReport {
    pub blocks: Vec<Block>,
}

impl SlackReport {
    pub fn builder() -> SlackReportBuilder {
        SlackReportBuilder::default()
    }
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

#[derive(Default)]
pub struct SlackReportBuilder {
    title: String,
    test_results: Vec<TestResult>,
}

impl SlackReportBuilder {
    pub fn with_title(mut self, title: String) -> SlackReportBuilder {
        self.title = title;
        self
    }

    pub fn with_test_results(mut self, test_blocks: Vec<TestResult>) -> SlackReportBuilder {
        self.test_results = test_blocks;
        self
    }

    pub fn build(self) -> SlackReport {
        let header_block = Block::Header {
            text: PlainText {
                text: self.title,
                emoji: true,
            },
        };

        let mut section_blocks: Vec<Block> = self
            .test_results
            .into_iter()
            .flat_map(|t| {
                vec![
                    Block::Divider,
                    Block::Section {
                        text: MarkdownText {
                            text: t.to_string(),
                        },
                    },
                ]
            })
            .collect();

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
                    text: test_result.to_string(),
                },
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_create_report_in_slack_format() {
        todo!()
    }
}
