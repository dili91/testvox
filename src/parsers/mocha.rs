use anyhow::{anyhow, Result};
use serde::Deserialize;

use crate::models::test_result::{TestResult, TestResultBuilder};
use crate::models::test_status::TestStatus;
use crate::parsers::TestParser;

#[derive(Deserialize, Debug)]
struct MochaError {
    message: Option<String>,
    stack: Option<String>,
    // Other fields like 'name', 'expected', 'actual' can be here if needed
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MochaTest {
    title: String,
    full_title: String,
    duration: Option<u64>, // milliseconds
    state: Option<String>,  // "passed", "failed", "pending" / undefined
    err: Option<MochaError>,
    // Present in some formats, indicates if explicitly skipped
    skipped: Option<bool>, 
    // Present in some formats, direct boolean indicators
    pass: Option<bool>,
    fail: Option<bool>,
    pending: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MochaSuite {
    title: String,
    suites: Vec<MochaSuite>,
    tests: Vec<MochaTest>,
    // uuid: String, // Useful for some structures, but not always essential for parsing
    // duration: u64,
    // root: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MochaStats {
    // suites: u32,
    // tests: u32,
    passes: u32,
    pending: u32,
    failures: u32,
    duration: Option<u64>, // milliseconds
}

// This is the top-level structure for Mocha JSON reports
// It might vary slightly based on the specific Mocha reporter used.
// Some reporters have a `results` array containing one or more root suites.
// Others might have top-level `passes`, `failures`, `pending` arrays directly.
#[derive(Deserialize, Debug)]
struct MochaReport {
    stats: MochaStats,
    // Results array containing root suites (common with mochawesome or similar)
    results: Option<Vec<MochaSuite>>,
    // Direct arrays of tests (older or simpler reporters)
    passes: Option<Vec<MochaTest>>,
    failures: Option<Vec<MochaTest>>,
    pending: Option<Vec<MochaTest>>,
    // Some reporters might have suites directly at the root if not using 'results'
    suites: Option<Vec<MochaSuite>>, 
}

pub struct MochaTestParser {
    content: String,
}

impl MochaTestParser {
    pub fn from(content: String) -> Self {
        MochaTestParser { content }
    }

    fn process_suite(suite: &MochaSuite, suite_path: &mut Vec<String>, results: &mut Vec<TestResult>) {
        suite_path.push(suite.title.clone());

        for test in &suite.tests {
            let mut builder = TestResult::builder();
            builder = builder.with_name(test.title.clone());

            // Construct a full suite name if nested
            let current_suite_name = if suite_path.join(" - ").is_empty() || suite.title.is_empty() { // Mocha root suites can have empty titles
                 None
            } else {
                 Some(suite_path.join(" - "))
            };
            if let Some(name) = current_suite_name {
                 builder = builder.with_suite_name(name);   
            }


            if let Some(duration_ms) = test.duration {
                builder = builder.with_execution_time(duration_ms as f32 / 1000.0);
            }

            let status = match test.state.as_deref() {
                Some("passed") => TestStatus::Passed,
                Some("failed") => TestStatus::Failed,
                Some("pending") => TestStatus::Skipped,
                Some(_) | None => { // Infer from boolean flags if state is not definitive
                    if test.pass.unwrap_or(false) { TestStatus::Passed }
                    else if test.fail.unwrap_or(false) { TestStatus::Failed }
                    else if test.pending.unwrap_or(false) { TestStatus::Skipped }
                    else if test.skipped.unwrap_or(false) { TestStatus::Skipped }
                    else { TestStatus::Skipped } // Default for ambiguous cases
                }
            };
            builder = builder.with_status(status.clone());

            if status == TestStatus::Failed {
                if let Some(err) = &test.err {
                    if let Some(message) = &err.message {
                        builder = builder.with_failure_message(message.clone());
                    } else if let Some(stack) = &err.stack { // Fallback to stack if message is empty
                        builder = builder.with_failure_message(stack.clone());
                    }
                }
            }
            results.push(builder.build());
        }

        for sub_suite in &suite.suites {
            Self::process_suite(sub_suite, suite_path, results);
        }
        suite_path.pop();
    }

    // Helper to process flat list of tests (for older Mocha formats)
    fn process_flat_tests(tests: &[MochaTest], status_override: TestStatus, results: &mut Vec<TestResult>) {
        for test in tests {
            let mut builder = TestResult::builder();
            builder = builder.with_name(test.title.clone());
            // Suite name might be part of fullTitle, but parsing it reliably is complex from a flat list.
            // For now, we omit suite_name for tests from flat lists unless fullTitle provides it.
            // Example: "Suite Name Test Name" -> parse out "Suite Name"
            // This can be a future enhancement. For now, full_title is not used directly here.

            if let Some(duration_ms) = test.duration {
                builder = builder.with_execution_time(duration_ms as f32 / 1000.0);
            }

            builder = builder.with_status(status_override.clone());

            if status_override == TestStatus::Failed {
                 if let Some(err) = &test.err {
                    if let Some(message) = &err.message {
                        builder = builder.with_failure_message(message.clone());
                    } else if let Some(stack) = &err.stack {
                         builder = builder.with_failure_message(stack.clone());
                    }
                }
            }
            results.push(builder.build());
        }
    }
}

impl TestParser for MochaTestParser {
    fn parse(&self) -> Result<Vec<TestResult>> {
        let report: MochaReport = serde_json::from_str(&self.content)
            .map_err(|e| anyhow!("Failed to parse Mocha JSON: {}", e))?;

        let mut results: Vec<TestResult> = Vec::new();
        let mut suite_path: Vec<String> = Vec::new();

        // Process tests from the `results` array (nested structure)
        if let Some(report_results) = &report.results {
            for root_suite in report_results {
                // Mocha often has a single root suite, which might have an empty title
                // or represent the file. We pass its title to process_suite.
                Self::process_suite(root_suite, &mut suite_path, &mut results);
                assert!(suite_path.is_empty(), "Suite path should be empty after processing a root suite");
            }
        } 
        // Process tests from top-level `suites` array if `results` is not present
        else if let Some(suites) = &report.suites {
             for suite in suites {
                Self::process_suite(suite, &mut suite_path, &mut results);
                assert!(suite_path.is_empty(), "Suite path should be empty after processing a root suite");
            }
        }

        // Process tests from flat `passes`, `failures`, `pending` arrays
        // This handles older formats or specific reporter configurations.
        // We add these tests only if they weren't already processed via `results` or `suites`
        // (which can be tricky to ensure without UUIDs, so for now, it might add duplicates if a report mixes structures).
        // A more robust solution would involve collecting all test UUIDs from `results`/`suites` and skipping duplicates.
        // For now, we assume these arrays are used if `results` and `suites` are empty or not present.
        if results.is_empty() {
            if let Some(passed_tests) = &report.passes {
                Self::process_flat_tests(passed_tests, TestStatus::Passed, &mut results);
            }
            if let Some(failed_tests) = &report.failures {
                Self::process_flat_tests(failed_tests, TestStatus::Failed, &mut results);
            }
            if let Some(pending_tests) = &report.pending {
                Self::process_flat_tests(pending_tests, TestStatus::Skipped, &mut results);
            }
        }


        if results.is_empty() && report.stats.passes == 0 && report.stats.failures == 0 && report.stats.pending == 0 {
            // No tests found, but also no tests reported in stats. This could be an empty run.
            // Or, the JSON structure is not recognized.
            // For now, return empty results. Consider logging a warning if stats indicate tests should be present.
        }
        
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parse_simple_mocha_report() -> Result<()> {
        let report_content = indoc! {r#"
            {
              "stats": {
                "suites": 1,
                "tests": 3,
                "passes": 1,
                "pending": 1,
                "failures": 1,
                "start": "2023-10-27T10:00:00.000Z",
                "end": "2023-10-27T10:00:01.000Z",
                "duration": 1000
              },
              "results": [
                {
                  "uuid": "suite-uuid-1",
                  "title": "My Test Suite",
                  "fullFile": "",
                  "file": "",
                  "beforeHooks": [],
                  "afterHooks": [],
                  "tests": [
                    {
                      "title": "Test A Passed",
                      "fullTitle": "My Test Suite Test A Passed",
                      "timedOut": false,
                      "duration": 50,
                      "state": "passed",
                      "speed": "fast",
                      "pass": true,
                      "fail": false,
                      "pending": false,
                      "context": null,
                      "code": "",
                      "err": {},
                      "uuid": "test-uuid-1",
                      "parentUUID": "suite-uuid-1",
                      "skipped": false
                    },
                    {
                      "title": "Test B Failed",
                      "fullTitle": "My Test Suite Test B Failed",
                      "timedOut": false,
                      "duration": 20,
                      "state": "failed",
                      "speed": "fast",
                      "pass": false,
                      "fail": true,
                      "pending": false,
                      "context": null,
                      "code": "",
                      "err": {
                        "message": "AssertionError: expected true to be false",
                        "stack": "AssertionError: expected true to be false\n    at Context.<anonymous> (test.js:12:15)"
                      },
                      "uuid": "test-uuid-2",
                      "parentUUID": "suite-uuid-1",
                      "skipped": false
                    },
                    {
                      "title": "Test C Pending",
                      "fullTitle": "My Test Suite Test C Pending",
                      "timedOut": false,
                      "duration": 0,
                      "state": "pending",
                      "speed": null,
                      "pass": false,
                      "fail": false,
                      "pending": true,
                      "context": null,
                      "code": "",
                      "err": {},
                      "uuid": "test-uuid-3",
                      "parentUUID": "suite-uuid-1",
                      "skipped": false
                    }
                  ],
                  "suites": [],
                  "passes": [],
                  "failures": [],
                  "pending": [],
                  "skipped": [],
                  "duration": 70,
                  "root": true,
                  "rootEmpty": false,
                  "_timeout": 2000
                }
              ]
            }
        "#};

        let parser = MochaTestParser::from(report_content.to_string());
        let results = parser.parse()?;

        assert_eq!(results.len(), 3);

        let passed_test = results.iter().find(|r| r.name == "Test A Passed").unwrap();
        assert_eq!(passed_test.status, TestStatus::Passed);
        assert_eq!(passed_test.suite_name, Some("My Test Suite".to_string()));
        assert_eq!(passed_test.execution_time, Some(0.05));

        let failed_test = results.iter().find(|r| r.name == "Test B Failed").unwrap();
        assert_eq!(failed_test.status, TestStatus::Failed);
        assert_eq!(failed_test.suite_name, Some("My Test Suite".to_string()));
        assert_eq!(failed_test.failure_message, Some("AssertionError: expected true to be false".to_string()));
        assert_eq!(failed_test.execution_time, Some(0.02));

        let pending_test = results.iter().find(|r| r.name == "Test C Pending").unwrap();
        assert_eq!(pending_test.status, TestStatus::Skipped);
        assert_eq!(pending_test.suite_name, Some("My Test Suite".to_string()));
        assert_eq!(pending_test.execution_time, Some(0.0));

        Ok(())
    }
    
    #[test]
    fn parse_mocha_report_with_nested_suites() -> Result<()> {
        let report_content = indoc! {r#"
            {
              "stats": { "suites": 2, "tests": 1, "passes": 1, "pending": 0, "failures": 0, "duration": 10 },
              "results": [
                {
                  "title": "Outer Suite",
                  "suites": [
                    {
                      "title": "Inner Suite",
                      "tests": [
                        {
                          "title": "Nested Test Passed",
                          "fullTitle": "Outer Suite Inner Suite Nested Test Passed",
                          "duration": 5,
                          "state": "passed",
                          "pass": true
                        }
                      ],
                      "suites": [],
                      "passes": [], "failures": [], "pending": [], "skipped": [], "duration": 5
                    }
                  ],
                  "tests": [],
                  "passes": [], "failures": [], "pending": [], "skipped": [], "duration": 5, "root": true
                }
              ]
            }
        "#};
        let parser = MochaTestParser::from(report_content.to_string());
        let results = parser.parse()?;
        assert_eq!(results.len(), 1);
        let test = &results[0];
        assert_eq!(test.name, "Nested Test Passed");
        assert_eq!(test.status, TestStatus::Passed);
        assert_eq!(test.suite_name, Some("Outer Suite - Inner Suite".to_string()));
        assert_eq!(test.execution_time, Some(0.005));
        Ok(())
    }

    #[test]
    fn parse_mocha_report_from_flat_arrays() -> Result<()> {
        let report_content = indoc! {r#"
            {
              "stats": { "suites": 0, "tests": 2, "passes": 1, "pending": 0, "failures": 1, "duration": 10 },
              "passes": [
                {
                  "title": "Standalone Passed",
                  "fullTitle": "Standalone Passed",
                  "duration": 3,
                  "state": "passed",
                  "err": {}
                }
              ],
              "failures": [
                {
                  "title": "Standalone Failed",
                  "fullTitle": "Standalone Failed",
                  "duration": 2,
                  "state": "failed",
                  "err": { "message": "It broke" }
                }
              ],
              "pending": []
            }
        "#};
        let parser = MochaTestParser::from(report_content.to_string());
        let results = parser.parse()?;
        assert_eq!(results.len(), 2);

        let passed_test = results.iter().find(|r| r.name == "Standalone Passed").unwrap();
        assert_eq!(passed_test.status, TestStatus::Passed);
        assert_eq!(passed_test.execution_time, Some(0.003));
        
        let failed_test = results.iter().find(|r| r.name == "Standalone Failed").unwrap();
        assert_eq!(failed_test.status, TestStatus::Failed);
        assert_eq!(failed_test.failure_message, Some("It broke".to_string()));
        assert_eq!(failed_test.execution_time, Some(0.002));
        Ok(())
    }
    
    #[test]
    fn parse_mocha_report_empty_results_array() -> Result<()> {
        let report_content = indoc! {r#"
            {
              "stats": { "suites": 0, "tests": 0, "passes": 0, "pending": 0, "failures": 0, "duration": 0 },
              "results": []
            }
        "#};
        let parser = MochaTestParser::from(report_content.to_string());
        let results = parser.parse()?;
        assert_eq!(results.len(), 0);
        Ok(())
    }

    #[test]
    fn parse_mocha_report_no_tests_at_all() -> Result<()> {
        let report_content = indoc! {r#"
            {
              "stats": { "suites": 0, "tests": 0, "passes": 0, "pending": 0, "failures": 0, "duration": 0 }
            }
        "#};
         let parser = MochaTestParser::from(report_content.to_string());
        let results = parser.parse()?;
        assert_eq!(results.len(), 0);
        Ok(())
    }

    #[test]
    fn parse_mocha_report_root_suite_with_empty_title() -> Result<()> {
        // Some reporters (like `mocha-json-stream-reporter`) might produce a root suite with an empty title
        // that directly contains other suites or tests.
        let report_content = indoc! {r#"
            {
              "stats": { "suites": 1, "tests": 1, "passes": 1, "pending": 0, "failures": 0, "duration": 10 },
              "results": [
                {
                  "title": "", 
                  "suites": [
                    {
                      "title": "Actual Suite",
                      "tests": [
                        {
                          "title": "My Test",
                          "fullTitle": "Actual Suite My Test",
                          "duration": 5,
                          "state": "passed",
                          "pass": true
                        }
                      ],
                      "suites": [], "duration": 5
                    }
                  ],
                  "tests": [], "duration": 5, "root": true
                }
              ]
            }
        "#};
        let parser = MochaTestParser::from(report_content.to_string());
        let results = parser.parse()?;
        assert_eq!(results.len(), 1);
        let test = &results[0];
        assert_eq!(test.name, "My Test");
        assert_eq!(test.status, TestStatus::Passed);
        assert_eq!(test.suite_name, Some("Actual Suite".to_string())); // Not " - Actual Suite"
        Ok(())
    }

     #[test]
    fn parse_mocha_report_with_suites_array_instead_of_results() -> Result<()> {
        let report_content = indoc! {r#"
            {
              "stats": { "suites": 1, "tests": 1, "passes": 1, "pending": 0, "failures": 0, "duration": 10 },
              "suites": [ 
                {
                  "title": "Top Suite",
                  "suites": [],
                  "tests": [
                    {
                      "title": "Test in Top Suite",
                      "fullTitle": "Top Suite Test in Top Suite",
                      "duration": 5,
                      "state": "passed",
                      "pass": true
                    }
                  ],
                  "duration": 5
                }
              ]
            }
        "#};
        let parser = MochaTestParser::from(report_content.to_string());
        let results = parser.parse()?;
        assert_eq!(results.len(), 1);
        let test = &results[0];
        assert_eq!(test.name, "Test in Top Suite");
        assert_eq!(test.status, TestStatus::Passed);
        assert_eq!(test.suite_name, Some("Top Suite".to_string()));
        Ok(())
    }
}
