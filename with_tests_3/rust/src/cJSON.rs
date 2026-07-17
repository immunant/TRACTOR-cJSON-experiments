// =============== BEGIN cJSON_h ================
pub const CJSON_VERSION_MAJOR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const CJSON_VERSION_MINOR: ::core::ffi::c_int = 7 as ::core::ffi::c_int;

pub const CJSON_VERSION_PATCH: ::core::ffi::c_int = 19 as ::core::ffi::c_int;

pub const cJSON_Invalid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const cJSON_False: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;

pub const cJSON_True: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;

pub const cJSON_NULL: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;

pub const cJSON_Number: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;

pub const cJSON_String: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;

pub const cJSON_Array: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int;

pub const cJSON_Object: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int;

pub const cJSON_Raw: ::core::ffi::c_int = 128;

pub const cJSON_IsReference: ::core::ffi::c_int = 256 as ::core::ffi::c_int;

pub const cJSON_StringIsConst: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct cJSON {
    pub next: *mut crate::src::cJSON::cJSON,
    pub prev: *mut crate::src::cJSON::cJSON,
    pub child: *mut crate::src::cJSON::cJSON,
    pub type_0: ::core::ffi::c_int,
    pub valuestring: *mut ::core::ffi::c_char,
    pub valueint: ::core::ffi::c_int,
    pub valuedouble: ::core::ffi::c_double,
    pub string: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct cJSON_Hooks {
    pub malloc_fn:
        Option<extern "C" fn(crate::__stddef_size_t_h::size_t) -> *mut ::core::ffi::c_void>,
    pub free_fn: Option<extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
}

pub type cJSON_bool = ::core::ffi::c_int;

pub const CJSON_NESTING_LIMIT: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;

pub const CJSON_CIRCULAR_LIMIT: ::core::ffi::c_int = 10000 as ::core::ffi::c_int;
pub use crate::__stddef_size_t_h::size_t;

pub use crate::float_h::DBL_EPSILON;


pub use crate::internal::__DBL_EPSILON__;
pub use crate::internal::__INT_MAX__;
pub use crate::limits_h::INT_MAX;
pub use crate::limits_h::INT_MIN;

pub use crate::stdlib::lconv;
pub use crate::stdlib::localeconv;













#[derive(Copy, Clone)]
#[repr(C)]

pub struct internal_hooks {
    pub allocate:
        Option<extern "C" fn(crate::__stddef_size_t_h::size_t) -> *mut ::core::ffi::c_void>,
    pub deallocate: Option<extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub reallocate: Option<
        extern "C" fn(
            *mut ::core::ffi::c_void,
            crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct error {
    pub json: usize,
    pub position: crate::__stddef_size_t_h::size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct parse_buffer<'a> {
    pub content: &'a [::core::ffi::c_uchar],
    pub length: crate::__stddef_size_t_h::size_t,
    pub offset: crate::__stddef_size_t_h::size_t,
    pub depth: crate::__stddef_size_t_h::size_t,
    pub hooks: internal_hooks,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct printbuffer {
    pub buffer: *mut ::core::ffi::c_uchar,
    pub length: crate::__stddef_size_t_h::size_t,
    pub offset: crate::__stddef_size_t_h::size_t,
    pub depth: crate::__stddef_size_t_h::size_t,
    pub noalloc: crate::src::cJSON::cJSON_bool,
    pub format: crate::src::cJSON::cJSON_bool,
    pub hooks: internal_hooks,
}

/* A read-only, owned view used by the printing implementation.  cJSON nodes
 * are deliberately C-visible, so reading them is an FFI-boundary operation;
 * once copied into this representation the recursive formatter needs neither
 * raw pointers nor unsafe operations. */
enum PrintableValue {
    Null,
    False,
    True,
    Number {
        valueint: ::core::ffi::c_int,
        valuedouble: ::core::ffi::c_double,
    },
    String(Option<Vec<u8>>),
    Raw(Option<Vec<u8>>),
    Array(Vec<PrintableValue>),
    Object(Vec<(Option<Vec<u8>>, PrintableValue)>),
    Invalid,
}

/* Parsing uses an owned representation for the same reason printing does: the
 * recursive parser should not need to manipulate the C ABI's linked nodes.
 * The exported parse entry points materialize this value only after a complete
 * parse succeeds. */
enum ParsedValue {
    Null,
    False,
    True,
    Number {
        valueint: ::core::ffi::c_int,
        valuedouble: ::core::ffi::c_double,
    },
    String(Vec<u8>),
    Array(Vec<ParsedValue>),
    Object(Vec<(Vec<u8>, ParsedValue)>),
}

pub const true_0: crate::src::cJSON::cJSON_bool = 1 as ::core::ffi::c_int;

pub const false_0: crate::src::cJSON::cJSON_bool = 0 as ::core::ffi::c_int;

static global_error: std::sync::Mutex<error> = std::sync::Mutex::new(error {
    json: 0,
    position: 0 as crate::__stddef_size_t_h::size_t,
});
#[export_name = "cJSON_GetErrorPtr"]

pub unsafe extern "C" fn cJSON_GetErrorPtr_ffi() -> *const ::core::ffi::c_char {
    let error = *global_error
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if error.json == 0 {
        ::core::ptr::null()
    } else {
        error.json.wrapping_add(error.position) as *const ::core::ffi::c_char
    }
}
#[export_name = "cJSON_GetStringValue"]

pub unsafe extern "C" fn cJSON_GetStringValue_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> *mut ::core::ffi::c_char {
    match unsafe { item.as_ref() } {
        Some(item) if cjson_type_is(item, cJSON_String) != 0 => item.valuestring,
        _ => ::core::ptr::null_mut::<::core::ffi::c_char>(),
    }
}
#[export_name = "cJSON_GetNumberValue"]

pub unsafe extern "C" fn cJSON_GetNumberValue_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> ::core::ffi::c_double {
    match unsafe { item.as_ref() } {
        Some(item) if cjson_type_is(item, cJSON_Number) != 0 => item.valuedouble,
        _ => 0.0f64 / 0.0f64,
    }
}
fn cjson_version() -> &'static [u8; 7] {
    b"1.7.19\0"
}
#[export_name = "cJSON_Version"]

pub unsafe extern "C" fn cJSON_Version_ffi() -> *const ::core::ffi::c_char {
    cjson_version().as_ptr().cast::<::core::ffi::c_char>()
}
fn c_string_cmp(
    string1: &::core::ffi::CStr,
    string2: &::core::ffi::CStr,
    case_sensitive: bool,
) -> ::core::ffi::c_int {
    for (&left, &right) in string1
        .to_bytes_with_nul()
        .iter()
        .zip(string2.to_bytes_with_nul().iter())
    {
        let left = if case_sensitive {
            left
        } else {
            left.to_ascii_lowercase()
        };
        let right = if case_sensitive {
            right
        } else {
            right.to_ascii_lowercase()
        };
        if left != right {
            return left as ::core::ffi::c_int - right as ::core::ffi::c_int;
        }
        if left == 0 {
            return 0;
        }
    }

    /* Both inputs are C strings, so the loop always reaches their trailing NUL. */
    0
}

static global_hooks: std::sync::Mutex<internal_hooks> = std::sync::Mutex::new(internal_hooks {
    allocate: Some(crate::stdlib::malloc),
    deallocate: Some(crate::stdlib::free),
    reallocate: Some(crate::stdlib::realloc),
});

fn current_hooks() -> internal_hooks {
    *global_hooks
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

macro_rules! cjson_strdup {
    ($string:expr, $hooks:expr $(,)?) => {{
        let string = $string;
        let hooks = &$hooks;
        if string.is_null() {
            ::core::ptr::null_mut::<::core::ffi::c_uchar>()
        } else {
            let length = crate::stdlib::strlen(string as *const ::core::ffi::c_char)
                .wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 1]>());
            let copy = (*hooks).allocate.expect("non-null function pointer")(length)
                as *mut ::core::ffi::c_uchar;
            if copy.is_null() {
                ::core::ptr::null_mut::<::core::ffi::c_uchar>()
            } else {
                crate::stdlib::memcpy(
                    copy as *mut ::core::ffi::c_void,
                    string as *const ::core::ffi::c_void,
                    length,
                );
                copy
            }
        }
    }};
}

fn init_hooks(hooks: Option<&cJSON_Hooks>, defaults: internal_hooks) {
    let mut hooks_state = global_hooks
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let (allocate, deallocate) = match hooks {
        Some(hooks) => (
            hooks.malloc_fn.or(defaults.allocate),
            hooks.free_fn.or(defaults.deallocate),
        ),
        None => (defaults.allocate, defaults.deallocate),
    };
    hooks_state.allocate = allocate;
    hooks_state.deallocate = deallocate;
    hooks_state.reallocate = if hooks.is_some()
        && (hooks_state.allocate != defaults.allocate
            || hooks_state.deallocate != defaults.deallocate)
    {
        None
    } else {
        defaults.reallocate
    };
}

#[export_name = "cJSON_InitHooks"]
pub unsafe extern "C" fn cJSON_InitHooks_ffi(hooks: *mut crate::src::cJSON::cJSON_Hooks) {
    let defaults = internal_hooks {
        allocate: Some(crate::stdlib::malloc),
        deallocate: Some(crate::stdlib::free),
        reallocate: Some(crate::stdlib::realloc),
    };
    init_hooks(unsafe { hooks.as_ref() }, defaults);
}

fn cJSON_New_Item(hooks: &internal_hooks) -> Option<::core::ptr::NonNull<cJSON>> {
    let node = ::core::ptr::NonNull::new(
        hooks.allocate.expect("non-null function pointer")(::core::mem::size_of::<
            crate::src::cJSON::cJSON,
        >()) as *mut crate::src::cJSON::cJSON,
    );
    if let Some(node) = node {
        crate::stdlib::memset(
            node.as_ptr() as *mut ::core::ffi::c_void,
            '\0' as ::core::ffi::c_int,
            ::core::mem::size_of::<crate::src::cJSON::cJSON>(),
        );
    }
    node
}
#[export_name = "cJSON_Delete"]
pub unsafe extern "C" fn cJSON_Delete_ffi(mut item: *mut crate::src::cJSON::cJSON) {
    let hooks = current_hooks();
    let mut next: *mut crate::src::cJSON::cJSON =
        ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    while !item.is_null() {
        next = (*item).next as *mut crate::src::cJSON::cJSON;
        if (*item).type_0 & crate::src::cJSON::cJSON_IsReference == 0 && !(*item).child.is_null() {
            cJSON_Delete_ffi((*item).child as *mut crate::src::cJSON::cJSON);
        }
        if (*item).type_0 & crate::src::cJSON::cJSON_IsReference == 0
            && !(*item).valuestring.is_null()
        {
            hooks.deallocate.expect("non-null function pointer")(
                (*item).valuestring as *mut ::core::ffi::c_void,
            );
            (*item).valuestring = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if (*item).type_0 & crate::src::cJSON::cJSON_StringIsConst == 0 && !(*item).string.is_null()
        {
            hooks.deallocate.expect("non-null function pointer")(
                (*item).string as *mut ::core::ffi::c_void,
            );
            (*item).string = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        hooks.deallocate.expect("non-null function pointer")(
            item as *mut ::core::ffi::c_void,
        );
        item = next;
    }
}
fn number_valueint(number: ::core::ffi::c_double) -> ::core::ffi::c_int {
    if number >= crate::limits_h::INT_MAX as ::core::ffi::c_double {
        crate::limits_h::INT_MAX
    } else if number <= crate::limits_h::INT_MIN as ::core::ffi::c_double {
        crate::limits_h::INT_MIN
    } else {
        number as ::core::ffi::c_int
    }
}

fn parse_number(input_buffer: &mut parse_buffer<'_>) -> Option<ParsedValue> {
    let Some(input) = input_buffer.content.get(input_buffer.offset..) else {
        return None;
    };

    /* Match the decimal prefix consumed by strtod. An incomplete exponent is
     * intentionally left for the caller, matching cJSON's permissive parser. */
    let mut end = 0;
    if input.first() == Some(&b'-') {
        end = 1;
    }

    let integer_start = end;
    while input.get(end).is_some_and(u8::is_ascii_digit) {
        end += 1;
    }
    let mut has_digits = end != integer_start;

    if input.get(end) == Some(&b'.') {
        end += 1;
        let fractional_start = end;
        while input.get(end).is_some_and(u8::is_ascii_digit) {
            end += 1;
        }
        has_digits |= end != fractional_start;
    }

    if !has_digits {
        return None;
    }

    let mantissa_end = end;
    if matches!(input.get(end), Some(b'e' | b'E')) {
        let mut exponent_end = end + 1;
        if matches!(input.get(exponent_end), Some(b'+' | b'-')) {
            exponent_end += 1;
        }
        let exponent_start = exponent_end;
        while input.get(exponent_end).is_some_and(u8::is_ascii_digit) {
            exponent_end += 1;
        }
        if exponent_end != exponent_start {
            end = exponent_end;
        } else {
            end = mantissa_end;
        }
    }

    let Ok(number_text) = core::str::from_utf8(&input[..end]) else {
        return None;
    };
    let Ok(number) = number_text.parse::<f64>() else {
        return None;
    };

    input_buffer.offset += end;
    Some(ParsedValue::Number {
        valueint: number_valueint(number),
        valuedouble: number,
    })
}
fn set_number_helper(
    object: &mut cJSON,
    number: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    object.valueint = number_valueint(number);
    object.valuedouble = number;
    object.valuedouble
}
#[export_name = "cJSON_SetNumberHelper"]

pub unsafe extern "C" fn cJSON_SetNumberHelper_ffi(
    object: *mut crate::src::cJSON::cJSON,
    number: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    unsafe { object.as_mut() }
        .map(|object| set_number_helper(object, number))
        .unwrap_or(0.0f64 / 0.0f64)
}
#[export_name = "cJSON_SetValuestring"]

pub unsafe extern "C" fn cJSON_SetValuestring_ffi(
    mut object: *mut crate::src::cJSON::cJSON,
    mut valuestring: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut copy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut v1_len: crate::__stddef_size_t_h::size_t = 0;
    let mut v2_len: crate::__stddef_size_t_h::size_t = 0;
    if object.is_null()
        || (*object).type_0 & crate::src::cJSON::cJSON_String == 0
        || (*object).type_0 & crate::src::cJSON::cJSON_IsReference != 0
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if (*object).valuestring.is_null() || valuestring.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    v1_len = crate::stdlib::strlen(valuestring);
    v2_len = crate::stdlib::strlen((*object).valuestring);
    if v1_len <= v2_len {
        if !(valuestring.offset(v1_len as isize)
            < (*object).valuestring as *const ::core::ffi::c_char
            || (*object).valuestring.offset(v2_len as isize)
                < valuestring as *mut ::core::ffi::c_char)
        {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        crate::stdlib::strcpy((*object).valuestring, valuestring);
        return (*object).valuestring;
    }
    copy = cjson_strdup!(
        valuestring as *const ::core::ffi::c_uchar,
        current_hooks(),
    ) as *mut ::core::ffi::c_char;
    if copy.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !(*object).valuestring.is_null() {
        cJSON_free_ffi((*object).valuestring as *mut ::core::ffi::c_void);
    }
    (*object).valuestring = copy;
    return copy;
}
fn ensure(
    p: &mut printbuffer,
    mut needed: crate::__stddef_size_t_h::size_t,
    initial_contents: Option<&[u8]>,
) -> *mut ::core::ffi::c_uchar {
    let mut newbuffer: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut newsize: crate::__stddef_size_t_h::size_t = 0 as crate::__stddef_size_t_h::size_t;
    let mut copy_source: *const ::core::ffi::c_void = ::core::ptr::null();
    let mut copy_length: crate::__stddef_size_t_h::size_t = 0;
    let mut return_after_copy = false;
    let mut deallocate_old_buffer = false;
    if p.buffer.is_null() && initial_contents.is_none() {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    if p.length > 0 as crate::__stddef_size_t_h::size_t && p.offset >= p.length {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    if needed > crate::limits_h::INT_MAX as crate::__stddef_size_t_h::size_t {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    if let Some(initial_contents) = initial_contents {
        if !p.buffer.is_null() {
            newbuffer = ensure(p, needed, None);
            if newbuffer.is_null() {
                return newbuffer;
            }
            copy_source = initial_contents.as_ptr().cast::<::core::ffi::c_void>();
            copy_length = initial_contents.len();
            return_after_copy = true;
        }
    }
    if !return_after_copy {
        needed = needed.wrapping_add(
            p.offset
                .wrapping_add(1 as crate::__stddef_size_t_h::size_t),
        );
        if let Some(initial_contents) = initial_contents {
            if !p.buffer.is_null() || initial_contents.len() > needed {
                return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
            }
            newsize = needed;
            newbuffer = p.hooks.allocate.expect("non-null function pointer")(newsize)
                as *mut ::core::ffi::c_uchar;
            if newbuffer.is_null() {
                return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
            }
            copy_source = initial_contents.as_ptr().cast::<::core::ffi::c_void>();
            copy_length = initial_contents.len();
        } else if needed <= p.length {
            return p.buffer.wrapping_add(p.offset);
        } else if p.noalloc != 0 {
            return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        } else {
            if needed
                > (crate::limits_h::INT_MAX / 2 as ::core::ffi::c_int)
                    as crate::__stddef_size_t_h::size_t
            {
                if needed <= crate::limits_h::INT_MAX as crate::__stddef_size_t_h::size_t {
                    newsize = crate::limits_h::INT_MAX as crate::__stddef_size_t_h::size_t;
                } else {
                    return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                }
            } else {
                newsize = needed.wrapping_mul(2 as crate::__stddef_size_t_h::size_t);
            }
            if p.hooks.reallocate.is_some() {
                newbuffer = p.hooks.reallocate.expect("non-null function pointer")(
                    p.buffer as *mut ::core::ffi::c_void,
                    newsize,
                ) as *mut ::core::ffi::c_uchar;
                if newbuffer.is_null() {
                    p.hooks.deallocate.expect("non-null function pointer")(
                        p.buffer as *mut ::core::ffi::c_void,
                    );
                    p.length = 0 as crate::__stddef_size_t_h::size_t;
                    p.buffer = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                    return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                }
            } else {
                newbuffer = p.hooks.allocate.expect("non-null function pointer")(newsize)
                    as *mut ::core::ffi::c_uchar;
                if newbuffer.is_null() {
                    p.hooks.deallocate.expect("non-null function pointer")(
                        p.buffer as *mut ::core::ffi::c_void,
                    );
                    p.length = 0 as crate::__stddef_size_t_h::size_t;
                    p.buffer = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                    return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                }
                copy_source = p.buffer as *const ::core::ffi::c_void;
                copy_length = p.offset.wrapping_add(1 as crate::__stddef_size_t_h::size_t);
                deallocate_old_buffer = true;
            }
        }
    }
    if copy_length != 0 {
        crate::stdlib::memcpy(
            newbuffer as *mut ::core::ffi::c_void,
            copy_source,
            copy_length,
        );
    }
    if return_after_copy {
        return newbuffer;
    }
    if deallocate_old_buffer {
        p.hooks.deallocate.expect("non-null function pointer")(
            p.buffer as *mut ::core::ffi::c_void,
        );
    }
    p.length = newsize;
    p.buffer = newbuffer;
    newbuffer.wrapping_add(p.offset)
}

fn compare_double(
    a: ::core::ffi::c_double,
    b: ::core::ffi::c_double,
) -> crate::src::cJSON::cJSON_bool {
    let max_value = if a.abs() > b.abs() {
        a.abs()
    } else {
        b.abs()
    };
    ((a - b).abs() <= max_value * crate::float_h::DBL_EPSILON) as ::core::ffi::c_int
}

/* cJSON formats non-integer numbers with `%g`: start with 15 significant
 * digits and retry with 17 when the shorter spelling loses too much
 * precision. Rust's exponential formatter provides the correctly rounded
 * significant digits; this helper only applies `%g`'s notation and exponent
 * spelling rules. */
fn format_g(value: f64, precision: usize) -> String {
    use ::core::fmt::Write;

    let mut scientific = String::new();
    write!(&mut scientific, "{:.*e}", precision.saturating_sub(1), value)
        .expect("writing into a String cannot fail");
    let (mantissa, exponent) = scientific
        .split_once('e')
        .expect("Rust exponential formatting always includes an exponent");
    let exponent = exponent
        .parse::<i32>()
        .expect("Rust exponential formatting has a decimal exponent");
    let negative = mantissa.starts_with('-');
    let mut digits: String = mantissa
        .bytes()
        .filter(|byte| byte.is_ascii_digit())
        .map(char::from)
        .collect();
    while digits.len() > 1 && digits.ends_with('0') {
        digits.pop();
    }

    let sign = if negative { "-" } else { "" };
    if exponent < -4 || exponent >= precision as i32 {
        let mut result = String::from(sign);
        result.push_str(&digits[..1]);
        if digits.len() > 1 {
            result.push('.');
            result.push_str(&digits[1..]);
        }
        result.push('e');
        result.push(if exponent < 0 { '-' } else { '+' });
        let exponent = exponent.unsigned_abs().to_string();
        if exponent.len() == 1 {
            result.push('0');
        }
        result.push_str(&exponent);
        return result;
    }

    let decimal_position = exponent + 1;
    if decimal_position <= 0 {
        let mut result = String::from(sign);
        result.push_str("0.");
        for _ in 0..-decimal_position {
            result.push('0');
        }
        result.push_str(&digits);
        result
    } else if decimal_position as usize >= digits.len() {
        let mut result = String::from(sign);
        result.push_str(&digits);
        for _ in digits.len()..decimal_position as usize {
            result.push('0');
        }
        result
    } else {
        let decimal_position = decimal_position as usize;
        let mut result = String::from(sign);
        result.push_str(&digits[..decimal_position]);
        result.push('.');
        result.push_str(&digits[decimal_position..]);
        result
    }
}

fn format_number(valueint: ::core::ffi::c_int, value: f64) -> Option<Vec<u8>> {
    if !value.is_finite() {
        return Some(b"null".to_vec());
    }
    if value == valueint as f64 {
        return Some(valueint.to_string().into_bytes());
    }

    let mut number = format_g(value, 15);
    let parsed = number.parse::<f64>().ok()?;
    if compare_double(parsed, value) == 0 {
        number = format_g(value, 17);
    }
    Some(number.into_bytes())
}

fn format_number_for_print(
    valueint: ::core::ffi::c_int,
    valuedouble: ::core::ffi::c_double,
    decimal_point: ::core::ffi::c_uchar,
) -> Option<Vec<u8>> {
    let mut number = format_number(valueint, valuedouble)?;
    if number.len() > 25 {
        return None;
    }

    for byte in &mut number {
        if *byte as ::core::ffi::c_int == decimal_point as ::core::ffi::c_int {
            *byte = b'.';
        }
    }
    Some(number)
}

fn parse_hex4(input: [u8; 4]) -> ::core::ffi::c_uint {
    let mut value: ::core::ffi::c_uint = 0;
    for (index, byte) in input.into_iter().enumerate() {
        let digit = match byte {
            b'0'..=b'9' => (byte - b'0') as ::core::ffi::c_uint,
            b'A'..=b'F' => (10 + byte - b'A') as ::core::ffi::c_uint,
            b'a'..=b'f' => (10 + byte - b'a') as ::core::ffi::c_uint,
            _ => return 0,
        };
        value = value.wrapping_add(digit);
        if index < 3 {
            value <<= 4;
        }
    }
    value
}

fn utf16_literal_to_utf8(input: &[u8], output: &mut Vec<u8>) -> Option<usize> {
    if input.len() < 6 {
        return None;
    }
    let first_code = parse_hex4(input[2..6].try_into().ok()?);
    if (0xdc00..=0xdfff).contains(&first_code) {
        return None;
    }
    let (codepoint, sequence_length) = if (0xd800..=0xdbff).contains(&first_code) {
        if input.len() < 12 || input[6] != b'\\' || input[7] != b'u' {
            return None;
        }
        let second_code = parse_hex4(input[8..12].try_into().ok()?);
        if !(0xdc00..=0xdfff).contains(&second_code) {
            return None;
        }
        (
            0x10000 + ((first_code & 0x3ff) << 10 | second_code & 0x3ff),
            12,
        )
    } else {
        (first_code, 6)
    };
    let character = char::from_u32(codepoint)?;
    let mut utf8 = [0; 4];
    output.extend_from_slice(character.encode_utf8(&mut utf8).as_bytes());
    Some(sequence_length)
}

fn decode_json_string(input: &[u8]) -> Result<(Vec<u8>, usize), usize> {
    if input.first() != Some(&b'"') {
        return Err(0);
    }

    let mut output = Vec::with_capacity(input.len().saturating_sub(1));
    let mut position = 1;
    while position < input.len() {
        match input[position] {
            b'"' => return Ok((output, position + 1)),
            b'\\' => {
                let Some(&escape) = input.get(position + 1) else {
                    return Err(position);
                };
                match escape {
                    b'b' => output.push(b'\x08'),
                    b'f' => output.push(b'\x0c'),
                    b'n' => output.push(b'\n'),
                    b'r' => output.push(b'\r'),
                    b't' => output.push(b'\t'),
                    b'"' | b'\\' | b'/' => output.push(escape),
                    b'u' => {
                        let Some(sequence_length) =
                            utf16_literal_to_utf8(&input[position..], &mut output)
                        else {
                            return Err(position);
                        };
                        position += sequence_length;
                        continue;
                    }
                    _ => return Err(position),
                }
                position += 2;
            }
            byte => {
                output.push(byte);
                position += 1;
            }
        }
    }
    Err(position)
}

fn parse_string(input_buffer: &mut parse_buffer<'_>) -> Option<Vec<u8>> {
    let buffer = input_buffer;
    let input = &buffer.content[buffer.offset..];
    let (decoded, consumed) = match decode_json_string(input) {
        Ok(value) => value,
        Err(failure_offset) => {
            buffer.offset = buffer.offset.wrapping_add(failure_offset);
            return None;
        }
    };
    buffer.offset = buffer.offset.wrapping_add(consumed);
    Some(decoded)
}

fn encode_json_string(input: Option<&[u8]>) -> Vec<u8> {
    let input = input.unwrap_or_default();
    let mut output = Vec::with_capacity(input.len().saturating_add(3));
    output.push(b'"');

    for &byte in input {
        match byte {
            b'"' | b'\\' => {
                output.push(b'\\');
                output.push(byte);
            }
            b'\x08' => output.extend_from_slice(b"\\b"),
            b'\x0c' => output.extend_from_slice(b"\\f"),
            b'\n' => output.extend_from_slice(b"\\n"),
            b'\r' => output.extend_from_slice(b"\\r"),
            b'\t' => output.extend_from_slice(b"\\t"),
            0..=31 => {
                const HEX: &[u8; 16] = b"0123456789abcdef";
                output.extend_from_slice(&[
                    b'\\',
                    b'u',
                    b'0',
                    b'0',
                    HEX[(byte >> 4) as usize],
                    HEX[(byte & 0x0f) as usize],
                ]);
            }
            _ => output.push(byte),
        }
    }

    output.extend_from_slice(b"\"\0");
    output
}

fn append_print_bytes(
    output_buffer: &mut printbuffer,
    bytes: &[u8],
) -> crate::src::cJSON::cJSON_bool {
    debug_assert_eq!(bytes.last(), Some(&b'\0'));
    let output = ensure(output_buffer, bytes.len(), Some(bytes));
    if output.is_null() {
        return false_0;
    }
    output_buffer.offset = output_buffer
        .offset
        .wrapping_add(bytes.len().wrapping_sub(1));
    true_0
}

fn print_string(input: Option<&[u8]>, output_buffer: &mut printbuffer) -> crate::src::cJSON::cJSON_bool {
    let encoded = encode_json_string(input);
    append_print_bytes(output_buffer, &encoded)
}

fn buffer_skip_whitespace(buffer: &mut parse_buffer<'_>) {
    if buffer.offset >= buffer.length {
        return;
    }
    while buffer.offset < buffer.length && buffer.content[buffer.offset] <= b' ' {
        buffer.offset = buffer.offset.wrapping_add(1);
    }
    if buffer.offset == buffer.length {
        buffer.offset = buffer.offset.wrapping_sub(1);
    }
}

fn skip_utf8_bom(buffer: &mut parse_buffer<'_>) {
    if buffer.offset == 0
        && buffer.offset.wrapping_add(4) < buffer.length
        && buffer.content.starts_with(b"\xEF\xBB\xBF")
    {
        buffer.offset = buffer.offset.wrapping_add(3);
    }
}

/* This macro is intentionally invoked only by exported parse functions. It
 * translates the safe parse tree into the public, C-owned cJSON layout. */
macro_rules! materialize_parsed_value {
    ($root:expr, $value:expr, $hooks:expr) => {{
        let hooks = $hooks;
        let root = $root;
        let mut pending = vec![($value, root)];
        while let Some((value, node)) = pending.pop() {
            if node.is_null() {
                cJSON_Delete_ffi(root);
                return ::core::ptr::null_mut::<cJSON>();
            }
            match value {
                ParsedValue::Null => (*node).type_0 = cJSON_NULL,
                ParsedValue::False => (*node).type_0 = cJSON_False,
                ParsedValue::True => { (*node).type_0 = cJSON_True; (*node).valueint = 1; }
                ParsedValue::Number { valueint, valuedouble } => {
                    (*node).type_0 = cJSON_Number;
                    (*node).valueint = *valueint;
                    (*node).valuedouble = *valuedouble;
                }
                ParsedValue::String(bytes) => {
                    let string = hooks.allocate.expect("non-null function pointer")(bytes.len() + 1)
                        as *mut ::core::ffi::c_char;
                    if string.is_null() { cJSON_Delete_ffi(root); return ::core::ptr::null_mut::<cJSON>(); }
                    ::core::ptr::copy_nonoverlapping(bytes.as_ptr(), string.cast(), bytes.len());
                    *string.add(bytes.len()) = 0;
                    (*node).type_0 = cJSON_String;
                    (*node).valuestring = string;
                }
                ParsedValue::Array(values) => {
                    (*node).type_0 = cJSON_Array;
                    let mut tail = ::core::ptr::null_mut::<cJSON>();
                    for value in values {
                        let child = cJSON_New_Item(&hooks).map_or(::core::ptr::null_mut(), ::core::ptr::NonNull::as_ptr);
                        if child.is_null() { cJSON_Delete_ffi(root); return ::core::ptr::null_mut::<cJSON>(); }
                        if tail.is_null() { (*node).child = child; } else { (*tail).next = child; (*child).prev = tail; }
                        tail = child;
                        pending.push((value, child));
                    }
                    if !(*node).child.is_null() { (*(*node).child).prev = tail; }
                }
                ParsedValue::Object(values) => {
                    (*node).type_0 = cJSON_Object;
                    let mut tail = ::core::ptr::null_mut::<cJSON>();
                    for (key, value) in values {
                        let child = cJSON_New_Item(&hooks).map_or(::core::ptr::null_mut(), ::core::ptr::NonNull::as_ptr);
                        if child.is_null() { cJSON_Delete_ffi(root); return ::core::ptr::null_mut::<cJSON>(); }
                        let key_copy = hooks.allocate.expect("non-null function pointer")(key.len() + 1) as *mut ::core::ffi::c_char;
                        if key_copy.is_null() { cJSON_Delete_ffi(child); cJSON_Delete_ffi(root); return ::core::ptr::null_mut::<cJSON>(); }
                        ::core::ptr::copy_nonoverlapping(key.as_ptr(), key_copy.cast(), key.len());
                        *key_copy.add(key.len()) = 0;
                        (*child).string = key_copy;
                        if tail.is_null() { (*node).child = child; } else { (*tail).next = child; (*child).prev = tail; }
                        tail = child;
                        pending.push((value, child));
                    }
                    if !(*node).child.is_null() { (*(*node).child).prev = tail; }
                }
            }
        }
        root
    }};
}

#[export_name = "cJSON_ParseWithOpts"]
pub unsafe extern "C" fn cJSON_ParseWithOpts_ffi(
    mut value: *const ::core::ffi::c_char,
    mut return_parse_end: *mut *const ::core::ffi::c_char,
    mut require_null_terminated: crate::src::cJSON::cJSON_bool,
) -> *mut crate::src::cJSON::cJSON {
    let mut buffer_length: crate::__stddef_size_t_h::size_t = 0;
    if value.is_null() {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    }
    buffer_length = crate::stdlib::strlen(value)
        .wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 1]>());
    return cJSON_ParseWithLengthOpts_ffi(
        value,
        buffer_length,
        return_parse_end,
        require_null_terminated,
    );
}
#[export_name = "cJSON_ParseWithLengthOpts"]
pub unsafe extern "C" fn cJSON_ParseWithLengthOpts_ffi(
    mut value: *const ::core::ffi::c_char,
    mut buffer_length: crate::__stddef_size_t_h::size_t,
    mut return_parse_end: *mut *const ::core::ffi::c_char,
    mut require_null_terminated: crate::src::cJSON::cJSON_bool,
) -> *mut crate::src::cJSON::cJSON {
    let mut buffer: parse_buffer = parse_buffer {
        content: &[],
        length: 0 as crate::__stddef_size_t_h::size_t,
        offset: 0 as crate::__stddef_size_t_h::size_t,
        depth: 0 as crate::__stddef_size_t_h::size_t,
        hooks: internal_hooks {
            allocate: None,
            deallocate: None,
            reallocate: None,
        },
    };
    let mut item: *mut crate::src::cJSON::cJSON =
        ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    {
        let mut error_state = global_error
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        error_state.json = 0;
        error_state.position = 0 as crate::__stddef_size_t_h::size_t;
    }
    '_fail: {
        if !(value.is_null() || 0 as crate::__stddef_size_t_h::size_t == buffer_length) {
            buffer.content = ::core::slice::from_raw_parts(
                value as *const ::core::ffi::c_uchar,
                buffer_length,
            );
            buffer.length = buffer_length;
            buffer.offset = 0 as crate::__stddef_size_t_h::size_t;
            buffer.hooks = current_hooks();
            item = cJSON_New_Item(&buffer.hooks)
                .map_or(::core::ptr::null_mut(), ::core::ptr::NonNull::as_ptr);
            if !item.is_null() {
                skip_utf8_bom(&mut buffer);
                buffer_skip_whitespace(&mut buffer);
                if let Some(parsed) = parse_value(&mut buffer) {
                    if require_null_terminated != 0 {
                        buffer_skip_whitespace(&mut buffer);
                        if buffer.offset >= buffer.length
                            || buffer.content[buffer.offset] as ::core::ffi::c_int
                                != '\0' as ::core::ffi::c_int
                        {
                            break '_fail;
                        }
                    }
                    item = materialize_parsed_value!(item, &parsed, buffer.hooks);
                    if !item.is_null() {
                        if !return_parse_end.is_null() {
                            *return_parse_end = value.add(buffer.offset);
                        }
                        return item;
                    }
                }
            }
        }
    }
    if !item.is_null() {
        cJSON_Delete_ffi(item);
    }
    if !value.is_null() {
        let mut local_error: error = error {
            json: 0,
            position: 0,
        };
        local_error.json = value as usize;
        local_error.position = 0 as crate::__stddef_size_t_h::size_t;
        if buffer.offset < buffer.length {
            local_error.position = buffer.offset;
        } else if buffer.length > 0 as crate::__stddef_size_t_h::size_t {
            local_error.position = buffer
                .length
                .wrapping_sub(1 as crate::__stddef_size_t_h::size_t);
        }
        if !return_parse_end.is_null() {
            *return_parse_end = local_error
                .json
                .wrapping_add(local_error.position)
                as *const ::core::ffi::c_char;
        }
        *global_error
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = local_error;
    }
    return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
}
#[export_name = "cJSON_Parse"]
pub unsafe extern "C" fn cJSON_Parse_ffi(
    mut value: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    return cJSON_ParseWithOpts_ffi(
        value,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        0 as crate::src::cJSON::cJSON_bool,
    );
}
#[export_name = "cJSON_ParseWithLength"]
pub unsafe extern "C" fn cJSON_ParseWithLength_ffi(
    mut value: *const ::core::ffi::c_char,
    mut buffer_length: crate::__stddef_size_t_h::size_t,
) -> *mut crate::src::cJSON::cJSON {
    return cJSON_ParseWithLengthOpts_ffi(
        value,
        buffer_length,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        0 as crate::src::cJSON::cJSON_bool,
    );
}
const DEFAULT_PRINT_BUFFER_SIZE: crate::__stddef_size_t_h::size_t = 256;

macro_rules! printable_snapshot {
    ($item:expr) => {{
        enum SnapshotTask<'a> {
            Visit(&'a cJSON),
            FinishArray(usize),
            FinishObject(Vec<Option<Vec<u8>>>),
        }

        let mut tasks = vec![SnapshotTask::Visit(&*$item)];
        let mut values = Vec::<PrintableValue>::new();
        while let Some(task) = tasks.pop() {
            match task {
                SnapshotTask::Visit(node) => {
                    match node.type_0 & 0xff as ::core::ffi::c_int {
                        crate::src::cJSON::cJSON_NULL => values.push(PrintableValue::Null),
                        crate::src::cJSON::cJSON_False => values.push(PrintableValue::False),
                        crate::src::cJSON::cJSON_True => values.push(PrintableValue::True),
                        crate::src::cJSON::cJSON_Number => values.push(PrintableValue::Number {
                            valueint: node.valueint,
                            valuedouble: node.valuedouble,
                        }),
                        crate::src::cJSON::cJSON_String => values.push(PrintableValue::String(
                            if node.valuestring.is_null() {
                                None
                            } else {
                                Some(::core::ffi::CStr::from_ptr(node.valuestring.cast_const())
                                    .to_bytes()
                                    .to_vec())
                            },
                        )),
                        crate::src::cJSON::cJSON_Raw => values.push(PrintableValue::Raw(
                            if node.valuestring.is_null() {
                                None
                            } else {
                                Some(::core::ffi::CStr::from_ptr(node.valuestring.cast_const())
                                    .to_bytes()
                                    .to_vec())
                            },
                        )),
                        crate::src::cJSON::cJSON_Array | crate::src::cJSON::cJSON_Object => {
                            let mut children = Vec::new();
                            let mut keys = Vec::new();
                            let mut child = node.child;
                            while !child.is_null() {
                                let child_node = &*child;
                                children.push(child_node);
                                if node.type_0 & 0xff as ::core::ffi::c_int
                                    == crate::src::cJSON::cJSON_Object
                                {
                                    keys.push(if child_node.string.is_null() {
                                        None
                                    } else {
                                        Some(::core::ffi::CStr::from_ptr(
                                            child_node.string.cast_const(),
                                        )
                                        .to_bytes()
                                        .to_vec())
                                    });
                                }
                                child = child_node.next;
                            }
                            if node.type_0 & 0xff as ::core::ffi::c_int
                                == crate::src::cJSON::cJSON_Array
                            {
                                tasks.push(SnapshotTask::FinishArray(children.len()));
                            } else {
                                tasks.push(SnapshotTask::FinishObject(keys));
                            }
                            for child in children.into_iter().rev() {
                                tasks.push(SnapshotTask::Visit(child));
                            }
                        }
                        _ => values.push(PrintableValue::Invalid),
                    }
                }
                SnapshotTask::FinishArray(count) => {
                    let start = values.len().saturating_sub(count);
                    let children = values.split_off(start);
                    values.push(PrintableValue::Array(children));
                }
                SnapshotTask::FinishObject(keys) => {
                    let start = values.len().saturating_sub(keys.len());
                    let children = values.split_off(start);
                    values.push(PrintableValue::Object(keys.into_iter().zip(children).collect()));
                }
            }
        }
        values.pop().unwrap_or(PrintableValue::Invalid)
    }};
}

macro_rules! print_with_format {
    ($item:expr, $format:expr, $hooks:expr, $decimal_point:expr) => {{
    let hooks = $hooks;
    let mut buffer: [printbuffer; 1] = [printbuffer {
        buffer: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        length: 0,
        offset: 0,
        depth: 0,
        noalloc: 0,
        format: 0,
        hooks: internal_hooks {
            allocate: None,
            deallocate: None,
            reallocate: None,
        },
    }; 1];
    let mut printed: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    crate::stdlib::memset(
        &raw mut buffer as *mut printbuffer as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[printbuffer; 1]>(),
    );
    (*(&raw mut buffer as *mut printbuffer)).buffer =
        hooks.allocate.expect("non-null function pointer")(DEFAULT_PRINT_BUFFER_SIZE)
            as *mut ::core::ffi::c_uchar;
    (*(&raw mut buffer as *mut printbuffer)).length = DEFAULT_PRINT_BUFFER_SIZE;
    (*(&raw mut buffer as *mut printbuffer)).format = $format;
    (*(&raw mut buffer as *mut printbuffer)).hooks = hooks;
    '_fail: {
        if !(*(&raw mut buffer as *mut printbuffer)).buffer.is_null() && !$item.is_null() {
            let item = printable_snapshot!($item);
            if print_value(&item, &mut buffer[0], $decimal_point) != 0 {
                if hooks.reallocate.is_some() {
                    printed = hooks.reallocate.expect("non-null function pointer")(
                        (*(&raw mut buffer as *mut printbuffer)).buffer as *mut ::core::ffi::c_void,
                        (*(&raw mut buffer as *mut printbuffer))
                            .offset
                            .wrapping_add(1 as crate::__stddef_size_t_h::size_t),
                    ) as *mut ::core::ffi::c_uchar;
                    if printed.is_null() {
                        break '_fail;
                    } else {
                        (*(&raw mut buffer as *mut printbuffer)).buffer =
                            ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                    }
                } else {
                    printed = hooks.allocate.expect("non-null function pointer")(
                        (*(&raw mut buffer as *mut printbuffer))
                            .offset
                            .wrapping_add(1 as crate::__stddef_size_t_h::size_t),
                    ) as *mut ::core::ffi::c_uchar;
                    if printed.is_null() {
                        break '_fail;
                    } else {
                        crate::stdlib::memcpy(
                            printed as *mut ::core::ffi::c_void,
                            (*(&raw mut buffer as *mut printbuffer)).buffer
                                as *const ::core::ffi::c_void,
                            if (*(&raw mut buffer as *mut printbuffer)).length
                                < (*(&raw mut buffer as *mut printbuffer))
                                    .offset
                                    .wrapping_add(1 as crate::__stddef_size_t_h::size_t)
                            {
                                (*(&raw mut buffer as *mut printbuffer)).length
                            } else {
                                (*(&raw mut buffer as *mut printbuffer))
                                    .offset
                                    .wrapping_add(1 as crate::__stddef_size_t_h::size_t)
                            },
                        );
                        *printed.offset((*(&raw mut buffer as *mut printbuffer)).offset as isize) =
                            '\0' as ::core::ffi::c_uchar;
                        hooks.deallocate.expect("non-null function pointer")(
                            (*(&raw mut buffer as *mut printbuffer)).buffer
                                as *mut ::core::ffi::c_void,
                        );
                        (*(&raw mut buffer as *mut printbuffer)).buffer =
                            ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                    }
                }
                return printed as *mut ::core::ffi::c_char;
            }
        }
    }
    if !(*(&raw mut buffer as *mut printbuffer)).buffer.is_null() {
        hooks.deallocate.expect("non-null function pointer")(
            (*(&raw mut buffer as *mut printbuffer)).buffer as *mut ::core::ffi::c_void,
        );
        (*(&raw mut buffer as *mut printbuffer)).buffer =
            ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    if !printed.is_null() {
        hooks.deallocate.expect("non-null function pointer")(
            printed as *mut ::core::ffi::c_void,
        );
        printed = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    ::core::ptr::null_mut::<::core::ffi::c_uchar>()
    }};
}
#[export_name = "cJSON_Print"]

pub unsafe extern "C" fn cJSON_Print_ffi(
    mut item: *const crate::src::cJSON::cJSON,
) -> *mut ::core::ffi::c_char {
    let decimal_point = *(*crate::stdlib::localeconv()).decimal_point as ::core::ffi::c_uchar;
    print_with_format!(item, true_0, current_hooks(), decimal_point)
        as *mut ::core::ffi::c_char
}
#[export_name = "cJSON_PrintUnformatted"]

pub unsafe extern "C" fn cJSON_PrintUnformatted_ffi(
    mut item: *const crate::src::cJSON::cJSON,
) -> *mut ::core::ffi::c_char {
    let decimal_point = *(*crate::stdlib::localeconv()).decimal_point as ::core::ffi::c_uchar;
    print_with_format!(item, false_0, current_hooks(), decimal_point)
        as *mut ::core::ffi::c_char
}
#[export_name = "cJSON_PrintBuffered"]

pub unsafe extern "C" fn cJSON_PrintBuffered_ffi(
    mut item: *const crate::src::cJSON::cJSON,
    mut prebuffer: ::core::ffi::c_int,
    mut fmt: crate::src::cJSON::cJSON_bool,
) -> *mut ::core::ffi::c_char {
    let hooks = current_hooks();
    let mut p: printbuffer = printbuffer {
        buffer: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        length: 0 as crate::__stddef_size_t_h::size_t,
        offset: 0 as crate::__stddef_size_t_h::size_t,
        depth: 0 as crate::__stddef_size_t_h::size_t,
        noalloc: 0 as crate::src::cJSON::cJSON_bool,
        format: 0 as crate::src::cJSON::cJSON_bool,
        hooks: internal_hooks {
            allocate: None,
            deallocate: None,
            reallocate: None,
        },
    };
    if prebuffer < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    p.buffer = hooks.allocate.expect("non-null function pointer")(
        prebuffer as crate::__stddef_size_t_h::size_t,
    ) as *mut ::core::ffi::c_uchar;
    if p.buffer.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    p.length = prebuffer as crate::__stddef_size_t_h::size_t;
    p.offset = 0 as crate::__stddef_size_t_h::size_t;
    p.noalloc = false_0;
    p.format = fmt;
    let decimal_point = *(*crate::stdlib::localeconv()).decimal_point as ::core::ffi::c_uchar;
    p.hooks = hooks;
    if item.is_null() {
        hooks.deallocate.expect("non-null function pointer")(
            p.buffer as *mut ::core::ffi::c_void,
        );
        p.buffer = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let item = printable_snapshot!(item);
    if print_value(&item, &mut p, decimal_point) == 0 {
        hooks.deallocate.expect("non-null function pointer")(
            p.buffer as *mut ::core::ffi::c_void,
        );
        p.buffer = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return p.buffer as *mut ::core::ffi::c_char;
}
#[export_name = "cJSON_PrintPreallocated"]

pub unsafe extern "C" fn cJSON_PrintPreallocated_ffi(
    mut item: *mut crate::src::cJSON::cJSON,
    mut buffer: *mut ::core::ffi::c_char,
    length: ::core::ffi::c_int,
    format: crate::src::cJSON::cJSON_bool,
) -> crate::src::cJSON::cJSON_bool {
    let mut p: printbuffer = printbuffer {
        buffer: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        length: 0 as crate::__stddef_size_t_h::size_t,
        offset: 0 as crate::__stddef_size_t_h::size_t,
        depth: 0 as crate::__stddef_size_t_h::size_t,
        noalloc: 0 as crate::src::cJSON::cJSON_bool,
        format: 0 as crate::src::cJSON::cJSON_bool,
        hooks: internal_hooks {
            allocate: None,
            deallocate: None,
            reallocate: None,
        },
    };
    if length < 0 as ::core::ffi::c_int || buffer.is_null() {
        return false_0;
    }
    p.buffer = buffer as *mut ::core::ffi::c_uchar;
    p.length = length as crate::__stddef_size_t_h::size_t;
    p.offset = 0 as crate::__stddef_size_t_h::size_t;
    p.noalloc = true_0;
    p.format = format;
    let decimal_point = *(*crate::stdlib::localeconv()).decimal_point as ::core::ffi::c_uchar;
    p.hooks = current_hooks();
    if item.is_null() {
        return false_0;
    }
    let item = printable_snapshot!(item);
    return print_value(&item, &mut p, decimal_point);
}
fn parse_value(input_buffer: &mut parse_buffer<'_>) -> Option<ParsedValue> {
    if input_buffer
            .offset
            .wrapping_add(4 as crate::__stddef_size_t_h::size_t)
            <= input_buffer.length
        && (&input_buffer.content)[input_buffer.offset..].starts_with(b"null")
    {
        input_buffer.offset = input_buffer
            .offset
            .wrapping_add(4 as crate::__stddef_size_t_h::size_t);
        return Some(ParsedValue::Null);
    }
    if input_buffer
            .offset
            .wrapping_add(5 as crate::__stddef_size_t_h::size_t)
            <= input_buffer.length
        && (&input_buffer.content)[input_buffer.offset..].starts_with(b"false")
    {
        input_buffer.offset = input_buffer
            .offset
            .wrapping_add(5 as crate::__stddef_size_t_h::size_t);
        return Some(ParsedValue::False);
    }
    if input_buffer
            .offset
            .wrapping_add(4 as crate::__stddef_size_t_h::size_t)
            <= input_buffer.length
        && (&input_buffer.content)[input_buffer.offset..].starts_with(b"true")
    {
        input_buffer.offset = input_buffer
            .offset
            .wrapping_add(4 as crate::__stddef_size_t_h::size_t);
        return Some(ParsedValue::True);
    }
    if input_buffer
            .offset
            .wrapping_add(0 as crate::__stddef_size_t_h::size_t)
            < input_buffer.length
        && input_buffer.content[input_buffer.offset] as ::core::ffi::c_int
            == '"' as ::core::ffi::c_int
    {
        return parse_string(input_buffer).map(ParsedValue::String);
    }
    if input_buffer
            .offset
            .wrapping_add(0 as crate::__stddef_size_t_h::size_t)
            < input_buffer.length
        && (input_buffer.content[input_buffer.offset] as ::core::ffi::c_int
            == '-' as ::core::ffi::c_int
            || input_buffer.content[input_buffer.offset] as ::core::ffi::c_int
                >= '0' as ::core::ffi::c_int
                && input_buffer.content[input_buffer.offset]
                    as ::core::ffi::c_int
                    <= '9' as ::core::ffi::c_int)
    {
        return parse_number(input_buffer);
    }
    if input_buffer
            .offset
            .wrapping_add(0 as crate::__stddef_size_t_h::size_t)
            < input_buffer.length
        && input_buffer.content[input_buffer.offset] as ::core::ffi::c_int
            == '[' as ::core::ffi::c_int
    {
        return parse_array(input_buffer);
    }
    if input_buffer
            .offset
            .wrapping_add(0 as crate::__stddef_size_t_h::size_t)
            < input_buffer.length
        && input_buffer.content[input_buffer.offset] as ::core::ffi::c_int
            == '{' as ::core::ffi::c_int
    {
        return parse_object(input_buffer);
    }
    None
}

fn print_value(
    item: &PrintableValue,
    output_buffer: &mut printbuffer,
    decimal_point: ::core::ffi::c_uchar,
) -> crate::src::cJSON::cJSON_bool {
    match item {
        PrintableValue::Null => append_print_bytes(output_buffer, b"null\0"),
        PrintableValue::False => append_print_bytes(output_buffer, b"false\0"),
        PrintableValue::True => append_print_bytes(output_buffer, b"true\0"),
        PrintableValue::Number {
            valueint,
            valuedouble,
        } => {
            let Some(mut number) = format_number_for_print(*valueint, *valuedouble, decimal_point) else {
                return false_0;
            };
            number.push(b'\0');
            append_print_bytes(output_buffer, &number)
        }
        PrintableValue::Raw(Some(raw)) => {
            let mut raw = raw.clone();
            raw.push(b'\0');
            append_print_bytes(output_buffer, &raw)
        }
        PrintableValue::Raw(None) => false_0,
        PrintableValue::String(value) => print_string(value.as_deref(), output_buffer),
        PrintableValue::Array(values) => print_array(values, output_buffer, decimal_point),
        PrintableValue::Object(values) => print_object(values, output_buffer, decimal_point),
        PrintableValue::Invalid => false_0,
    }
}

fn parse_array(input_buffer: &mut parse_buffer<'_>) -> Option<ParsedValue> {
    if input_buffer.depth
        >= crate::src::cJSON::CJSON_NESTING_LIMIT as crate::__stddef_size_t_h::size_t
    {
        return None;
    }
    input_buffer.depth = input_buffer.depth.wrapping_add(1);
    input_buffer.offset = input_buffer.offset.wrapping_add(1);
    buffer_skip_whitespace(input_buffer);
    let result = (|| {
        if input_buffer.content.get(input_buffer.offset) == Some(&b']') {
            input_buffer.offset = input_buffer.offset.wrapping_add(1);
            return Some(ParsedValue::Array(Vec::new()));
        }
        let mut values = Vec::new();
        loop {
            values.push(parse_value(input_buffer)?);
            buffer_skip_whitespace(input_buffer);
            match input_buffer.content.get(input_buffer.offset) {
                Some(b',') => {
                    input_buffer.offset = input_buffer.offset.wrapping_add(1);
                    buffer_skip_whitespace(input_buffer);
                }
                Some(b']') => {
                    input_buffer.offset = input_buffer.offset.wrapping_add(1);
                    return Some(ParsedValue::Array(values));
                }
                _ => return None,
            }
        }
    })();
    input_buffer.depth = input_buffer.depth.wrapping_sub(1);
    result
}

fn print_array(
    values: &[PrintableValue],
    output_buffer: &mut printbuffer,
    decimal_point: ::core::ffi::c_uchar,
) -> crate::src::cJSON::cJSON_bool {
    if append_print_bytes(output_buffer, b"[\0") == 0 {
        return false_0;
    }
    output_buffer.depth = output_buffer.depth.wrapping_add(1);
    for (index, value) in values.iter().enumerate() {
        if print_value(value, output_buffer, decimal_point) == 0 {
            return false_0;
        }
        if index + 1 < values.len()
            && append_print_bytes(
                output_buffer,
                if output_buffer.format != 0 { b", \0" } else { b",\0" },
            ) == 0
        {
            return false_0;
        }
    }
    if append_print_bytes(output_buffer, b"]\0") == 0 {
        return false_0;
    }
    output_buffer.depth = output_buffer.depth.wrapping_sub(1);
    true_0
}

fn parse_object(input_buffer: &mut parse_buffer<'_>) -> Option<ParsedValue> {
    if input_buffer.depth
        >= crate::src::cJSON::CJSON_NESTING_LIMIT as crate::__stddef_size_t_h::size_t
    {
        return None;
    }
    input_buffer.depth = input_buffer.depth.wrapping_add(1);
    input_buffer.offset = input_buffer.offset.wrapping_add(1);
    buffer_skip_whitespace(input_buffer);
    let result = (|| {
        if input_buffer.content.get(input_buffer.offset) == Some(&b'}') {
            input_buffer.offset = input_buffer.offset.wrapping_add(1);
            return Some(ParsedValue::Object(Vec::new()));
        }
        let mut values = Vec::new();
        loop {
            let key = parse_string(input_buffer)?;
            buffer_skip_whitespace(input_buffer);
            if input_buffer.content.get(input_buffer.offset) != Some(&b':') {
                return None;
            }
            input_buffer.offset = input_buffer.offset.wrapping_add(1);
            buffer_skip_whitespace(input_buffer);
            let value = parse_value(input_buffer)?;
            values.push((key, value));
            buffer_skip_whitespace(input_buffer);
            match input_buffer.content.get(input_buffer.offset) {
                Some(b',') => {
                    input_buffer.offset = input_buffer.offset.wrapping_add(1);
                    buffer_skip_whitespace(input_buffer);
                }
                Some(b'}') => {
                    input_buffer.offset = input_buffer.offset.wrapping_add(1);
                    return Some(ParsedValue::Object(values));
                }
                _ => return None,
            }
        }
    })();
    input_buffer.depth = input_buffer.depth.wrapping_sub(1);
    result
}

fn print_object(
    values: &[(Option<Vec<u8>>, PrintableValue)],
    output_buffer: &mut printbuffer,
    decimal_point: ::core::ffi::c_uchar,
) -> crate::src::cJSON::cJSON_bool {
    if append_print_bytes(
        output_buffer,
        if output_buffer.format != 0 { b"{\n\0" } else { b"{\0" },
    ) == 0
    {
        return false_0;
    }
    output_buffer.depth = output_buffer.depth.wrapping_add(1);
    for (index, (key, value)) in values.iter().enumerate() {
        if output_buffer.format != 0 {
            let mut indentation = vec![b'\t'; output_buffer.depth];
            indentation.push(b'\0');
            if append_print_bytes(output_buffer, &indentation) == 0 {
                return false_0;
            }
        }
        if print_string(key.as_deref(), output_buffer) == 0
            || append_print_bytes(
                output_buffer,
                if output_buffer.format != 0 { b":\t\0" } else { b":\0" },
            ) == 0
            || print_value(value, output_buffer, decimal_point) == 0
        {
            return false_0;
        }
        if index + 1 < values.len() && append_print_bytes(output_buffer, b",\0") == 0 {
            return false_0;
        }
        if output_buffer.format != 0 && append_print_bytes(output_buffer, b"\n\0") == 0 {
            return false_0;
        }
    }
    if output_buffer.format != 0 {
        let mut indentation = vec![b'\t'; output_buffer.depth.wrapping_sub(1)];
        indentation.push(b'}');
        indentation.push(b'\0');
        if append_print_bytes(output_buffer, &indentation) == 0 {
            return false_0;
        }
    } else if append_print_bytes(output_buffer, b"}\0") == 0 {
        return false_0;
    }
    output_buffer.depth = output_buffer.depth.wrapping_sub(1);
    true_0
}
#[export_name = "cJSON_GetArraySize"]

pub unsafe extern "C" fn cJSON_GetArraySize_ffi(
    array: *const crate::src::cJSON::cJSON,
) -> ::core::ffi::c_int {
    let Some(array) = (unsafe { array.as_ref() }) else {
        return 0;
    };
    let mut child = array.child;
    let mut size = 0usize;
    while let Some(current_child) = unsafe { child.as_ref() } {
        size = size.wrapping_add(1);
        child = current_child.next;
    }
    size as ::core::ffi::c_int
}
#[export_name = "cJSON_GetArrayItem"]

pub unsafe extern "C" fn cJSON_GetArrayItem_ffi(
    array: *const crate::src::cJSON::cJSON,
    mut index: ::core::ffi::c_int,
) -> *mut crate::src::cJSON::cJSON {
    if index < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    }
    let Some(array) = (unsafe { array.as_ref() }) else {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    };
    let mut current_child = array.child;
    while index > 0 {
        let Some(current) = (unsafe { current_child.as_ref() }) else {
            return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
        };
        index -= 1;
        current_child = current.next;
    }
    current_child
}
#[export_name = "cJSON_GetObjectItem"]

pub unsafe extern "C" fn cJSON_GetObjectItem_ffi(
    object: *const crate::src::cJSON::cJSON,
    string: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    let Some(object) = (unsafe { object.as_ref() }) else {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    };
    if string.is_null() {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    }
    let mut current_element = object.child;
    let string = unsafe { ::core::ffi::CStr::from_ptr(string) };
    while let Some(current) = unsafe { current_element.as_ref() } {
        let current_string = current.string;
        if !current_string.is_null()
            && c_string_cmp(
                string,
                unsafe { ::core::ffi::CStr::from_ptr(current_string) },
                false,
            ) == 0
        {
            return current_element;
        }
        current_element = current.next;
    }
    ::core::ptr::null_mut::<crate::src::cJSON::cJSON>()
}
#[export_name = "cJSON_GetObjectItemCaseSensitive"]

pub unsafe extern "C" fn cJSON_GetObjectItemCaseSensitive_ffi(
    object: *const crate::src::cJSON::cJSON,
    string: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    let Some(object) = (unsafe { object.as_ref() }) else {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    };
    if string.is_null() {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    }
    let mut current_element = object.child;
    let string = unsafe { ::core::ffi::CStr::from_ptr(string) };
    while let Some(current) = unsafe { current_element.as_ref() } {
        let current_string = current.string;
        if current_string.is_null() {
            return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
        }
        if c_string_cmp(
            string,
            unsafe { ::core::ffi::CStr::from_ptr(current_string) },
            true,
        ) == 0
        {
            return current_element;
        }
        current_element = current.next;
    }
    ::core::ptr::null_mut::<crate::src::cJSON::cJSON>()
}
#[export_name = "cJSON_HasObjectItem"]

pub unsafe extern "C" fn cJSON_HasObjectItem_ffi(
    object: *const crate::src::cJSON::cJSON,
    string: *const ::core::ffi::c_char,
) -> crate::src::cJSON::cJSON_bool {
    if !cJSON_GetObjectItem_ffi(object, string).is_null() {
        1 as crate::src::cJSON::cJSON_bool
    } else {
        0 as crate::src::cJSON::cJSON_bool
    }
}
#[export_name = "cJSON_AddItemToArray"]
pub unsafe extern "C" fn cJSON_AddItemToArray_ffi(
    mut array: *mut crate::src::cJSON::cJSON,
    mut item: *mut crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    let mut child: *mut crate::src::cJSON::cJSON =
        ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    if item.is_null() || array.is_null() || array == item {
        return false_0;
    }
    child = (*array).child as *mut crate::src::cJSON::cJSON;
    if child.is_null() {
        (*array).child = item as *mut crate::src::cJSON::cJSON;
        (*item).prev = item as *mut crate::src::cJSON::cJSON;
        (*item).next = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    } else if !(*child).prev.is_null() {
        (*(*child).prev).next = item as *mut crate::src::cJSON::cJSON;
        (*item).prev = (*child).prev as *mut crate::src::cJSON::cJSON;
        (*(*array).child).prev = item as *mut crate::src::cJSON::cJSON;
    }
    return true_0;
}
#[export_name = "cJSON_AddItemToObject"]

pub unsafe extern "C" fn cJSON_AddItemToObject_ffi(
    mut object: *mut crate::src::cJSON::cJSON,
    mut string: *const ::core::ffi::c_char,
    mut item: *mut crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    if object.is_null() || string.is_null() || item.is_null() || object == item {
        return false_0;
    }
    let new_key = cjson_strdup!(string as *const ::core::ffi::c_uchar, current_hooks())
        as *mut ::core::ffi::c_char;
    if new_key.is_null() {
        return false_0;
    }
    let new_type = (*item).type_0 & !crate::src::cJSON::cJSON_StringIsConst;
    if (*item).type_0 & crate::src::cJSON::cJSON_StringIsConst == 0 && !(*item).string.is_null() {
        current_hooks().deallocate.expect("non-null function pointer")(
            (*item).string as *mut ::core::ffi::c_void,
        );
    }
    (*item).string = new_key;
    (*item).type_0 = new_type;
    cJSON_AddItemToArray_ffi(object, item)
}
#[export_name = "cJSON_AddItemToObjectCS"]

pub unsafe extern "C" fn cJSON_AddItemToObjectCS_ffi(
    mut object: *mut crate::src::cJSON::cJSON,
    mut string: *const ::core::ffi::c_char,
    mut item: *mut crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    if object.is_null() || string.is_null() || item.is_null() || object == item {
        return false_0;
    }
    if (*item).type_0 & crate::src::cJSON::cJSON_StringIsConst == 0 && !(*item).string.is_null() {
        current_hooks().deallocate.expect("non-null function pointer")(
            (*item).string as *mut ::core::ffi::c_void,
        );
    }
    (*item).string = string as *mut ::core::ffi::c_char;
    (*item).type_0 |= crate::src::cJSON::cJSON_StringIsConst;
    cJSON_AddItemToArray_ffi(object, item)
}
#[export_name = "cJSON_AddItemReferenceToArray"]

pub unsafe extern "C" fn cJSON_AddItemReferenceToArray_ffi(
    mut array: *mut crate::src::cJSON::cJSON,
    mut item: *mut crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    let mut reference: *mut crate::src::cJSON::cJSON =
        ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    if array.is_null() {
        return false_0;
    }
    if !item.is_null() {
        reference = cJSON_New_Item(&current_hooks())
            .map_or(::core::ptr::null_mut(), ::core::ptr::NonNull::as_ptr);
        if !reference.is_null() {
            crate::stdlib::memcpy(
                reference as *mut ::core::ffi::c_void,
                item as *const ::core::ffi::c_void,
                ::core::mem::size_of::<crate::src::cJSON::cJSON>(),
            );
            (*reference).string = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*reference).type_0 |= crate::src::cJSON::cJSON_IsReference;
            (*reference).prev = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
            (*reference).next = (*reference).prev;
        }
    }
    cJSON_AddItemToArray_ffi(array, reference)
}
#[export_name = "cJSON_AddItemReferenceToObject"]

pub unsafe extern "C" fn cJSON_AddItemReferenceToObject_ffi(
    mut object: *mut crate::src::cJSON::cJSON,
    mut string: *const ::core::ffi::c_char,
    mut item: *mut crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    let mut reference: *mut crate::src::cJSON::cJSON =
        ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    if object.is_null() || string.is_null() {
        return false_0;
    }
    if !item.is_null() {
        reference = cJSON_New_Item(&current_hooks())
            .map_or(::core::ptr::null_mut(), ::core::ptr::NonNull::as_ptr);
        if !reference.is_null() {
            crate::stdlib::memcpy(
                reference as *mut ::core::ffi::c_void,
                item as *const ::core::ffi::c_void,
                ::core::mem::size_of::<crate::src::cJSON::cJSON>(),
            );
            (*reference).string = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*reference).type_0 |= crate::src::cJSON::cJSON_IsReference;
            (*reference).prev = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
            (*reference).next = (*reference).prev;
        }
    }
    cJSON_AddItemToObject_ffi(object, string, reference)
}
#[export_name = "cJSON_AddNullToObject"]

pub unsafe extern "C" fn cJSON_AddNullToObject_ffi(
    object: *mut crate::src::cJSON::cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    let null = cJSON_CreateNull_ffi();
    if cJSON_AddItemToObject_ffi(object, name, null) != 0 {
        return null;
    }
    cJSON_Delete_ffi(null);
    return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
}
#[export_name = "cJSON_AddTrueToObject"]

pub unsafe extern "C" fn cJSON_AddTrueToObject_ffi(
    object: *mut crate::src::cJSON::cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    let true_item = cJSON_CreateTrue_ffi();
    if cJSON_AddItemToObject_ffi(object, name, true_item) != 0 {
        return true_item;
    }
    cJSON_Delete_ffi(true_item);
    return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
}
#[export_name = "cJSON_AddFalseToObject"]

pub unsafe extern "C" fn cJSON_AddFalseToObject_ffi(
    object: *mut crate::src::cJSON::cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    let false_item = cJSON_CreateFalse_ffi();
    if cJSON_AddItemToObject_ffi(object, name, false_item) != 0 {
        return false_item;
    }
    cJSON_Delete_ffi(false_item);
    return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
}
#[export_name = "cJSON_AddBoolToObject"]

pub unsafe extern "C" fn cJSON_AddBoolToObject_ffi(
    object: *mut crate::src::cJSON::cJSON,
    name: *const ::core::ffi::c_char,
    boolean: crate::src::cJSON::cJSON_bool,
) -> *mut crate::src::cJSON::cJSON {
    let bool_item = cJSON_CreateBool_ffi(boolean);
    if cJSON_AddItemToObject_ffi(object, name, bool_item) != 0 {
        return bool_item;
    }
    cJSON_Delete_ffi(bool_item);
    return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
}
#[export_name = "cJSON_AddNumberToObject"]

pub unsafe extern "C" fn cJSON_AddNumberToObject_ffi(
    object: *mut crate::src::cJSON::cJSON,
    name: *const ::core::ffi::c_char,
    number: ::core::ffi::c_double,
) -> *mut crate::src::cJSON::cJSON {
    let number_item = cJSON_CreateNumber_ffi(number);
    if cJSON_AddItemToObject_ffi(object, name, number_item) != 0 {
        return number_item;
    }
    cJSON_Delete_ffi(number_item);
    return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
}
#[export_name = "cJSON_AddStringToObject"]

pub unsafe extern "C" fn cJSON_AddStringToObject_ffi(
    object: *mut crate::src::cJSON::cJSON,
    name: *const ::core::ffi::c_char,
    string: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    let string_item = cJSON_CreateString_ffi(string);
    if cJSON_AddItemToObject_ffi(object, name, string_item) != 0 {
        return string_item;
    }
    cJSON_Delete_ffi(string_item);
    return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
}
#[export_name = "cJSON_AddRawToObject"]

pub unsafe extern "C" fn cJSON_AddRawToObject_ffi(
    object: *mut crate::src::cJSON::cJSON,
    name: *const ::core::ffi::c_char,
    raw: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    let raw_item = cJSON_CreateRaw_ffi(raw);
    if cJSON_AddItemToObject_ffi(object, name, raw_item) != 0 {
        return raw_item;
    }
    cJSON_Delete_ffi(raw_item);
    return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
}
#[export_name = "cJSON_AddObjectToObject"]

pub unsafe extern "C" fn cJSON_AddObjectToObject_ffi(
    object: *mut crate::src::cJSON::cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    let object_item = cJSON_CreateObject_ffi();
    if cJSON_AddItemToObject_ffi(object, name, object_item) != 0 {
        return object_item;
    }
    cJSON_Delete_ffi(object_item);
    return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
}
#[export_name = "cJSON_AddArrayToObject"]

pub unsafe extern "C" fn cJSON_AddArrayToObject_ffi(
    object: *mut crate::src::cJSON::cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    let array = cJSON_CreateArray_ffi();
    if cJSON_AddItemToObject_ffi(object, name, array) != 0 {
        return array;
    }
    cJSON_Delete_ffi(array);
    return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
}
#[export_name = "cJSON_DetachItemViaPointer"]

pub unsafe extern "C" fn cJSON_DetachItemViaPointer_ffi(
    mut parent: *mut crate::src::cJSON::cJSON,
    item: *mut crate::src::cJSON::cJSON,
) -> *mut crate::src::cJSON::cJSON {
    if parent.is_null() || item.is_null() || item != (*parent).child && (*item).prev.is_null() {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    }
    if item != (*parent).child {
        (*(*item).prev).next = (*item).next;
    }
    if !(*item).next.is_null() {
        (*(*item).next).prev = (*item).prev;
    }
    if item == (*parent).child {
        (*parent).child = (*item).next;
    } else if (*item).next.is_null() {
        (*(*parent).child).prev = (*item).prev;
    }
    (*item).prev = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    (*item).next = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    return item;
}
#[export_name = "cJSON_DetachItemFromArray"]

pub unsafe extern "C" fn cJSON_DetachItemFromArray_ffi(
    mut array: *mut crate::src::cJSON::cJSON,
    mut which: ::core::ffi::c_int,
) -> *mut crate::src::cJSON::cJSON {
    if which < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    }
    cJSON_DetachItemViaPointer_ffi(
        array,
        cJSON_GetArrayItem_ffi(array, which),
    )
}
#[export_name = "cJSON_DeleteItemFromArray"]

pub unsafe extern "C" fn cJSON_DeleteItemFromArray_ffi(
    mut array: *mut crate::src::cJSON::cJSON,
    mut which: ::core::ffi::c_int,
) {
    cJSON_Delete_ffi(cJSON_DetachItemFromArray_ffi(array, which));
}
#[export_name = "cJSON_DetachItemFromObject"]

pub unsafe extern "C" fn cJSON_DetachItemFromObject_ffi(
    mut object: *mut crate::src::cJSON::cJSON,
    mut string: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    let to_detach = cJSON_GetObjectItem_ffi(object, string);
    cJSON_DetachItemViaPointer_ffi(object, to_detach)
}
#[export_name = "cJSON_DetachItemFromObjectCaseSensitive"]

pub unsafe extern "C" fn cJSON_DetachItemFromObjectCaseSensitive_ffi(
    mut object: *mut crate::src::cJSON::cJSON,
    mut string: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    let to_detach = cJSON_GetObjectItemCaseSensitive_ffi(object, string);
    cJSON_DetachItemViaPointer_ffi(object, to_detach)
}
#[export_name = "cJSON_DeleteItemFromObject"]

pub unsafe extern "C" fn cJSON_DeleteItemFromObject_ffi(
    mut object: *mut crate::src::cJSON::cJSON,
    mut string: *const ::core::ffi::c_char,
) {
    cJSON_Delete_ffi(cJSON_DetachItemFromObject_ffi(object, string));
}
#[export_name = "cJSON_DeleteItemFromObjectCaseSensitive"]

pub unsafe extern "C" fn cJSON_DeleteItemFromObjectCaseSensitive_ffi(
    mut object: *mut crate::src::cJSON::cJSON,
    mut string: *const ::core::ffi::c_char,
) {
    cJSON_Delete_ffi(cJSON_DetachItemFromObjectCaseSensitive_ffi(object, string));
}
#[export_name = "cJSON_InsertItemInArray"]
pub unsafe extern "C" fn cJSON_InsertItemInArray_ffi(
    mut array: *mut crate::src::cJSON::cJSON,
    mut which: ::core::ffi::c_int,
    mut newitem: *mut crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    let mut after_inserted: *mut crate::src::cJSON::cJSON =
        ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    if which < 0 as ::core::ffi::c_int || newitem.is_null() {
        return false_0;
    }
    after_inserted = cJSON_GetArrayItem_ffi(array, which);
    if after_inserted.is_null() {
        return cJSON_AddItemToArray_ffi(array, newitem);
    }
    if after_inserted != (*array).child && (*after_inserted).prev.is_null() {
        return false_0;
    }
    (*newitem).next = after_inserted as *mut crate::src::cJSON::cJSON;
    (*newitem).prev = (*after_inserted).prev;
    (*after_inserted).prev = newitem as *mut crate::src::cJSON::cJSON;
    if after_inserted == (*array).child {
        (*array).child = newitem as *mut crate::src::cJSON::cJSON;
    } else {
        (*(*newitem).prev).next = newitem as *mut crate::src::cJSON::cJSON;
    }
    return true_0;
}
#[export_name = "cJSON_ReplaceItemViaPointer"]

pub unsafe extern "C" fn cJSON_ReplaceItemViaPointer_ffi(
    parent: *mut crate::src::cJSON::cJSON,
    item: *mut crate::src::cJSON::cJSON,
    mut replacement: *mut crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    if parent.is_null() || (*parent).child.is_null() || replacement.is_null() || item.is_null() {
        return false_0;
    }
    if replacement == item {
        return true_0;
    }
    (*replacement).next = (*item).next;
    (*replacement).prev = (*item).prev;
    if !(*replacement).next.is_null() {
        (*(*replacement).next).prev = replacement as *mut crate::src::cJSON::cJSON;
    }
    if (*parent).child == item {
        if (*(*parent).child).prev == (*parent).child {
            (*replacement).prev = replacement as *mut crate::src::cJSON::cJSON;
        }
        (*parent).child = replacement as *mut crate::src::cJSON::cJSON;
    } else {
        if !(*replacement).prev.is_null() {
            (*(*replacement).prev).next = replacement as *mut crate::src::cJSON::cJSON;
        }
        if (*replacement).next.is_null() {
            (*(*parent).child).prev = replacement as *mut crate::src::cJSON::cJSON;
        }
    }
    (*item).next = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    (*item).prev = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    cJSON_Delete_ffi(item);
    true_0
}
#[export_name = "cJSON_ReplaceItemInArray"]
pub unsafe extern "C" fn cJSON_ReplaceItemInArray_ffi(
    mut array: *mut crate::src::cJSON::cJSON,
    mut which: ::core::ffi::c_int,
    mut newitem: *mut crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    if which < 0 as ::core::ffi::c_int {
        return false_0;
    }
    return cJSON_ReplaceItemViaPointer_ffi(
        array,
        cJSON_GetArrayItem_ffi(array, which),
        newitem,
    );
}
#[export_name = "cJSON_ReplaceItemInObject"]
pub unsafe extern "C" fn cJSON_ReplaceItemInObject_ffi(
    mut object: *mut crate::src::cJSON::cJSON,
    mut string: *const ::core::ffi::c_char,
    mut newitem: *mut crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    if newitem.is_null() || string.is_null() {
        return false_0;
    }
    if (*newitem).type_0 & crate::src::cJSON::cJSON_StringIsConst == 0
        && !(*newitem).string.is_null()
    {
        cJSON_free_ffi((*newitem).string as *mut ::core::ffi::c_void);
    }
    (*newitem).string =
        cjson_strdup!(string as *const ::core::ffi::c_uchar, current_hooks())
            as *mut ::core::ffi::c_char;
    if (*newitem).string.is_null() {
        return false_0;
    }
    (*newitem).type_0 &= !crate::src::cJSON::cJSON_StringIsConst;
    cJSON_ReplaceItemViaPointer_ffi(object, cJSON_GetObjectItem_ffi(object, string), newitem)
}
#[export_name = "cJSON_ReplaceItemInObjectCaseSensitive"]
pub unsafe extern "C" fn cJSON_ReplaceItemInObjectCaseSensitive_ffi(
    mut object: *mut crate::src::cJSON::cJSON,
    mut string: *const ::core::ffi::c_char,
    mut newitem: *mut crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    if newitem.is_null() || string.is_null() {
        return false_0;
    }
    if (*newitem).type_0 & crate::src::cJSON::cJSON_StringIsConst == 0
        && !(*newitem).string.is_null()
    {
        cJSON_free_ffi((*newitem).string as *mut ::core::ffi::c_void);
    }
    (*newitem).string =
        cjson_strdup!(string as *const ::core::ffi::c_uchar, current_hooks())
            as *mut ::core::ffi::c_char;
    if (*newitem).string.is_null() {
        return false_0;
    }
    (*newitem).type_0 &= !crate::src::cJSON::cJSON_StringIsConst;
    cJSON_ReplaceItemViaPointer_ffi(
        object,
        cJSON_GetObjectItemCaseSensitive_ffi(object, string),
        newitem,
    )
}
#[export_name = "cJSON_CreateNull"]

pub unsafe extern "C" fn cJSON_CreateNull_ffi() -> *mut crate::src::cJSON::cJSON {
    let mut item: *mut crate::src::cJSON::cJSON = cJSON_New_Item(&current_hooks())
        .map_or(::core::ptr::null_mut(), ::core::ptr::NonNull::as_ptr);
    if !item.is_null() {
        (*item).type_0 = crate::src::cJSON::cJSON_NULL;
    }
    item
}
#[export_name = "cJSON_CreateTrue"]

pub unsafe extern "C" fn cJSON_CreateTrue_ffi() -> *mut crate::src::cJSON::cJSON {
    let mut item: *mut crate::src::cJSON::cJSON = cJSON_New_Item(&current_hooks())
        .map_or(::core::ptr::null_mut(), ::core::ptr::NonNull::as_ptr);
    if !item.is_null() {
        (*item).type_0 = crate::src::cJSON::cJSON_True;
    }
    item
}
#[export_name = "cJSON_CreateFalse"]

pub unsafe extern "C" fn cJSON_CreateFalse_ffi() -> *mut crate::src::cJSON::cJSON {
    let mut item: *mut crate::src::cJSON::cJSON = cJSON_New_Item(&current_hooks())
        .map_or(::core::ptr::null_mut(), ::core::ptr::NonNull::as_ptr);
    if !item.is_null() {
        (*item).type_0 = crate::src::cJSON::cJSON_False;
    }
    item
}
#[export_name = "cJSON_CreateBool"]

pub unsafe extern "C" fn cJSON_CreateBool_ffi(
    mut boolean: crate::src::cJSON::cJSON_bool,
) -> *mut crate::src::cJSON::cJSON {
    let mut item: *mut crate::src::cJSON::cJSON = cJSON_New_Item(&current_hooks())
        .map_or(::core::ptr::null_mut(), ::core::ptr::NonNull::as_ptr);
    if !item.is_null() {
        (*item).type_0 = if boolean != 0 {
            crate::src::cJSON::cJSON_True
        } else {
            crate::src::cJSON::cJSON_False
        };
    }
    item
}
#[export_name = "cJSON_CreateNumber"]
pub unsafe extern "C" fn cJSON_CreateNumber_ffi(
    mut num: ::core::ffi::c_double,
) -> *mut crate::src::cJSON::cJSON {
    let mut item: *mut crate::src::cJSON::cJSON = cJSON_New_Item(&current_hooks())
        .map_or(::core::ptr::null_mut(), ::core::ptr::NonNull::as_ptr);
    if !item.is_null() {
        (*item).type_0 = crate::src::cJSON::cJSON_Number;
        (*item).valuedouble = num;
        if num >= crate::limits_h::INT_MAX as ::core::ffi::c_double {
            (*item).valueint = crate::limits_h::INT_MAX;
        } else if num <= crate::limits_h::INT_MIN as ::core::ffi::c_double {
            (*item).valueint = crate::limits_h::INT_MIN;
        } else {
            (*item).valueint = num as ::core::ffi::c_int;
        }
    }
    return item;
}
#[export_name = "cJSON_CreateString"]
pub unsafe extern "C" fn cJSON_CreateString_ffi(
    mut string: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    let mut item: *mut crate::src::cJSON::cJSON = cJSON_New_Item(&current_hooks())
        .map_or(::core::ptr::null_mut(), ::core::ptr::NonNull::as_ptr);
    if !item.is_null() {
        (*item).type_0 = crate::src::cJSON::cJSON_String;
        (*item).valuestring =
            cjson_strdup!(string as *const ::core::ffi::c_uchar, current_hooks())
                as *mut ::core::ffi::c_char;
        if (*item).valuestring.is_null() {
            cJSON_Delete_ffi(item);
            return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
        }
    }
    return item;
}
#[export_name = "cJSON_CreateStringReference"]
pub unsafe extern "C" fn cJSON_CreateStringReference_ffi(
    string: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    let mut item: *mut crate::src::cJSON::cJSON = cJSON_New_Item(&current_hooks())
        .map_or(::core::ptr::null_mut(), ::core::ptr::NonNull::as_ptr);
    if !item.is_null() {
        (*item).type_0 = crate::src::cJSON::cJSON_String | crate::src::cJSON::cJSON_IsReference;
        (*item).valuestring = string as *mut ::core::ffi::c_char;
    }
    return item;
}
#[export_name = "cJSON_CreateObjectReference"]
pub unsafe extern "C" fn cJSON_CreateObjectReference_ffi(
    child: *const crate::src::cJSON::cJSON,
) -> *mut crate::src::cJSON::cJSON {
    let mut item: *mut crate::src::cJSON::cJSON = cJSON_New_Item(&current_hooks())
        .map_or(::core::ptr::null_mut(), ::core::ptr::NonNull::as_ptr);
    if !item.is_null() {
        (*item).type_0 = crate::src::cJSON::cJSON_Object | crate::src::cJSON::cJSON_IsReference;
        (*item).child = child as *mut crate::src::cJSON::cJSON;
    }
    return item;
}
#[export_name = "cJSON_CreateArrayReference"]
pub unsafe extern "C" fn cJSON_CreateArrayReference_ffi(
    child: *const crate::src::cJSON::cJSON,
) -> *mut crate::src::cJSON::cJSON {
    let mut item: *mut crate::src::cJSON::cJSON = cJSON_New_Item(&current_hooks())
        .map_or(::core::ptr::null_mut(), ::core::ptr::NonNull::as_ptr);
    if !item.is_null() {
        (*item).type_0 = crate::src::cJSON::cJSON_Array | crate::src::cJSON::cJSON_IsReference;
        (*item).child = child as *mut crate::src::cJSON::cJSON;
    }
    return item;
}
#[export_name = "cJSON_CreateRaw"]

pub unsafe extern "C" fn cJSON_CreateRaw_ffi(
    mut raw: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    let mut item: *mut crate::src::cJSON::cJSON = cJSON_New_Item(&current_hooks())
        .map_or(::core::ptr::null_mut(), ::core::ptr::NonNull::as_ptr);
    if !item.is_null() {
        (*item).type_0 = crate::src::cJSON::cJSON_Raw;
        (*item).valuestring =
            cjson_strdup!(raw as *const ::core::ffi::c_uchar, current_hooks())
                as *mut ::core::ffi::c_char;
        if (*item).valuestring.is_null() {
            cJSON_Delete_ffi(item);
            return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
        }
    }
    item
}
#[export_name = "cJSON_CreateArray"]
pub unsafe extern "C" fn cJSON_CreateArray_ffi() -> *mut crate::src::cJSON::cJSON {
    let mut item: *mut crate::src::cJSON::cJSON = cJSON_New_Item(&current_hooks())
        .map_or(::core::ptr::null_mut(), ::core::ptr::NonNull::as_ptr);
    if !item.is_null() {
        (*item).type_0 = crate::src::cJSON::cJSON_Array;
    }
    return item;
}
#[export_name = "cJSON_CreateObject"]
pub unsafe extern "C" fn cJSON_CreateObject_ffi() -> *mut crate::src::cJSON::cJSON {
    let mut item: *mut crate::src::cJSON::cJSON = cJSON_New_Item(&current_hooks())
        .map_or(::core::ptr::null_mut(), ::core::ptr::NonNull::as_ptr);
    if !item.is_null() {
        (*item).type_0 = crate::src::cJSON::cJSON_Object;
    }
    return item;
}
#[export_name = "cJSON_CreateIntArray"]
pub unsafe extern "C" fn cJSON_CreateIntArray_ffi(
    mut numbers: *const ::core::ffi::c_int,
    mut count: ::core::ffi::c_int,
) -> *mut crate::src::cJSON::cJSON {
    let mut n: *mut crate::src::cJSON::cJSON = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    let mut p: *mut crate::src::cJSON::cJSON = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    let mut a: *mut crate::src::cJSON::cJSON = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    }
    let numbers = unsafe { ::core::slice::from_raw_parts(numbers, count as usize) };
    a = cJSON_CreateArray_ffi();
    for (index, &number) in numbers.iter().enumerate() {
        if a.is_null() {
            break;
        }
        n = cJSON_CreateNumber_ffi(number as ::core::ffi::c_double);
        if n.is_null() {
            cJSON_Delete_ffi(a);
            return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
        }
        if index == 0 {
            (*a).child = n as *mut crate::src::cJSON::cJSON;
        } else {
            (*p).next = n as *mut crate::src::cJSON::cJSON;
            (*n).prev = p as *mut crate::src::cJSON::cJSON;
        }
        p = n;
    }
    if !a.is_null() && !(*a).child.is_null() {
        (*(*a).child).prev = n as *mut crate::src::cJSON::cJSON;
    }
    return a;
}
#[export_name = "cJSON_CreateFloatArray"]
pub unsafe extern "C" fn cJSON_CreateFloatArray_ffi(
    mut numbers: *const ::core::ffi::c_float,
    mut count: ::core::ffi::c_int,
) -> *mut crate::src::cJSON::cJSON {
    let mut n: *mut crate::src::cJSON::cJSON = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    let mut p: *mut crate::src::cJSON::cJSON = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    let mut a: *mut crate::src::cJSON::cJSON = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    }
    let numbers = unsafe { ::core::slice::from_raw_parts(numbers, count as usize) };
    a = cJSON_CreateArray_ffi();
    for (index, &number) in numbers.iter().enumerate() {
        if a.is_null() {
            break;
        }
        n = cJSON_CreateNumber_ffi(number as ::core::ffi::c_double);
        if n.is_null() {
            cJSON_Delete_ffi(a);
            return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
        }
        if index == 0 {
            (*a).child = n as *mut crate::src::cJSON::cJSON;
        } else {
            (*p).next = n as *mut crate::src::cJSON::cJSON;
            (*n).prev = p as *mut crate::src::cJSON::cJSON;
        }
        p = n;
    }
    if !a.is_null() && !(*a).child.is_null() {
        (*(*a).child).prev = n as *mut crate::src::cJSON::cJSON;
    }
    return a;
}
#[export_name = "cJSON_CreateDoubleArray"]
pub unsafe extern "C" fn cJSON_CreateDoubleArray_ffi(
    mut numbers: *const ::core::ffi::c_double,
    mut count: ::core::ffi::c_int,
) -> *mut crate::src::cJSON::cJSON {
    let mut n: *mut crate::src::cJSON::cJSON = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    let mut p: *mut crate::src::cJSON::cJSON = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    let mut a: *mut crate::src::cJSON::cJSON = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    }
    let numbers = unsafe { ::core::slice::from_raw_parts(numbers, count as usize) };
    a = cJSON_CreateArray_ffi();
    for (index, &number) in numbers.iter().enumerate() {
        if a.is_null() {
            break;
        }
        n = cJSON_CreateNumber_ffi(number);
        if n.is_null() {
            cJSON_Delete_ffi(a);
            return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
        }
        if index == 0 {
            (*a).child = n as *mut crate::src::cJSON::cJSON;
        } else {
            (*p).next = n as *mut crate::src::cJSON::cJSON;
            (*n).prev = p as *mut crate::src::cJSON::cJSON;
        }
        p = n;
    }
    if !a.is_null() && !(*a).child.is_null() {
        (*(*a).child).prev = n as *mut crate::src::cJSON::cJSON;
    }
    return a;
}
#[export_name = "cJSON_CreateStringArray"]
pub unsafe extern "C" fn cJSON_CreateStringArray_ffi(
    mut strings: *const *const ::core::ffi::c_char,
    mut count: ::core::ffi::c_int,
) -> *mut crate::src::cJSON::cJSON {
    let mut n: *mut crate::src::cJSON::cJSON = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    let mut p: *mut crate::src::cJSON::cJSON = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    let mut a: *mut crate::src::cJSON::cJSON = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    if count < 0 as ::core::ffi::c_int || strings.is_null() {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    }
    let strings = unsafe { ::core::slice::from_raw_parts(strings, count as usize) };
    a = cJSON_CreateArray_ffi();
    for (index, &string) in strings.iter().enumerate() {
        if a.is_null() {
            break;
        }
        n = cJSON_CreateString_ffi(string);
        if n.is_null() {
            cJSON_Delete_ffi(a);
            return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
        }
        if index == 0 {
            (*a).child = n as *mut crate::src::cJSON::cJSON;
        } else {
            (*p).next = n as *mut crate::src::cJSON::cJSON;
            (*n).prev = p as *mut crate::src::cJSON::cJSON;
        }
        p = n;
    }
    if !a.is_null() && !(*a).child.is_null() {
        (*(*a).child).prev = n as *mut crate::src::cJSON::cJSON;
    }
    return a;
}
#[export_name = "cJSON_Duplicate"]

pub unsafe extern "C" fn cJSON_Duplicate_ffi(
    mut item: *const crate::src::cJSON::cJSON,
    mut recurse: crate::src::cJSON::cJSON_bool,
) -> *mut crate::src::cJSON::cJSON {
    cJSON_Duplicate_rec_ffi(item, 0 as crate::__stddef_size_t_h::size_t, recurse)
}
#[export_name = "cJSON_Duplicate_rec"]
pub unsafe extern "C" fn cJSON_Duplicate_rec_ffi(
    mut item: *const crate::src::cJSON::cJSON,
    mut depth: crate::__stddef_size_t_h::size_t,
    mut recurse: crate::src::cJSON::cJSON_bool,
) -> *mut crate::src::cJSON::cJSON {
    let mut newitem: *mut crate::src::cJSON::cJSON =
        ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    let mut child: *mut crate::src::cJSON::cJSON =
        ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    let mut next: *mut crate::src::cJSON::cJSON =
        ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    let mut newchild: *mut crate::src::cJSON::cJSON =
        ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    '_fail: {
        if !item.is_null() {
            newitem = cJSON_New_Item(&current_hooks())
                .map_or(::core::ptr::null_mut(), ::core::ptr::NonNull::as_ptr);
            if !newitem.is_null() {
                (*newitem).type_0 = (*item).type_0 & !crate::src::cJSON::cJSON_IsReference;
                (*newitem).valueint = (*item).valueint;
                (*newitem).valuedouble = (*item).valuedouble;
                if !(*item).valuestring.is_null() {
                    (*newitem).valuestring = cjson_strdup!(
                        (*item).valuestring as *mut ::core::ffi::c_uchar,
                        current_hooks(),
                    ) as *mut ::core::ffi::c_char;
                    if (*newitem).valuestring.is_null() {
                        break '_fail;
                    }
                }
                if !(*item).string.is_null() {
                    (*newitem).string =
                        if (*item).type_0 & crate::src::cJSON::cJSON_StringIsConst != 0 {
                            (*item).string
                        } else {
                            cjson_strdup!(
                                (*item).string as *mut ::core::ffi::c_uchar,
                                current_hooks(),
                            ) as *mut ::core::ffi::c_char
                        };
                    if (*newitem).string.is_null() {
                        break '_fail;
                    }
                }
                if recurse == 0 {
                    return newitem;
                }
                child = (*item).child as *mut crate::src::cJSON::cJSON;
                while !child.is_null() {
                    if depth
                        >= crate::src::cJSON::CJSON_CIRCULAR_LIMIT
                            as crate::__stddef_size_t_h::size_t
                    {
                        break '_fail;
                    }
                    newchild = cJSON_Duplicate_rec_ffi(
                        child,
                        depth.wrapping_add(1 as crate::__stddef_size_t_h::size_t),
                        true_0,
                    );
                    if newchild.is_null() {
                        break '_fail;
                    }
                    if !next.is_null() {
                        (*next).next = newchild as *mut crate::src::cJSON::cJSON;
                        (*newchild).prev = next as *mut crate::src::cJSON::cJSON;
                        next = newchild;
                    } else {
                        (*newitem).child = newchild as *mut crate::src::cJSON::cJSON;
                        next = newchild;
                    }
                    child = (*child).next as *mut crate::src::cJSON::cJSON;
                }
                if !newitem.is_null() && !(*newitem).child.is_null() {
                    (*(*newitem).child).prev = newchild as *mut crate::src::cJSON::cJSON;
                }
                return newitem;
            }
        }
    }
    if !newitem.is_null() {
        cJSON_Delete_ffi(newitem);
    }
    return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
}
fn skip_oneline_comment(input: &[u8], read: &mut usize) {
    *read += 2;
    while input[*read] != 0 {
        if input[*read] == b'\n' {
            *read += 1;
            return;
        }
        *read += 1;
    }
}

fn skip_multiline_comment(input: &[u8], read: &mut usize) {
    *read += 2;
    while input[*read] != 0 {
        if input[*read] == b'*' && input.get(*read + 1) == Some(&b'/') {
            *read += 2;
            return;
        }
        *read += 1;
    }
}

fn minify_string(input: &mut [u8], read: &mut usize, write: &mut usize) {
    input[*write] = input[*read];
    *read += 1;
    *write += 1;

    while input[*read] != 0 {
        input[*write] = input[*read];
        if input[*read] == b'"' {
            *read += 1;
            *write += 1;
            return;
        }
        if input[*read] == b'\\' && input.get(*read + 1) == Some(&b'"') {
            input[*write + 1] = b'"';
            *read += 1;
            *write += 1;
        }
        *read += 1;
        *write += 1;
    }
}
fn minify_json(json: &mut [u8]) {
    let mut read = 0;
    let mut write = 0;

    while json[read] != 0 {
        match json[read] {
            b' ' | b'\t' | b'\r' | b'\n' => read += 1,
            b'/' if json.get(read + 1) == Some(&b'/') => skip_oneline_comment(json, &mut read),
            b'/' if json.get(read + 1) == Some(&b'*') => skip_multiline_comment(json, &mut read),
            b'/' => read += 1,
            b'"' => minify_string(json, &mut read, &mut write),
            _ => {
                json[write] = json[read];
                read += 1;
                write += 1;
            }
        }
    }
    json[write] = 0;
}
#[export_name = "cJSON_Minify"]

pub unsafe extern "C" fn cJSON_Minify_ffi(mut json: *mut ::core::ffi::c_char) {
    if json.is_null() {
        return;
    }

    let length = unsafe { ::core::ffi::CStr::from_ptr(json) }
        .to_bytes_with_nul()
        .len();
    let json = unsafe { ::core::slice::from_raw_parts_mut(json.cast::<u8>(), length) };
    minify_json(json);
}
fn cjson_type_is(item: &cJSON, expected_type: ::core::ffi::c_int) -> cJSON_bool {
    (item.type_0 & 0xff as ::core::ffi::c_int == expected_type) as ::core::ffi::c_int
}

fn cjson_is_bool(item: &cJSON) -> cJSON_bool {
    (item.type_0 & (cJSON_True | cJSON_False) != 0) as ::core::ffi::c_int
}
#[export_name = "cJSON_IsInvalid"]

pub unsafe extern "C" fn cJSON_IsInvalid_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    unsafe { item.as_ref() }
        .map(|item| cjson_type_is(item, cJSON_Invalid))
        .unwrap_or(false_0)
}
#[export_name = "cJSON_IsFalse"]

pub unsafe extern "C" fn cJSON_IsFalse_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    unsafe { item.as_ref() }
        .map(|item| cjson_type_is(item, cJSON_False))
        .unwrap_or(false_0)
}
#[export_name = "cJSON_IsTrue"]

pub unsafe extern "C" fn cJSON_IsTrue_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    unsafe { item.as_ref() }
        .map(|item| cjson_type_is(item, cJSON_True))
        .unwrap_or(false_0)
}
#[export_name = "cJSON_IsBool"]

pub unsafe extern "C" fn cJSON_IsBool_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    unsafe { item.as_ref() }
        .map(cjson_is_bool)
        .unwrap_or(false_0)
}
#[export_name = "cJSON_IsNull"]

pub unsafe extern "C" fn cJSON_IsNull_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    unsafe { item.as_ref() }
        .map(|item| cjson_type_is(item, cJSON_NULL))
        .unwrap_or(false_0)
}
#[export_name = "cJSON_IsNumber"]

pub unsafe extern "C" fn cJSON_IsNumber_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    unsafe { item.as_ref() }
        .map(|item| cjson_type_is(item, cJSON_Number))
        .unwrap_or(false_0)
}
#[export_name = "cJSON_IsString"]

pub unsafe extern "C" fn cJSON_IsString_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    unsafe { item.as_ref() }
        .map(|item| cjson_type_is(item, cJSON_String))
        .unwrap_or(false_0)
}
#[export_name = "cJSON_IsArray"]

pub unsafe extern "C" fn cJSON_IsArray_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    unsafe { item.as_ref() }
        .map(|item| cjson_type_is(item, cJSON_Array))
        .unwrap_or(false_0)
}
#[export_name = "cJSON_IsObject"]

pub unsafe extern "C" fn cJSON_IsObject_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    unsafe { item.as_ref() }
        .map(|item| cjson_type_is(item, cJSON_Object))
        .unwrap_or(false_0)
}
#[export_name = "cJSON_IsRaw"]

pub unsafe extern "C" fn cJSON_IsRaw_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    unsafe { item.as_ref() }
        .map(|item| cjson_type_is(item, cJSON_Raw))
        .unwrap_or(false_0)
}
#[export_name = "cJSON_Compare"]
pub unsafe extern "C" fn cJSON_Compare_ffi(
    a: *const crate::src::cJSON::cJSON,
    b: *const crate::src::cJSON::cJSON,
    case_sensitive: crate::src::cJSON::cJSON_bool,
) -> crate::src::cJSON::cJSON_bool {
    if a.is_null()
        || b.is_null()
        || (*a).type_0 & 0xff as ::core::ffi::c_int != (*b).type_0 & 0xff as ::core::ffi::c_int
    {
        return false_0;
    }
    match (*a).type_0 & 0xff as ::core::ffi::c_int {
        crate::src::cJSON::cJSON_False
        | crate::src::cJSON::cJSON_True
        | crate::src::cJSON::cJSON_NULL
        | crate::src::cJSON::cJSON_Number
        | crate::src::cJSON::cJSON_String
        | crate::src::cJSON::cJSON_Raw
        | crate::src::cJSON::cJSON_Array
        | crate::src::cJSON::cJSON_Object => {}
        _ => return false_0,
    }
    if a == b {
        return true_0;
    }
    match (*a).type_0 & 0xff as ::core::ffi::c_int {
        crate::src::cJSON::cJSON_False
        | crate::src::cJSON::cJSON_True
        | crate::src::cJSON::cJSON_NULL => return true_0,
        crate::src::cJSON::cJSON_Number => {
            if compare_double((*a).valuedouble, (*b).valuedouble) != 0 {
                return true_0;
            }
            return false_0;
        }
        crate::src::cJSON::cJSON_String | crate::src::cJSON::cJSON_Raw => {
            if (*a).valuestring.is_null() || (*b).valuestring.is_null() {
                return false_0;
            }
            let left = ::core::ffi::CStr::from_ptr((*a).valuestring);
            let right = ::core::ffi::CStr::from_ptr((*b).valuestring);
            if c_string_cmp(left, right, true) == 0 {
                return true_0;
            }
            return false_0;
        }
        crate::src::cJSON::cJSON_Array => {
            let mut a_element: *mut crate::src::cJSON::cJSON =
                (*a).child as *mut crate::src::cJSON::cJSON;
            let mut b_element: *mut crate::src::cJSON::cJSON =
                (*b).child as *mut crate::src::cJSON::cJSON;
            while !a_element.is_null() && !b_element.is_null() {
                if cJSON_Compare_ffi(a_element, b_element, case_sensitive) == 0 {
                    return false_0;
                }
                a_element = (*a_element).next as *mut crate::src::cJSON::cJSON;
                b_element = (*b_element).next as *mut crate::src::cJSON::cJSON;
            }
            if a_element != b_element {
                return false_0;
            }
            return true_0;
        }
        crate::src::cJSON::cJSON_Object => {
            let mut a_element_0: *mut crate::src::cJSON::cJSON =
                ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
            let mut b_element_0: *mut crate::src::cJSON::cJSON =
                ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
            a_element_0 = (if !a.is_null() {
                (*a).child
            } else {
                ::core::ptr::null_mut::<crate::src::cJSON::cJSON>()
            }) as *mut crate::src::cJSON::cJSON;
            while !a_element_0.is_null() {
                b_element_0 = if case_sensitive != 0 {
                    cJSON_GetObjectItemCaseSensitive_ffi(b, (*a_element_0).string)
                } else {
                    cJSON_GetObjectItem_ffi(b, (*a_element_0).string)
                };
                if b_element_0.is_null() {
                    return false_0;
                }
                if cJSON_Compare_ffi(a_element_0, b_element_0, case_sensitive) == 0 {
                    return false_0;
                }
                a_element_0 = (*a_element_0).next as *mut crate::src::cJSON::cJSON;
            }
            b_element_0 = (if !b.is_null() {
                (*b).child
            } else {
                ::core::ptr::null_mut::<crate::src::cJSON::cJSON>()
            }) as *mut crate::src::cJSON::cJSON;
            while !b_element_0.is_null() {
                a_element_0 = if case_sensitive != 0 {
                    cJSON_GetObjectItemCaseSensitive_ffi(a, (*b_element_0).string)
                } else {
                    cJSON_GetObjectItem_ffi(a, (*b_element_0).string)
                };
                if a_element_0.is_null() {
                    return false_0;
                }
                if cJSON_Compare_ffi(b_element_0, a_element_0, case_sensitive) == 0 {
                    return false_0;
                }
                b_element_0 = (*b_element_0).next as *mut crate::src::cJSON::cJSON;
            }
            return true_0;
        }
        _ => return false_0,
    };
}
#[export_name = "cJSON_malloc"]

pub unsafe extern "C" fn cJSON_malloc_ffi(
    mut size: crate::__stddef_size_t_h::size_t,
) -> *mut ::core::ffi::c_void {
    current_hooks().allocate.expect("non-null function pointer")(size)
}
#[export_name = "cJSON_free"]

pub unsafe extern "C" fn cJSON_free_ffi(object: *mut ::core::ffi::c_void) {
    current_hooks().deallocate.expect("non-null function pointer")(object);
}
