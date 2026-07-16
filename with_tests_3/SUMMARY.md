# with_tests_3 CRISP run summary

This summary covers the completed CRISP-level safety-loop edits in
`run_20260716_112439.log`. Each row is the final edit returned by one agent
invocation, rather than its intermediate sandbox attempts.

- Total accepted edits: 5
- Total rejected edits: 0
- Total tokens used: 340,220
- Mean tokens per completed CRISP-level row: 68,044
- Median tokens per completed CRISP-level row: 69,603
- Total completed-step runtime: 11:06
- Initial unsafe count: 1,359
- Final unsafe count: 997
- Net unsafe operations removed by accepted edits: 362
- Average unsafe delta per completed CRISP-level row: mean `-72.4`, median `-25`
- Omitted from the table: the block beginning at
  `run_20260716_112439.log:24837` did not launch an agent or return an edit;
  CRISP exited immediately because all five configured safety tries had been
  consumed.

All five returned edits were accepted. The agent-side safety checker reported
increases during five of its 16 intermediate `cargo check-unsafe2` runs, but
the agent revised those attempts before returning each final edit. Every
completed row passed the final unsafe comparison and all 18 translated-library
test executables. The run's final test exit code was 0.

The sandbox Git repository was initialized successfully for all five agent
turns. `git status`, `git diff`, `git diff --check`, and the one `git log`
query all worked; the history query found the expected single `CRISP sandbox
baseline` commit. The generated `.gitignore` kept `target/` and `unsafe_json/`
out of status output. The agent made five unsuccessful `git apply` attempts
while editing: four used malformed patches and one generated a patch whose
temporary source path could not be opened. These were patch-construction
errors rather than repository errors, and the agent recovered using other edit
methods before returning accepted changes. `.codex/` remained untracked in
each turn, while `build-rust-tests/` appeared after tests until the agent
removed it in the final turn.

| # | Log start | Duration | Unsafe count | Delta | Tokens used | Final edit summary | Result |
|---:|---|---:|---:|---:|---:|---|---|
| 1 | `run_20260716_112439.log:25` | 1:34 | 1351 | -8 | 49,336 | Made `compare_double` safe and replaced imported `fabs` calls with `f64::abs()`. | accepted |
| 2 | `run_20260716_112439.log:6531` | 2:00 | 1244 | -107 | 71,979 | Replaced raw UTF-16 escape decoding with bounded byte-array/slice helpers and a safe `Vec<u8>` JSON-string decoder. | accepted |
| 3 | `run_20260716_112439.log:11293` | 2:35 | 1225 | -19 | 69,337 | Simplified parser cursor helpers by validating the raw buffer once and operating through a local mutable reference. | accepted |
| 4 | `run_20260716_112439.log:15139` | 2:31 | 1022 | -203 | 79,965 | Stored parser input as a bounded byte slice, shared mutable parser state by reference, and replaced raw token reads with slice operations. | accepted |
| 5 | `run_20260716_112439.log:21491` | 2:26 | 997 | -25 | 69,603 | Removed four non-exported raw-pointer parser façades, leaving their ABI logic only in the exported FFI wrappers. | accepted |
