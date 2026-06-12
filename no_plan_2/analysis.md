# CRISP cJSON agent-edit analysis

Source log: `/home/legare/cJSON_lib/no_plan_2/crisp.log`

This report summarizes the CRISP safety-refactoring loop recorded in the log. I treated an edit as **accepted** when `do_safety_step_agent result[0]` was a code node ID, because CRISP only returns a node there after the agent-produced code passes the configured build/test step and `compare_unsafe2` reports no unsafe-count regression. I treated an edit as **rejected** when `do_safety_step_agent result[0] = None`. Two invocations started but did not reach a CRISP verdict before the log moved on or ended.

## Overall result

- Initial unsafe count before the first agent iteration: **1359**.
- Last completed accepted state in the log: **879** unsafe operations.
- Net reduction across accepted changes: **480** unsafe operations.
- Completed agent attempts with CRISP verdicts: **49**.
- Accepted attempts: **31**.
- Rejected attempts: **18**.
- Started but no CRISP verdict in the log: **2**.
- Agent token usage reported by Codex: **4,054,926** tokens across **49** agent sessions (`tokens used` lines in `crisp.log`).

All code edits were in `rust/src/cJSON.rs`. The early accepted work mainly converted query/predicate helpers and simple constructors to safe Rust wrappers. The largest accepted reductions came from replacing `static mut global_hooks` with a mutex-backed global, removing unsafe from `cJSON_Duplicate_rec`, and cleaning up object/array creation paths. The dominant rejected pattern was repeated attempted refactoring of `get_object_item` and related object lookup functions that moved raw-pointer work into closures or helper calls in a way `check_unsafe2` counted as new raw pointer dereferences, unsafe calls, or raw pointer types.

## Accepted edits

| # | Log line | Unsafe count | Summary |
|---:|---:|---:|---|
| 1 | 616 | 1359 -> 1333 | Converted `cJSON_GetStringValue`, `cJSON_GetNumberValue`, and the `cJSON_Is*` predicates to safe `Option<&cJSON>` implementations. Added a shared safe `cJSON_IsType` helper while keeping exported `_ffi` wrappers raw-pointer-compatible. |
| 2 | 4260 | 1333 -> 1327 | Small follow-up cleanup in the same predicate/value-access area, preserving the FFI wrappers and reducing remaining unsafe operations. |
| 3 | 6568 | 1327 -> 1322 | Further simplified `cJSON_GetNumberValue` and nearby wrapper code to avoid unnecessary unsafe implementation operations. |
| 4 | 8738 | 1322 -> 1314 | Refactored printing-buffer offset handling around `update_offset` and related helper code. CRISP accepted the final candidate despite noisy intermediate checker output in the agent transcript. |
| 5 | 12201 | 1314 -> 1309 | Narrow cleanup in `print`, removing a small amount of unsafe implementation surface. |
| 6 | 15469 | 1309 -> 1296 | Changed parse/print buffer hook handling, including `internal_hooks`, `printbuffer`, and `cJSON_ParseWithLengthOpts`, to pass hook values more safely. |
| 7 | 18271 | 1296 -> 1285 | Cleaned up number parsing/creation and array-addition paths, including `parse_number`, `cJSON_CreateNumber`, and `cJSON_AddItemToArray_ffi`. |
| 9 | 25059 | 1285 -> 1279 | Refactored object/array creation entry points and numeric array constructors (`cJSON_CreateObject_ffi`, `cJSON_CreateIntArray_ffi`, `cJSON_CreateFloatArray_ffi`, `cJSON_CreateDoubleArray_ffi`). |
| 10 | 30625 | 1279 -> 1188 | Replaced `static mut global_hooks` with `Mutex<internal_hooks>`, added `lock_global_hooks` / `current_global_hooks`, and updated many users (`cJSON_InitHooks`, allocation/deallocation, parse, print, add-item, create helpers) to take hook snapshots instead of using `&raw mut global_hooks`. This was the largest early reduction. |
| 12 | 58447 | 1188 -> 1167 | Reduced unsafe operations in UTF-16/UTF-8 string literal conversion, centered on `utf16_literal_to_utf8`. |
| 13 | 62698 | 1167 -> 1160 | Cleaned up `printbuffer`, `cJSON_Version_ffi`, case-insensitive comparison, and `cJSON_malloc_ffi`-adjacent code. |
| 15 | 70653 | 1160 -> 1142 | Converted more raw pointer boundaries to safer internal forms around `cJSON_ParseWithLengthOpts_ffi`, `get_array_item`, `get_object_item`, `add_item_to_array`, detach-object helpers, and `cJSON_CreateStringArray_ffi`. |
| 17 | 83216 | 1142 -> 1139 | Small accepted cleanup touching memory/string helper imports and call sites (`memcpy`, `malloc`, `case_insensitive_strcmp`, `cJSON_New_Item`, `print`). |
| 24 | 111515 | 1139 -> 1135 | Refactored string replacement/comparison paths, including `cJSON_SetValuestring`, `replace_item_in_object`, and `cJSON_Compare_ffi`. |
| 26 | 120521 | 1135 -> 1130 | Reduced unsafe around `cJSON_strdup` and `cJSON_InitHooks`. |
| 27 | 124057 | 1130 -> 1127 | Continued constructor cleanup in `cJSON_CreateObject_ffi`, `cJSON_CreateIntArray_ffi`, and `cJSON_CreateFloatArray_ffi`. |
| 28 | 128409 | 1127 -> 1120 | Cleaned up `cJSON_CreateDoubleArray_ffi` and related string-array construction handling. |
| 30 | 135890 | 1120 -> 1118 | Added/refined safe C-string comparison handling (`case_insensitive_cstr_cmp`) and touched `cJSON_Version_ffi` / hook initialization. |
| 31 | 141376 | 1118 -> 1004 | Major refactor of `cJSON_Duplicate_rec_ffi` / duplicate-recursive logic, removing a large amount of unsafe recursive-copy machinery. |
| 32 | 149393 | 1004 -> 983 | Cleaned replacement/object-case-sensitive and raw-string creation paths (`cJSON_ReplaceItemInObjectCaseSensitive_ffi`, `cJSON_CreateRaw_ffi`). |
| 33 | 155491 | 983 -> 983 | Accepted non-regressing cleanup around add-item and reference constructors (`cJSON_AddItemToArray_ffi`, `add_item_to_object`, `cJSON_CreateStringReference`, `cJSON_CreateObjectReference`, `cJSON_CreateArrayReference`). It did not reduce the measured count. |
| 34 | 159145 | 983 -> 969 | Refactored array/object query helpers and reference creation (`cJSON_GetArraySize_ffi`, `cJSON_HasObjectItem_ffi`, `create_reference`). |
| 35 | 167006 | 969 -> 965 | Further hook/string allocation cleanup using `current_global_hooks`, touching `cJSON_GetErrorPtr_ffi`, `cJSON_SetValuestring`, `add_item_to_object`, `replace_item_in_object`, `cJSON_CreateString`, `cJSON_CreateRaw`, and `cJSON_Duplicate_rec`. |
| 36 | 173978 | 965 -> 957 | Refactored string/raw object-addition and number/string-array creation paths (`cJSON_AddStringToObject`, `cJSON_AddRawToObject`, `cJSON_CreateNumber_ffi`, `cJSON_CreateStringArray`). |
| 37 | 184646 | 957 -> 955 | Small accepted cleanup; the diff was minimal and did not expose a clear named Rust function in the hunk context. |
| 39 | 192898 | 955 -> 923 | Broad cleanup across hook initialization, parsing arrays/objects, suffix object handling, object replacement, and primitive/string/raw/reference constructors. |
| 42 | 204317 | 923 -> 922 | Narrow accepted cleanup across `cJSON_InitHooks_ffi`, `cJSON_New_Item`, `cJSON_SetNumberHelper`, parse helpers, `create_reference`, object replacement, and basic constructors. |
| 45 | 218712 | 922 -> 897 | Significant constructor cleanup centered on `cJSON_New_Item`, `cJSON_CreateObject_ffi`, and numeric array constructors. |
| 47 | 233759 | 897 -> 896 | Small accepted cleanup in object-item case-sensitive and object-has-item FFI wrappers. |
| 48 | 242653 | 896 -> 885 | Refactored comparison and raw-type helpers (`cJSON_IsRaw_ffi`, `cJSON_Compare`, `cJSON_Compare_ffi`), including safe child/next/name/value access helpers. |
| 50 | 253965 | 885 -> 879 | Final completed accepted edit in the log. Refactored object lookup/detach/replace/compare paths, including `case_insensitive_cstr_cmp`, `cJSON_GetArrayItem_ffi`, `cJSON_HasObjectItem`, detach-object helpers, `replace_item_in_object`, and `cJSON_Compare`. |

## Rejected edits

| # | Log line | Unsafe count stayed | Attempted change | CRISP rejection reason |
|---:|---:|---:|---|---|
| 8 | 22568 | 1285 | Tried another accessor-style cleanup around `cJSON_GetErrorPtr_ffi` / string-value handling. | `cJSON_GetStringValue_ffi::{closure#0}` introduced raw pointer types in the closure signature. |
| 11 | 54481 | 1188 | Tried to refactor `cJSON_Version_ffi` and `get_object_item`. | `get_object_item` raw pointer dereferences increased `7 -> 8`; unsafe function calls increased `2 -> 3`. |
| 14 | 67054 | 1160 | Tried comparison/accessor cleanup involving `cJSON_GetErrorPtr_ffi` and case-insensitive comparison. | `cJSON_GetStringValue_ffi::{closure#0}` introduced raw pointer types in the closure signature. |
| 16 | 78774 | 1142 | Tried another `get_object_item` cleanup. | `get_object_item` unsafe function calls increased `2 -> 3`. |
| 18 | 88178 | 1139 | Retried `get_object_item` refactoring. | `get_object_item` raw pointer dereferences increased `7 -> 8`; unsafe function calls increased `2 -> 3`. |
| 19 | 90838 | 1139 | Retried `get_object_item` refactoring. | `get_object_item` raw pointer dereferences increased `7 -> 9`; unsafe function calls increased `2 -> 3`. |
| 20 | 94061 | 1139 | Retried `get_object_item` refactoring. | `get_object_item` raw pointer dereferences increased `7 -> 8`; unsafe function calls increased `2 -> 3`. |
| 21 | 97429 | 1139 | Retried `get_object_item` refactoring. | `get_object_item` raw pointer dereferences increased `7 -> 8`; unsafe function calls increased `2 -> 3`. |
| 22 | 104209 | 1139 | Retried `get_object_item` refactoring. | `get_object_item` unsafe function calls increased `2 -> 3`. |
| 23 | 108414 | 1139 | Larger `get_object_item` rewrite attempt. | `get_object_item` raw pointer dereferences increased `7 -> 8`; unsafe function calls increased `2 -> 3`. |
| 25 | 116187 | 1135 | Retried `get_object_item` rewrite after the accepted string/compare cleanup. | `get_object_item` unsafe function calls increased `2 -> 3`. |
| 29 | 132528 | 1120 | Tried to use a closure/helper inside `get_object_item`. | `get_object_item` unsafe function calls increased `2 -> 3`; the new closure added one raw pointer dereference and one unsafe call. |
| 38 | 189275 | 955 | Tried to refactor array item access and array detach/insert/replace helpers. | `get_array_item` unsafe calls increased `0 -> 1`; new closures in `cJSON_GetArrayItem_ffi` and array mutation helpers introduced raw pointer types / extra unsafe calls. |
| 41 | 201409 | 923 | After a log restart at the same unsafe count, tried a smaller `case_insensitive_cstr_cmp` / `get_object_item` cleanup. | `get_object_item` raw pointer dereferences increased `7 -> 8`; unsafe function calls increased `1 -> 3`. |
| 43 | 212027 | 922 | Retried `get_object_item` cleanup. | `get_object_item` raw pointer dereferences increased `7 -> 8`; unsafe function calls increased `1 -> 3`. |
| 44 | 215254 | 922 | Retried `get_object_item` cleanup with a slightly larger diff. | `get_object_item` raw pointer dereferences increased `7 -> 8`; unsafe function calls increased `1 -> 3`. |
| 46 | 225004 | 897 | Tried broader object lookup/detach/replace/compare refactoring. | Multiple increases: `get_object_item` unsafe calls `1 -> 2`, `cJSON_HasObjectItem` unsafe calls `1 -> 3`, detach-object helpers unsafe calls `2 -> 4`, plus related object replacement/comparison regressions. |
| 49 | 249728 | 885 | Retried `case_insensitive_cstr_cmp` / `get_object_item` cleanup. | `get_object_item` raw pointer dereferences increased `7 -> 8`; unsafe function calls increased `1 -> 3`. |

## Started but not completed in the log

| # | Log line | Unsafe count | What happened |
|---:|---:|---:|---|
| 40 | 200482 | 923 | An agent invocation began, scanned the crate, then the log shows CRISP starting/restarting from the same node before a `do_safety_step_agent` verdict for this invocation. I did not count this as accepted or rejected. |
| 51 | 266641 | 879 | A final agent invocation began at the end of the log and reached only the initial scan / `cargo check-unsafe2` setup before EOF. No edit or CRISP verdict is visible. |

## Repeated failure pattern

Most rejected attempts tried to make `get_object_item` more ergonomic by using closures, `Option`, `NonNull`, or helper conversion at call sites. Those attempts often looked directionally safer in normal Rust style, but `check_unsafe2` rejected them because the transformation introduced new unsafe-adjacent surfaces that were not present in the previous baseline: closure signatures containing raw pointers, extra `as_ref` / pointer-derived calls, or additional dereferences inside `get_object_item` itself.

The successful attempts generally worked when they either:

- moved null checks and raw pointer conversion cleanly into already-unsafe FFI wrappers;
- converted implementation functions to `Option<&T>`, slices, or `CStr` without adding raw-pointer closures;
- replaced global mutable state with a safe synchronization primitive; or
- removed whole unsafe-heavy helper bodies rather than wrapping the same unsafe operations in new helpers.
