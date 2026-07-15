# cJSON translation with upstream C tests

This experiment runs cJSON's upstream C tests against CRISP's translated Rust
shared library. `test_translated.sh` builds `rust/target/debug/libcjson.so`,
builds the C test executables, and runs them with CTest.

The upstream test tree originally contained two kinds of tests:

- API/ABI tests that included `cJSON.h`; these link directly to the Rust library.
- White-box parser/printer tests that included `cJSON.c` to call private C
  helpers. These have been adapted to exercise the same observable behavior
  through exported parse, create, type-check, and print functions.

All 18 core cJSON test executables now link to the Rust library. Together they
register 150 Unity test cases; 149 run and one non-finite-number case remains
ignored by upstream.

Only the selected test sources, their input fixtures, and the small Unity C
test framework needed to compile them are included in this experiment.

Run `./init.sh` once to initialize/transpile the CRISP workspace. Afterwards,
run `bash ./test_translated.sh` to exercise the same command used by CRISP.

agent: gpt-5.5
