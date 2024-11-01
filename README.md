# pytestresultsdiff
compare properties of Pytest results

## installation

download a binary from the [releases page](https://github.com/zacharyburnett/pytestresultsdiff/releases)

### from source

```console
git clone https://github.com/zacharyburnett/pytestresultsdiff
cd pytestresultsdiff
cargo build --release
```

## usage 

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
