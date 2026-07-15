# with_tests_1 CRISP run summary

This summary covers the completed CRISP-level safety-loop edits in
`run_20260714_225119.log`. Each row is the final edit returned by one agent
invocation, rather than its intermediate sandbox attempts.

- Total accepted edits: 32
- Total rejected edits: 0
- Total tokens used: 2,945,590
- Mean tokens per completed CRISP-level row: 92,050
- Median tokens per completed CRISP-level row: 77,245
- Total completed-step runtime: 1:04:30
- Initial unsafe count: 1,359
- Final unsafe count: 722
- Net unsafe operations removed by accepted edits: 637
- Average unsafe delta per completed CRISP-level row: mean `-19.91`, median `-11.5`
- Omitted from the table: the trailing invocation beginning at
  `run_20260714_225119.log:202217` is incomplete; the log ends three seconds
  into the agent's initial source inspection, before it returned an edit.

All 32 returned edits were accepted by CRISP. Six intermediate agent-side
`cargo check-unsafe2` runs reported increases, but the agent revised or
discarded those approaches before returning the final edits represented here.
The translated-library test command reported all 18 C test executables passing
throughout the completed rows.

| # | Log start | Duration | Unsafe count | Delta | Tokens used | Final edit summary | Result |
|---:|---|---:|---:|---:|---:|---|---|
| 1 | `run_20260714_225119.log:25` | 1:17 | 1351 | -8 | 85,271 | Made `compare_double` safe and replaced the `fabs` FFI call with `f64::abs()`. | accepted |
| 2 | `run_20260714_225119.log:4528` | 2:02 | 1332 | -19 | 64,639 | Converted `parse_hex4` to a safe four-byte slice helper and updated both UTF-16 decoding call sites. | accepted |
| 3 | `run_20260714_225119.log:8556` | 3:13 | 1328 | -4 | 87,411 | Replaced `tolower` FFI use in `case_insensitive_strcmp` with Rust ASCII lowercasing. | accepted |
| 4 | `run_20260714_225119.log:12681` | 3:07 | 1302 | -26 | 109,792 | Converted string/number accessors and all `cJSON_Is*` predicates to safe `Option<&cJSON>` implementations behind unchanged FFI wrappers. | accepted |
| 5 | `run_20260714_225119.log:23065` | 1:39 | 1297 | -5 | 63,196 | Made `cJSON_Version` a safe immutable static string helper, removing `static mut` and `sprintf`. | accepted |
| 6 | `run_20260714_225119.log:27179` | 1:56 | 1291 | -6 | 138,290 | Converted `cJSON_SetNumberHelper` to a safe `Option<&mut cJSON>` implementation while preserving the exported ABI and saturation behavior. | accepted |
| 7 | `run_20260714_225119.log:32399` | 3:02 | 1279 | -12 | 117,644 | Converted case-insensitive comparison to a safe `CStr` helper and removed libc `strcmp` from object lookup. | accepted |
| 8 | `run_20260714_225119.log:37294` | 2:53 | 1274 | -5 | 107,415 | Removed `cast_away_const` and used direct const-to-mut casts only in the existing unsafe reference-assignment paths. | accepted |
| 9 | `run_20260714_225119.log:44774` | 3:13 | 1160 | -114 | 187,477 | Rewrote `cJSON_Minify` and its comment/string helpers with safe slice and index logic. | accepted |
| 10 | `run_20260714_225119.log:55752` | 1:58 | 1147 | -13 | 56,941 | Replaced mutable global parse-error state with a `Mutex`, storing the JSON address as `usize` and converting it at the FFI boundary. | accepted |
| 11 | `run_20260714_225119.log:60883` | 1:14 | 1142 | -5 | 119,079 | Replaced `print`'s local mutable static default buffer size with an immutable local value. | accepted |
| 12 | `run_20260714_225119.log:64417` | 3:36 | 1105 | -37 | 115,367 | Made `utf16_literal_to_utf8` a safe slice/index helper and used a bounded mutable output slice in that part of `parse_string`. | accepted |
| 13 | `run_20260714_225119.log:80565` | 4:24 | 1021 | -84 | 143,087 | Replaced mutable global allocator hooks with a `Mutex`, safe snapshot/set helpers, and an FFI-only raw hook boundary. | accepted |
| 14 | `run_20260714_225119.log:108356` | 3:26 | 1011 | -10 | 191,798 | Converted `suffix_object` to a safe `&mut cJSON` helper and updated array append/constructor callers. | accepted |
| 15 | `run_20260714_225119.log:121969` | 2:07 | 1009 | -2 | 200,758 | Removed the private `cJSON_malloc` implementation and moved hook dispatch into its exported FFI wrapper. | accepted |
| 16 | `run_20260714_225119.log:127114` | 3:34 | 973 | -36 | 121,758 | Removed nine private unsafe `cJSON_Add*ToObject` trampolines and moved their bodies into exported wrappers. | accepted |
| 17 | `run_20260714_225119.log:137498` | 1:11 | 967 | -6 | 54,337 | Removed the private item-reference add trampolines for arrays and objects. | accepted |
| 18 | `run_20260714_225119.log:140797` | 1:07 | 961 | -6 | 53,624 | Removed private `cJSON_AddItemToArray`, `cJSON_AddItemToObject`, and `cJSON_AddItemToObjectCS` trampolines. | accepted |
| 19 | `run_20260714_225119.log:144641` | 1:35 | 950 | -11 | 61,398 | Removed private array/object query trampolines and moved their behavior into exported FFI wrappers. | accepted |
| 20 | `run_20260714_225119.log:150222` | 1:20 | 912 | -38 | 67,755 | Removed private detach/delete trampolines and moved their behavior into exported FFI wrappers. | accepted |
| 21 | `run_20260714_225119.log:153834` | 1:17 | 900 | -12 | 56,273 | Removed private string, object, and array reference constructors in favor of their exported wrappers. | accepted |
| 22 | `run_20260714_225119.log:157677` | 1:24 | 890 | -10 | 68,431 | Removed the four private `cJSON_Print*` convenience trampolines and retained their behavior in exported wrappers. | accepted |
| 23 | `run_20260714_225119.log:161507` | 1:34 | 871 | -19 | 78,192 | Collapsed the private `cJSON_SetValuestring` implementation into its exported FFI entry point. | accepted |
| 24 | `run_20260714_225119.log:164316` | 1:21 | 864 | -7 | 49,875 | Removed private parse convenience wrappers for `cJSON_Parse`, `cJSON_ParseWithOpts`, and `cJSON_ParseWithLength`. | accepted |
| 25 | `run_20260714_225119.log:167316` | 1:19 | 848 | -16 | 51,688 | Moved array insertion and replacement implementation bodies into exported FFI wrappers. | accepted |
| 26 | `run_20260714_225119.log:170308` | 1:37 | 832 | -16 | 70,369 | Removed object-replacement trampolines and moved their logic into exported FFI wrappers. | accepted |
| 27 | `run_20260714_225119.log:174309` | 1:15 | 808 | -24 | 81,861 | Moved `cJSON_ReplaceItemViaPointer` into its exported wrapper and updated related replacement callers. | accepted |
| 28 | `run_20260714_225119.log:177965` | 1:19 | 766 | -42 | 76,297 | Moved typed array constructors into exported wrappers and removed the private `cJSON_Duplicate` trampoline. | accepted |
| 29 | `run_20260714_225119.log:182775` | 1:16 | 764 | -2 | 48,610 | Removed the private `cJSON_free` trampoline and dispatched through allocator hooks only from FFI wrappers. | accepted |
| 30 | `run_20260714_225119.log:185669` | 1:03 | 755 | -9 | 64,119 | Moved true, false, and boolean constructors into exported wrappers and updated object-add callers. | accepted |
| 31 | `run_20260714_225119.log:189054` | 1:33 | 725 | -30 | 81,080 | Moved null, number, string, raw, array, and object constructors into exported FFI wrappers. | accepted |
| 32 | `run_20260714_225119.log:196396` | 1:38 | 722 | -3 | 71,758 | Removed private `get_array_item`; raw traversal now resides only in the exported array access/detach/insert/replace wrappers. | accepted |
