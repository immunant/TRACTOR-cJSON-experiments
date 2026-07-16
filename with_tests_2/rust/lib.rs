#![allow(clippy::missing_safety_doc)]
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
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
}
pub mod float_h {
    pub const DBL_EPSILON: ::core::ffi::c_double = crate::internal::__DBL_EPSILON__;
}
pub mod limits_h {
    pub const INT_MAX: ::core::ffi::c_int = crate::internal::__INT_MAX__;

    pub const INT_MIN: ::core::ffi::c_int = -crate::internal::__INT_MAX__ - 1 as ::core::ffi::c_int;
}
pub mod stdlib {
    unsafe extern "C" {
        pub safe fn localeconv() -> *mut crate::stdlib::lconv;
        pub fn sprintf(
            __s: *mut ::core::ffi::c_char,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;

        pub fn sscanf(
            __s: *const ::core::ffi::c_char,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        pub safe fn malloc(__size: crate::__stddef_size_t_h::size_t) -> *mut ::core::ffi::c_void;

        pub safe fn realloc(
            __ptr: *mut ::core::ffi::c_void,
            __size: crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void;

        pub safe fn free(__ptr: *mut ::core::ffi::c_void);
        pub fn memcpy(
            __dest: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void;

        pub fn memset(
            __s: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __n: crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void;

        pub fn strcpy(
            __dest: *mut ::core::ffi::c_char,
            __src: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;

        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;

        pub fn strlen(__s: *const ::core::ffi::c_char) -> crate::__stddef_size_t_h::size_t;
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct lconv {
        pub decimal_point: *mut ::core::ffi::c_char,
        pub thousands_sep: *mut ::core::ffi::c_char,
        pub grouping: *mut ::core::ffi::c_char,
        pub int_curr_symbol: *mut ::core::ffi::c_char,
        pub currency_symbol: *mut ::core::ffi::c_char,
        pub mon_decimal_point: *mut ::core::ffi::c_char,
        pub mon_thousands_sep: *mut ::core::ffi::c_char,
        pub mon_grouping: *mut ::core::ffi::c_char,
        pub positive_sign: *mut ::core::ffi::c_char,
        pub negative_sign: *mut ::core::ffi::c_char,
        pub int_frac_digits: ::core::ffi::c_char,
        pub frac_digits: ::core::ffi::c_char,
        pub p_cs_precedes: ::core::ffi::c_char,
        pub p_sep_by_space: ::core::ffi::c_char,
        pub n_cs_precedes: ::core::ffi::c_char,
        pub n_sep_by_space: ::core::ffi::c_char,
        pub p_sign_posn: ::core::ffi::c_char,
        pub n_sign_posn: ::core::ffi::c_char,
        pub __int_p_cs_precedes: ::core::ffi::c_char,
        pub __int_p_sep_by_space: ::core::ffi::c_char,
        pub __int_n_cs_precedes: ::core::ffi::c_char,
        pub __int_n_sep_by_space: ::core::ffi::c_char,
        pub __int_p_sign_posn: ::core::ffi::c_char,
        pub __int_n_sign_posn: ::core::ffi::c_char,
    }
}
pub mod src {
    pub mod cJSON;
} // mod src
