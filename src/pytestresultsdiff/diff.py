import json
from difflib import Differ
from enum import Enum
from pathlib import Path

import typer
from junitparser import JUnitXml


class TestCaseProperty(Enum):
    is_passed = "is_passed"
    is_skipped = "is_skipped"
    time = "time"
    # system_err = "system_err"
    # system_out = "system_out"


def diff(
    results_xml_1: Path,
    results_xml_2: Path,
    # property: TestCaseProperty,
    output_filename: Path = None,
):
    results_1 = JUnitXml.fromfile(results_xml_1)
    results_2 = JUnitXml.fromfile(results_xml_2)

    # differ = Differ() if property in (TestCaseProperty.system_err, TestCaseProperty.system_out) else None

    differences = {}
    for suite_1 in results_1:
        for suite_2 in results_2:
            if suite_2.name == suite_1.name:
                for case_1 in suite_1:
                    case_1_name = f"{case_1.classname}.{case_1.name}"
                    for case_2 in suite_2:
                        case_2_name = f"{case_1.classname}.{case_1.name}"
                        if case_1_name == case_2_name:
                            for property in TestCaseProperty:
                                case_1_property = getattr(case_1, property.value)
                                case_2_property = getattr(case_2, property.value)
                                if case_1_property != case_2_property:
                                    if case_1_name not in differences:
                                        differences[case_1_name] = {}
                                    differences[case_1_name][property.value] = (
                                        case_1_property,
                                        case_2_property,
                                    )

    if output_filename is not None:
        with open(output_filename, "w") as output_file:
            json.dump(differences, output_file, indent="  ")
    else:
        print(json.dumps(differences, indent="  "))


def main():
    typer.run(diff)
