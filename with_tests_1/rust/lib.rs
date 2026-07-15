#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(raw_ref_op)]
#![feature(register_tool)]
#![register_tool(c2rust)]

pub mod __stddef_size_t_h {
    pub type size_t = usize;
}
pub mod internal {
    pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;

    pub const __DBL_EPSILON__: ::core::ffi::c_double = 2.2204460492503131e-16f64;
}
pub mod float_h {
    pub const DBL_EPSILON: ::core::ffi::c_double = crate::internal::__DBL_EPSILON__;
}
pub mod limits_h {
    pub const INT_MAX: ::core::ffi::c_int = crate::internal::__INT_MAX__;

    pub const INT_MIN: ::core::ffi::c_int =
        -2147483647 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
}
pub mod stdlib {
    extern "C" {
        pub fn malloc(__size: crate::__stddef_size_t_h::size_t) -> *mut ::core::ffi::c_void;

        pub fn realloc(
            __ptr: *mut ::core::ffi::c_void,
            __size: crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void;

        pub fn free(__ptr: *mut ::core::ffi::c_void);
    }
}
pub mod src {
    pub mod cJSON;
} // mod src
