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
  - Well, there is unstable support for custom allocators in std collections.
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
- It would be Rust idiomatic to make the child list into a `Vec`, but that means
  that the objects in the list don't have stable addresses, which can break
  things like reference objects. We effectively need every object to be boxed,
  so we could do a `Vec<Box<cJSON>>` (which is what with_plan_4 does), but that
  adds extra indirection and we still need to maintain the linked list of
  objects.
- The `prev` pointer is tricky to handle, since it doesn't play well with Rust
  ownership.
  - Most of the logic doesn't use `prev`, so in theory we could just store it as
    a raw pointer in the Rust struct and let the client C code read it. But
    there are a couple of places where it does get dereferenced, mostly when
    adding and removing elements from objects/arrays.
  - Almost all of the places where we touch `prev` we also have access to the
    parent object, which means in theory we could walk the list of children to
    find the previous child without going through `prev`. That would impact
    performance though, since we're going from a constant time operation to a
    linear time one.
- `ReplaceItemViaPointer` poses an aliasing problem. It takes a const ptr to the parent object, 

# FFI Boundary Reasoning

- Pointer at FFI boundary (in fns, specifically)
  - Convert to ref.
    - For input ptrs, the function is not taking ownership of the pointee.
    - If mutation then mut ref, otherwise shared ref.
    - For output ptrs, returned ptr must be derived from 
  - Convert to slice.
    - Is there ptr math? i.e. are we calculating an offset from the ptr?
    - Need to be able to derive the length up front, which isn't always
      possible.
  - Convert to box.
    - For input pointers, are we receiving ownership of an object that we
      created? We need to control both object creation and object destruction
      for box to be valid.
    - For output pointers, are we giving away ownership that we expect to be
      returned to us later?
    - Good for alloc/delete function pairs, which is a common pattern.
  - Convert to rc.
    - We control construction and destruction, but we need shared ownership
      through a thin ptr.
    - Probably only correct if the original was also doing reference counting,
      otherwise we're potentially changing the semantics of when the object is
      deleted.
  - Is the pointer typed? If not we need to reconstruct type info somehow.
    Generally there's some additional context that's telling the C code what
    type the pointer is, and the pointer is being cast to that type before being
    dereferenced.
- Pointer in data structures
  - Does the data structure own the pointee? Then prolly box.
  - Is the pointer an array? Then prolly vec.
  - Does the pointee itself have pointers? Are we dealing with a graph of
    objects? That's more complex to handle.
    - Is it a DAG? That can potentially modeled with boxes. cJSON objects are
      not quite a DAG because of the `prev` ptr, but we can effectively ignore
      `prev` since the library internals never actually touch it.
  - Is ownership overloaded? i.e. can the same pointer sometimes be owning and
    sometimes be borrowing?
    - If it's always owned then we can box, but if it's sometimes borrowed then
      we can't.
    - Sometimes we can do rc instead, but that may not always match the
      semantics of the C.
    - In cJSON we have the ability to create object references, where they don't
      own their `child` ptr. This means we can't make `child` a box, but we also
      don't want it to be an rc because then a reference object will prevent the
      child object from being deleted when the actual owning parent gets
      deleted.
