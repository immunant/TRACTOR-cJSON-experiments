use core::ffi::{c_int, c_char, c_double};
use std::ffi::{CStr, CString};

// The original definition of the `cJSON` type.
#[repr(C)]
pub struct cJSON_ffi {
    pub next: *mut cJSON,
    pub prev: *mut cJSON,
    pub child: *mut cJSON,
    pub type_0: c_int,
    pub valuestring: *mut c_char,
    pub valueint: c_int,
    pub valuedouble: c_double,
    pub string: *mut c_char,
}

// Attempt at a clever solution: Use different types for owning vs reference
// objects.
//
// The `type` field gives us the information we need to differentiate between
// owning and ref objects, and we interpret the `child` and `valuestring` fields
// differently between the two cases: Owning objects can use a regular box, ref
// objects use refs. This gives us two safe representations of `cJSON` that give
// us accurate ownership semantics.
//
// When receiving a `*mut cJSON`, we read the `type` field and then cast the
// pointer to either `cJSON_owning` or `cJSON_ref` before converting the ptr to
// a ref.
//
// There are problems though:
//
// - What type do the pointers to next and child objects need to be? We need to
//   deref the pointer and look at the object to know which one it is.
//   - We can get around this by having the fields be raw pointers, but that
//     then means we can't safely traverse the graph of objects, defeating the
//     purpose.
//   - The tricky part is that the type information is a property of the object,
//     whereas in Rust we want the type information to be a property of the
//     pointer. Having to deref the pointer to figure out what type it is is a
//     fundamentally weird operation in Rust.
// - For our non-owning ptrs, can those be refs? Rust has strict rules about
//   when it's valid to construct a ref from a ptr, and it would be very easy to
//   trigger UB if we are not extremely careful about when and how we construct
//   those refs.
//   - What if we added an invariant stating that a ref object must not be used
//     if the thing it refers to has been freed? In theory the user could free
//     the child object first, then try to free the ref object. That would then
//     mean that we construct a Rust ref (i.e. `child`) that is dangling, which
//     is UB. But if the user is required to free the ref object before freeing
//     the child
/// object, then this repr should be sound.
// - For the purpose of traversal, the distinction between the two doesn't
//   matter, but we need to juggle the two types in some way as we walk the
//   graph.
// - The `string` field poses an extra layer of complexity since it can also be
//   either owning or non-owning, and that can be intermixed freely with an
//   owning/non-owning object.
//
// The motivating observation here is:
//
// - We want `child` and `valuestring` to be different types depending on if the
//   object is owning or ref: We want to use a box when owning, and a regular
//   ref (or something equivalent to it) for ref objects.
// - We have a value that tells us which of the two cases we have, effectively
//   acting as a discriminant.
//
// A way of handling situations like this is to use the discriminant to cast the
// pointer to a more specific type. We see this in `B01_corganic/collided_lib`,
// where we use an enum to determine what type to cast a pointer to. The setup
// here is similar, but the type information is intrusive, so we have to deref
// the pointer to figure out what type it is. That's not strictly a deal
// breaker, but it interacts badly with the fact that we have a graph of these
// pointers. That graph aspect means that we can't isolate the type check at the
// FFI boundary, we need to be doing this check repeatedly as we traverse the
// graph.

/// An owning version of the `cJSON` struct, where the `child` and `valuestring`
/// pointers are owned by this object and will be freed when this object is
/// freed.
#[repr(C)]
pub struct cJSON_owning {
    pub next: Option<Box<cJSON>>,
    pub prev: *mut cJSON,
    pub child: Option<Box<cJSON>>,
    pub type_0: c_int,
    pub valuestring: Option<Box<CStr>>,
    pub valueint: c_int,
    pub valuedouble: c_double,
    pub string: Option<Box<CStr>>,
}

/// A non-owning version of `cJSON`, where `child` and `valuestring` are not
/// owned by this object and will not be freed when this object is freed.
#[repr(C)]
pub struct cJSON_ref<'a> {
    // A ref object still owns its `next` object.
    pub next: Option<Box<cJSON>>,
    pub prev: *mut cJSON,
    pub child: Option<&'a cJSON_ref>,
    pub type_0: c_int,
    pub valuestring: Option<&'a CStr>,
    pub valueint: c_int,
    pub valuedouble: c_double,
    pub string: Option<&'a CStr>,
}


// Following from the above approach, we might want to use an actual enum to
// capture the same behavior, but that is not ABI compatible with the original
// library.

enum cJSON<'a> {
    Owned {
        child: Option<Box<cJSON>>,
        valuestring: Option<Box<CStr>>,
    }
    Ref {
        child: Option<&'a cJSON>,
        valuestring: Option<&'a CStr>,
    }
}

// Really we want one enum that describes the different value types, including
// whether the value is owned or borrowed. This would be the idiomatic and safe
// way of handling the `type` field if we could freely change the internal
// representation.

struct cJSON<'a> {
    next: Option<Box<cJSON<'a>>>,
    prev: *mut cJSON<'a>,
    value: JsonValue<'a>,
    string: Option<Cow<'a, CStr>>,
}

enum JsonValue<'a> {
    ObjectArray(Box<cJSON<'a>>),
    ObjectArrayRef(&'a cJSON<'a>),
    String(Box<CStr>),
    StringRef(&'a CStr),
    Int(c_int),
    Double(c_double),
    True,
    False,
}

// A potential problem with the above is that it's not ABI compatible with the
// original C. In particular, there's no longer a `child` field, since it has
// been absorbed into `JsonValue`. `type`, `valuestring`, and `valuedouble` can
// be altered because they're exposed through getter functions, but the
// child/sibling ptrs don't have getters, nor do `valueint` and `string`.
