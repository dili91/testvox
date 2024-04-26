use serde::Serialize;

#[derive(Serialize)]
pub struct Report {
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
