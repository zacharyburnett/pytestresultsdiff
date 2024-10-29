from pytestresultsdiff.diff import diff, TestCaseProperty
from pathlib import Path
import pytest

DATA_DIRECTORY = Path(__file__).parent / "data"


@pytest.mark.parametrize("property", list(TestCaseProperty))
def test_diff(property):
    diff(
        DATA_DIRECTORY / "romancal_24Q4_B15.0.0_results-Linux-x64-py3.11.xml",
        DATA_DIRECTORY / "romancal_nightly_results-Linux-x64-py3.11.xml",
        property,
    )
