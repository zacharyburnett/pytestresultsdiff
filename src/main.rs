use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use clap::Parser;

#[derive(clap::ValueEnum, Clone, serde::Serialize)]
#[serde(rename_all = "snake_case")]
enum TestCaseProperty {
    IsPassed,
    IsSkipped,
    Time,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct PytestResultsDiffCommand {
    /// filenames of `results.xml` to compare
    results_xmls: Vec<std::path::PathBuf>,
}

fn main() {
    let arguments = PytestResultsDiffCommand::parse();

    let mut test_runs: Vec<junit_parser::TestSuites> = vec![];
    for results_xml in arguments.results_xmls {
        test_runs.push(
            junit_parser::from_reader(BufReader::new(File::open(results_xml).unwrap())).unwrap(),
        );
    }

    let mut test_cases: HashMap<String, Vec<junit_parser::TestCase>> = HashMap::new();
    for test_run in test_runs {
        for test_suite in test_run.suites {
            for test_case in test_suite.cases {
                if !test_cases.contains_key(&test_case.name) {
                    test_cases.insert(test_case.name.clone(), vec![]);
                }
                test_cases.get_mut(&test_case.name).unwrap().push(test_case);
            }
        }
    }

    let mut test_case_differences: HashMap<String, serde_json::Value> = HashMap::new();
    for (name, versions) in test_cases {
        let mut difference: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();

        let times = versions
            .iter()
            .map(|version| version.time)
            .collect::<Vec<f64>>();
        let reference_time = &times[0];
        for time in &times {
            if time != reference_time {
                difference.insert(String::from("time"), json!(times));
                break;
            }
        }

        let statuses = versions
            .iter()
            .map(|version| version.status.clone())
            .collect::<Vec<junit_parser::TestStatus>>();
        let reference_status = &statuses[0];
        for status in &statuses {
            if status.is_success() != reference_status.is_success()
                || status.is_error() != reference_status.is_error()
                || status.is_failure() != reference_status.is_failure()
                || status.is_skipped() != reference_status.is_skipped()
            {
                difference.insert(String::from("status"), json!(statuses));
                break;
            }
        }

        let mut extras: HashMap<String, Vec<Option<String>>> = HashMap::new();
        let standard = vec![
            String::from("time"),
            String::from("classname"),
            String::from("name"),
            String::from("system-err"),
            String::from("system-out"),
        ];
        for version in &versions {
            for property in version.properties.hashmap.keys() {
                if !standard.contains(property) {
                    continue;
                }
                extras.insert(property.clone(), vec![None; versions.len()]);
            }
        }
        for version in &versions {
            for (property, value) in &version.properties.hashmap {
                if !standard.contains(property) {
                    continue;
                }
                extras.get_mut(property).unwrap().push(Some(value.clone()));
            }
        }

        for values in extras.values() {
            let reference_value = &values[0];
            for value in values {
                if value != reference_value {
                    difference.insert(String::from("extra"), json!(extras.clone()));
                    break;
                }
            }
        }

        if difference.len() > 0 {
            test_case_differences.insert(name, serde_json::Value::Object(difference));
        }
    }

    println!(
        "{}",
        serde_json::to_string_pretty(&test_case_differences).unwrap()
    );
}
