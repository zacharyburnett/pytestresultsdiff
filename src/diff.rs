use serde_json::json;
use std::collections::HashMap;

#[derive(clap::ValueEnum, Clone, serde::Serialize)]
#[serde(rename_all = "snake_case")]
enum TestCaseProperty {
    IsPassed,
    IsSkipped,
    Time,
    #[cfg(feature = "system-err")]
    SystemErr,
    #[cfg(feature = "system-out")]
    SystemOut,
}

pub fn diff_results(
    results_xmls: Vec<std::path::PathBuf>,
    time_relative_tolerance: Option<f64>,
    time_absolute_tolerance: Option<f64>,
) -> HashMap<String, serde_json::Value> {
    let time_relative_tolerance = time_relative_tolerance.unwrap_or(0.1);
    let time_absolute_tolerance = time_absolute_tolerance.unwrap_or(0.1);

    let mut test_runs: Vec<junit_parser::TestSuites> = vec![];
    for results_xml in results_xmls {
        let suites = junit_parser::from_reader(
            crate::file::read_path(&results_xml.to_str().unwrap().to_owned()).unwrap(),
        );
        match suites {
            Ok(suites) => {
                test_runs.push(suites);
            }
            Err(error) => panic!("{}", error.to_string()),
        }
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
            if (time - reference_time).abs()
                > time * time_relative_tolerance + time_absolute_tolerance
            {
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

        #[cfg(feature = "system-err")]
        {
            let system_errs = versions
                .iter()
                .map(|version| version.system_err.clone())
                .collect::<Vec<Option<String>>>();
            let reference_system_err = &system_errs[0];
            for system_err in &system_errs {
                if system_err != reference_system_err {
                    difference.insert(String::from("system-err"), json!(system_errs));
                    break;
                }
            }
        }

        #[cfg(feature = "system-out")]
        {
            let system_outs = versions
                .iter()
                .map(|version| version.system_out.clone())
                .collect::<Vec<Option<String>>>();
            let reference_system_out = &system_outs[0];
            for system_out in &system_outs {
                if system_out != reference_system_out {
                    difference.insert(String::from("system-out"), json!(system_outs));
                    break;
                }
            }
        }

        #[cfg(feature = "extra-properties")]
        {
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
        }

        if difference.len() > 0 {
            test_case_differences.insert(name, serde_json::Value::Object(difference));
        }
    }
    test_case_differences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_results() {
        let data_directory = std::path::Path::new(file!()).parent().unwrap().join("data");
        let reference_diff = serde_json::from_reader(std::io::BufReader::new(
            std::fs::File::open(data_directory.join("diff.json")).unwrap(),
        ))
        .unwrap();

        assert_eq!(
            diff_results(
                vec![
                    data_directory.join("romancal_24Q4_B15.0.0_results-Linux-x64-py3.11.xml"),
                    data_directory.join("romancal_nightly_results-Linux-x64-py3.11.xml")
                ],
                Some(0.1),
                Some(0.1),
            ),
            reference_diff
        );
    }
}
