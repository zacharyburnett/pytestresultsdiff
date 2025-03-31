import argparse
from pathlib import Path
import json


def generate_markdown_table(
    results_diff_json: Path,
    property_names: list[str],
    run_names: list[str] | None = None,
) -> str:
    if run_names is None:
        run_names = ["A", "B"]

    with open(results_diff_json) as results_diff_file:
        results_diff = json.load(results_diff_file)

    header = ["test case"]
    header.extend(
        f"{run_name} {property_name}"
        for run_name in run_names
        for property_name in property_names
    )

    rows = []
    for test_case, properties in results_diff.items():
        row = {header[0]: test_case}
        for property in property_names:
            if property in properties:
                row[property] = properties[property]

        # only append the row
        if len(row) > 1:
            rows.append(row)

    markdown_table_lines = [
        f"| {' | '.join(header)} |",
        f"| {' | '.join(('---' for _ in range(len(header))))} |",
    ]

    for row in rows:
        row_strings = []
        for property, value in row.items():
            if isinstance(value, dict):
                value = data_to_details(value)
            elif isinstance(value, str) and len(value) > 0:
                value = f"`{value}`"
            elif "peakmem" in property:
                value = f"{int(float(value) / 1000000)}MB"
            elif "time" in property:
                value = f"{float(value)}s"
            else:
                value = str(value)
            row_strings.append(value)

        markdown_table_lines.append(f"| {' | '.join(row_strings)} |")

    return "\n".join(markdown_table_lines)


def data_to_details(data: dict) -> str:
    if isinstance(data, dict):
        return "".join(
            f"<details><summary>`{key}`</summary>{data_to_details(value)}</details>"
            for key, value in data.items()
        )
    elif isinstance(data, str):
        if len(data) > 0:
            escaped_data = data.replace("\n", "\\n")
            return f"`{escaped_data}`"
        else:
            return ""
    else:
        return ""


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        prog="generate_markdown_table",
        description="reads pytestresultsdiff JSON and creates a comparison table for the specified properties",
    )

    parser.add_argument("results-diff-json", help="filename of pytestresultsdiff JSON")
    parser.add_argument("properties", nargs="+", help="properties to compare")
    parser.add_argument("--run-names", help="comma-separated list of run names")

    arguments = parser.parse_args()
    run_names = (
        arguments.run_names.split(",") if arguments.run_names is not None else None
    )

    markdown_table = generate_markdown_table(
        getattr(arguments, "results-diff-json"), arguments.properties, run_names
    )

    print(markdown_table)

