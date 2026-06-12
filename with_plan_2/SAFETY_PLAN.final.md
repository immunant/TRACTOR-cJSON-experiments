Target: incremental implementation-side unsafe reductions in `rust/src/cJSON.rs`, currently focused on parser helpers before larger ownership changes.

Completed:
- Earlier iterations made value predicates/getters, `cJSON_SetNumberHelper`, and `cJSON_Version` safe implementation functions while preserving exported wrapper ABI.
- Earlier iterations added safe `cJSON_InitNumber(&mut cJSON, f64)` / `cJSON_InitType(&mut cJSON, c_int)` helpers and routed constructors through them after allocation.
- Earlier iterations converted numeric/string array factory implementation functions to slice inputs; exported wrappers keep raw pointer/count ABI.
- Earlier iterations made minify helpers and implementation `cJSON_Minify` safe over c_char slices; the exported wrapper still converts the C string pointer to a slice.
- Earlier iterations removed unnecessary unsafe around `global_hooks`, the module `NULL` raw pointer constant, `memset`, and the `fabs` FFI import.
- Earlier iteration converted `parse_hex4` to a safe slice helper.
- Earlier iteration converted `utf16_literal_to_utf8` from an `unsafe extern "C"` raw input/output pointer helper into a safe helper taking input/output slices plus a mutable output offset.
- Earlier iteration changed `parse_string`'s output buffer writes to use a safe mutable slice and offset for decoded string bytes, including unicode escape output.
- Earlier iteration changed `parse_string`'s input scan/decode traversal from raw pointer arithmetic/dereferences to a safe input slice plus byte indices. It preserves the old failure-offset behavior by keeping `input_position` at the same logical byte.
- Earlier iteration extracted `scan_json_string` and `decode_json_string` as safe slice-based helpers. `parse_string` now keeps raw work to hook allocation/deallocation, output raw-slice conversion, and assigning the allocated string pointer into the item.
- Earlier iteration rewrote `buffer_skip_whitespace` and `skip_utf8_bom` to convert the raw `parse_buffer.content` pointer to a slice once, then use safe indexing/slice comparison. This removed repeated raw pointer dereferences/offsets in both helpers and removed the `strncmp` call from `skip_utf8_bom`.
- Earlier iteration converted `parse_value`'s literal and leading-byte dispatch to a single `parse_buffer` reference plus safe slice/byte checks. This removed the remaining `strncmp` uses and the `strncmp` FFI import.
- Earlier iteration converted `parse_number`'s input scan, temporary number-buffer copy, NUL terminator write, and decimal-point replacement to safe slice indexing over the parse input and hook-allocated temporary buffer. Hook allocation/deallocation and `strtod` remain unchanged to preserve allocator and locale behavior.
- Earlier iteration converted `parse_array` and `parse_object` delimiter/current-byte checks from repeated raw `content.offset(offset)` dereferences to one input slice plus safe `get(...).copied()` checks. Node allocation, linked-list pointer manipulation, child cleanup, and nested parser calls remain unchanged.
- Latest iteration changed internal `parse_buffer` storage from raw `content: *const c_uchar` plus length to safe `content: &[c_uchar]` plus the existing length mirror, and updated `parse_number`, `parse_string`, `parse_value`, `parse_array`, `parse_object`, `buffer_skip_whitespace`, and `skip_utf8_bom` to take `&mut parse_buffer` instead of `*mut parse_buffer`. The only raw-to-slice conversion for parser input now happens at the parse boundary in `cJSON_ParseWithLengthOpts`.
- Verified with `cargo build --manifest-path rust/Cargo.toml`.
- Verified with `cargo check-unsafe2 --manifest-path rust/Cargo.toml`.

Remaining / next steps:
- Continue reducing parser helper unsafe now that parse input is slice-backed. The next parser signature target is the `item: *mut cJSON` argument in `parse_number`, `parse_string`, `parse_value`, `parse_array`, and `parse_object`; converting some of these to `&mut cJSON` will require care around linked-list node pointers and recursive calls.
- Consider removing the redundant `parse_buffer.length` field after callers are stable, replacing uses with `parse_buffer.content.len()` so length cannot drift from the slice.
- In `parse_array`/`parse_object`, the next direct reductions are node/list related: allocation via `cJSON_New_Item`, raw child links, `cJSON_Delete(head)`, and assigning child/name fields.
- `parse_string` still has raw `item` access, allocator/deallocator hook calls, the allocated output raw pointer, and assigning that pointer to `item.valuestring`. A narrow RAII owner for hook-allocated strings looked tempting, but avoid adding a new struct with a raw pointer field or unsafe `Drop` until there is a broader allocator/string ownership abstraction.
- Constructors and array factories still remain unsafe because node allocation/deallocation must stay paired with `cJSON_InitHooks`; do not partially convert nodes to `Box` until allocator-hook behavior is redesigned.
- The print path still contains unsafe allocator, `memcpy`, `sprintf`/`sscanf`, and raw output writes; string/number print helpers are likely smaller targets than list ownership.

Notes / pitfalls:
- Do not change signatures of functions with `#[export_name]`.
- `check-unsafe2` compares against the fixed `unsafe_json/cjson.json` baseline; use pass/fail plus source inspection.
- Avoid `raw_ptr.as_mut()` in implementation functions for now: a previous attempt increased `calls_unsafe`; a null check plus a single `&mut *ptr` dereference was accepted.
- Preserve `skip_utf8_bom` behavior on real parser entry inputs: it is called with offset 0 and a nonempty safe slice, and it only skips the BOM under the original `offset + 4 < length` guard.
- `parse_array` and `parse_object` intentionally keep the original failure behavior where `depth` is not decremented after a failed parse once it has been incremented.
- The one `content` slice in `parse_array`/`parse_object` is valid across nested parse calls because parser helpers mutate only offset/depth and tree nodes, not the input slice or length.
- `skip_utf8_bom`/`buffer_skip_whitespace` now mutate in place and return `()`. The parse entry path deliberately calls them separately before `parse_value` to preserve the old chained behavior without returning raw parser pointers.
- Keep raw pointer/count validation behavior in exported array wrappers: negative count or null array pointer returns null before creating a slice.
- Avoid converting `case_insensitive_strcmp` to ASCII-only Rust folding unless deliberately changing semantics; current behavior uses C `tolower` and may be locale-sensitive for non-ASCII bytes.
- For `parse_hex4`/unicode parsing, keep four-hex-digit parsing behavior: invalid digits return 0, and callers decide whether that is valid NUL or failure by context.
- `parse_string` is called only after callers have checked the current byte is `"`, so the new safe indexing relies on the same precondition the old pointer code relied on.
- There is no Git repo at `/root/work`; use direct inspection or `diff` against saved copies if future iterations need patch summaries.
