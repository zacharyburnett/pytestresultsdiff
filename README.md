# pytestresultsdiff
compare properties of Pytest results

## GitHub Actions workflow

```yaml
      - run: pytest --junitxml=${{ runner.temp }}/currentresults.xml
      - id: pytestresultsdiff
        uses: zacharyburnett/pytestresultsdiff@0.5.0
        with:
          results-xmls: >-
            ${{ runner.temp }}/currentresults.xml
            oldresults.xml
          time-relative-tolerance: 0.1
          #time-absolute-tolerance: 0.1
      - run: echo ${{ steps.pytestresultsdiff.outputs.diff }}
```

> [!TIP]
> Remember to use newline stripping (`>-`) if entering `results.xml` filenames on multiple lines.

> [!TIP]
> `results-xmls` also accepts URLs to XML files.

## console executable

```console
❯ pytestresultsdiff --help
compare properties of Pytest results

Usage: pytestresultsdiff [OPTIONS] [RESULTS_XMLS]...

Arguments:
  [RESULTS_XMLS]...  filenames of `results.xml` to compare

Options:
  -r, --time-relative-tolerance <TIME_RELATIVE_TOLERANCE>
          fractional tolerance for time deviation [default: 0.1]
  -a, --time-absolute-tolerance <TIME_ABSOLUTE_TOLERANCE>
          absolute tolerance (in seconds) for time deviation [default: 0.1]
  -h, --help
          Print help
  -V, --version
          Print version
```
