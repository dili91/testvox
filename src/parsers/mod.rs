pub mod junit;

use crate::models::test_result::TestResult;
use anyhow::Result;

pub trait TestParser {
    fn parse(&self) -> Result<Vec<TestResult>>;
}
