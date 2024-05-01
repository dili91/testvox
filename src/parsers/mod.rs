pub mod junit;

use crate::TestResult;
use anyhow::Result;

pub trait TestParser {
    fn parse(&self) -> Result<Vec<TestResult>>;
}
