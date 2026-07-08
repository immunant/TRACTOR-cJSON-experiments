#!/usr/bin/env python3
"""Extract CRISP safety-loop step data from a log file.

This is intentionally lightweight: it identifies CRISP-level agent invocations,
completion result hashes, token totals, before/after unsafe counts, and the final
assistant summary block after `tokens used`.
"""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any


START_MARKER = "** do_safety_step_agent"
RESULT_MARKER = "do_safety_step_agent result[0]"
COUNT_RE = re.compile(r"(\d+) unsafe operations remaining")
TOKEN_VALUE_RE = re.compile(r"([0-9,]+)\s*$")
TIMESTAMP_RE = re.compile(r"^\[[^]]+\]\s*")
TIMESTAMP_VALUE_RE = re.compile(r"^\[(\d\d):(\d\d):(\d\d)")
AGENT_EXEC_RE = re.compile(r"^\[[^]]+\]\s*exec\s*$")
CHECK_UNSAFE2_INCREASE_RE = re.compile(r": .*\bincreased: \d+ -> \d+")

# Lines that usually mean the final assistant prose block has ended.
SUMMARY_STOP_PREFIXES = (
    "cd ",
    "agent_safety result",
    "do_safety_step_agent result",
    "  * test_op",
    "code = ",
    "run <function",
    " ** ",
    "Traceback ",
)


def strip_agent_prefix(line: str) -> str:
    return TIMESTAMP_RE.sub("", line).rstrip()


def timestamp_seconds(line: str) -> int | None:
    match = TIMESTAMP_VALUE_RE.match(line)
    if not match:
        return None
    hours, minutes, seconds = (int(group) for group in match.groups())
    return hours * 3600 + minutes * 60 + seconds


def format_duration(seconds: int | None) -> str:
    if seconds is None:
        return ""
    hours = seconds // 3600
    minutes = (seconds % 3600) // 60
    secs = seconds % 60
    if hours:
        return f"{hours}:{minutes:02d}:{secs:02d}"
    return f"{minutes}:{secs:02d}"


def block_duration_seconds(lines: list[str], start_idx: int, end_idx: int) -> int | None:
    timestamps = [
        seconds
        for idx in range(start_idx, end_idx)
        if (seconds := timestamp_seconds(lines[idx])) is not None
    ]
    if not timestamps:
        return None
    start = timestamps[0]
    end = timestamps[-1]
    if end < start:
        end += 24 * 3600
    return end - start


def parse_token_and_summary(lines: list[str], start_idx: int, end_idx: int) -> tuple[int | None, int | None, list[str]]:
    """Return token line number, token value, and final assistant summary lines.

    `start_idx` and `end_idx` are zero-based indexes bounding one CRISP step.
    """
    token_line_no = None
    token_value = None
    summary: list[str] = []

    for idx in range(start_idx, end_idx):
        if "tokens used" not in lines[idx]:
            continue

        token_line_no = idx + 1
        if idx + 1 < len(lines):
            match = TOKEN_VALUE_RE.search(lines[idx + 1].strip())
            if match:
                token_value = int(match.group(1).replace(",", ""))

        j = idx + 2
        while j < end_idx:
            line = lines[j]
            if any(line.startswith(prefix) for prefix in SUMMARY_STOP_PREFIXES):
                break
            cleaned = strip_agent_prefix(line)
            if cleaned.strip():
                summary.append(cleaned)
            j += 1
        # Use the last token block in a step if there are multiple.
    return token_line_no, token_value, summary


def latest_count_before(counts: list[tuple[int, int]], line_no: int) -> tuple[int | None, int | None]:
    prior = [(ln, count) for ln, count in counts if ln < line_no]
    return max(prior, default=(None, None), key=lambda item: item[0] or -1)


def first_count_after(counts: list[tuple[int, int]], line_no: int) -> tuple[int | None, int | None]:
    later = [(ln, count) for ln, count in counts if ln > line_no]
    return min(later, default=(None, None), key=lambda item: item[0] or 10**18)


def parse_agent_check_unsafe2_runs(lines: list[str], start_idx: int, end_idx: int) -> list[dict[str, Any]]:
    """Find agent-invoked cargo check-unsafe2 command executions in a step.

    This intentionally ignores CRISP's own post-agent `check_unsafe2_op`, whose
    log lines are not timestamped agent `exec` command blocks.
    """
    runs: list[dict[str, Any]] = []
    idx = start_idx
    while idx < end_idx:
        if not AGENT_EXEC_RE.match(lines[idx]):
            idx += 1
            continue

        cmd_start = idx + 1
        if cmd_start >= end_idx:
            idx += 1
            continue

        command_lines: list[str] = []
        cmd_end = cmd_start
        while cmd_end < end_idx:
            cleaned = strip_agent_prefix(lines[cmd_end])
            command_lines.append(cleaned)
            if " in /root/work" in cleaned or " in /root/work/" in cleaned:
                break
            # If this was not a shell command line, stop early.
            if cmd_end == cmd_start and not cleaned.startswith("/bin/bash -lc"):
                break
            cmd_end += 1

        command_text = "\n".join(command_lines)
        if "cargo check-unsafe2" not in command_text:
            idx = max(cmd_end + 1, idx + 1)
            continue

        block_end = cmd_end + 1
        while block_end < end_idx and not AGENT_EXEC_RE.match(lines[block_end]):
            block_end += 1

        increase_lines = []
        for line_idx in range(cmd_end + 1, block_end):
            cleaned = strip_agent_prefix(lines[line_idx]).strip()
            if CHECK_UNSAFE2_INCREASE_RE.search(cleaned):
                increase_lines.append({"line": line_idx + 1, "text": cleaned})

        runs.append(
            {
                "line": cmd_start + 1,
                "command": command_text,
                "reported_increase": bool(increase_lines),
                "increase_lines": increase_lines,
            }
        )
        idx = block_end

    return runs


def parse_log(path: Path) -> dict[str, Any]:
    lines = path.read_text(errors="replace").splitlines()

    starts = [idx + 1 for idx, line in enumerate(lines) if START_MARKER in line]
    counts = [
        (idx + 1, int(match.group(1)))
        for idx, line in enumerate(lines)
        if (match := COUNT_RE.search(line))
    ]

    completed: list[dict[str, Any]] = []
    incomplete: list[dict[str, Any]] = []

    for step_index, start_line in enumerate(starts, start=1):
        start_idx = start_line - 1
        end_line = starts[step_index] if step_index < len(starts) else len(lines) + 1
        end_idx = end_line - 1
        block = lines[start_idx:end_idx]

        result_line = None
        result_hash = None
        failure_lines: list[dict[str, Any]] = []
        for offset, line in enumerate(block):
            abs_line = start_line + offset
            if RESULT_MARKER in line and result_line is None:
                result_line = abs_line
                result_hash = line.split("=", 1)[-1].strip()
            if "agent safety attempt" in line and "failed:" in line:
                failure_lines.append({"line": abs_line, "text": line.strip()})

        before_line, before_count = latest_count_before(counts, start_line)
        token_line, tokens_used, summary_lines = parse_token_and_summary(lines, start_idx, end_idx)
        check_unsafe2_runs = parse_agent_check_unsafe2_runs(lines, start_idx, end_idx)
        duration_seconds = block_duration_seconds(lines, start_idx, end_idx)

        item: dict[str, Any] = {
            "step": step_index,
            "start_line": start_line,
            "duration_seconds": duration_seconds,
            "duration": format_duration(duration_seconds),
            "before_count_line": before_line,
            "before_count": before_count,
            "token_line": token_line,
            "tokens_used": tokens_used,
            "summary_lines": summary_lines,
            "summary_text": " ".join(summary_lines),
            "failure_lines": failure_lines,
            "agent_check_unsafe2_runs": check_unsafe2_runs,
            "agent_check_unsafe2_count": len(check_unsafe2_runs),
            "agent_check_unsafe2_increase_count": sum(
                1 for run in check_unsafe2_runs if run["reported_increase"]
            ),
        }

        if result_line is not None:
            after_line, after_count = first_count_after(counts, result_line)
            item.update(
                {
                    "completed": True,
                    "result_line": result_line,
                    "result_hash": result_hash,
                    "after_count_line": after_line,
                    "after_count": after_count,
                    "delta": None
                    if before_count is None or after_count is None
                    else after_count - before_count,
                }
            )
            completed.append(item)
        else:
            item.update({"completed": False, "result_line": None, "result_hash": None})
            incomplete.append(item)

    return {
        "log": str(path),
        "line_count": len(lines),
        "start_count": len(starts),
        "completed_count": len(completed),
        "incomplete_count": len(incomplete),
        "unsafe_counts": [{"line": ln, "count": count} for ln, count in counts],
        "completed_steps": completed,
        "incomplete_steps": incomplete,
        "total_tokens_completed": sum(
            step["tokens_used"] or 0 for step in completed
        ),
        "total_duration_seconds_completed": sum(
            step["duration_seconds"] or 0 for step in completed
        ),
        "total_duration_completed": format_duration(
            sum(step["duration_seconds"] or 0 for step in completed)
        ),
        "agent_check_unsafe2_count": sum(
            step["agent_check_unsafe2_count"] for step in completed + incomplete
        ),
        "agent_check_unsafe2_increase_count": sum(
            step["agent_check_unsafe2_increase_count"] for step in completed + incomplete
        ),
    }


def emit_tsv(data: dict[str, Any]) -> str:
    columns = [
        "step",
        "start_line",
        "duration",
        "duration_seconds",
        "before_count",
        "after_count",
        "delta",
        "tokens_used",
        "result_line",
        "summary_text",
    ]
    out = ["\t".join(columns)]
    for step in data["completed_steps"]:
        out.append("\t".join("" if step.get(col) is None else str(step.get(col)) for col in columns))
    return "\n".join(out)


def emit_markdown(data: dict[str, Any]) -> str:
    out = [
        "| # | Log start | Duration | Unsafe count | Delta | Tokens used | Raw summary |",
        "|---:|---|---:|---:|---:|---:|---|",
    ]
    log_name = Path(data["log"]).name
    for idx, step in enumerate(data["completed_steps"], start=1):
        after = step.get("after_count")
        delta = step.get("delta")
        tokens = step.get("tokens_used")
        summary = (step.get("summary_text") or "").replace("|", "\\|")
        after_text = "" if after is None else str(after)
        delta_text = "" if delta is None else str(delta)
        token_text = "" if tokens is None else f"{tokens:,}"
        out.append(
            f"| {idx} | `{log_name}:{step['start_line']}` | "
            f"{step.get('duration') or ''} | {after_text} | {delta_text} | {token_text} | {summary} |"
        )
    return "\n".join(out)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("log", type=Path, help="CRISP log file to parse")
    parser.add_argument(
        "--format",
        choices=("json", "tsv", "markdown"),
        default="json",
        help="output format",
    )
    args = parser.parse_args()

    data = parse_log(args.log)
    if args.format == "json":
        print(json.dumps(data, indent=2))
    elif args.format == "tsv":
        print(emit_tsv(data))
    else:
        print(emit_markdown(data))


if __name__ == "__main__":
    main()
