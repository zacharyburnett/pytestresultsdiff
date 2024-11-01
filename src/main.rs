mod diff;
use crate::diff::diff_results;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct PytestResultsDiffCommand {
    /// filenames of `results.xml` to compare
    results_xmls: Vec<std::path::PathBuf>,
}

fn main() {
    let arguments = PytestResultsDiffCommand::parse();
    let test_case_differences = diff_results(arguments.results_xmls);
    println!(
        "{}",
        serde_json::to_string_pretty(&test_case_differences).unwrap()
    );
}
