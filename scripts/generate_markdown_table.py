import argparse
import os
import json
import sys


KNOWN_NUMERICAL_PROPERTIES = [
    "time",
    "peakmem",
]


def sort_table_differences(
    rows: list[dict[str, str | list[str | float | int]]],
) -> list[dict[str, str | list[str | float | int]]]:
    test_cases = [
        [row[property] for property in KNOWN_NUMERICAL_PROPERTIES if property in row]
        for row in rows
    ]
    differences = []
    for test_case in test_cases:
        test_case_differences = []
        for property_values in test_case:
            if property_values is None:
                property_difference is None
            else:
                property_values = [
                    value for value in property_values if value is not None
                ]
                if len(property_values) > 0:
                    property_difference = property_values[0]
                    for index in range(1, len(property_values)):
                        property_difference -= property_values[index]
            test_case_differences.append(property_difference)
        differences.append(tuple(test_case_differences))
    return [rows[differences.index(difference)] for difference in sorted(differences)]


def generate_markdown_table(
    results_diff_json: os.PathLike | dict,
    property_names: list[str],
    run_names: list[str] | None = None,
) -> str | None:
    if run_names is None:
        run_names = ["A", "B"]

    if not isinstance(results_diff_json, dict):
        with open(results_diff_json) as results_diff_file:
            results_diff = json.load(results_diff_file)
    else:
        results_diff = results_diff_json

    header = ["test case"]
    header.extend(
        f"{run_name} {property_name}"
        for property_name in property_names
        for run_name in run_names
    )

    markdown_table_lines = [
        f"| {' | '.join(header)} |",
        f"| {' | '.join(('---' for _ in range(len(header))))} |",
    ]

    rows = []
    for test_case, properties in results_diff.items():
        row = {
            header[0]: test_case,
            **{
                property: properties[property] if property in properties else None
                for property in property_names
            },
        }

        # only append the row
        if not all(entry is None for entry in list(row.values())[1:]):
            rows.append(row)

    # sort rows by difference
    rows = sort_table_differences(
        rows,
    )

    for row in rows:
        row_as_strings: list[str] = []
        for property, entry in row.items():
            if entry is None:
                row_as_strings.extend("" for _ in run_names)
            elif isinstance(entry, list):
                if "peakmem" in property:
                    # only append value if there are differences greater than the displayed number of digits
                    entry = [
                        # peakmem comes in bytes
                        round(float(value) / 1000000)
                        for value in entry
                    ]
                    if any(entry[index] != entry[0] for index in range(1, len(entry))):
                        row_as_strings.extend(f"`{value:.0f}MB`" for value in entry)
                elif "time" in property:
                    # only append value if there are differences greater than the displayed number of digits
                    entry = [round(float(value), ndigits=1) for value in entry]
                    if any(entry[index] != entry[0] for index in range(1, len(entry))):
                        # time comes in seconds
                        row_as_strings.extend(f"`{value:.1f}s`" for value in entry)
                elif "status" in property:
                    row_as_strings.extend(
                        " ".join(value.keys())
                        if isinstance(value, dict)
                        else f"{value}"
                        for value in entry
                    )
                else:
                    row_as_strings.extend(
                        data_to_details(value)
                        if isinstance(value, dict)
                        else f"{value}"
                        for value in entry
                    )
            else:
                if property == "test case":
                    # format test case name as inline code
                    entry = f"`{entry}`"
                row_as_strings.append(entry)

        if any(value != "" for value in row_as_strings[1:]):
            markdown_table_lines.append(f"| {' | '.join(row_as_strings)} |")

    if len(markdown_table_lines) <= 2:
        markdown_table_lines.append(
            f"| _no change_ | {' | '.join('' for _ in header[1:])} |"
        )

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

    parser.add_argument(
        "results-diff-json",
        help="filename of pytestresultsdiff JSON, or - to read from stdin",
    )
    parser.add_argument("properties", nargs="+", help="properties to compare")
    parser.add_argument("--run-names", help="comma-separated list of run names")

    arguments = parser.parse_args()

    results_diff_json = getattr(arguments, "results-diff-json")
    if results_diff_json == "-":
        results_diff_json = json.loads(sys.stdin.read())
        print(results_diff_json)

    run_names = (
        arguments.run_names.split(",") if arguments.run_names is not None else None
    )

    markdown_table = generate_markdown_table(
        results_diff_json, arguments.properties, run_names
    )

    if markdown_table is not None:
        print(markdown_table)
