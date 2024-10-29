# pytestresultsdiff
compare properties of Pytest results

[![PyPI - Version](https://img.shields.io/pypi/v/pytestresultsdiff.svg)](https://pypi.org/project/pytestresultsdiff)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/pytestresultsdiff.svg)](https://pypi.org/project/pytestresultsdiff)

```console
pip install pytestresultsdiff
```

```console
❯ pytestresultsdiff --help
                                                                                 
 Usage: pytestresultsdiff [OPTIONS] RESULTS_XML_1 RESULTS_XML_2 PROPERTY:{is_pa  
                          ssed|is_skipped|time|system_err|system_out}            
                                                                                 
╭─ Arguments ───────────────────────────────────────────────────────────────────╮
│ *    results_xml_1      PATH                       [default: None] [required] │
│ *    results_xml_2      PATH                       [default: None] [required] │
│ *    property           PROPERTY:{is_passed|is_sk  [default: None] [required] │
│                         ipped|time|system_err|sys                             │
│                         tem_out}                                              │
╰───────────────────────────────────────────────────────────────────────────────╯
╭─ Options ─────────────────────────────────────────────────────────────────────╮
│ --help          Show this message and exit.                                   │
╰───────────────────────────────────────────────────────────────────────────────╯
```
