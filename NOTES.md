# Run Notes

- **no_plan_3** - `SAFETY_PLAN.md` disabled, ran to completion (0 unsafe detected) in ~7 hrs.
  - Changed the `cJSON` struct to safe fields. This is not correct from the
    perspective of API compatability, but does reflect the kind of safety
    transformations that we want the agent performing.
  - Sets the `type` field to `cJSON_IsReference` when cloning a `cJSON`, which
    seems correct for the original API, but is maybe not correct given how
    `cJSON` was refactored to make safe.
  - Does the `NaN = 0.0/0.0` thing.
- **with_plan_4** - Persistent safety plan, ran to completion (0 unsafe detected) in ~5.5 hrs.
  - Appended safe Rust fields to `cJSON` struct, which mostly preserves API
    compatability, but is unsound if client code ever tries to construct their
    own `cJSON` object, since those Rust fields would be uninitialized.
  - Doesn't respect the custom malloc/free hooks, uses std collections.
  - Uses `Vec<u8>` instead of `CString` for some reason?
  - Aggresively uses `Option<&cJSON>`, even in cases where it would be
    reasonable to drop the `Option` and use a `&cJSON` directly. Basically, we
    have a million unnecessary null checks in the safe core, when it would
    probably make more sense to do the null check at the FFI layer.
    - Sometimes we unwrap the `Option`, then re-wrap the reference in `Some` to
      pass it to another function that expects `Option<&cJSON>`.
  - Ends up doing a `0.0/0.0` to try to get NaN, which isn't correct in Rust.
  - Doesn't handle `CreateStringReference` correctly, makes a copy of the owned
    data. That's the simpler/safer thing to do in Rust, but violates the
    semantics of the original API.

# Translation Notes

- Main data structure is `cJSON` struct, which represents a single JSON value.
  - JSON objects and arrays are represented as a linked list of values. Parent
    object/array points to first element, then elements are a doubly-linked
    list. Object fields carry their name in the `string` field.
  - Basically the only data structure in the library, and all fields are
    effectively public which makes it hard/impossible to make the library
    internals safe.
  - Supports adding reference values (borrowed, as opposed to owned values) when
    adding fields to arrays/objects. This is marked in the `type` field so that
    it can tell which objects are owned vs borrowed when doing things like
    deleting a `cJSON` object.
- Allows setting custom malloc/free hooks, which makes using std collections
  like `Box` and `Vec`... impossible? And means we can't do any kind of safe
  allocation, really, since we have to go through an FFI on every allocation.
- What is the contract for the public API? Is it valid for the client to ever
  construct a cJSON struct themself, or are they required to always go through
  one of the exposed functions that allocates one?
  - Technically the API would allow a client to manually construct `cJSON`
    objects on the stack and link them together manually in order to do a kind
    of no-alloc serializion. idk if that's really the intended usage of the API,
    but Hyrum's law means that someone is doing this.
  - If you were to make the `cJSON` struct opaque (and maybe expose a couple of
    other things through functions (and maybe also disabled the custom alloc
    hooks, (and also maybe disabled support for ref objects))), you could do
    away with the linked list structure and make everything clean safe Rust with
    a minimal FFI boundary.
  - If you require 100% API compatability, then arguably a safe translation
    isn't possible. The API surface exposes too many implementation details in a
    way that prevent any internal refactorings that would allow for safe Rust
    idioms.
- Pointer graphs in general might be hard to handle. Probably the most common
  example of an unsafe data structure.
- LMAO `#define NaN 0.0/0.0`. That's not portable C, but follows the IEEE 60559
  standard I guess. Absolutley not valid to do in Rust, though.
- `CreateStringReference` and `SetValuestring` impose some limitations on how we
  handle owned strings.
  - `CreateStringReference` means that we need to be able to be able to share
    the string buffer between `cJSON` objects. Maybe the buffer needs to be an
    `Rc`?
  - `SetValueString` writes into the existing string buffer if it's owned, but
    it will only do this if the new string is shorter than the original? Kinda
    weird, but avoids having to track the capacity of the buffer.
