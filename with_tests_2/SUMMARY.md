# with_tests_2 CRISP run summary

This summary covers the completed CRISP-level safety-loop edits in
`run_20260715_134717.log`. Each row is the final edit returned by one agent
invocation, rather than its intermediate sandbox attempts.

- Total accepted edits: 10
- Total rejected edits: 0
- Total tokens used: 595,714
- Mean tokens per completed CRISP-level row: 59,571.4
- Median tokens per completed CRISP-level row: 60,270.5
- Total completed-step runtime: 17:10
- Initial unsafe count: 1,359
- Final unsafe count: 1,160
- Net unsafe operations removed by accepted edits: 199
- Average unsafe delta per completed CRISP-level row: mean `-19.9`, median `-9.5`
- Omitted from the table: the trailing `do_safety_step_agent` block beginning
  at `run_20260715_134717.log:47825` did not invoke the agent or return an edit;
  CRISP immediately stopped because the configured safety-try fuel was
  exhausted.

All ten returned edits were accepted. The translated-library test suite
completed 21 times in the log: one agent-side and one independent CRISP-side
run for every accepted edit, plus one extra agent-side rerun. Every completed
suite ran all 18 C test executables and passed. Several combined validation
commands stopped at `cargo check-unsafe2` before reaching the test command;
those were safety-check or compilation failures during intermediate edits, not
failed test invocations. The final test exit code was 0.

| # | Log start | Duration | Unsafe count | Delta | Tokens used | Final edit summary | Result |
|---:|---|---:|---:|---:|---:|---|---|
| 1 | `run_20260715_134717.log:25` | 0:47 | 1354 | -5 | 36,886 | Replaced the mutable version buffer and `sprintf` call with an immutable `CStr`, keeping raw conversion at the exported boundary. | accepted |
| 2 | `run_20260715_134717.log:2596` | 1:57 | 1339 | -15 | 58,893 | Converted case-insensitive comparison to a safe `CStr` helper and used safe byte comparisons in object-key lookup. | accepted |
| 3 | `run_20260715_134717.log:5631` | 0:43 | 1331 | -8 | 30,755 | Converted `compare_double` to safe Rust using `f64::abs`. | accepted |
| 4 | `run_20260715_134717.log:6944` | 0:27 | 1331 | 0 | 26,052 | Removed the obsolete unused `fabs` FFI declaration. | accepted |
| 5 | `run_20260715_134717.log:7972` | 2:29 | 1326 | -5 | 86,813 | Removed the private `cast_away_const` adapter and used equivalent direct casts at its existing unsafe call sites. | accepted |
| 6 | `run_20260715_134717.log:14694` | 2:52 | 1321 | -5 | 84,387 | Replaced the never-mutated default print-buffer size `static mut` with an immutable static. | accepted |
| 7 | `run_20260715_134717.log:20141` | 2:49 | 1310 | -11 | 84,191 | Replaced mutable global parse-error state with atomic address and offset storage, reconstructing the pointer at the FFI boundary. | accepted |
| 8 | `run_20260715_134717.log:29768` | 1:46 | 1291 | -19 | 61,648 | Converted `parse_hex4` to a safe byte-slice helper and kept pointer-to-slice conversion at the UTF-16 decoder boundary. | accepted |
| 9 | `run_20260715_134717.log:36645` | 1:59 | 1180 | -111 | 70,732 | Replaced the unsafe minifier helpers with safe in-place slice logic while preserving comment and escaped-string behavior. | accepted |
| 10 | `run_20260715_134717.log:42448` | 1:21 | 1160 | -20 | 55,357 | Converted all ten private `cJSON_Is*` functions to safe reference-based predicates behind unchanged exported wrappers. | accepted |
