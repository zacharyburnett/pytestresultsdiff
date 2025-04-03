use serde_json::json;

use std::collections::HashMap;

#[derive(clap::ValueEnum, Clone, serde::Serialize)]
#[serde(rename_all = "snake_case")]
enum TestCaseProperty {
    IsPassed,
    IsSkipped,
    Time,
    #[cfg(feature = "extra-properties")]
    PeakMem,
    #[cfg(feature = "system-err")]
    SystemErr,
    #[cfg(feature = "system-out")]
    SystemOut,
}

/// peakmem absolute tolerance is in MB
pub fn diff_results(
    results_xmls: Vec<std::path::PathBuf>,
    time_relative_tolerance: f64,
    time_absolute_tolerance: f64,
    #[cfg(feature = "extra-properties")] peakmem_relative_tolerance: f64,
    #[cfg(feature = "extra-properties")] peakmem_absolute_tolerance: f64,
) -> Result<HashMap<String, serde_json::Value>, String> {
    let mut test_runs: Vec<junit_parser::TestSuites> = vec![];
    for results_xml in results_xmls {
        let suites = junit_parser::from_reader(
            crate::file::read_path(&results_xml.to_str().unwrap().to_owned())
                .map_err(|err| format!("{err}"))?,
        );
        test_runs.push(suites.map_err(|err| format!("{err}"))?);
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

        let statuses = versions
            .iter()
            .map(|version| &version.status)
            .collect::<Vec<&junit_parser::TestStatus>>();
        if let Some(reference_status) = statuses.first() {
            for status in &statuses {
                if status.is_success() != reference_status.is_success()
                    || status.is_error() != reference_status.is_error()
                    || status.is_failure() != reference_status.is_failure()
                    || status.is_skipped() != reference_status.is_skipped()
                {
                    // stop at the first difference and insert the entire list of versions
                    difference.insert(String::from("status"), json!(statuses));
                    break;
                }
            }
        }

        let times = versions
            .iter()
            .map(|version| version.time)
            .collect::<Vec<f64>>();
        if let Some(reference_time) = times.first() {
            for time in &times {
                if (time - reference_time).abs()
                    > time * time_relative_tolerance + time_absolute_tolerance
                {
                    // stop at the first difference and insert the entire list of versions
                    difference.insert(String::from("time"), json!(times));
                    break;
                }
            }
        }

        #[cfg(feature = "system-err")]
        {
            let system_errs = versions
                .iter()
                .map(|version| &version.system_err)
                .collect::<Vec<&Option<String>>>();
            if let Some(reference_system_err) = system_errs.first() {
                for system_err in &system_errs {
                    if system_err != reference_system_err {
                        // stop at the first difference and insert the entire list of versions
                        difference.insert(String::from("system-err"), json!(system_errs));
                        break;
                    }
                }
            }
        }

        #[cfg(feature = "system-out")]
        {
            let system_outs = versions
                .iter()
                .map(|version| &version.system_out)
                .collect::<Vec<&Option<String>>>();
            if let Some(reference_system_out) = system_outs.first() {
                for system_out in &system_outs {
                    if system_out != reference_system_out {
                        // stop at the first difference and insert the entire list of versions
                        difference.insert(String::from("system-out"), json!(system_outs));
                        break;
                    }
                }
            }
        }

        #[cfg(feature = "extra-properties")]
        {
            let nonextra_properties = vec![
                String::from("classname"),
                String::from("name"),
                String::from("time"),
                String::from("system-err"),
                String::from("system-out"),
                String::from("tracked-time"),
                String::from("tracked-peakmem"),
            ];

            let mut tracked_times: Vec<f64> = vec![];
            for version in &versions {
                if let Some(string) = version.properties.hashmap.get("tracked-time") {
                    tracked_times.push(string.parse().map_err(|err| format!("{err}"))?);
                }
            }
            if let Some(reference_time) = tracked_times.first() {
                for tracked_time in &tracked_times {
                    if (tracked_time - reference_time).abs()
                        > tracked_time * time_relative_tolerance + time_absolute_tolerance
                    {
                        // stop at the first difference and insert the entire list of versions
                        difference.insert(String::from("time"), json!(tracked_times));
                        break;
                    }
                }
            }

            let mut tracked_peakmems: Vec<f64> = vec![];
            for version in &versions {
                if let Some(string) = version.properties.hashmap.get("tracked-peakmem") {
                    tracked_peakmems.push(string.parse().map_err(|err| format!("{err}"))?);
                }
            }
            if let Some(reference_peakmem) = tracked_peakmems.first() {
                for peakmem in &tracked_peakmems {
                    if (peakmem - reference_peakmem).abs()
                        > peakmem * peakmem_relative_tolerance
                            + (peakmem_absolute_tolerance * 1000000.0)
                    {
                        // stop at the first difference and insert the entire list of versions
                        difference.insert(String::from("peakmem"), json!(tracked_peakmems));
                        break;
                    }
                }
            }

            let mut extra_properties: HashMap<String, Vec<Option<String>>> = HashMap::new();
            for version in &versions {
                for (index, (property, value)) in version.properties.hashmap.iter().enumerate() {
                    if !nonextra_properties.contains(property) {
                        if !extra_properties.contains_key(property) {
                            // instatiate extra property versions as a list of Nones, since some tests may not have the property
                            let mut values = if index > 0 { vec![None; index] } else { vec![] };
                            values.push(Some(value.to_owned()));
                            extra_properties.insert(property.to_owned(), values);
                        } else {
                            if let Some(values) = extra_properties.get_mut(property) {
                                values.push(Some(value.to_owned()));
                            }
                        }
                    }
                }
            }

            for (property, values) in &extra_properties {
                if let Some(reference_property) = values.first() {
                    for value in values {
                        if value != reference_property {
                            // stop at the first difference and insert the entire list of extra properties
                            difference.insert(property.to_owned(), json!(values));
                            break;
                        }
                    }
                }
            }
        }

        if difference.len() > 0 {
            test_case_differences.insert(name, serde_json::Value::Object(difference));
        }
    }

    Ok(test_case_differences)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_results_peakmem() {
        let data_directory = std::path::Path::new(file!())
            .parent()
            .unwrap()
            .join("data/peakmem/");
        let reference_diff = serde_json::from_reader(std::io::BufReader::new(
            std::fs::File::open(data_directory.join("memdiff.json")).unwrap(),
        ))
        .unwrap();

        assert_eq!(
            diff_results(
                vec![
                    data_directory.join("main.xml"),
                    data_directory.join("pr.xml")
                ],
                0.1,
                0.1,
                #[cfg(feature = "extra-properties")]
                0.1,
                #[cfg(feature = "extra-properties")]
                0.1,
            )
            .unwrap(),
            reference_diff
        );
    }

    #[test]
    fn test_diff_results_time() {
        let data_directory = std::path::Path::new(file!())
            .parent()
            .unwrap()
            .join("data/time/");
        let reference_diff = serde_json::from_reader(std::io::BufReader::new(
            std::fs::File::open(data_directory.join("timediff.json")).unwrap(),
        ))
        .unwrap();

        assert_eq!(
            diff_results(
                vec![
                    data_directory.join("romancal_24Q4_B15.0.0_results-Linux-x64-py3.11.xml"),
                    data_directory.join("romancal_nightly_results-Linux-x64-py3.11.xml")
                ],
                0.1,
                0.1,
                #[cfg(feature = "extra-properties")]
                0.1,
                #[cfg(feature = "extra-properties")]
                0.1,
            )
            .unwrap(),
            reference_diff
        );
    }
}
