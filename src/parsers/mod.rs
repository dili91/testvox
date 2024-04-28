pub mod junit;

pub mod parsers {
    use crate::TestReport;
    use anyhow::Result;

    pub trait TestParser {
        fn parse(&self) -> Result<TestReport>;
    }
}
