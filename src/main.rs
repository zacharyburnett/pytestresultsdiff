mod diff;
mod file;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct PytestResultsDiffCommand {
    /// filenames of `results.xml` to compare
    results_xmls: Vec<std::path::PathBuf>,
    /// fractional tolerance for time deviation
    #[clap(long, short = 't', default_value_t = 0.0)]
    time_relative_tolerance: f64,
    /// absolute tolerance (in seconds) for time deviation
    #[clap(long, short = 'T', default_value_t = 0.1)]
    time_absolute_tolerance: f64,
    /// fractional tolerance for peakmem deviation
    #[cfg(feature = "extra-properties")]
    #[clap(long, short = 'm', default_value_t = 0.0)]
    peakmem_relative_tolerance: f64,
    /// absolute tolerance (in MB) for peakmem deviation
    #[cfg(feature = "extra-properties")]
    #[clap(long, short = 'M', default_value_t = 1.0)]
    peakmem_absolute_tolerance: f64,
}

fn main() {
    let arguments = PytestResultsDiffCommand::parse();
    let test_case_differences = crate::diff::diff_results(
        arguments.results_xmls,
        arguments.time_relative_tolerance,
        arguments.time_absolute_tolerance,
        #[cfg(feature = "extra-properties")]
        arguments.peakmem_relative_tolerance,
        #[cfg(feature = "extra-properties")]
        arguments.peakmem_absolute_tolerance,
    )
    .unwrap();
    println!(
        "{}",
        serde_json::to_string_pretty(&test_case_differences).unwrap()
    );
}
