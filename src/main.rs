mod diff;
mod file;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct PytestResultsDiffCommand {
    /// filenames of `results.xml` to compare
    results_xmls: Vec<std::path::PathBuf>,
    /// fractional tolerance for time deviation
    #[clap(long, short = 'r', default_value_t = 0.1)]
    peakmem_relative_tolerance: f64,
    /// absolute tolerance (in seconds) for time deviation
    #[clap(long, short = 'a', default_value_t = 0.1)]
    peakmem_absolute_tolerance: f64,
}

fn main() {
    let arguments = PytestResultsDiffCommand::parse();
    let test_case_differences = crate::diff::diff_results(
        arguments.results_xmls,
        Some(arguments.peakmem_relative_tolerance),
        Some(arguments.peakmem_absolute_tolerance),
    )
    .unwrap();
    println!(
        "{}",
        serde_json::to_string_pretty(&test_case_differences).unwrap()
    );
}
