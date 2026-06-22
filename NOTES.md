# Run Notes

- **no_plan_3** - `SAFETY_PLAN.md` disabled, ran to completion (0 unsafe detected) in ~7 hrs.
  - Changed the `cJSON` struct to safe fields. This is not correct from the
    perspective of API compatability, but does reflect the kind of safety
    transformations that we want the agent performing.
- **with_plan_4** - Persistent safety plan, ran to completion (0 unsafe detected) in ~5.5 hrs.
  - Appended safe Rust fields to `cJSON` struct, which mostly preserves API
    compatability, but is unsound if client code ever tries to construct their
    own `cJSON` object, since those Rust fields would be uninitialized.
  - Doesn't respect the custom malloc/free hooks, uses std collections.
  - Uses `Vec<u8>` instead of `CString` for some reason?

# Translation Notes

- Main data structure is `cJSON` struct, which represents a single JSON value.
  - JSON objects and arrays are represented as a linked list of values. Parent
    object/array points to first element, then elements are a doubly-linked
    list. Object fields carry their name in the `string` field.
  - Basically the only data structure in the library, and all fields are
    effectively public which makes it hard/impossible to make the library
    internals safe.
- Allows setting custom malloc/free hooks, which makes using std collections
  like `Box` and `Vec`... impossible? And means we can't do any kind of safe
  allocation, really, since we have to go through an FFI on every allocation.
