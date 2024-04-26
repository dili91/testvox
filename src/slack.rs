use serde::Serialize;

use crate::{JunitTestReport, MarkdownTestResult};

#[derive(Serialize)]
pub struct SlackReport {
    #[serde(skip)]
    pub title: String,
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

// TODO: this trait should return a builder instead. title should be a property of the builder,
// as well as flags to understand what statuses to display
impl From<JunitTestReport> for SlackReport {
    fn from(report: JunitTestReport) -> Self {
        let header_block = Block::Header {
            text: PlainText {
                text:
                    ":java::fire: Acceptance tests are failing in Sandbox on the Java backend library!"
                        .to_string(),
                emoji: true,
            },
        };

        let mut section_blocks: Vec<Block> = report
            .test_results
            .into_iter()
            //.filter(|t| t.status != TestStatus::Passed) TODO: control this via params
            .map(|t| {
                vec![
                    Block::Divider,
                    Block::Section {
                        text: MarkdownText {
                            text: t.to_string(),
                        },
                    },
                ]
            })
            .flatten()
            .collect();

        let mut blocks = vec![header_block];
        blocks.append(&mut section_blocks);

        Self {
            title:
                ":java::fire: Acceptance tests are failing in Sandbox on the Java backend library!"
                    .to_string(),
            blocks,
        }
    }
}
