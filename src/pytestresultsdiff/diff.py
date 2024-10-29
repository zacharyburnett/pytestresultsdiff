from enum import Enum
from pathlib import Path
from difflib import Differ

import typer
from junitparser import JUnitXml


class TestCaseProperty(Enum):
    is_passed = "is_passed"
    is_skipped = "is_skipped"
    time = "time"
    system_err = "system_err"
    system_out = "system_out"


def diff(results_xml_1: Path, results_xml_2: Path, property: TestCaseProperty):
    results_1 = JUnitXml.fromfile(results_xml_1)
    results_2 = JUnitXml.fromfile(results_xml_2)

    differ = (
        Differ()
        if property in (TestCaseProperty.system_err, TestCaseProperty.system_out)
        else None
    )

    for suite_1 in results_1:
        for suite_2 in results_2:
            if suite_2.name == suite_1.name:
                for case_1 in suite_1:
                    for case_2 in suite_2:
                        if (
                            case_2.classname == case_1.classname
                            and case_2.name == case_1.name
                        ):
                            property_1 = getattr(case_1, property.value)
                            property_2 = getattr(case_2, property.value)
                            if property_1 != property_2:
                                if property == TestCaseProperty.time:
                                    print(
                                        f"{case_1.classname}.{case_1.name}: {property_1} != {property_2} ({round(property_2 - property_1, 3)})"
                                    )
                                elif property in (
                                    TestCaseProperty.system_err,
                                    TestCaseProperty.system_out,
                                ):
                                    print(
                                        f"{case_1.classname}.{case_1.name}: \n{''.join(differ.compare(property_1.splitlines(keepends=True), property_2.splitlines(keepends=True)))}"
                                    )
                                else:
                                    print(
                                        f"{case_1.classname}.{case_1.name}: {property_1} != {property_2}"
                                    )


def main():
    typer.run(diff)
