use serde_json::json;
use std::collections::HashMap;

#[derive(clap::ValueEnum, Clone, serde::Serialize)]
#[serde(rename_all = "snake_case")]
enum TestCaseProperty {
    IsPassed,
    IsSkipped,
    Time,
    #[cfg(feature = "peakmem")]
    PeakMem,
    #[cfg(feature = "system-err")]
    SystemErr,
    #[cfg(feature = "system-out")]
    SystemOut,
}

pub fn diff_results(
    results_xmls: Vec<std::path::PathBuf>,
    time_relative_tolerance: f64,
    time_absolute_tolerance: f64,
    #[cfg(feature = "peakmem")] peakmem_relative_tolerance: f64,
    #[cfg(feature = "peakmem")] peakmem_absolute_tolerance: f64,
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

        let times = versions
            .iter()
            .map(|version| version.time)
            .collect::<Vec<f64>>();
        if let Some(reference_time) = times.first() {
            for time in &times {
                if (time - reference_time).abs()
                    > time * time_relative_tolerance + time_absolute_tolerance
                {
                    difference.insert(String::from("time"), json!(times));
                }
            }
        }

        #[cfg(feature = "peakmem")]
        {
            let mut peakmems: Vec<f64> = vec![];
            for version in &versions {
                if let Some(string) = version.properties.hashmap.get("tracked-peakmem") {
                    peakmems.push(string.parse().map_err(|err| format!("{err}"))?);
                }
            }
            if let Some(reference_peakmem) = peakmems.first() {
                for peakmem in &peakmems {
                    if (peakmem - reference_peakmem).abs()
                        > peakmem * peakmem_relative_tolerance + peakmem_absolute_tolerance
                    {
                        difference.insert(String::from("peakmem"), json!(peakmems));
                        break;
                    }
                }
            }
        }

        #[cfg(feature = "system-err")]
        {
            let system_errs = versions
                .iter()
                .map(|version| version.system_err.clone())
                .collect::<Vec<Option<String>>>();
            if let Some(reference_system_err) = system_errs.first() {
                for system_err in &system_errs {
                    if system_err != reference_system_err {
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
                .map(|version| version.system_out.clone())
                .collect::<Vec<Option<String>>>();
            if let Some(reference_system_out) = system_outs.first() {
                for system_out in &system_outs {
                    if system_out != reference_system_out {
                        difference.insert(String::from("system-out"), json!(system_outs));
                        break;
                    }
                }
            }
        }

        #[cfg(feature = "extra-properties")]
        {
            #[allow(unused_mut)]
            let mut standard_properties = vec![
                String::from("classname"),
                String::from("name"),
                String::from("time"),
            ];

            #[cfg(feature = "peakmem")]
            standard_properties.push(String::from("peakmem"));

            #[cfg(feature = "system-err")]
            standard_properties.push(String::from("system-err"));

            #[cfg(feature = "system-out")]
            standard_properties.push(String::from("system-out"));

            let mut extra_properties: HashMap<String, Vec<Option<String>>> = HashMap::new();
            for version in &versions {
                for property in version.properties.hashmap.keys() {
                    if !standard_properties.contains(property) {
                        continue;
                    }
                    extra_properties.insert(property.clone(), vec![None; versions.len()]);
                }
            }
            for version in &versions {
                for (property, value) in &version.properties.hashmap {
                    if !standard_properties.contains(property) {
                        continue;
                    }
                    extra_properties
                        .get_mut(property)
                        .ok_or_else(|| format!("`{property}` not found in extras"))?
                        .push(Some(value.clone()));
                }
            }

            for values in extra_properties.values() {
                let reference_value = &values[0];
                for value in values {
                    if value != reference_value {
                        difference.insert(String::from("extra"), json!(extra_properties.clone()));
                        break;
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
    fn test_diff_results() {
        let data_directory = std::path::Path::new(file!()).parent().unwrap().join("data");
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
                #[cfg(feature = "peakmem")]
                0.1,
                #[cfg(feature = "peakmem")]
                0.1,
            )
            .unwrap(),
            reference_diff
        );
    }
}
