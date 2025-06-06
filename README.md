# pytestresultsdiff
[![build](https://github.com/zacharyburnett/pytestresultsdiff/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/zacharyburnett/pytestresultsdiff/actions/workflows/build.yml)
[![test](https://github.com/zacharyburnett/pytestresultsdiff/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/zacharyburnett/pytestresultsdiff/actions/workflows/test.yml)

compare properties of Pytest results

## `zacharyburnett/pytestresultsdiff` GitHub Actions workflow

```yaml
      - run: pytest --junitxml=${{ runner.temp }}/currentresults.xml
      - id: pytestresultsdiff
        uses: zacharyburnett/pytestresultsdiff@1.1.1
        with:
          results-xmls: >-
            oldresults.xml
            ${{ runner.temp }}/currentresults.xml
          #time-relative-tolerance: 0.0
          time-absolute-tolerance: 0.1 # seconds
          #peakmem-relative-tolerance: 0.0
          peakmem-absolute-tolerance: 1.0 # megabytes
          summary-table-properties: status time peakmem
          summary-table-run-names: old,current
          output-file: ${{ runner.temp }}/resultsdiff.json
          #features: extra-properties,url,system-err,system-out
      - run: echo ${{ steps.pytestresultsdiff.outputs.diff }}
      - run: cat ${{ runner.temp }}/resultsdiff.json
```

> [!TIP]
> Remember to use newline stripping (`>-`) if entering `results.xml` filenames on multiple lines.

> [!TIP]
> `results-xmls` also accepts URLs to XML files with the `url` feature.

```yaml
inputs:
  results-xmls:
    description: "space-separated filenames of `results.xml` to compare"
    required: true
  time-relative-tolerance:
    description: "fractional tolerance for time deviation [default: 0.0]" 
    required: false
    default: "0.0"
  time-absolute-tolerance:
    description: "absolute tolerance (in seconds) for time deviation [default: 0.1]"
    required: false
    default: "0.1"
  peakmem-relative-tolerance:
    description: "fractional tolerance for peakmem deviation [default: 0.0]" 
    required: false
    default: "0.0"
  peakmem-absolute-tolerance:
    description: "absolute tolerance (in MB) for peakmem deviation [default: 1.0]"
    required: false
    default: "1.0"
  features:
    description: "comma-separated list of crate features: `extra-properties,url,system-err,system-out`"
    required: false
    default: "extra-properties"
  summary-table-properties:
    description: "space-separated list of properties with which to build a summary table"
    required: false
    default: ""
  summary-table-run-names:
    description: "comma-separated list of run names for the summary table header"
    required: false
    default: "A,B"
  output-file:
    description: "filename of JSON to write to"
    required: false
    default: ""
```

## `pytestresultsdiff` console executable

```
compare properties of Pytest results

Usage: pytestresultsdiff [OPTIONS] [RESULTS_XMLS]...

Arguments:
  [RESULTS_XMLS]...  filenames of `results.xml` to compare

Options:
  -t, --time-relative-tolerance <TIME_RELATIVE_TOLERANCE>
          fractional tolerance for time deviation [default: 0]
  -T, --time-absolute-tolerance <TIME_ABSOLUTE_TOLERANCE>
          absolute tolerance (in seconds) for time deviation [default: 0.1]
  -m, --peakmem-relative-tolerance <PEAKMEM_RELATIVE_TOLERANCE>
          fractional tolerance for peakmem deviation [default: 0]
  -M, --peakmem-absolute-tolerance <PEAKMEM_ABSOLUTE_TOLERANCE>
          absolute tolerance (in MB) for peakmem deviation [default: 1]
  -h, --help
          Print help
  -V, --version
          Print version
```

```shell
pytestresultsdiff src/data/time/romancal_24Q4_B15.0.0_results-Linux-x64-py3.11.xml src/data/time/romancal_nightly_results-Linux-x64-py3.11.xml
```
```json
{
  "romancal.regtest.test_wfi_pipeline::test_level2_image_processing_pipeline": {
    "status": [
      "Success",
      {
        "Failure": {
          "failure_type": "",
          "message": "AssertionError: \n  Diff report for:\n      result file: /runner/_work/_temp/pytest_basetemp/popen-gw1/test_level2_image_processing_p0/r0000101001001001001_01101_0001_WFI01_cal_repoint.asdf\n          model type: ImageModel\n      truth file: /runner/_work/_temp/pytest_basetemp/popen-gw1/test_level2_image_processing_p0/truth/r0000101001001001001_01101_0001_WFI01_cal_repoint.asdf\n          model type: ImageModel\n  \n  {'arrays_differ': {\"root['roman']['data']\": {'abs_diff': <Quantity 32778.504 DN / s>,\n                                               'n_diffs': 471,\n                                               'worst_abs_diff': {'index': (2955,\n                                                                            3321),\n                                                                  'value': <Quantity 7411.377 DN / s>},\n                                               'worst_fractional_diff': {'index': (2492,\n                                                                                   1865),\n                                                                         'value': <Quantity 863315.94>}},\n                     \"root['roman']['dq']\": {'abs_diff': 12884901877,\n                                             'n_diffs': 4,\n                                             'worst_abs_diff': {'index': (2951,\n                                                                          3317),\n                                                                'value': 4294967292},\n                                             'worst_fractional_diff': {'index': (2952,\n                                                                                 3320),\n                                                                       'value': inf}},\n                     \"root['roman']['err']\": {'abs_diff': <Quantity 1347.6384 DN / s>,\n                                              'n_diffs': 7,\n                                              'worst_abs_diff': {'index': (2952,\n                                                                           3320),\n                                                                 'value': <Quantity 577.6964 DN / s>},\n                                              'worst_fractional_diff': {'index': (2492,\n                                                                                  1865),\n                                                                        'value': <Quantity 947606.2>}},\n                     \"root['roman']['var_flat']\": {'abs_diff': <Quantity 494019.94 DN2 / s2>,\n                                                   'n_diffs': 7,\n                                                   'worst_abs_diff': {'index': (2952,\n                                                                                3320),\n                                                                      'value': <Quantity 333064.88 DN2 / s2>},\n                                                   'worst_fractional_diff': {'index': (2492,\n                                                                                       1865),\n                                                                             'value': <Quantity 7.453108e+11>}},\n                     \"root['roman']['var_poisson']\": {'abs_diff': <Quantity 3126.605 DN2 / s2>,\n                                                      'n_diffs': 7,\n                                                      'worst_abs_diff': {'index': (2955,\n                                                                                   3321),\n                                                                         'value': <Quantity 675.62225 DN2 / s2>},\n                                                      'worst_fractional_diff': {'index': (1828,\n                                                                                          711),\n                                                                                'value': <Quantity inf>}},\n                     \"root['roman']['var_rnoise']\": {'abs_diff': <Quantity 12.500684 DN2 / s2>,\n                                                     'n_diffs': 7,\n                                                     'worst_abs_diff': {'index': (2952,\n                                                                                  3320),\n                                                                        'value': <Quantity 3.4215055 DN2 / s2>},\n                                                     'worst_fractional_diff': {'index': (1828,\n                                                                                         711),\n                                                                               'value': <Quantity inf>}}},\n   'values_changed': {\"root['roman']['meta']['wcs_fit_results']['<rot>']\": {'new_value': -8.284282814118433e-05,\n                                                                            'old_value': -5.9008763525942403e-05},\n                      \"root['roman']['meta']['wcs_fit_results']['center'][0]\": {'new_value': -249.45443937059298,\n                                                                                'old_value': -249.45421319021025},\n                      \"root['roman']['meta']['wcs_fit_results']['center'][1]\": {'new_value': 109.61109764361109,\n                                                                                'old_value': 109.61122052869052},\n                      \"root['roman']['meta']['wcs_fit_results']['mae']\": {'new_value': 0.003221932838851946,\n                                                                          'old_value': 0.002684014626406904},\n                      \"root['roman']['meta']['wcs_fit_results']['matrix'][0][1]\": {'new_value': -1.4458801127269767e-06,\n                                                                                   'old_value': -1.0298972110582511e-06},\n                      \"root['roman']['meta']['wcs_fit_results']['matrix'][1][0]\": {'new_value': 1.4458801127269767e-06,\n                                                                                   'old_value': 1.0298972110582511e-06},\n                      \"root['roman']['meta']['wcs_fit_results']['proper_rot']\": {'new_value': -8.284282814118433e-05,\n                                                                                 'old_value': -5.9008763525942403e-05},\n                      \"root['roman']['meta']['wcs_fit_results']['rmse']\": {'new_value': 0.004752892068723334,\n                                                                           'old_value': 0.003440309699826702},\n                      \"root['roman']['meta']['wcs_fit_results']['rot'][0]\": {'new_value': -8.284282814118433e-05,\n                                                                             'old_value': -5.9008763525942403e-05},\n                      \"root['roman']['meta']['wcs_fit_results']['rot'][1]\": {'new_value': -8.284282814118433e-05,\n                                                                             'old_value': -5.9008763525942403e-05},\n                      \"root['roman']['meta']['wcs_fit_results']['shift'][0]\": {'new_value': -0.00849888422641429,\n                                                                               'old_value': -0.008499128453755975},\n                      \"root['roman']['meta']['wcs_fit_results']['shift'][1]\": {'new_value': 0.014497927842124192,\n                                                                               'old_value': 0.014101158752967921}}}\nassert False\n +  where False = <romancal.regtest.regtestdata.DiffResult object at 0x7f37e715d310>.identical",
          "text": "rtdata = {'input': '/runner/_work/_temp/pytest_basetemp/popen-gw1/test_level2_image_processing_p0/r0000101001001001001_01101_00...nt.asdf',\n 'truth_remote': 'roman-pipeline/dev/truth/WFI/image/r0000101001001001001_01101_0001_WFI01_cal_repoint.asdf'}\nignore_asdf_paths = {'ignore': ['asdf_library', 'history', 'roman.meta.ref_file.crds.sw_version', 'roman.meta.calibration_software_version', 'roman.cal_logs', 'roman.meta.date', ...]}\n\n    @pytest.mark.bigdata\n    @pytest.mark.soctests\n    def test_level2_image_processing_pipeline(rtdata, ignore_asdf_paths):\n        \"\"\"Tests for flat field imaging processing requirements DMS86 & DMS 87\"\"\"\n        input_data = \"r0000101001001001001_01101_0001_WFI01_uncal.asdf\"\n        rtdata.get_data(f\"WFI/image/{input_data}\")\n        rtdata.input = input_data\n    \n        # Test Pipeline\n        output = \"r0000101001001001001_01101_0001_WFI01_cal.asdf\"\n        rtdata.output = output\n        args = [\n            \"roman_elp\",\n            rtdata.input,\n        ]\n        ExposurePipeline.from_cmdline(args)\n        rtdata.get_truth(f\"truth/WFI/image/{output}\")\n        diff = compare_asdf(rtdata.output, rtdata.truth, **ignore_asdf_paths)\n        assert diff.identical, diff.report()\n    \n        # Perform DMS tests\n        # Initial prep\n        model = rdm.open(rtdata.output)\n        pipeline = ExposurePipeline()\n    \n        # DMS280 result is an ImageModel\n        pipeline.log.info(\n            \"DMS280 MSG: Testing that result is a Level 2 model.......\"\n            + passfail(isinstance(model, rdm.datamodels.ImageModel))\n        )\n    \n        # DMS86 instrument artifact correction tests\n        pipeline.log.info(\n            \"Status of the step:             assign_wcs    \"\n            + str(model.meta.cal_step.assign_wcs)\n        )\n        pipeline.log.info(\n            \"DMS86 MSG: Testing completion of wcs assignment inLevel 2 image output.......\"\n            + passfail(model.meta.cal_step.assign_wcs == \"COMPLETE\")\n        )\n        assert model.meta.cal_step.assign_wcs == \"COMPLETE\"\n        pipeline.log.info(\n            \"Status of the step:             flat_field    \"\n            + str(model.meta.cal_step.flat_field)\n        )\n        pipeline.log.info(\n            \"DMS86 MSG: Testing completion of flat fielding inLevel 2 image output.......\"\n            + passfail(model.meta.cal_step.flat_field == \"PASS\")\n        )\n        assert model.meta.cal_step.flat_field == \"COMPLETE\"\n        pipeline.log.info(\n            \"Status of the step:             dark          \" + str(model.meta.cal_step.dark)\n        )\n        pipeline.log.info(\n            \"DMS86 MSG: Testing completion of dark correction inLevel 2 image output.......\"\n            + passfail(model.meta.cal_step.dark == \"COMPLETE\")\n        )\n        assert model.meta.cal_step.dark == \"COMPLETE\"\n        pipeline.log.info(\n            \"Status of the step:             dq_init       \"\n            + str(model.meta.cal_step.dq_init)\n        )\n        pipeline.log.info(\n            \"DMS86 MSG: Testing completion of data quality correction in Level 2 image\"\n            \" output.......\" + passfail(model.meta.cal_step.dq_init == \"COMPLETE\")\n        )\n        assert model.meta.cal_step.dq_init == \"COMPLETE\"\n        pipeline.log.info(\n            \"Status of the step:             jump          \" + str(model.meta.cal_step.jump)\n        )\n        pipeline.log.info(\n            \"DMS86 MSG: Testing completion of jump detection inLevel 2 image output.......\"\n            + passfail(model.meta.cal_step.jump == \"COMPLETE\")\n        )\n        assert model.meta.cal_step.jump == \"COMPLETE\"\n        uneven = len({len(x) for x in model.meta.exposure.read_pattern}) > 1\n        pipeline.log.info(\n            \"DMS361: Testing that jump detection detected jumps in uneven ramp in \"\n            \"Level 2 image output.......\"\n            + passfail(uneven & np.any(model.dq & pixel.JUMP_DET))\n        )\n        assert uneven & np.any(model.dq & pixel.JUMP_DET)\n        pipeline.log.info(\n            \"Status of the step:             linearity     \"\n            + str(model.meta.cal_step.linearity)\n        )\n        pipeline.log.info(\n            \"DMS86 MSG: Testing completion of linearity correction in Level 2 image\"\n            \" output.......\" + passfail(model.meta.cal_step.linearity == \"COMPLETE\")\n        )\n        assert model.meta.cal_step.linearity == \"COMPLETE\"\n        pipeline.log.info(\n            \"Status of the step:             ramp_fit      \"\n            + str(model.meta.cal_step.ramp_fit)\n        )\n        pipeline.log.info(\n            \"DMS86 MSG: Testing completion of ramp fitting inLevel 2 image output.......\"\n            + passfail(model.meta.cal_step.ramp_fit == \"COMPLETE\")\n        )\n        assert model.meta.cal_step.ramp_fit == \"COMPLETE\"\n        pipeline.log.info(\n            \"Status of the step:             saturation    \"\n            + str(model.meta.cal_step.saturation)\n        )\n        pipeline.log.info(\n            \"DMS86 MSG: Testing completion of saturation detection in Level 2 image\"\n            \" output.......\" + passfail(model.meta.cal_step.saturation == \"COMPLETE\")\n        )\n        assert model.meta.cal_step.saturation == \"COMPLETE\"\n    \n        # DMS-129 tests\n        # check if assign_wcs step is complete\n        pipeline.log.info(\n            \"DMS-129 MSG: Status of the step:             assign_wcs    \"\n            + str(model.meta.cal_step.assign_wcs)\n        )\n        pipeline.log.info(\n            \"DMS-129 MSG: Testing completion of WCS assignment inLevel 2 image\"\n            \" output.......\" + passfail(model.meta.cal_step.assign_wcs == \"COMPLETE\")\n        )\n        assert model.meta.cal_step.assign_wcs == \"COMPLETE\"\n        # check if WCS exists\n        pipeline.log.info(\"DMS-129 MSG: Testing that a WCS object exists    \")\n        pipeline.log.info(\n            \"DMS-129 MSG: Testing that WCS exists inLevel 2 image output.......\"\n            + passfail(model.meta.wcs is not None)\n        )\n        assert model.meta.wcs is not None\n        pipeline.log.info(\n            \"DMS-129 MSG: Testing that geometric distortion information is available\"\n            \" inLevel 2 image output.......\"\n            + passfail(\"v2v3\" in model.meta.wcs.available_frames)\n        )\n        assert \"v2v3\" in model.meta.wcs.available_frames\n        # compare coordinates before and after distortion correction has been applied\n        # 1 - get new image array based on the model\n        x0, y0 = grid_from_bounding_box(model.meta.wcs.bounding_box)\n        # 2 - apply the distortion-corrected WCS solution to new image array\n        corrected_coords = model.meta.wcs(x0, y0)\n        # 3 - apply the transformation from 'v2v3' to 'world' without distortion correction\n        original_coords = model.meta.wcs.get_transform(\"v2v3\", \"world\")(x0, y0)\n        # compare both results to make sure they don't match\n        # (which means the distortion correction was actually applied to the model)\n        pipeline.log.info(\n            \"DMS-129 MSG: Testing that distortion correction was applied toLevel 2 image\"\n            \" output.......\"\n            + passfail(\n                (corrected_coords[0] != original_coords[0]).all()\n                & (corrected_coords[1] != original_coords[1]).all()\n            )\n        )\n        assert (corrected_coords[0] != original_coords[0]).all()\n        assert (corrected_coords[1] != original_coords[1]).all()\n    \n        # DMS87 data quality tests\n        pipeline.log.info(\n            \"DMS87 MSG: Testing existence of data quality array (dq) in Level 2 image\"\n            \" output.......\" + passfail(\"dq\" in model.keys())\n        )\n        assert \"dq\" in model.keys()\n        pipeline.log.info(\n            \"DMS87 MSG: Testing existence of general error array (err) in Level 2 image\"\n            \" output.......\" + passfail(\"err\" in model.keys())\n        )\n        assert \"err\" in model.keys()\n        pipeline.log.info(\n            \"DMS87 MSG: Testing existence of Poisson noise variancearray (var_poisson) in\"\n            \" Level 2 image output.......\" + passfail(\"var_poisson\" in model.keys())\n        )\n        assert \"var_poisson\" in model.keys()\n        pipeline.log.info(\n            \"DMS87 MSG: Testing existence of read noise variance array (var_rnoise) in\"\n            \" level 2 image output.......\" + passfail(\"var_rnoise\" in model.keys())\n        )\n        assert \"var_rnoise\" in model.keys()\n        pipeline.log.info(\n            \"DMS87 MSG: Testing existence of flatfield uncertainty variance array\"\n            \" (var_flat) in Level 2 image output....\" + passfail(\"var_flat\" in model.keys())\n        )\n        assert \"var_flat\" in model.keys()\n    \n        # DMS88 total exposure time test\n        pipeline.log.info(\n            \"DMS88 MSG: Testing existence of total exposure time (exposure_time) in Level 2\"\n            \" image output.......\" + passfail(\"exposure_time\" in model.meta.exposure)\n        )\n        assert \"exposure_time\" in model.meta.exposure\n    \n        # DMS-136 PSF tests\n        pipeline.log.info(\n            \"DMS-136 MSG: Testing existence of  detector and \"\n            \"optical element (detector & optical_element) in Level 2 \"\n            \"image output.......\" + passfail(\"exposure_time\" in model.meta.exposure)\n        )\n        assert \"detector\" in model.meta.instrument\n        assert \"optical_element\" in model.meta.instrument\n    \n        # DMS89 WCS tests\n        pipeline.log.info(\n            \"DMS89 MSG: Testing that the wcs boundingbox was generated.......\"\n            + passfail(len(model.meta.wcs.bounding_box) == 2)\n        )\n        assert len(model.meta.wcs.bounding_box) == 2\n    \n        # Save original wcs information\n        orig_wcs = copy.deepcopy(model.meta.wcs)\n        del model.meta[\"wcs\"]\n    \n        # Create new pointing for the model\n        # RA & Dec are each shifted + 10 degrees, unless they are near\n        # the upper limit, in which case they are shifted -10 degrees.\n        delta = [10.0, 10.0]\n        if model.meta.wcsinfo.ra_ref >= 350.0:\n            delta[0] *= -1.0\n    \n        if model.meta.wcsinfo.dec_ref >= 80.0:\n            delta[1] *= -1.0\n    \n        model.meta.wcsinfo.ra_ref += delta[0]\n        model.meta.wcsinfo.dec_ref += delta[1]\n    \n        # Create new wcs object for the new pointing\n        model = AssignWcsStep.call(model)\n    \n        rtdata.output = output.rsplit(\".\", 1)[0] + \"_repoint.asdf\"\n        model.to_asdf(rtdata.output)\n    \n        # Test that repointed file matches truth\n        rtdata.get_truth(\"truth/WFI/image/\" + output.rsplit(\".\", 1)[0] + \"_repoint.asdf\")\n        diff = compare_asdf(rtdata.output, rtdata.truth, **ignore_asdf_paths)\n>       assert diff.identical, diff.report()\nE       AssertionError: \nE         Diff report for:\nE             result file: /runner/_work/_temp/pytest_basetemp/popen-gw1/test_level2_image_processing_p0/r0000101001001001001_01101_0001_WFI01_cal_repoint.asdf\nE                 model type: ImageModel\nE             truth file: /runner/_work/_temp/pytest_basetemp/popen-gw1/test_level2_image_processing_p0/truth/r0000101001001001001_01101_0001_WFI01_cal_repoint.asdf\nE                 model type: ImageModel\nE         \nE         {'arrays_differ': {\"root['roman']['data']\": {'abs_diff': <Quantity 32778.504 DN / s>,\nE                                                      'n_diffs': 471,\nE                                                      'worst_abs_diff': {'index': (2955,\nE                                                                                   3321),\nE                                                                         'value': <Quantity 7411.377 DN / s>},\nE                                                      'worst_fractional_diff': {'index': (2492,\nE                                                                                          1865),\nE                                                                                'value': <Quantity 863315.94>}},\nE                            \"root['roman']['dq']\": {'abs_diff': 12884901877,\nE                                                    'n_diffs': 4,\nE                                                    'worst_abs_diff': {'index': (2951,\nE                                                                                 3317),\nE                                                                       'value': 4294967292},\nE                                                    'worst_fractional_diff': {'index': (2952,\nE                                                                                        3320),\nE                                                                              'value': inf}},\nE                            \"root['roman']['err']\": {'abs_diff': <Quantity 1347.6384 DN / s>,\nE                                                     'n_diffs': 7,\nE                                                     'worst_abs_diff': {'index': (2952,\nE                                                                                  3320),\nE                                                                        'value': <Quantity 577.6964 DN / s>},\nE                                                     'worst_fractional_diff': {'index': (2492,\nE                                                                                         1865),\nE                                                                               'value': <Quantity 947606.2>}},\nE                            \"root['roman']['var_flat']\": {'abs_diff': <Quantity 494019.94 DN2 / s2>,\nE                                                          'n_diffs': 7,\nE                                                          'worst_abs_diff': {'index': (2952,\nE                                                                                       3320),\nE                                                                             'value': <Quantity 333064.88 DN2 / s2>},\nE                                                          'worst_fractional_diff': {'index': (2492,\nE                                                                                              1865),\nE                                                                                    'value': <Quantity 7.453108e+11>}},\nE                            \"root['roman']['var_poisson']\": {'abs_diff': <Quantity 3126.605 DN2 / s2>,\nE                                                             'n_diffs': 7,\nE                                                             'worst_abs_diff': {'index': (2955,\nE                                                                                          3321),\nE                                                                                'value': <Quantity 675.62225 DN2 / s2>},\nE                                                             'worst_fractional_diff': {'index': (1828,\nE                                                                                                 711),\nE                                                                                       'value': <Quantity inf>}},\nE                            \"root['roman']['var_rnoise']\": {'abs_diff': <Quantity 12.500684 DN2 / s2>,\nE                                                            'n_diffs': 7,\nE                                                            'worst_abs_diff': {'index': (2952,\nE                                                                                         3320),\nE                                                                               'value': <Quantity 3.4215055 DN2 / s2>},\nE                                                            'worst_fractional_diff': {'index': (1828,\nE                                                                                                711),\nE                                                                                      'value': <Quantity inf>}}},\nE          'values_changed': {\"root['roman']['meta']['wcs_fit_results']['<rot>']\": {'new_value': -8.284282814118433e-05,\nE                                                                                   'old_value': -5.9008763525942403e-05},\nE                             \"root['roman']['meta']['wcs_fit_results']['center'][0]\": {'new_value': -249.45443937059298,\nE                                                                                       'old_value': -249.45421319021025},\nE                             \"root['roman']['meta']['wcs_fit_results']['center'][1]\": {'new_value': 109.61109764361109,\nE                                                                                       'old_value': 109.61122052869052},\nE                             \"root['roman']['meta']['wcs_fit_results']['mae']\": {'new_value': 0.003221932838851946,\nE                                                                                 'old_value': 0.002684014626406904},\nE                             \"root['roman']['meta']['wcs_fit_results']['matrix'][0][1]\": {'new_value': -1.4458801127269767e-06,\nE                                                                                          'old_value': -1.0298972110582511e-06},\nE                             \"root['roman']['meta']['wcs_fit_results']['matrix'][1][0]\": {'new_value': 1.4458801127269767e-06,\nE                                                                                          'old_value': 1.0298972110582511e-06},\nE                             \"root['roman']['meta']['wcs_fit_results']['proper_rot']\": {'new_value': -8.284282814118433e-05,\nE                                                                                        'old_value': -5.9008763525942403e-05},\nE                             \"root['roman']['meta']['wcs_fit_results']['rmse']\": {'new_value': 0.004752892068723334,\nE                                                                                  'old_value': 0.003440309699826702},\nE                             \"root['roman']['meta']['wcs_fit_results']['rot'][0]\": {'new_value': -8.284282814118433e-05,\nE                                                                                    'old_value': -5.9008763525942403e-05},\nE                             \"root['roman']['meta']['wcs_fit_results']['rot'][1]\": {'new_value': -8.284282814118433e-05,\nE                                                                                    'old_value': -5.9008763525942403e-05},\nE                             \"root['roman']['meta']['wcs_fit_results']['shift'][0]\": {'new_value': -0.00849888422641429,\nE                                                                                      'old_value': -0.008499128453755975},\nE                             \"root['roman']['meta']['wcs_fit_results']['shift'][1]\": {'new_value': 0.014497927842124192,\nE                                                                                      'old_value': 0.014101158752967921}}}\nE       assert False\nE        +  where False = <romancal.regtest.regtestdata.DiffResult object at 0x7f37e715d310>.identical\n\n/runner/_work/RegressionTests/RegressionTests/romancal/regtest/test_wfi_pipeline.py:256: AssertionError"
        }
      }
    ]
  },
  ...
  "romancal.source_catalog.tests.test_source_catalog::test_l3_source_catalog[3-50-5-False]": {
    "time": [
      62.664,
      53.992
    ]
  },
  "romancal.tweakreg.tests.test_astrometric_utils::test_get_catalog_using_epoch[10-10-2000]": {
    "time": [
      0.666,
      0.258
    ]
  },
  "romancal.tweakreg.tests.test_astrometric_utils::test_get_catalog_using_valid_parameters[10--10-0.1-GAIADR2]": {
    "time": [
      0.143,
      0.281
    ]
  }
}
```

```shell
pytestresultsdiff src/data/peakmem/main.xml src/data/peakmem/pr.xml > examplediff.json
cat examplediff.json
```
```json
{
  'romancal.regtest.test_catalog::test_log_tracked_resources[L3]': {
    'peakmem': [
      1721952705.0,
      2721952705.0
    ],
    'time': [
      100.37806803395506,
      101.37806803395506
    ]
  }
}
```

## `scripts/generate_markdown_table.py` Python script

```
usage: generate_markdown_table [-h] [--run-names RUN_NAMES] results-diff-json properties [properties ...]

reads pytestresultsdiff JSON and creates a comparison table for the specified properties

positional arguments:
  results-diff-json     filename of pytestresultsdiff JSON, or - to read from stdin
  properties            properties to compare

options:
  -h, --help            show this help message and exit
  --run-names RUN_NAMES
                        comma-separated list of run names
```

```shell
python scripts/generate_markdown_table.py examplediff.json status time peakmem --run-names main,pr
```
| test case | main status | pr status | main time | pr time | main peakmem | pr peakmem |
| --- | --- | --- | --- | --- | --- | --- |
| `romancal.regtest.test_catalog::test_log_tracked_resources[L3]` |  |  | `100.4s` | `101.4s` | `1722MB` | `2722MB` |
