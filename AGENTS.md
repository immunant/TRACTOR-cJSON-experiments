# CRISP cJSON Experiment Handoff

This note is for future agent sessions working on `/home/legare/cJSON_lib`. The project is about studying CRISP runs on `cJSON`: CRISP starts from a C-to-Rust translation and repeatedly asks an LLM agent to reduce unsafe Rust while preserving behavior. The major comparison is between runs with no persistent safety plan and runs with a persistent `SAFETY_PLAN.md`.

## Important Paths

- `/home/legare/cJSON_lib/no_plan_3` - main no-plan run, completed to `0` unsafe operations.
- `/home/legare/cJSON_lib/with_plan_4` - main with-plan run, completed to `0` unsafe operations.
- `/home/legare/cJSON_lib/no_plan_3/summary.md` - chronological table of accepted/rejected CRISP-level edits for `no_plan_3`.
- `/home/legare/cJSON_lib/with_plan_4/summary.md` - same for `with_plan_4`.
- `/home/legare/cJSON_lib/parse_crisp_log.py` - lightweight parser for CRISP logs.
- `/home/legare/cJSON_lib/CRISP_LOG_PARSING.md` - notes on interpreting logs and parser output.
- `/home/legare/cJSON_lib/with_plan_4/rust_original` - checkout of the original post-transpile Rust code for `with_plan_4`.
- `/home/legare/cJSON_lib/Tractor-Crisp` - CRISP source tree used for several local changes during these experiments.
- `/home/legare/Tractor-Crisp` - another CRISP checkout; current sessions may start here, but most experiment artifacts live under `/home/legare/cJSON_lib`.

> no_plan_1-2 and with_plan_1-3 were earlier setup runs to figure out how to do
> the experiment, and are likely not going to be analysed further. Other 

## Runtime Method

When asked for total runtime, we measured only completed CRISP-level safety-loop steps listed in the summary tables. The measurement is per completed row, from the first timestamp inside that row's `do_safety_step_agent` block through the last timestamp before the next step or final footer. This excludes trailing partial/incomplete blocks and omitted agent executions that failed before returning an edit.

| Run | Completed steps counted | Completed-step runtime |
|---|---:|---:|
| `no_plan_3` | 89 | `7:05:22` |
| `with_plan_4` | 82 | `5:39:24` |

## What The Summaries Mean

The summary tables list one row per final edit returned to CRISP, not every intermediate attempt the agent made internally. Inside one agent invocation, the agent may try several edits, run `cargo build`, run `cargo check-unsafe2`, see errors, and revise. Those internal attempts are usually omitted.

Rows can be:

- `accepted` - CRISP accepted the returned code node after tests and unsafe comparison.
- `rejected: ...` - the agent returned an edit, but CRISP rejected it, usually because `compare_unsafe2_op` saw increased unsafe.
- Omitted/incomplete - the log contains an agent invocation that did not return a completed edit, was killed, failed before returning, or was cut off by timeout.

The `Unsafe count` column is the final unsafe count after the step. The `Delta` column is `new - old`, so negative numbers mean unsafe operations were removed.

## Log Parsing Workflow

Use the parser:

```sh
python3 /home/legare/cJSON_lib/parse_crisp_log.py --format tsv /home/legare/cJSON_lib/no_plan_3/run_7.log
python3 /home/legare/cJSON_lib/parse_crisp_log.py --format json /home/legare/cJSON_lib/with_plan_4/run_5.log
```

Useful parser fields:

- `completed_steps` - CRISP-level steps with `do_safety_step_agent result[...]`.
- `incomplete_steps` - started agent invocations with no result.
- `tokens_used` - parsed from the agent's final `tokens used` block.
- `before_count`, `after_count`, `delta` - unsafe count context from nearby `count_unsafe2` output.
- `agent_check_unsafe2_runs` - `cargo check-unsafe2` invocations run by the agent inside the sandbox.
- `agent_check_unsafe2_increase_count` - how many agent-side check invocations reported any increased unsafe count.

Manual cross-checks are still important. If `result_hash` is `None`, inspect that block directly: it usually means CRISP rejected the returned edit. Search around that line for `compare_unsafe2_op`, `check_unsafe2_op`, and messages like `raw pointer derefs increased`.

## CRISP CLI Usage

Run the CRISP CLI from the experiment directory containing `crisp.toml`:

```sh
cd /home/legare/cJSON_lib/with_plan_4
uv --project /home/legare/Tractor-Crisp run crisp checkout <node_hash> --path <output_dir>
```

Examples from this investigation:

- Original post-transpile Rust for `with_plan_4` was checked out to `/home/legare/cJSON_lib/with_plan_4/rust_original`.
- The code node used was `c96637657cacb048f80e12c838525cbd367800f8ca77418c8a44dcb367bf2184`, the first counted Rust code version with `1359 unsafe operations remaining`.

If you need a rejected diff, use `crisp checkout` on the old and new code hashes from the relevant `compare_unsafe2_op` block, then diff the checked-out directories.

## Useful Commands

Create compressed archives from `/home/legare/cJSON_lib`:

```sh
tar -czf with_plan_4.tar.gz with_plan_4
tar -czf no_plan_3.tar.gz no_plan_3
```

Exclude Rust build artifacts:

```sh
tar --exclude='*/target' -czf with_plan_4.tar.gz with_plan_4
tar --exclude='*/target' -czf no_plan_3.tar.gz no_plan_3
```

Parse a log:

```sh
python3 parse_crisp_log.py --format tsv no_plan_3/run_7.log
python3 parse_crisp_log.py --format json with_plan_4/run_5.log
```

Search logs for rejected unsafe increases:

```sh
rg -n "increased:|do_safety_step_agent result\\[0\\] = None|compare_unsafe2_op|check_unsafe2_op" no_plan_3/run_*.log with_plan_4/run_*.log
```

## How To Continue The Analysis

When a new run log appears:

1. Run `parse_crisp_log.py` on it.
2. Check `completed_count`, `incomplete_count`, and any `result_hash = None` cases.
3. Manually inspect rejected blocks around `compare_unsafe2_op`.
4. Append only final CRISP-level edits to the appropriate `summary.md`.
5. Update totals: accepted, rejected, total tokens, final unsafe count, net unsafe removed, and omitted incomplete blocks.
6. If comparing runtime, sum only completed summary-table rows unless explicitly asked for wall-clock process time.
