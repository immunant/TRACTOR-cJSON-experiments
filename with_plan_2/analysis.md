# CRISP cJSON planned-run agent-edit analysis

Source log: `/home/legare/cJSON_lib/with_plan_2/crisp.log`

This report summarizes the CRISP safety-refactoring loop with persistent `SAFETY_PLAN.md` enabled. I used the same acceptance rule as for the non-plan run: an edit is **accepted** when `do_safety_step_agent result[0]` is a code node ID, and **rejected** when `do_safety_step_agent result[0] = None`. CRISP accepts only when the agent-produced code passes the configured build/test step and `compare_unsafe2` reports no unsafe-count regression. I also inspected `crisp-storage` MVIR nodes to summarize actual code diffs and the accepted safety-plan file.

## Overall result

- Initial unsafe count before the first agent iteration: **1359**.
- Final unsafe count: **794**.
- Net reduction across accepted changes: **565** unsafe operations.
- Final test exit code: **0**.
- Completed agent attempts: **60**.
- Accepted attempts: **21**.
- Rejected attempts: **39**.
- Termination reason: **stopping due to 5 consecutive failures**.
- Agent token usage reported by Codex: **4,264,681** tokens across **60** agent sessions (`tokens used` lines in `crisp.log`).

All code edits were in `rust/src/cJSON.rs`; accepted plan updates were in `SAFETY_PLAN.md`. The persistent plan clearly shaped the run: it kept a running target list and helped the agent progress through value predicates, numeric initialization, constructors, string/minify helpers, and then parser helpers. The run made deeper progress than `no_plan_2` in unsafe count terms, but eventually got stuck on parser refactors around `parse_buffer`, parser destination `item` arguments, and raw linked-list/tree-node operations.

The final accepted `SAFETY_PLAN.md` says the current target is parser-helper cleanup before larger ownership changes. It records completed safe conversions for value predicates/getters, `cJSON_SetNumberHelper`, `cJSON_Version`, `cJSON_InitNumber`, `cJSON_InitType`, array factory slice inputs, minify helpers, `parse_hex4`, `utf16_literal_to_utf8`, string scanning/decoding, whitespace/BOM skipping, `parse_value`, `parse_number`, `parse_array`, `parse_object`, and `parse_buffer.content` as a safe slice. It warns that converting parser `item: *mut cJSON` arguments to `&mut cJSON` is the next target but interacts with linked-list node pointers and recursive parser calls.

## Accepted edits

| # | Log line | Unsafe count | Summary |
|---:|---:|---:|---|
| 1 | 616 | 1359 -> 1333 | Converted implementation-side `cJSON_GetStringValue`, `cJSON_GetNumberValue`, and the `cJSON_Is*` predicates to safe `Option<&cJSON>` implementations, while keeping exported `_ffi` wrappers ABI-compatible. Created the initial `SAFETY_PLAN.md`. |
| 2 | 8276 | 1333 -> 1327 | Converted `cJSON_SetNumberHelper` to a safe implementation over `&mut cJSON`; the exported wrapper still accepts the raw pointer. |
| 5 | 15877 | 1327 -> 1322 | Cleaned up number-value wrapper/accessor code, including `cJSON_GetNumberValue_ffi`, without changing FFI signatures. |
| 6 | 18228 | 1322 -> 1314 | Added/used safe numeric initialization (`cJSON_InitNumber`) and routed `parse_number`, `cJSON_SetNumberHelper`, and `cJSON_CreateNumber` through safer field-setting logic. |
| 7 | 25492 | 1314 -> 1314 | Added safe type initialization (`cJSON_InitType`) and routed simple constructors (`cJSON_CreateNull`, `cJSON_CreateTrue`, `cJSON_CreateFalse`, `cJSON_CreateBool`, `cJSON_CreateArray`, `cJSON_CreateObject`) through it. This was accepted as non-regressing but did not reduce the measured count. |
| 11 | 43108 | 1314 -> 1302 | Refactored string/raw/reference constructors (`cJSON_CreateString`, `cJSON_CreateStringReference`, `cJSON_CreateObjectReference`, `cJSON_CreateArrayReference`, `cJSON_CreateRaw`) and related array-addition code using the existing safe init helpers. |
| 13 | 50347 | 1302 -> 1282 | Converted numeric/string array factory implementations toward slice-based inputs while preserving exported raw pointer/count wrappers. Touched `cJSON_CreateIntArray_ffi`, `cJSON_CreateFloatArray_ffi`, `cJSON_CreateDoubleArray_ffi`, and `cJSON_CreateStringArray_ffi`. |
| 15 | 61965 | 1282 -> 1274 | Reduced unsafe in print-buffer offset maintenance, centered on `update_offset` and adjacent `sscanf`/buffer code. |
| 17 | 72712 | 1274 -> 1160 | Major reduction in `cJSON_Duplicate_rec_ffi` / recursive duplication logic. This was the largest single accepted improvement in the planned run. |
| 24 | 109569 | 1160 -> 1155 | Small cleanup in print-buffer / print / `cJSON_malloc_ffi`-adjacent code. |
| 25 | 113029 | 1155 -> 1134 | Cleaned comparison/printing helpers, including `case_insensitive_strcmp` and `print`. |
| 26 | 117024 | 1134 -> 1134 | Accepted non-regressing cleanup around low-level helper imports/usages (`memcpy`, `lconv`, `cJSON_New_Item`). |
| 28 | 123923 | 1134 -> 1115 | Refactored number/string formatting and Unicode conversion area, including `print_number` and `utf16_literal_to_utf8`. |
| 29 | 127686 | 1115 -> 1078 | Converted `parse_hex4` and parts of `parse_string` to safer slice/index based handling. |
| 30 | 135715 | 1078 -> 1026 | Large follow-up conversion of `utf16_literal_to_utf8`, replacing raw input/output pointer traversal with safer slice-based logic. |
| 31 | 146255 | 1026 -> 1026 | Accepted non-regressing follow-up in the Unicode conversion area; it mainly reorganized `utf16_literal_to_utf8` without changing the measured unsafe count. |
| 33 | 159756 | 1026 -> 1004 | Extracted/refined safe string scanning and output helpers around `utf16_literal_to_utf8`, `scan_json_string`, and `print_string`. |
| 36 | 174852 | 1004 -> 943 | Removed remaining unsafe/import surface in string comparison and print-preallocation paths, including `strcmp` and `cJSON_PrintPreallocated_ffi`. |
| 37 | 179374 | 943 -> 924 | Converted `parse_number` input scanning and temporary number-buffer handling to safer slice/index operations while preserving allocator/locale behavior. |
| 38 | 183921 | 924 -> 825 | Converted `parse_array` and `parse_object` delimiter/current-byte checks to safe slice reads, leaving allocation and linked-list pointer manipulation for later. |
| 40 | 218499 | 825 -> 794 | Final accepted edit. Converted parser input storage and helper signatures: `parse_buffer.content` became a safe slice, and `parse_number`, `decode_json_string`, `parse_string`, `parse_value`, `parse_array`, `parse_object`, whitespace/BOM helpers, print value helpers, and parse entry code were adjusted around that boundary. |

## Rejected edits

| # | Log line | Unsafe count stayed | Attempted area | Rejection reason |
|---:|---:|---:|---|---|
| 3 | 10506 | 1327 | `cJSON_CreateNumber` using safe numeric helper. | `cJSON_CreateNumber` unsafe function calls increased `1 -> 2`. |
| 4 | 13140 | 1327 | Retried `cJSON_CreateNumber` / `cJSON_SetNumberHelper` cleanup. | `cJSON_CreateNumber` unsafe function calls increased `1 -> 2`. |
| 8 | 28615 | 1314 | New string/reference/child init helpers for constructors. | New helper signatures introduced raw pointer types: `cJSON_InitOwnedValueString`, `cJSON_InitValueStringReference`, closure raw pointer params, and `cJSON_InitChildReference`. |
| 9 | 32978 | 1314 | Retried valuestring/reference/child init helpers plus add-item/object paths. | New `cJSON_InitValuestring`, `cJSON_InitValuestringReference`, closure raw pointer params, and `cJSON_InitChildReference` raw pointer signatures. |
| 10 | 38144 | 1314 | Retried owned string/reference child initialization helpers. | New `cJSON_InitOwnedValueString`, `cJSON_InitStringReference`, closure raw pointer params, and `cJSON_InitChildReference` raw pointer signatures. |
| 12 | 46407 | 1302 | Array factory rewrite across int/float/double/string arrays. | `check_unsafe2` / rustc wrapper panicked with `not yet implemented`; no normal increased-count diagnostic was produced. |
| 14 | 56804 | 1282 | Broad wrapper/constructor cleanup around parse entry, references, constructors, and duplication. | Raw pointer derefs increased in many functions: `cJSON_ParseWithLengthOpts` `7 -> 8`, `create_reference` `6 -> 7`, simple constructors `1 -> 2`, and more. |
| 16 | 69420 | 1274 | `printbuffer` / parse input global-error cleanup. | `GLOBAL_ERROR_JSON` gained a raw pointer type in its signature. |
| 18 | 82045 | 1160 | `print_number` / `utf16_literal_to_utf8` cleanup. | `utf16_literal_to_utf8` raw pointer derefs increased `10 -> 18`; unsafe calls increased `13 -> 17`. |
| 19 | 87260 | 1160 | Retried `utf16_literal_to_utf8` cleanup. | Same `utf16_literal_to_utf8` regression: derefs `10 -> 18`, unsafe calls `13 -> 17`. |
| 20 | 91104 | 1160 | Retried `utf16_literal_to_utf8` / print-number cleanup. | Same `utf16_literal_to_utf8` regression: derefs `10 -> 18`, unsafe calls `13 -> 17`. |
| 21 | 94941 | 1160 | `printbuffer` / parser global-error cleanup. | `GLOBAL_ERROR_JSON` gained a raw pointer type in its signature. |
| 22 | 100318 | 1160 | `parse_string` cleanup. | `parse_string` raw pointer derefs increased `40 -> 41`; unsafe calls increased `32 -> 34`. |
| 23 | 105083 | 1160 | Another `utf16_literal_to_utf8` rewrite. | `utf16_literal_to_utf8` derefs increased `10 -> 18`; unsafe calls increased `13 -> 17`. |
| 27 | 119418 | 1134 | Parser/global-error cleanup. | `GLOBAL_ERROR_JSON` gained a raw pointer type in its signature. |
| 32 | 155666 | 1026 | `internal_hooks`, `printbuffer`, `utf16_literal_to_utf8`, `scan_json_string`, parse entry. | `GLOBAL_ERROR_JSON` gained a raw pointer type in its signature. |
| 34 | 164423 | 1004 | `parse_string` / print string pointer cleanup. | `print_string_ptr` introduced a `strlen` foreign-function use (`0 -> 1`). |
| 35 | 170979 | 1004 | Parser global-error cleanup. | `global_error_json` gained a raw pointer type in its signature. |
| 39 | 189760 | 825 | Large print/parser rewrite across set-valuestring, number/string printing, buffered/preallocated printing, parse/print array/object. | `ensure` unsafe calls increased `8 -> 10`; `print_string_ptr` introduced `strlen` foreign-function use (`0 -> 1`). |
| 41 | 231896 | 794 | Parser helper `item` / parse-object conversion attempt. | `cJSON_ParseWithLengthOpts` raw pointer derefs increased `6 -> 7`; `parse_object` derefs increased `7 -> 8`. |
| 42 | 241468 | 794 | Retried parser helper destination conversion. | Same parser regression: `cJSON_ParseWithLengthOpts` `6 -> 7`, `parse_object` `7 -> 8`. |
| 43 | 249901 | 794 | Retried parser helper destination conversion. | Same parser regression: `cJSON_ParseWithLengthOpts` `6 -> 7`, `parse_object` `7 -> 8`. |
| 44 | 254460 | 794 | Retried parser helper conversion with `parse_string` / parse entry changes. | Same parser regression: `cJSON_ParseWithLengthOpts` `6 -> 7`, `parse_object` `7 -> 8`. |
| 45 | 259841 | 794 | Parser helper conversion plus add-item/has-object-item touches. | `cJSON_ParseWithLengthOpts` `6 -> 7`, `parse_object` `7 -> 8`, and `add_item_to_array` derefs increased `8 -> 10`. |
| 46 | 269190 | 794 | Retried parser helper destination conversion. | `cJSON_ParseWithLengthOpts` `6 -> 7`, `parse_object` `7 -> 8`. |
| 47 | 274153 | 794 | Retried parser helper destination conversion following the plan. | `cJSON_ParseWithLengthOpts` `6 -> 7`, `parse_object` `7 -> 8`. |
| 48 | 278744 | 794 | Parser helper conversion with print-value/array/object adjustments. | `cJSON_ParseWithLengthOpts` `6 -> 7`, `parse_object` `7 -> 8`. |
| 49 | 282904 | 794 | Another parser helper conversion attempt. | `cJSON_ParseWithLengthOpts` `6 -> 7`, `parse_object` `7 -> 8`. |
| 50 | 288648 | 794 | `printbuffer` / parser global-error cleanup. | `GLOBAL_ERROR_JSON` gained a raw pointer type in its signature. |
| 51 | 295937 | 794 | Retried parser helper destination conversion. | `cJSON_ParseWithLengthOpts` `6 -> 7`, `parse_object` `7 -> 8`. |
| 52 | 300987 | 794 | Parser helper conversion with print-value/array/object adjustments. | `cJSON_ParseWithLengthOpts` raw pointer derefs increased `6 -> 7`. |
| 53 | 306751 | 794 | Retried parser helper destination conversion. | `cJSON_ParseWithLengthOpts` `6 -> 7`, `parse_object` `7 -> 8`. |
| 54 | 312412 | 794 | Retried parser helper destination conversion. | `cJSON_ParseWithLengthOpts` `6 -> 7`, `parse_object` `7 -> 8`. |
| 55 | 317275 | 794 | `tolower` / `printbuffer` / parse entry cleanup. | `GLOBAL_ERROR_JSON` gained a raw pointer type in its signature. |
| 56 | 320104 | 794 | Parser helper conversion with print-value/array/object adjustments. | `cJSON_ParseWithLengthOpts` `6 -> 7`, `parse_object` `7 -> 8`. |
| 57 | 325686 | 794 | Parser destination conversion attempt; candidate plan said it converted `parse_number`, `parse_string`, `parse_value`, `parse_array`, and `parse_object` `item` args to `&mut cJSON`. | `cJSON_ParseWithLengthOpts` `6 -> 7`, `parse_object` `7 -> 8`. |
| 58 | 330762 | 794 | Retried parser destination conversion. | `cJSON_ParseWithLengthOpts` raw pointer derefs increased `6 -> 7`. |
| 59 | 336633 | 794 | Retried parser destination conversion. | `cJSON_ParseWithLengthOpts` `6 -> 7`, `parse_object` `7 -> 8`. |
| 60 | 342859 | 794 | Final attempt, again on parser destination conversion under the stronger “must reduce unsafe” suffix. | `cJSON_ParseWithLengthOpts` raw pointer derefs increased `6 -> 7`; CRISP then stopped after five consecutive failures. |

## Plan Behavior

The accepted plan file was useful early and mid-run. It prevented the agent from repeatedly rediscovering already-completed work and documented important pitfalls, including:

- preserve exported `#[export_name]` signatures;
- avoid adding helper structs with raw pointer fields or unsafe `Drop` before there is a broader allocator/string ownership design;
- avoid `raw_ptr.as_mut()` in implementation functions because earlier attempts increased unsafe calls;
- keep parser failure-offset and depth behavior compatible with the C translation;
- keep raw pointer/count validation behavior in exported array wrappers.

However, once the run reached 794 unsafe operations, the same persistent target also kept steering the agent back to parser destination-argument conversion. Attempts 41-49 and 51-60 repeatedly produced candidate edits around `cJSON_ParseWithLengthOpts`, `parse_value`, `parse_array`, and `parse_object`; `check_unsafe2` rejected them because they added a raw pointer dereference at the parse boundary and often another in `parse_object`. Attempts 50 and 55 tried a different global-error route but were rejected because `GLOBAL_ERROR_JSON`/`global_error_json` acquired raw pointer types.

## Main Takeaways

- The planned run removed **565** unsafe operations, compared with **480** in the non-plan run, and ended at **794** instead of **879**.
- The persistent plan improved continuity for multi-step parser/string refactors and recorded useful “do not repeat” notes.
- The final stall was not due to tests: the final accepted code still passed tests. It was due to `check_unsafe2` rejecting repeated parser-boundary rewrites for increasing raw pointer dereferences or raw pointer type surface.
- The likely next successful work needs to avoid the tempting parser `item: *mut cJSON -> &mut cJSON` conversion unless it can be done without adding new boundary dereferences counted by `check_unsafe2`, or it needs to tackle a different remaining cluster first, such as print helpers or allocator/string ownership, with a design that does not introduce raw-pointer helper fields.
