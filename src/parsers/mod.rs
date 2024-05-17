/// module that includes models and logic to parse Junit results
pub mod junit;

use crate::models::test_result::TestResult;
use anyhow::Result;

/// Generic trait describing all test parser's common methods
pub trait TestParser {
    /// Method that parse test results into a list of domain `TestResult` objects 
    fn parse(&self) -> Result<Vec<TestResult>>;
}
