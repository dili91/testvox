pub mod junit;

pub mod parsers {
    use crate::TestResult;
    use anyhow::Result;

    pub trait TestParser {
        fn parse(&self) -> Result<Vec<TestResult>>;
    }
}
