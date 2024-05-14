pub mod junit;

use anyhow::Result;

use crate::models::test_result::TestResult;

pub trait TestParser {
    fn parse(&self) -> Result<Vec<TestResult>>;
}
