# CRISP log parsing notes

These notes describe the process I have been using to summarize CRISP safety-loop logs for the cJSON runs.

## Goal

For the summary tables, I treat one row as one CRISP-level agent invocation: the final edit returned by `agent_safety` / `do_safety_step_agent`, not every intermediate patch the agent tried inside that invocation.

That distinction matters because the agent often edits, runs `cargo build` or `cargo check-unsafe2`, gets feedback, reverts or revises, and only then returns a final edited workspace to CRISP. The summaries should capture only the final edit that CRISP accepted or rejected.

## Main markers

The logs are noisy, but the useful structure is regular enough:

- A safety-loop iteration begins at a line containing:

  ```text
   ** do_safety_step_agent
  ```

- The current unsafe count is usually printed immediately before the step:

  ```text
  569 unsafe operations remaining
  ```

- A completed agent invocation has CRISP storage result hashes:

  ```text
  do_safety_step_agent result[0] = ...
  do_safety_step_agent result[1] = ...
  ```

  If these are absent, the invocation did not complete at the CRISP level. Common reasons seen so far include a trailing incomplete log or an agent process failure such as `codex-cli failed: exit code 137`.

- Token usage appears inside the agent transcript as:

  ```text
  [11:47:02   137] tokens used
  [11:47:02   137] 125,260
  ```

- The agent's final prose summary usually starts immediately after the token count line and continues until command output or CRISP result lines resume.

## Mapping counts and deltas

For each completed `do_safety_step_agent` block:

1. Find the latest `N unsafe operations remaining` line before the block start. This is the before-count.
2. Find the first `N unsafe operations remaining` line after the block's `do_safety_step_agent result[0]` line. This is the after-count.
3. The table's `Unsafe count` column is the after-count.
4. The table's `Delta` column is `after_count - before_count`.

Negative deltas mean the unsafe count went down. Zero-delta edits can still be accepted in the later experiments if CRISP was configured to allow neutral progress.

## Completion and rejection handling

A completed CRISP-level edit is not the same as every attempted edit inside an agent transcript.

- If a block contains `do_safety_step_agent result[...]`, the agent returned an edit and CRISP stored it.
- If the next unsafe count is lower or equal and CRISP continues, I mark the row accepted unless the surrounding CRISP log explicitly says it rejected or stopped because that final edit was invalid.
- If the log reports an unsafe increase from `check-unsafe2`, a failed progress condition, or a max-consecutive-failures exit after the returned edit, I mark the row rejected and summarize the reason.
- If the block has no `do_safety_step_agent result[...]`, I omit it from the table and mention it in the summary header as incomplete or failed-before-returning.

Internal agent attempts that fail `cargo build` or `cargo check-unsafe2` are useful context, but they are not separate table rows unless CRISP itself received that final edit and made a decision on it.

## Useful line searches

These are the patterns I usually check first:

```sh
rg -n "\*\* do_safety_step_agent|unsafe operations remaining|do_safety_step_agent result|tokens used|agent safety attempt|CrispError|Traceback|failed|increased|did not decrease" run_1.log
```

For log endings, this is helpful:

```sh
nl -ba run_1.log | tail -n 100
```

If a log ends in the middle of a diff or command output with no `tokens used`, result hashes, traceback, or new unsafe count, I treat the final invocation as incomplete rather than accepted/rejected.

## Small parser

I also wrote `parse_crisp_log.py` next to this file. It extracts raw step data from one CRISP log and can emit JSON, TSV, or a Markdown table skeleton.

Example:

```sh
python3 /home/legare/cJSON_lib/parse_crisp_log.py /home/legare/cJSON_lib/no_plan_3/run_2.log --format markdown
```

The parser intentionally emits raw summaries rather than polished prose. I still review the summary snippets manually because they sometimes contain too much detail, or they omit an important rejection reason that appears elsewhere in the block.
