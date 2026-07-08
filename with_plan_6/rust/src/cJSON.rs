extern "C" {
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
#[repr(C)]
pub struct cJSON {
    pub next: ::core::cell::Cell<Option<&'static cJSON>>,
    pub prev: ::core::cell::Cell<Option<&'static cJSON>>,
    pub child: ::core::cell::Cell<Option<&'static cJSON>>,
    pub type_0: ::core::ffi::c_int,
    pub valuestring: ::core::cell::RefCell<Option<Vec<::core::ffi::c_uchar>>>,
    pub valueint: ::core::ffi::c_int,
    pub valuedouble: ::core::ffi::c_double,
    pub string: ::core::cell::RefCell<Option<Vec<::core::ffi::c_uchar>>>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cJSON_Hooks {
    pub malloc_fn: Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>,
    pub free_fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
}
pub type cJSON_bool = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct internal_hooks {
    pub allocate: Option<extern "C" fn(size_t) -> *mut ::core::ffi::c_void>,
    pub deallocate: Option<extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub reallocate:
        Option<extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>,
}
#[repr(C)]
pub struct parse_buffer {
    pub content: Vec<::core::ffi::c_uchar>,
    pub length: size_t,
    pub offset: size_t,
    pub depth: size_t,
    pub hooks: internal_hooks,
}
macro_rules! finish_parse_with_length_opts {
    ($result:expr, $value:expr, $return_parse_end:expr) => {{
        let (item, parse_end_offset, success) = $result;
        if success != 0 {
            if let Some(return_parse_end) = $return_parse_end {
                *return_parse_end = $value.offset(parse_end_offset as isize);
            }
            item.map(::core::ptr::NonNull::as_ptr)
                .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
        } else {
            if let Some(return_parse_end) = $return_parse_end {
                *return_parse_end = $value.offset(parse_end_offset as isize);
            }
            GLOBAL_ERROR_JSON_ADDRESS
                .store($value as usize, ::std::sync::atomic::Ordering::Relaxed);
            GLOBAL_ERROR_POSITION.store(parse_end_offset, ::std::sync::atomic::Ordering::Relaxed);
            ::core::ptr::null_mut::<cJSON>()
        }
    }};
}
macro_rules! hook_strdup_ffi {
    ($bytes:expr, $hooks:expr) => {{
        let bytes = $bytes;
        if !matches!(bytes.last(), Some(0)) {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        } else {
            let copy = ($hooks).allocate.expect("non-null function pointer")(bytes.len() as size_t)
                as *mut ::core::ffi::c_uchar;
            if copy.is_null() {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            } else {
                let copy_slice = ::core::slice::from_raw_parts_mut(copy, bytes.len());
                copy_slice.copy_from_slice(bytes);
                copy as *mut ::core::ffi::c_char
            }
        }
    }};
}
enum PrintBufferStorage<'a> {
    Dynamic(Vec<::core::ffi::c_uchar>),
    Preallocated(&'a mut [::core::ffi::c_uchar]),
}
pub struct printbuffer<'a> {
    storage: PrintBufferStorage<'a>,
    pub length: size_t,
    pub offset: size_t,
    pub depth: size_t,
    pub format: cJSON_bool,
}
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
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
pub const CJSON_NESTING_LIMIT: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const CJSON_CIRCULAR_LIMIT: ::core::ffi::c_int = 10000 as ::core::ffi::c_int;
pub const true_0: cJSON_bool = 1 as ::core::ffi::c_int;
pub const false_0: cJSON_bool = 0 as ::core::ffi::c_int;
static GLOBAL_ERROR_JSON_ADDRESS: ::std::sync::atomic::AtomicUsize =
    ::std::sync::atomic::AtomicUsize::new(0);
static GLOBAL_ERROR_POSITION: ::std::sync::atomic::AtomicUsize =
    ::std::sync::atomic::AtomicUsize::new(0);

pub fn cJSON_GetErrorPtr() -> Option<::core::num::NonZeroUsize> {
    let json_address = GLOBAL_ERROR_JSON_ADDRESS.load(::std::sync::atomic::Ordering::Relaxed);
    let position = GLOBAL_ERROR_POSITION.load(::std::sync::atomic::Ordering::Relaxed);

    ::core::num::NonZeroUsize::new(json_address.wrapping_add(position))
}
#[export_name = "cJSON_GetErrorPtr"]
pub unsafe extern "C" fn cJSON_GetErrorPtr_ffi() -> *const ::core::ffi::c_char {
    match cJSON_GetErrorPtr() {
        Some(error) => error.get() as *const ::core::ffi::c_char,
        None => ::core::ptr::null::<::core::ffi::c_char>(),
    }
}
pub fn cJSON_GetStringValue(
    item: Option<&cJSON>,
) -> Option<::core::ptr::NonNull<::core::ffi::c_char>> {
    if cJSON_IsString(item) == 0 {
        return None;
    }
    let item = item.expect("string item is present");
    let valuestring = item.valuestring.borrow();
    let valuestring = valuestring.as_ref()?;
    ::core::ptr::NonNull::new(valuestring.as_ptr() as *mut ::core::ffi::c_char)
}
#[export_name = "cJSON_GetStringValue"]
pub unsafe extern "C" fn cJSON_GetStringValue_ffi(item: *const cJSON) -> *mut ::core::ffi::c_char {
    match cJSON_GetStringValue(item.as_ref()) {
        Some(valuestring) => valuestring.as_ptr(),
        None => ::core::ptr::null_mut::<::core::ffi::c_char>(),
    }
}
pub fn cJSON_GetNumberValue(item: Option<&cJSON>) -> ::core::ffi::c_double {
    if cJSON_IsNumber(item) == 0 {
        return 0.0f64 / 0.0f64;
    }
    return item.expect("number item is present").valuedouble;
}
#[export_name = "cJSON_GetNumberValue"]
pub unsafe extern "C" fn cJSON_GetNumberValue_ffi(item: *const cJSON) -> ::core::ffi::c_double {
    cJSON_GetNumberValue(item.as_ref())
}
pub fn cJSON_Version() -> &'static ::std::ffi::CStr {
    return ::std::ffi::CStr::from_bytes_with_nul(b"1.7.19\0")
        .expect("static cJSON version string is NUL-terminated");
}
#[export_name = "cJSON_Version"]
pub unsafe extern "C" fn cJSON_Version_ffi() -> *const ::core::ffi::c_char {
    cJSON_Version().as_ptr()
}
fn c_string_bytes(string: &[::core::ffi::c_uchar]) -> &[::core::ffi::c_uchar] {
    let end = string
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(string.len());
    &string[..end]
}
fn c_string_bytes_with_nul(string: &[::core::ffi::c_uchar]) -> &[::core::ffi::c_uchar] {
    match string.iter().position(|&byte| byte == 0) {
        Some(end) => &string[..=end],
        None => string,
    }
}
fn case_insensitive_strcmp(
    string1: &[::core::ffi::c_uchar],
    string2: &[::core::ffi::c_uchar],
) -> ::core::ffi::c_int {
    if string1.as_ptr() == string2.as_ptr() {
        return 0 as ::core::ffi::c_int;
    }

    for (&byte1, &byte2) in c_string_bytes_with_nul(string1)
        .iter()
        .zip(c_string_bytes_with_nul(string2))
    {
        let lower1 = byte1.to_ascii_lowercase() as ::core::ffi::c_int;
        let lower2 = byte2.to_ascii_lowercase() as ::core::ffi::c_int;
        if lower1 != lower2 {
            return lower1 - lower2;
        }
        if byte1 as ::core::ffi::c_int == '\0' as i32 {
            return 0 as ::core::ffi::c_int;
        }
    }

    return 0 as ::core::ffi::c_int;
}
#[allow(non_upper_case_globals)]
static global_hooks: ::std::sync::Mutex<internal_hooks> = ::std::sync::Mutex::new(internal_hooks {
    allocate: Some(unsafe {
        ::core::mem::transmute(malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void)
    }),
    deallocate: Some(unsafe {
        ::core::mem::transmute(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ())
    }),
    reallocate: Some(unsafe {
        ::core::mem::transmute(
            realloc
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                ) -> *mut ::core::ffi::c_void,
        )
    }),
});
fn global_hooks_snapshot() -> internal_hooks {
    *global_hooks
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}
fn set_global_hooks(hooks: internal_hooks) {
    *global_hooks
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner()) = hooks;
}
#[export_name = "cJSON_InitHooks"]
pub unsafe extern "C" fn cJSON_InitHooks_ffi(mut hooks: *mut cJSON_Hooks) {
    let mut new_hooks = internal_hooks {
        allocate: Some(::core::mem::transmute(
            malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void,
        )),
        deallocate: Some(::core::mem::transmute(
            free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
        )),
        reallocate: Some(::core::mem::transmute(
            realloc
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                ) -> *mut ::core::ffi::c_void,
        )),
    };
    if hooks.is_null() {
        set_global_hooks(new_hooks);
        return;
    }
    if (*hooks).malloc_fn.is_some() {
        new_hooks.allocate = match (*hooks).malloc_fn {
            Some(hook) => Some(::core::mem::transmute(hook)),
            None => None,
        };
    }
    if (*hooks).free_fn.is_some() {
        new_hooks.deallocate = match (*hooks).free_fn {
            Some(hook) => Some(::core::mem::transmute(hook)),
            None => None,
        };
    }
    new_hooks.reallocate = None;
    if new_hooks.allocate
        == Some(::core::mem::transmute(
            malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void,
        ))
        && new_hooks.deallocate
            == Some(::core::mem::transmute(
                free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
            ))
    {
        new_hooks.reallocate = Some(::core::mem::transmute(
            realloc
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                ) -> *mut ::core::ffi::c_void,
        ));
    }
    set_global_hooks(new_hooks);
}
fn cJSON_strdup(string: &[u8]) -> Option<Vec<::core::ffi::c_uchar>> {
    if !matches!(string.last(), Some(0)) {
        return None;
    }
    Some(string.to_vec())
}
enum ValueStringInit<'a> {
    None,
    Copy {
        bytes: &'a [u8],
        failed: &'a mut bool,
    },
    Invalid {
        failed: &'a mut bool,
    },
}
fn cJSON_New_Item(
    _hooks: &internal_hooks,
    type_0: ::core::ffi::c_int,
    valueint: ::core::ffi::c_int,
    valuedouble: ::core::ffi::c_double,
    valuestring: ValueStringInit<'_>,
) -> Option<&'static mut cJSON> {
    let mut item = cJSON {
        next: ::core::cell::Cell::new(None),
        prev: ::core::cell::Cell::new(None),
        child: ::core::cell::Cell::new(None),
        type_0,
        valuestring: ::core::cell::RefCell::new(None),
        valueint,
        valuedouble,
        string: ::core::cell::RefCell::new(None),
    };
    match valuestring {
        ValueStringInit::None => {}
        ValueStringInit::Copy { bytes, failed } => {
            if let Some(copy) = cJSON_strdup(bytes) {
                *item.valuestring.borrow_mut() = Some(copy);
            } else {
                *failed = true;
                return None;
            };
        }
        ValueStringInit::Invalid { failed } => {
            *failed = true;
            return None;
        }
    }
    Some(Box::leak(Box::new(item)))
}
pub fn cJSON_Delete(item: Option<&cJSON>) {
    let mut item = item;

    while let Some(item_ref) = item {
        item = item_ref.next.get();

        if item_ref.type_0 & cJSON_IsReference == 0 {
            cJSON_Delete(item_ref.child.get());
        }
        item_ref.valuestring.borrow_mut().take();
        item_ref.string.borrow_mut().take();
    }
}
#[export_name = "cJSON_Delete"]
pub unsafe extern "C" fn cJSON_Delete_ffi(mut item: *mut cJSON) {
    cJSON_Delete(item.as_ref())
}
fn strtod_number_prefix_len(number: &[::core::ffi::c_uchar]) -> Option<size_t> {
    let mut i = 0 as size_t;
    if matches!(number.get(i).copied(), Some(b'+' | b'-')) {
        i = i.wrapping_add(1);
    }

    let mut digits = 0 as size_t;
    while matches!(number.get(i).copied(), Some(b'0'..=b'9')) {
        digits = digits.wrapping_add(1);
        i = i.wrapping_add(1);
    }

    if matches!(number.get(i).copied(), Some(b'.')) {
        i = i.wrapping_add(1);
        while matches!(number.get(i).copied(), Some(b'0'..=b'9')) {
            digits = digits.wrapping_add(1);
            i = i.wrapping_add(1);
        }
    }

    if digits == 0 {
        return None;
    }

    let mantissa_end = i;
    if matches!(number.get(i).copied(), Some(b'e' | b'E')) {
        let mut exponent_i = i.wrapping_add(1);
        if matches!(number.get(exponent_i).copied(), Some(b'+' | b'-')) {
            exponent_i = exponent_i.wrapping_add(1);
        }

        let exponent_start = exponent_i;
        while matches!(number.get(exponent_i).copied(), Some(b'0'..=b'9')) {
            exponent_i = exponent_i.wrapping_add(1);
        }

        if exponent_i > exponent_start {
            i = exponent_i;
        } else {
            i = mantissa_end;
        }
    }

    Some(i)
}
fn parse_number(item: &mut cJSON, input_buffer: &mut parse_buffer) -> cJSON_bool {
    if input_buffer.content.is_empty() {
        return false_0;
    }

    let remaining = match input_buffer
        .content
        .get(input_buffer.offset..input_buffer.length)
    {
        Some(remaining) => remaining,
        None => return false_0,
    };
    let number_string_length = remaining
        .iter()
        .take_while(|&&byte| matches!(byte, b'0'..=b'9' | b'+' | b'-' | b'e' | b'E' | b'.'))
        .count();
    let number_bytes = match remaining.get(..number_string_length) {
        Some(number_bytes) => number_bytes,
        None => return false_0,
    };
    let parsed_length = match strtod_number_prefix_len(number_bytes) {
        Some(parsed_length) => parsed_length,
        None => return false_0,
    };
    let parsed_bytes = match number_bytes.get(..parsed_length) {
        Some(parsed_bytes) => parsed_bytes,
        None => return false_0,
    };
    let number = match ::core::str::from_utf8(parsed_bytes)
        .ok()
        .and_then(|number| number.parse::<::core::ffi::c_double>().ok())
    {
        Some(number) => number,
        None => return false_0,
    };

    item.valuedouble = number;
    if number >= INT_MAX as ::core::ffi::c_double {
        item.valueint = INT_MAX;
    } else if number <= INT_MIN as ::core::ffi::c_double {
        item.valueint = INT_MIN;
    } else {
        item.valueint = number as ::core::ffi::c_int;
    }
    item.type_0 = cJSON_Number;
    input_buffer.offset = input_buffer.offset.wrapping_add(parsed_length);
    return true_0;
}
fn cJSON_NumberValueInt(mut number: ::core::ffi::c_double) -> ::core::ffi::c_int {
    if number >= INT_MAX as ::core::ffi::c_double {
        INT_MAX
    } else if number <= INT_MIN as ::core::ffi::c_double {
        INT_MIN
    } else {
        number as ::core::ffi::c_int
    }
}
pub fn cJSON_SetNumberHelper(
    object: &mut cJSON,
    mut number: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    object.valueint = cJSON_NumberValueInt(number);
    object.valuedouble = number;
    return object.valuedouble;
}
#[export_name = "cJSON_SetNumberHelper"]
pub unsafe extern "C" fn cJSON_SetNumberHelper_ffi(
    mut object: *mut cJSON,
    mut number: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    cJSON_SetNumberHelper(&mut *object, number)
}
#[export_name = "cJSON_SetValuestring"]
pub unsafe extern "C" fn cJSON_SetValuestring_ffi(
    mut object: *mut cJSON,
    mut valuestring: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if object.is_null()
        || (*object).type_0 & cJSON_String == 0
        || (*object).type_0 & cJSON_IsReference != 0
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if valuestring.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    let new_bytes = ::std::ffi::CStr::from_ptr(valuestring).to_bytes_with_nul();
    let mut stored_valuestring = (*object).valuestring.borrow_mut();
    let Some(object_valuestring) = stored_valuestring.as_mut() else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    if new_bytes.len() <= object_valuestring.len() {
        let new_start = valuestring as usize;
        let new_end = new_start.wrapping_add(new_bytes.len());
        let old_start = object_valuestring.as_ptr() as usize;
        let old_end = old_start.wrapping_add(c_string_bytes_with_nul(object_valuestring).len());
        if new_start < old_end && old_start < new_end {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        object_valuestring.clear();
        object_valuestring.extend_from_slice(new_bytes);
        return object_valuestring.as_mut_ptr() as *mut ::core::ffi::c_char;
    }
    let copy = match cJSON_strdup(new_bytes) {
        Some(copy) => copy,
        None => return ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    *object_valuestring = copy;
    return object_valuestring.as_mut_ptr() as *mut ::core::ffi::c_char;
}
fn ensure<'buffer>(
    p: &'buffer mut printbuffer<'_>,
    mut needed: size_t,
) -> Option<&'buffer mut [::core::ffi::c_uchar]> {
    let requested = needed;
    if p.length > 0 as size_t && p.offset >= p.length {
        return None;
    }
    if needed > INT_MAX as size_t {
        return None;
    }
    needed = needed.wrapping_add(p.offset.wrapping_add(1 as size_t));
    let required = needed;
    let mut newsize = 0 as size_t;
    if required > (INT_MAX / 2 as ::core::ffi::c_int) as size_t {
        if required <= INT_MAX as size_t {
            newsize = INT_MAX as size_t;
        } else {
            return None;
        }
    } else {
        newsize = required.wrapping_mul(2 as size_t);
    }

    match &mut p.storage {
        PrintBufferStorage::Dynamic(buffer) => {
            if required > p.length {
                let additional = newsize.saturating_sub(buffer.len());
                if buffer.try_reserve(additional).is_err() {
                    p.length = 0 as size_t;
                    buffer.clear();
                    return None;
                }
                buffer.resize(newsize, 0 as ::core::ffi::c_uchar);
                p.length = newsize;
            }
            buffer.get_mut(p.offset..p.offset.wrapping_add(requested))
        }
        PrintBufferStorage::Preallocated(buffer) => {
            if required > p.length {
                return None;
            }
            buffer.get_mut(p.offset..p.offset.wrapping_add(requested))
        }
    }
}
fn compare_double(a: ::core::ffi::c_double, b: ::core::ffi::c_double) -> cJSON_bool {
    let maxVal: ::core::ffi::c_double = if a.abs() > b.abs() { a.abs() } else { b.abs() };
    return ((a - b).abs() <= maxVal * DBL_EPSILON) as ::core::ffi::c_int;
}
fn trim_fraction_trailing_zeroes(mut number: Vec<u8>) -> Vec<u8> {
    if number.iter().any(|&byte| byte == b'.') {
        while number.last().copied() == Some(b'0') {
            number.pop();
        }
        if number.last().copied() == Some(b'.') {
            number.pop();
        }
    }
    number
}

fn push_decimal_digits(number: &mut Vec<u8>, mut value: i64) {
    let mut digits = [0u8; 20];
    let mut index = digits.len();
    loop {
        index = index.wrapping_sub(1);
        digits[index] = b'0'.wrapping_add((value % 10) as u8);
        value /= 10;
        if value == 0 {
            break;
        }
    }
    number.extend_from_slice(&digits[index..]);
}

fn push_c_exponent(number: &mut Vec<u8>, exponent: i32) {
    number.push(b'e');
    if exponent < 0 {
        number.push(b'-');
    } else {
        number.push(b'+');
    }

    let exponent_abs = if exponent < 0 {
        -(exponent as i64)
    } else {
        exponent as i64
    };
    if exponent_abs < 10 {
        number.push(b'0');
    }
    push_decimal_digits(number, exponent_abs);
}

fn normalize_c_exponent(mut number: Vec<u8>) -> Vec<u8> {
    let Some(exponent_index) = number.iter().position(|&byte| byte == b'e') else {
        return number;
    };
    let exponent = match ::core::str::from_utf8(&number[exponent_index + 1..])
        .ok()
        .and_then(|exponent| exponent.parse::<i32>().ok())
    {
        Some(exponent) => exponent,
        None => return number,
    };
    number.truncate(exponent_index);
    push_c_exponent(&mut number, exponent);
    number
}

fn format_c_g(value: ::core::ffi::c_double, precision: usize) -> Option<Vec<u8>> {
    let options = lexical_core::WriteFloatOptions::builder()
        .max_significant_digits(::core::num::NonZeroUsize::new(precision))
        .positive_exponent_break(::core::num::NonZeroI32::new(precision as i32))
        .negative_exponent_break(::core::num::NonZeroI32::new(-4))
        .round_mode(lexical_core::write_float_options::RoundMode::Round)
        .trim_floats(true)
        .build_strict();
    let mut buffer = [0u8; 128];
    let number = lexical_core::write_with_options::<_, { lexical_core::format::STANDARD }>(
        value,
        &mut buffer,
        &options,
    );
    Some(normalize_c_exponent(number.to_vec()))
}

fn format_cjson_number(item: &cJSON) -> Option<Vec<u8>> {
    let d = item.valuedouble;
    if d.is_nan() || d.is_infinite() {
        return Some(b"null".to_vec());
    }
    if d == item.valueint as ::core::ffi::c_double {
        return Some(item.valueint.to_string().into_bytes());
    }

    let mut number = format_c_g(d, 15)?;
    let needs_more_precision = ::core::str::from_utf8(&number)
        .ok()
        .and_then(|number| number.parse::<::core::ffi::c_double>().ok())
        .map_or(true, |test| compare_double(test, d) == 0);
    if needs_more_precision {
        number = format_c_g(d, 17)?;
    }
    Some(number)
}

fn print_number(item: &cJSON, output_buffer: &mut printbuffer) -> cJSON_bool {
    let Some(number) = format_cjson_number(item) else {
        return false_0;
    };
    if number.len()
        > (::core::mem::size_of::<[::core::ffi::c_uchar; 26]>() as usize).wrapping_sub(1)
    {
        return false_0;
    }
    let Some(output) = ensure(
        output_buffer,
        number
            .len()
            .wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 1]>() as size_t),
    ) else {
        return false_0;
    };
    output[..number.len()].copy_from_slice(&number);
    output[number.len()] = '\0' as i32 as ::core::ffi::c_uchar;
    output_buffer.offset = output_buffer.offset.wrapping_add(number.len());
    return true_0;
}
fn parse_hex4(input: &[::core::ffi::c_uchar]) -> ::core::ffi::c_uint {
    let mut h: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut i: size_t = 0 as size_t;
    i = 0 as size_t;
    while i < 4 as size_t {
        let Some(input_byte) = input.get(i).copied() else {
            return 0 as ::core::ffi::c_uint;
        };
        if input_byte as ::core::ffi::c_int >= '0' as i32
            && input_byte as ::core::ffi::c_int <= '9' as i32
        {
            h = h.wrapping_add(
                (input_byte as ::core::ffi::c_uint).wrapping_sub('0' as i32 as ::core::ffi::c_uint),
            );
        } else if input_byte as ::core::ffi::c_int >= 'A' as i32
            && input_byte as ::core::ffi::c_int <= 'F' as i32
        {
            h = h.wrapping_add(
                (10 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_add(input_byte as ::core::ffi::c_uint)
                    .wrapping_sub('A' as i32 as ::core::ffi::c_uint),
            );
        } else if input_byte as ::core::ffi::c_int >= 'a' as i32
            && input_byte as ::core::ffi::c_int <= 'f' as i32
        {
            h = h.wrapping_add(
                (10 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_add(input_byte as ::core::ffi::c_uint)
                    .wrapping_sub('a' as i32 as ::core::ffi::c_uint),
            );
        } else {
            return 0 as ::core::ffi::c_uint;
        }
        if i < 3 as size_t {
            h = h << 4 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    return h;
}
fn utf16_literal_to_utf8(
    input: &[::core::ffi::c_uchar],
    output: &mut Vec<::core::ffi::c_uchar>,
) -> ::core::ffi::c_uchar {
    let mut c2rust_current_block: u64;
    let mut codepoint: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    let mut first_code: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut utf8_length: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
    let mut sequence_length: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
    if !(input.len() < 6 as usize) {
        first_code = parse_hex4(&input[2 as usize..]);
        if !(first_code >= 0xdc00 as ::core::ffi::c_uint
            && first_code <= 0xdfff as ::core::ffi::c_uint)
        {
            if first_code >= 0xd800 as ::core::ffi::c_uint
                && first_code <= 0xdbff as ::core::ffi::c_uint
            {
                let mut second_code: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                sequence_length = 12 as ::core::ffi::c_uchar;
                if input.len() < 12 as usize {
                    c2rust_current_block = 2136517548508416331;
                } else if input.get(6 as usize).copied()
                    != Some('\\' as i32 as ::core::ffi::c_uchar)
                    || input.get(7 as usize).copied() != Some('u' as i32 as ::core::ffi::c_uchar)
                {
                    c2rust_current_block = 2136517548508416331;
                } else {
                    second_code = parse_hex4(&input[8 as usize..]);
                    if second_code < 0xdc00 as ::core::ffi::c_uint
                        || second_code > 0xdfff as ::core::ffi::c_uint
                    {
                        c2rust_current_block = 2136517548508416331;
                    } else {
                        codepoint = (0x10000 as ::core::ffi::c_int as ::core::ffi::c_uint)
                            .wrapping_add(
                                (first_code & 0x3ff as ::core::ffi::c_uint)
                                    << 10 as ::core::ffi::c_int
                                    | second_code & 0x3ff as ::core::ffi::c_uint,
                            ) as ::core::ffi::c_ulong;
                        c2rust_current_block = 12039483399334584727;
                    }
                }
            } else {
                sequence_length = 6 as ::core::ffi::c_uchar;
                codepoint = first_code as ::core::ffi::c_ulong;
                c2rust_current_block = 12039483399334584727;
            }
            match c2rust_current_block {
                2136517548508416331 => {}
                _ => {
                    if codepoint < 0x80 as ::core::ffi::c_ulong {
                        utf8_length = 1 as ::core::ffi::c_uchar;
                        c2rust_current_block = 3437258052017859086;
                    } else if codepoint < 0x800 as ::core::ffi::c_ulong {
                        utf8_length = 2 as ::core::ffi::c_uchar;
                        c2rust_current_block = 3437258052017859086;
                    } else if codepoint < 0x10000 as ::core::ffi::c_ulong {
                        utf8_length = 3 as ::core::ffi::c_uchar;
                        c2rust_current_block = 3437258052017859086;
                    } else if codepoint <= 0x10ffff as ::core::ffi::c_ulong {
                        utf8_length = 4 as ::core::ffi::c_uchar;
                        c2rust_current_block = 3437258052017859086;
                    } else {
                        c2rust_current_block = 2136517548508416331;
                    }
                    match c2rust_current_block {
                        2136517548508416331 => {}
                        _ => {
                            let Some(character) = ::core::char::from_u32(codepoint as u32) else {
                                return 0 as ::core::ffi::c_uchar;
                            };
                            let mut utf8_buffer = [0 as ::core::ffi::c_uchar; 4];
                            let encoded = character.encode_utf8(&mut utf8_buffer);
                            output.extend_from_slice(encoded.as_bytes());
                            debug_assert_eq!(encoded.len(), utf8_length as usize);
                            return sequence_length;
                        }
                    }
                }
            }
        }
    }
    return 0 as ::core::ffi::c_uchar;
}
fn parse_string(item_ref: &mut cJSON, input_buffer_ref: &mut parse_buffer) -> cJSON_bool {
    let string_start = input_buffer_ref.offset;
    let initial_input_pointer = string_start.wrapping_add(1 as size_t);

    if input_buffer_ref.content.get(string_start).copied()
        != Some('"' as i32 as ::core::ffi::c_uchar)
    {
        input_buffer_ref.offset = initial_input_pointer;
        return false_0;
    }

    let mut input_end = initial_input_pointer;
    let mut skipped_bytes: size_t = 0 as size_t;
    loop {
        if input_end >= input_buffer_ref.length {
            input_buffer_ref.offset = initial_input_pointer;
            return false_0;
        }

        match input_buffer_ref.content.get(input_end).copied() {
            Some(byte) if byte == '"' as i32 as ::core::ffi::c_uchar => break,
            Some(byte) if byte == '\\' as i32 as ::core::ffi::c_uchar => {
                if input_end.wrapping_add(1 as size_t) >= input_buffer_ref.length {
                    input_buffer_ref.offset = initial_input_pointer;
                    return false_0;
                }
                skipped_bytes = skipped_bytes.wrapping_add(1 as size_t);
                input_end = input_end.wrapping_add(2 as size_t);
            }
            Some(_) => {
                input_end = input_end.wrapping_add(1 as size_t);
            }
            None => {
                input_buffer_ref.offset = initial_input_pointer;
                return false_0;
            }
        }
    }

    let allocation_length = input_end
        .wrapping_sub(string_start)
        .wrapping_sub(skipped_bytes);
    let mut decoded = Vec::with_capacity(allocation_length);
    let mut input_pointer = initial_input_pointer;
    while input_pointer < input_end {
        let Some(byte) = input_buffer_ref.content.get(input_pointer).copied() else {
            input_buffer_ref.offset = input_pointer;
            return false_0;
        };

        if byte != '\\' as i32 as ::core::ffi::c_uchar {
            decoded.push(byte);
            input_pointer = input_pointer.wrapping_add(1 as size_t);
            continue;
        }

        let mut sequence_length: ::core::ffi::c_uchar = 2 as ::core::ffi::c_uchar;
        let Some(escaped_byte) = input_buffer_ref
            .content
            .get(input_pointer.wrapping_add(1 as size_t))
            .copied()
        else {
            input_buffer_ref.offset = input_pointer;
            return false_0;
        };

        match escaped_byte as ::core::ffi::c_int {
            98 => decoded.push('\u{8}' as i32 as ::core::ffi::c_uchar),
            102 => decoded.push('\u{c}' as i32 as ::core::ffi::c_uchar),
            110 => decoded.push('\n' as i32 as ::core::ffi::c_uchar),
            114 => decoded.push('\r' as i32 as ::core::ffi::c_uchar),
            116 => decoded.push('\t' as i32 as ::core::ffi::c_uchar),
            34 | 92 | 47 => decoded.push(escaped_byte),
            117 => {
                sequence_length = utf16_literal_to_utf8(
                    &input_buffer_ref.content[input_pointer..input_end],
                    &mut decoded,
                );
                if sequence_length as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    input_buffer_ref.offset = input_pointer;
                    return false_0;
                }
            }
            _ => {
                input_buffer_ref.offset = input_pointer;
                return false_0;
            }
        }
        input_pointer = input_pointer.wrapping_add(sequence_length as size_t);
    }

    if decoded.len() > allocation_length {
        input_buffer_ref.offset = input_pointer;
        return false_0;
    }

    let allocation_size = allocation_length
        .wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 1]>() as size_t);
    let mut output_bytes = Vec::with_capacity(allocation_size);
    output_bytes.extend_from_slice(&decoded);
    output_bytes.push('\0' as i32 as ::core::ffi::c_uchar);
    output_bytes.resize(allocation_size, 0 as ::core::ffi::c_uchar);

    let Some(output) = cJSON_strdup(&output_bytes) else {
        input_buffer_ref.offset = initial_input_pointer;
        return false_0;
    };
    item_ref.type_0 = cJSON_String;
    *item_ref.valuestring.borrow_mut() = Some(output);
    input_buffer_ref.offset = input_end.wrapping_add(1 as size_t);
    return true_0;
}
fn print_string_ptr(
    input: Option<&[::core::ffi::c_uchar]>,
    output_buffer: &mut printbuffer,
) -> cJSON_bool {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    let input = input.map(c_string_bytes).unwrap_or(&[]);
    let mut escaped = Vec::with_capacity(
        input
            .len()
            .wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t),
    );
    escaped.push('"' as i32 as ::core::ffi::c_uchar);
    for &byte in input {
        match byte {
            b'\\' => escaped.extend_from_slice(b"\\\\"),
            b'"' => escaped.extend_from_slice(b"\\\""),
            b'\x08' => escaped.extend_from_slice(b"\\b"),
            b'\x0c' => escaped.extend_from_slice(b"\\f"),
            b'\n' => escaped.extend_from_slice(b"\\n"),
            b'\r' => escaped.extend_from_slice(b"\\r"),
            b'\t' => escaped.extend_from_slice(b"\\t"),
            0..=31 => {
                escaped.extend_from_slice(b"\\u00");
                escaped.push(HEX[(byte >> 4) as usize]);
                escaped.push(HEX[(byte & 0xf) as usize]);
            }
            _ => escaped.push(byte),
        }
    }
    escaped.push('"' as i32 as ::core::ffi::c_uchar);
    escaped.push('\0' as i32 as ::core::ffi::c_uchar);

    let Some(output) = ensure(output_buffer, escaped.len()) else {
        return false_0;
    };
    output.copy_from_slice(&escaped);
    output_buffer.offset = output_buffer
        .offset
        .wrapping_add(escaped.len().wrapping_sub(1));
    return true_0;
}
fn buffer_skip_whitespace(buffer: &mut parse_buffer) -> &mut parse_buffer {
    if buffer.offset >= buffer.length {
        return buffer;
    }
    while buffer.offset < buffer.length
        && buffer
            .content
            .get(buffer.offset)
            .is_some_and(|byte| *byte as ::core::ffi::c_int <= 32 as ::core::ffi::c_int)
    {
        buffer.offset = buffer.offset.wrapping_add(1);
    }
    if buffer.offset == buffer.length {
        buffer.offset = buffer.offset.wrapping_sub(1);
    }
    buffer
}
fn parse_buffer_byte(buffer: &parse_buffer, offset: size_t) -> Option<::core::ffi::c_uchar> {
    if offset < buffer.length {
        buffer.content.get(offset).copied()
    } else {
        None
    }
}
fn skip_utf8_bom(buffer: &mut parse_buffer) -> &mut parse_buffer {
    if buffer.offset != 0 as size_t {
        return buffer;
    }
    if buffer.offset.wrapping_add(4 as size_t) < buffer.length
        && buffer
            .content
            .get(buffer.offset..buffer.offset.wrapping_add(3 as size_t))
            == Some(&b"\xEF\xBB\xBF"[..])
    {
        buffer.offset = buffer.offset.wrapping_add(3 as size_t);
    }
    buffer
}
#[export_name = "cJSON_ParseWithOpts"]
pub unsafe extern "C" fn cJSON_ParseWithOpts_ffi(
    mut value: *const ::core::ffi::c_char,
    mut return_parse_end: *mut *const ::core::ffi::c_char,
    mut require_null_terminated: cJSON_bool,
) -> *mut cJSON {
    if value.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let buffer_length =
        strlen(value).wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 1]>() as size_t);
    let input = ::core::slice::from_raw_parts(value as *const ::core::ffi::c_uchar, buffer_length);
    finish_parse_with_length_opts!(
        cJSON_ParseWithLengthOpts(input, require_null_terminated),
        value,
        return_parse_end.as_mut()
    )
}
fn cJSON_ParseWithLengthOpts(
    value: &[::core::ffi::c_uchar],
    require_null_terminated: cJSON_bool,
) -> (Option<::core::ptr::NonNull<cJSON>>, size_t, cJSON_bool) {
    let mut c2rust_current_block: u64;
    let mut buffer: parse_buffer = parse_buffer {
        content: Vec::new(),
        length: 0 as size_t,
        offset: 0 as size_t,
        depth: 0 as size_t,
        hooks: internal_hooks {
            allocate: None,
            deallocate: None,
            reallocate: None,
        },
    };
    let mut item: Option<::core::ptr::NonNull<cJSON>> = None;
    GLOBAL_ERROR_JSON_ADDRESS.store(0, ::std::sync::atomic::Ordering::Relaxed);
    GLOBAL_ERROR_POSITION.store(0 as size_t, ::std::sync::atomic::Ordering::Relaxed);
    if !value.is_empty() {
        buffer.content = value.to_vec();
        buffer.length = value.len();
        buffer.offset = 0 as size_t;
        buffer.hooks = global_hooks_snapshot();
        let new_item = cJSON_New_Item(
            &buffer.hooks,
            cJSON_Invalid,
            0 as ::core::ffi::c_int,
            0.0f64,
            ValueStringInit::None,
        );
        if let Some(item_ref) = new_item {
            item = Some(::core::ptr::NonNull::from(&mut *item_ref));
            if !(parse_value(item_ref, buffer_skip_whitespace(skip_utf8_bom(&mut buffer))) == 0) {
                if require_null_terminated != 0 {
                    buffer_skip_whitespace(&mut buffer);
                    if buffer.offset >= buffer.length
                        || parse_buffer_byte(&buffer, buffer.offset)
                            != Some('\0' as i32 as ::core::ffi::c_uchar)
                    {
                        c2rust_current_block = 11194366360035625197;
                    } else {
                        c2rust_current_block = 1841672684692190573;
                    }
                } else {
                    c2rust_current_block = 1841672684692190573;
                }
                match c2rust_current_block {
                    11194366360035625197 => {}
                    _ => {
                        return (item, buffer.offset, true_0);
                    }
                }
            }
            cJSON_Delete(Some(item_ref));
        }
    }
    let mut local_error_position = 0 as size_t;
    if buffer.offset < buffer.length {
        local_error_position = buffer.offset;
    } else if buffer.length > 0 as size_t {
        local_error_position = buffer.length.wrapping_sub(1 as size_t);
    }
    (None, local_error_position, false_0)
}
#[export_name = "cJSON_ParseWithLengthOpts"]
pub unsafe extern "C" fn cJSON_ParseWithLengthOpts_ffi(
    mut value: *const ::core::ffi::c_char,
    mut buffer_length: size_t,
    mut return_parse_end: *mut *const ::core::ffi::c_char,
    mut require_null_terminated: cJSON_bool,
) -> *mut cJSON {
    GLOBAL_ERROR_JSON_ADDRESS.store(0, ::std::sync::atomic::Ordering::Relaxed);
    GLOBAL_ERROR_POSITION.store(0 as size_t, ::std::sync::atomic::Ordering::Relaxed);
    if value.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let input = ::core::slice::from_raw_parts(value as *const ::core::ffi::c_uchar, buffer_length);
    finish_parse_with_length_opts!(
        cJSON_ParseWithLengthOpts(input, require_null_terminated),
        value,
        return_parse_end.as_mut()
    )
}
#[export_name = "cJSON_Parse"]
pub unsafe extern "C" fn cJSON_Parse_ffi(mut value: *const ::core::ffi::c_char) -> *mut cJSON {
    if value.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let buffer_length =
        strlen(value).wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 1]>() as size_t);
    let input = ::core::slice::from_raw_parts(value as *const ::core::ffi::c_uchar, buffer_length);
    finish_parse_with_length_opts!(
        cJSON_ParseWithLengthOpts(input, 0 as cJSON_bool),
        value,
        None::<&mut *const ::core::ffi::c_char>
    )
}
#[export_name = "cJSON_ParseWithLength"]
pub unsafe extern "C" fn cJSON_ParseWithLength_ffi(
    mut value: *const ::core::ffi::c_char,
    mut buffer_length: size_t,
) -> *mut cJSON {
    GLOBAL_ERROR_JSON_ADDRESS.store(0, ::std::sync::atomic::Ordering::Relaxed);
    GLOBAL_ERROR_POSITION.store(0 as size_t, ::std::sync::atomic::Ordering::Relaxed);
    if value.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let input = ::core::slice::from_raw_parts(value as *const ::core::ffi::c_uchar, buffer_length);
    finish_parse_with_length_opts!(
        cJSON_ParseWithLengthOpts(input, 0 as cJSON_bool),
        value,
        None::<&mut *const ::core::ffi::c_char>
    )
}
fn print(
    item: Option<&cJSON>,
    format: cJSON_bool,
    default_buffer_size: size_t,
) -> Option<Vec<::core::ffi::c_uchar>> {
    let mut storage = Vec::new();
    if storage.try_reserve(default_buffer_size).is_err() {
        return None;
    }
    storage.resize(default_buffer_size, 0 as ::core::ffi::c_uchar);
    let mut buffer = printbuffer {
        storage: PrintBufferStorage::Dynamic(storage),
        length: default_buffer_size,
        offset: 0,
        depth: 0,
        format,
    };

    if print_value(item, &mut buffer) == 0 {
        return None;
    }

    let output_length = buffer.offset.wrapping_add(1 as size_t);

    let PrintBufferStorage::Dynamic(storage) = buffer.storage else {
        unreachable!("print uses dynamic storage");
    };
    let Some(source) = storage.get(..output_length) else {
        return None;
    };
    let mut output = source.to_vec();
    if let Some(byte) = output.get_mut(buffer.offset) {
        *byte = '\0' as i32 as ::core::ffi::c_uchar;
    }
    Some(output)
}
#[export_name = "cJSON_Print"]
pub unsafe extern "C" fn cJSON_Print_ffi(mut item: *const cJSON) -> *mut ::core::ffi::c_char {
    let hooks = global_hooks_snapshot();
    let item = item.as_ref();
    let initial_buffer = hooks.allocate.expect("non-null function pointer")(256 as size_t);
    if initial_buffer.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    hooks.deallocate.expect("non-null function pointer")(initial_buffer);
    let Some(output) = print(item, true_0, 256 as size_t) else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    hook_strdup_ffi!(&output, hooks)
}
#[export_name = "cJSON_PrintUnformatted"]
pub unsafe extern "C" fn cJSON_PrintUnformatted_ffi(
    mut item: *const cJSON,
) -> *mut ::core::ffi::c_char {
    let hooks = global_hooks_snapshot();
    let item = item.as_ref();
    let initial_buffer = hooks.allocate.expect("non-null function pointer")(256 as size_t);
    if initial_buffer.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    hooks.deallocate.expect("non-null function pointer")(initial_buffer);
    let Some(output) = print(item, false_0, 256 as size_t) else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    hook_strdup_ffi!(&output, hooks)
}
#[export_name = "cJSON_PrintBuffered"]
pub unsafe extern "C" fn cJSON_PrintBuffered_ffi(
    mut item: *const cJSON,
    mut prebuffer: ::core::ffi::c_int,
    mut fmt: cJSON_bool,
) -> *mut ::core::ffi::c_char {
    if prebuffer < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let hooks = global_hooks_snapshot();
    let initial_buffer = hooks.allocate.expect("non-null function pointer")(prebuffer as size_t);
    if initial_buffer.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    hooks.deallocate.expect("non-null function pointer")(initial_buffer);
    let Some(output) = print(item.as_ref(), fmt, prebuffer as size_t) else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    hook_strdup_ffi!(&output, hooks)
}
#[export_name = "cJSON_PrintPreallocated"]
pub unsafe extern "C" fn cJSON_PrintPreallocated_ffi(
    mut item: *mut cJSON,
    mut buffer: *mut ::core::ffi::c_char,
    length: ::core::ffi::c_int,
    format: cJSON_bool,
) -> cJSON_bool {
    if length < 0 as ::core::ffi::c_int || buffer.is_null() {
        return false_0;
    }
    let output =
        ::core::slice::from_raw_parts_mut(buffer as *mut ::core::ffi::c_uchar, length as usize);
    let mut p = printbuffer {
        storage: PrintBufferStorage::Preallocated(output),
        length: length as size_t,
        offset: 0 as size_t,
        depth: 0 as size_t,
        format,
    };
    return print_value(item.as_ref(), &mut p);
}
fn parse_value(item_ref: &mut cJSON, input_buffer_ref: &mut parse_buffer) -> cJSON_bool {
    if input_buffer_ref.content.is_empty() {
        return false_0;
    }

    let remaining = match input_buffer_ref
        .content
        .get(input_buffer_ref.offset..input_buffer_ref.length)
    {
        Some(remaining) => remaining,
        None => return false_0,
    };

    if remaining.starts_with(b"null") {
        item_ref.type_0 = cJSON_NULL;
        input_buffer_ref.offset = input_buffer_ref.offset.wrapping_add(4 as size_t);
        return true_0;
    }
    if remaining.starts_with(b"false") {
        item_ref.type_0 = cJSON_False;
        input_buffer_ref.offset = input_buffer_ref.offset.wrapping_add(5 as size_t);
        return true_0;
    }
    if remaining.starts_with(b"true") {
        item_ref.type_0 = cJSON_True;
        item_ref.valueint = 1 as ::core::ffi::c_int;
        input_buffer_ref.offset = input_buffer_ref.offset.wrapping_add(4 as size_t);
        return true_0;
    }

    match remaining.first().copied() {
        Some(b'"') => return parse_string(item_ref, input_buffer_ref),
        Some(b'-' | b'0'..=b'9') => return parse_number(item_ref, input_buffer_ref),
        Some(b'[') => return parse_array(item_ref, input_buffer_ref),
        Some(b'{') => return parse_object(item_ref, input_buffer_ref),
        _ => {}
    }

    return false_0;
}
fn print_value(item: Option<&cJSON>, output_buffer: &mut printbuffer) -> cJSON_bool {
    let Some(item_ref) = item else {
        return false_0;
    };
    match item_ref.type_0 & 0xff as ::core::ffi::c_int {
        cJSON_NULL => {
            let Some(output) = ensure(output_buffer, 5 as size_t) else {
                return false_0;
            };
            output.copy_from_slice(b"null\0");
            output_buffer.offset = output_buffer.offset.wrapping_add(4 as size_t);
            return true_0;
        }
        cJSON_False => {
            let Some(output) = ensure(output_buffer, 6 as size_t) else {
                return false_0;
            };
            output.copy_from_slice(b"false\0");
            output_buffer.offset = output_buffer.offset.wrapping_add(5 as size_t);
            return true_0;
        }
        cJSON_True => {
            let Some(output) = ensure(output_buffer, 5 as size_t) else {
                return false_0;
            };
            output.copy_from_slice(b"true\0");
            output_buffer.offset = output_buffer.offset.wrapping_add(4 as size_t);
            return true_0;
        }
        cJSON_Number => return print_number(item_ref, output_buffer),
        cJSON_Raw | cJSON_String => {
            let print_raw = (item_ref.type_0 & 0xff as ::core::ffi::c_int) == cJSON_Raw;
            let stored_string = item_ref.valuestring.borrow();
            let Some(stored_string) = stored_string.as_deref() else {
                return if print_raw {
                    false_0
                } else {
                    print_string_ptr(None, output_buffer)
                };
            };
            if !print_raw {
                return print_string_ptr(Some(stored_string), output_buffer);
            }
            let raw = c_string_bytes_with_nul(stored_string);
            let Some(output) = ensure(output_buffer, raw.len()) else {
                return false_0;
            };
            output.copy_from_slice(raw);
            output_buffer.offset = output_buffer.offset.wrapping_add(raw.len().wrapping_sub(1));
            return true_0;
        }
        cJSON_Array => return print_array(item_ref, output_buffer),
        cJSON_Object => return print_object(item_ref, output_buffer),
        _ => return false_0,
    };
}
fn parse_array(item: &mut cJSON, input_buffer: &mut parse_buffer) -> cJSON_bool {
    let mut c2rust_current_block: u64;
    let mut child_list = cJSON {
        next: ::core::cell::Cell::new(None),
        prev: ::core::cell::Cell::new(None),
        child: ::core::cell::Cell::new(None),
        type_0: cJSON_Array,
        valuestring: ::core::cell::RefCell::new(None),
        valueint: 0,
        valuedouble: 0.0,
        string: ::core::cell::RefCell::new(None),
    };
    let input_buffer_ref = input_buffer;
    if input_buffer_ref.depth >= CJSON_NESTING_LIMIT as size_t {
        return false_0;
    }
    input_buffer_ref.depth = input_buffer_ref.depth.wrapping_add(1);
    if parse_buffer_byte(input_buffer_ref, input_buffer_ref.offset)
        == Some('[' as i32 as ::core::ffi::c_uchar)
    {
        input_buffer_ref.offset = input_buffer_ref.offset.wrapping_add(1);
        buffer_skip_whitespace(input_buffer_ref);
        if parse_buffer_byte(input_buffer_ref, input_buffer_ref.offset)
            == Some(']' as i32 as ::core::ffi::c_uchar)
        {
            c2rust_current_block = 6773356538935231690;
        } else if parse_buffer_byte(input_buffer_ref, input_buffer_ref.offset).is_none() {
            input_buffer_ref.offset = input_buffer_ref.offset.wrapping_sub(1);
            c2rust_current_block = 1336238348363633231;
        } else {
            input_buffer_ref.offset = input_buffer_ref.offset.wrapping_sub(1);
            loop {
                let Some(current_item_ref) = cJSON_New_Item(
                    &input_buffer_ref.hooks,
                    cJSON_Invalid,
                    0 as ::core::ffi::c_int,
                    0.0f64,
                    ValueStringInit::None,
                ) else {
                    c2rust_current_block = 1336238348363633231;
                    break;
                };
                input_buffer_ref.offset = input_buffer_ref.offset.wrapping_add(1);
                buffer_skip_whitespace(input_buffer_ref);
                if parse_value(current_item_ref, input_buffer_ref) == 0 {
                    cJSON_Delete(Some(current_item_ref));
                    c2rust_current_block = 1336238348363633231;
                    break;
                }
                add_item_to_array(&child_list, current_item_ref);
                buffer_skip_whitespace(input_buffer_ref);
                if parse_buffer_byte(input_buffer_ref, input_buffer_ref.offset)
                    != Some(',' as i32 as ::core::ffi::c_uchar)
                {
                    c2rust_current_block = 15089075282327824602;
                    break;
                }
            }
            match c2rust_current_block {
                1336238348363633231 => {}
                _ => {
                    if parse_buffer_byte(input_buffer_ref, input_buffer_ref.offset)
                        != Some(']' as i32 as ::core::ffi::c_uchar)
                    {
                        c2rust_current_block = 1336238348363633231;
                    } else {
                        c2rust_current_block = 6773356538935231690;
                    }
                }
            }
        }
        match c2rust_current_block {
            1336238348363633231 => {}
            _ => {
                input_buffer_ref.depth = input_buffer_ref.depth.wrapping_sub(1);
                item.type_0 = cJSON_Array;
                item.child.set(child_list.child.get());
                input_buffer_ref.offset = input_buffer_ref.offset.wrapping_add(1);
                return true_0;
            }
        }
    }
    if let Some(child) = child_list.child.get() {
        cJSON_Delete(Some(child));
    }
    return false_0;
}
fn print_array(item: &cJSON, output_buffer: &mut printbuffer) -> cJSON_bool {
    let mut length: size_t = 0 as size_t;
    let Some(output) = ensure(output_buffer, 1 as size_t) else {
        return false_0;
    };
    output[0] = '[' as i32 as ::core::ffi::c_uchar;
    output_buffer.offset = output_buffer.offset.wrapping_add(1);
    output_buffer.depth = output_buffer.depth.wrapping_add(1);
    let mut current_index = 0 as size_t;
    while let Some(current_element_ref) = get_array_item(Some(item), current_index) {
        if print_value(Some(current_element_ref), output_buffer) == 0 {
            return false_0;
        }
        let has_next =
            get_array_item(Some(item), current_index.wrapping_add(1 as size_t)).is_some();
        if has_next {
            let formatted = output_buffer.format != 0;
            length = (if formatted {
                2 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as size_t;
            let Some(output) = ensure(output_buffer, length.wrapping_add(1 as size_t)) else {
                return false_0;
            };
            output[0] = ',' as i32 as ::core::ffi::c_uchar;
            let mut output_index = 1 as usize;
            if formatted {
                output[output_index] = ' ' as i32 as ::core::ffi::c_uchar;
                output_index = output_index.wrapping_add(1);
            }
            output[output_index] = '\0' as i32 as ::core::ffi::c_uchar;
            output_buffer.offset = output_buffer.offset.wrapping_add(length);
        }
        current_index = current_index.wrapping_add(1 as size_t);
    }
    let Some(output) = ensure(output_buffer, 2 as size_t) else {
        return false_0;
    };
    output[0] = ']' as i32 as ::core::ffi::c_uchar;
    output[1] = '\0' as i32 as ::core::ffi::c_uchar;
    output_buffer.offset = output_buffer.offset.wrapping_add(1 as size_t);
    output_buffer.depth = output_buffer.depth.wrapping_sub(1);
    return true_0;
}
fn parse_object(item: &mut cJSON, input_buffer: &mut parse_buffer) -> cJSON_bool {
    let mut c2rust_current_block: u64;
    let mut child_list = cJSON {
        next: ::core::cell::Cell::new(None),
        prev: ::core::cell::Cell::new(None),
        child: ::core::cell::Cell::new(None),
        type_0: cJSON_Object,
        valuestring: ::core::cell::RefCell::new(None),
        valueint: 0,
        valuedouble: 0.0,
        string: ::core::cell::RefCell::new(None),
    };
    let input_buffer_ref = input_buffer;
    if input_buffer_ref.depth >= CJSON_NESTING_LIMIT as size_t {
        return false_0;
    }
    input_buffer_ref.depth = input_buffer_ref.depth.wrapping_add(1);
    if parse_buffer_byte(input_buffer_ref, input_buffer_ref.offset)
        == Some('{' as i32 as ::core::ffi::c_uchar)
    {
        input_buffer_ref.offset = input_buffer_ref.offset.wrapping_add(1);
        buffer_skip_whitespace(input_buffer_ref);
        if parse_buffer_byte(input_buffer_ref, input_buffer_ref.offset)
            == Some('}' as i32 as ::core::ffi::c_uchar)
        {
            c2rust_current_block = 4359236900545362719;
        } else if parse_buffer_byte(input_buffer_ref, input_buffer_ref.offset).is_none() {
            input_buffer_ref.offset = input_buffer_ref.offset.wrapping_sub(1);
            c2rust_current_block = 9990476168629568694;
        } else {
            input_buffer_ref.offset = input_buffer_ref.offset.wrapping_sub(1);
            loop {
                let Some(current_item_ref) = cJSON_New_Item(
                    &input_buffer_ref.hooks,
                    cJSON_Invalid,
                    0 as ::core::ffi::c_int,
                    0.0f64,
                    ValueStringInit::None,
                ) else {
                    c2rust_current_block = 9990476168629568694;
                    break;
                };
                {
                    if input_buffer_ref.offset.wrapping_add(1 as size_t) >= input_buffer_ref.length
                    {
                        cJSON_Delete(Some(current_item_ref));
                        c2rust_current_block = 9990476168629568694;
                        break;
                    } else {
                        input_buffer_ref.offset = input_buffer_ref.offset.wrapping_add(1);
                        buffer_skip_whitespace(input_buffer_ref);
                        if parse_string(current_item_ref, input_buffer_ref) == 0 {
                            cJSON_Delete(Some(current_item_ref));
                            c2rust_current_block = 9990476168629568694;
                            break;
                        } else {
                            buffer_skip_whitespace(input_buffer_ref);
                            let parsed_key = current_item_ref.valuestring.borrow_mut().take();
                            *current_item_ref.string.borrow_mut() = parsed_key;
                            if parse_buffer_byte(input_buffer_ref, input_buffer_ref.offset)
                                != Some(':' as i32 as ::core::ffi::c_uchar)
                            {
                                cJSON_Delete(Some(current_item_ref));
                                c2rust_current_block = 9990476168629568694;
                                break;
                            } else {
                                input_buffer_ref.offset = input_buffer_ref.offset.wrapping_add(1);
                                buffer_skip_whitespace(input_buffer_ref);
                                if parse_value(current_item_ref, input_buffer_ref) == 0 {
                                    cJSON_Delete(Some(current_item_ref));
                                    c2rust_current_block = 9990476168629568694;
                                    break;
                                } else {
                                    add_item_to_array(&child_list, current_item_ref);
                                    buffer_skip_whitespace(input_buffer_ref);
                                    if parse_buffer_byte(input_buffer_ref, input_buffer_ref.offset)
                                        != Some(',' as i32 as ::core::ffi::c_uchar)
                                    {
                                        c2rust_current_block = 14359455889292382949;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            match c2rust_current_block {
                9990476168629568694 => {}
                _ => {
                    if parse_buffer_byte(input_buffer_ref, input_buffer_ref.offset)
                        != Some('}' as i32 as ::core::ffi::c_uchar)
                    {
                        c2rust_current_block = 9990476168629568694;
                    } else {
                        c2rust_current_block = 4359236900545362719;
                    }
                }
            }
        }
        match c2rust_current_block {
            9990476168629568694 => {}
            _ => {
                input_buffer_ref.depth = input_buffer_ref.depth.wrapping_sub(1);
                item.type_0 = cJSON_Object;
                item.child.set(child_list.child.get());
                input_buffer_ref.offset = input_buffer_ref.offset.wrapping_add(1);
                return true_0;
            }
        }
    }
    if let Some(child) = child_list.child.get() {
        cJSON_Delete(Some(child));
    }
    return false_0;
}
fn print_object(item: &cJSON, output_buffer: &mut printbuffer) -> cJSON_bool {
    let mut length: size_t = 0 as size_t;
    let formatted = output_buffer.format != 0;
    length = (if formatted {
        2 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    }) as size_t;
    let Some(output) = ensure(output_buffer, length.wrapping_add(1 as size_t)) else {
        return false_0;
    };
    output[0] = '{' as i32 as ::core::ffi::c_uchar;
    if formatted {
        output[1] = '\n' as i32 as ::core::ffi::c_uchar;
    }
    output_buffer.depth = output_buffer.depth.wrapping_add(1);
    output_buffer.offset = output_buffer.offset.wrapping_add(length);
    let mut current_index = 0 as size_t;
    while let Some(current_item_ref) = get_array_item(Some(item), current_index) {
        if output_buffer.format != 0 {
            let mut i: size_t = 0;
            let depth = output_buffer.depth;
            let Some(output) = ensure(output_buffer, depth) else {
                return false_0;
            };
            i = 0 as size_t;
            while i < depth {
                output[i as usize] = '\t' as i32 as ::core::ffi::c_uchar;
                i = i.wrapping_add(1);
            }
            output_buffer.offset = output_buffer.offset.wrapping_add(depth);
        }
        let key_item = cJSON {
            next: ::core::cell::Cell::new(None),
            prev: ::core::cell::Cell::new(None),
            child: ::core::cell::Cell::new(None),
            type_0: cJSON_String,
            valuestring: ::core::cell::RefCell::new(current_item_ref.string.borrow().clone()),
            valueint: 0,
            valuedouble: 0.0,
            string: ::core::cell::RefCell::new(None),
        };
        if print_value(Some(&key_item), output_buffer) == 0 {
            return false_0;
        }
        let formatted = output_buffer.format != 0;
        length = (if formatted {
            2 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) as size_t;
        let Some(output) = ensure(output_buffer, length) else {
            return false_0;
        };
        output[0] = ':' as i32 as ::core::ffi::c_uchar;
        if formatted {
            output[1] = '\t' as i32 as ::core::ffi::c_uchar;
        }
        output_buffer.offset = output_buffer.offset.wrapping_add(length);
        if print_value(Some(current_item_ref), output_buffer) == 0 {
            return false_0;
        }
        let has_next =
            get_array_item(Some(item), current_index.wrapping_add(1 as size_t)).is_some();
        let formatted = output_buffer.format != 0;
        length = ((if formatted {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as size_t)
            .wrapping_add(
                (if has_next {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as size_t,
            );
        let Some(output) = ensure(output_buffer, length.wrapping_add(1 as size_t)) else {
            return false_0;
        };
        let mut output_index = 0 as usize;
        if has_next {
            output[output_index] = ',' as i32 as ::core::ffi::c_uchar;
            output_index = output_index.wrapping_add(1);
        }
        if formatted {
            output[output_index] = '\n' as i32 as ::core::ffi::c_uchar;
            output_index = output_index.wrapping_add(1);
        }
        output[output_index] = '\0' as i32 as ::core::ffi::c_uchar;
        output_buffer.offset = output_buffer.offset.wrapping_add(length);
        current_index = current_index.wrapping_add(1 as size_t);
    }
    let formatted = output_buffer.format != 0;
    let depth = output_buffer.depth;
    let final_length = if formatted {
        depth.wrapping_add(1 as size_t)
    } else {
        2 as size_t
    };
    let Some(output) = ensure(output_buffer, final_length) else {
        return false_0;
    };
    let mut output_index = 0 as usize;
    if formatted {
        let mut i_0: size_t = 0;
        i_0 = 0 as size_t;
        while i_0 < depth.wrapping_sub(1 as size_t) {
            output[output_index] = '\t' as i32 as ::core::ffi::c_uchar;
            output_index = output_index.wrapping_add(1);
            i_0 = i_0.wrapping_add(1);
        }
    }
    output[output_index] = '}' as i32 as ::core::ffi::c_uchar;
    output[output_index.wrapping_add(1)] = '\0' as i32 as ::core::ffi::c_uchar;
    output_buffer.offset = output_buffer
        .offset
        .wrapping_add(final_length.wrapping_sub(1 as size_t));
    output_buffer.depth = output_buffer.depth.wrapping_sub(1);
    return true_0;
}
#[export_name = "cJSON_GetArraySize"]
pub unsafe extern "C" fn cJSON_GetArraySize_ffi(mut array: *const cJSON) -> ::core::ffi::c_int {
    let mut size: size_t = 0 as size_t;
    if array.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let mut child = (*array).child.get();
    while let Some(child_ref) = child {
        size = size.wrapping_add(1);
        child = child_ref.next.get();
    }
    return size as ::core::ffi::c_int;
}
fn get_array_item<'a>(array: Option<&'a cJSON>, mut index: size_t) -> Option<&'a cJSON> {
    let Some(array) = array else {
        return None;
    };
    let mut current_child = array.child.get();
    while let Some(current) = current_child {
        if index == 0 as size_t {
            return Some(current);
        }
        index = index.wrapping_sub(1);
        current_child = current.next.get();
    }
    return None;
}
#[export_name = "cJSON_GetArrayItem"]
pub unsafe extern "C" fn cJSON_GetArrayItem_ffi(
    mut array: *const cJSON,
    mut index: ::core::ffi::c_int,
) -> *mut cJSON {
    if index < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<cJSON>();
    }
    match get_array_item(array.as_ref(), index as size_t) {
        Some(item) => item as *const cJSON as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
fn get_object_item<'a>(
    object: Option<&'a cJSON>,
    name: Option<&[::core::ffi::c_uchar]>,
    case_sensitive: cJSON_bool,
) -> Option<&'a cJSON> {
    let Some(object) = object else {
        return None;
    };
    let name = name?;
    let mut child_index = 0 as size_t;
    loop {
        let Some(current) = get_array_item(Some(object), child_index) else {
            return None;
        };
        let current_string = current.string.borrow();
        if current_string.is_none() {
            if case_sensitive != 0 {
                return None;
            }
        } else {
            let current_name = current_string
                .as_deref()
                .expect("object key was checked present");
            if if case_sensitive != 0 {
                c_string_bytes(name) == c_string_bytes(current_name)
            } else {
                case_insensitive_strcmp(name, current_name) == 0 as ::core::ffi::c_int
            } {
                return Some(current);
            }
        }
        child_index = child_index.wrapping_add(1);
    }
}
#[export_name = "cJSON_GetObjectItem"]
pub unsafe extern "C" fn cJSON_GetObjectItem_ffi(
    object: *const cJSON,
    string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let name = if string.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(string).to_bytes_with_nul())
    };
    match get_object_item(object.as_ref(), name, false_0) {
        Some(item) => item as *const cJSON as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
#[export_name = "cJSON_GetObjectItemCaseSensitive"]
pub unsafe extern "C" fn cJSON_GetObjectItemCaseSensitive_ffi(
    object: *const cJSON,
    string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let name = if string.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(string).to_bytes_with_nul())
    };
    match get_object_item(object.as_ref(), name, true_0) {
        Some(item) => item as *const cJSON as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
#[export_name = "cJSON_HasObjectItem"]
pub unsafe extern "C" fn cJSON_HasObjectItem_ffi(
    mut object: *const cJSON,
    mut string: *const ::core::ffi::c_char,
) -> cJSON_bool {
    let name = if string.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(string).to_bytes_with_nul())
    };
    if get_object_item(object.as_ref(), name, false_0).is_some() {
        1 as cJSON_bool
    } else {
        0 as cJSON_bool
    }
}
fn suffix_object(prev: &'static cJSON, item: &'static cJSON) {
    prev.next.set(Some(item));
    item.prev.set(Some(prev));
}
fn create_reference(item: Option<&cJSON>, hooks: &internal_hooks) -> *mut cJSON {
    let Some(item) = item else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    match cJSON_New_Item(
        hooks,
        item.type_0 | cJSON_IsReference,
        item.valueint,
        item.valuedouble,
        ValueStringInit::None,
    ) {
        Some(reference) => {
            reference.child.set(item.child.get());
            *reference.valuestring.borrow_mut() = item.valuestring.borrow().clone();
            reference as *mut cJSON
        }
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
fn add_item_to_array(array: &cJSON, item: &'static cJSON) -> cJSON_bool {
    let child = array.child.get();
    if child.is_none() {
        array.child.set(Some(item));
        item.prev.set(Some(item));
        item.next.set(None);
    } else {
        let child_ref = child.expect("array child is present");
        if let Some(previous) = child_ref.prev.get() {
            suffix_object(previous, item);
            child_ref.prev.set(Some(item));
        }
    }
    return true_0;
}
#[export_name = "cJSON_AddItemToArray"]
pub unsafe extern "C" fn cJSON_AddItemToArray_ffi(
    mut array: *mut cJSON,
    mut item: *mut cJSON,
) -> cJSON_bool {
    if array.is_null() || item.is_null() || array == item {
        return false_0;
    }
    let item_ref: &'static cJSON = &*item;
    return add_item_to_array(&*array, item_ref);
}
fn add_item_to_object(
    object: &mut cJSON,
    string: &'static [::core::ffi::c_uchar],
    item: &'static mut cJSON,
    _hooks: &internal_hooks,
    constant_key: cJSON_bool,
) -> cJSON_bool {
    let (new_key, new_type) = if constant_key != 0 {
        (
            Some(c_string_bytes_with_nul(string).to_vec()),
            item.type_0 | cJSON_StringIsConst,
        )
    } else {
        let Some(new_key) = cJSON_strdup(c_string_bytes_with_nul(string)) else {
            return false_0;
        };
        (Some(new_key), item.type_0 & !cJSON_StringIsConst)
    };
    *item.string.borrow_mut() = new_key;
    item.type_0 = new_type;
    return add_item_to_array(object, item);
}
#[export_name = "cJSON_AddItemToObject"]
pub unsafe extern "C" fn cJSON_AddItemToObject_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
    mut item: *mut cJSON,
) -> cJSON_bool {
    let hooks = global_hooks_snapshot();
    if object.is_null() || string.is_null() || item.is_null() || object == item {
        return false_0;
    }
    return add_item_to_object(
        &mut *object,
        ::std::ffi::CStr::from_ptr(string).to_bytes_with_nul(),
        &mut *item,
        &hooks,
        false_0,
    );
}
#[export_name = "cJSON_AddItemToObjectCS"]
pub unsafe extern "C" fn cJSON_AddItemToObjectCS_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
    mut item: *mut cJSON,
) -> cJSON_bool {
    let hooks = global_hooks_snapshot();
    if object.is_null() || string.is_null() || item.is_null() || object == item {
        return false_0;
    }
    return add_item_to_object(
        &mut *object,
        ::std::ffi::CStr::from_ptr(string).to_bytes_with_nul(),
        &mut *item,
        &hooks,
        true_0,
    );
}
#[export_name = "cJSON_AddItemReferenceToArray"]
pub unsafe extern "C" fn cJSON_AddItemReferenceToArray_ffi(
    mut array: *mut cJSON,
    mut item: *mut cJSON,
) -> cJSON_bool {
    if array.is_null() {
        return false_0;
    }
    let hooks = global_hooks_snapshot();
    let reference = create_reference(item.as_ref(), &hooks);
    if reference.is_null() {
        return false_0;
    }
    return add_item_to_array(&mut *array, &mut *reference);
}
#[export_name = "cJSON_AddItemReferenceToObject"]
pub unsafe extern "C" fn cJSON_AddItemReferenceToObject_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
    mut item: *mut cJSON,
) -> cJSON_bool {
    if object.is_null() || string.is_null() {
        return false_0;
    }
    let hooks = global_hooks_snapshot();
    let reference = create_reference(item.as_ref(), &hooks);
    if reference.is_null() {
        return false_0;
    }
    return add_item_to_object(
        &mut *object,
        ::std::ffi::CStr::from_ptr(string).to_bytes_with_nul(),
        &mut *reference,
        &hooks,
        false_0,
    );
}
#[export_name = "cJSON_AddNullToObject"]
pub unsafe extern "C" fn cJSON_AddNullToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let mut null: *mut cJSON = cJSON_CreateNull();
    let hooks = global_hooks_snapshot();
    if !object.is_null()
        && !name.is_null()
        && !null.is_null()
        && object != null
        && add_item_to_object(
            &mut *object,
            ::std::ffi::CStr::from_ptr(name).to_bytes_with_nul(),
            &mut *null,
            &hooks,
            false_0,
        ) != 0
    {
        return null;
    }
    cJSON_Delete(null.as_ref());
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddTrueToObject"]
pub unsafe extern "C" fn cJSON_AddTrueToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let mut true_item: *mut cJSON = cJSON_CreateTrue();
    let hooks = global_hooks_snapshot();
    if !object.is_null()
        && !name.is_null()
        && !true_item.is_null()
        && object != true_item
        && add_item_to_object(
            &mut *object,
            ::std::ffi::CStr::from_ptr(name).to_bytes_with_nul(),
            &mut *true_item,
            &hooks,
            false_0,
        ) != 0
    {
        return true_item;
    }
    cJSON_Delete(true_item.as_ref());
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddFalseToObject"]
pub unsafe extern "C" fn cJSON_AddFalseToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let mut false_item: *mut cJSON = cJSON_CreateFalse();
    let hooks = global_hooks_snapshot();
    if !object.is_null()
        && !name.is_null()
        && !false_item.is_null()
        && object != false_item
        && add_item_to_object(
            &mut *object,
            ::std::ffi::CStr::from_ptr(name).to_bytes_with_nul(),
            &mut *false_item,
            &hooks,
            false_0,
        ) != 0
    {
        return false_item;
    }
    cJSON_Delete(false_item.as_ref());
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddBoolToObject"]
pub unsafe extern "C" fn cJSON_AddBoolToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
    boolean: cJSON_bool,
) -> *mut cJSON {
    let mut bool_item: *mut cJSON = cJSON_CreateBool(boolean);
    let hooks = global_hooks_snapshot();
    if !object.is_null()
        && !name.is_null()
        && !bool_item.is_null()
        && object != bool_item
        && add_item_to_object(
            &mut *object,
            ::std::ffi::CStr::from_ptr(name).to_bytes_with_nul(),
            &mut *bool_item,
            &hooks,
            false_0,
        ) != 0
    {
        return bool_item;
    }
    cJSON_Delete(bool_item.as_ref());
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddNumberToObject"]
pub unsafe extern "C" fn cJSON_AddNumberToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
    number: ::core::ffi::c_double,
) -> *mut cJSON {
    let mut number_item: *mut cJSON = cJSON_CreateNumber(number);
    let hooks = global_hooks_snapshot();
    if !object.is_null()
        && !name.is_null()
        && !number_item.is_null()
        && object != number_item
        && add_item_to_object(
            &mut *object,
            ::std::ffi::CStr::from_ptr(name).to_bytes_with_nul(),
            &mut *number_item,
            &hooks,
            false_0,
        ) != 0
    {
        return number_item;
    }
    cJSON_Delete(number_item.as_ref());
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddStringToObject"]
pub unsafe extern "C" fn cJSON_AddStringToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
    string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let string = if string.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(string))
    };
    let mut string_item: *mut cJSON = cJSON_CreateString(string);
    let hooks = global_hooks_snapshot();
    if !object.is_null()
        && !name.is_null()
        && !string_item.is_null()
        && object != string_item
        && add_item_to_object(
            &mut *object,
            ::std::ffi::CStr::from_ptr(name).to_bytes_with_nul(),
            &mut *string_item,
            &hooks,
            false_0,
        ) != 0
    {
        return string_item;
    }
    cJSON_Delete(string_item.as_ref());
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddRawToObject"]
pub unsafe extern "C" fn cJSON_AddRawToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
    raw: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let raw = if raw.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(raw))
    };
    let mut raw_item: *mut cJSON = cJSON_CreateRaw(raw);
    let hooks = global_hooks_snapshot();
    if !object.is_null()
        && !name.is_null()
        && !raw_item.is_null()
        && object != raw_item
        && add_item_to_object(
            &mut *object,
            ::std::ffi::CStr::from_ptr(name).to_bytes_with_nul(),
            &mut *raw_item,
            &hooks,
            false_0,
        ) != 0
    {
        return raw_item;
    }
    cJSON_Delete(raw_item.as_ref());
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddObjectToObject"]
pub unsafe extern "C" fn cJSON_AddObjectToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let mut object_item: *mut cJSON = cJSON_CreateObject();
    let hooks = global_hooks_snapshot();
    if !object.is_null()
        && !name.is_null()
        && !object_item.is_null()
        && object != object_item
        && add_item_to_object(
            &mut *object,
            ::std::ffi::CStr::from_ptr(name).to_bytes_with_nul(),
            &mut *object_item,
            &hooks,
            false_0,
        ) != 0
    {
        return object_item;
    }
    cJSON_Delete(object_item.as_ref());
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddArrayToObject"]
pub unsafe extern "C" fn cJSON_AddArrayToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let mut array: *mut cJSON = cJSON_CreateArray();
    let hooks = global_hooks_snapshot();
    if !object.is_null()
        && !name.is_null()
        && !array.is_null()
        && object != array
        && add_item_to_object(
            &mut *object,
            ::std::ffi::CStr::from_ptr(name).to_bytes_with_nul(),
            &mut *array,
            &hooks,
            false_0,
        ) != 0
    {
        return array;
    }
    cJSON_Delete(array.as_ref());
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_DetachItemViaPointer"]
pub unsafe extern "C" fn cJSON_DetachItemViaPointer_ffi(
    mut parent: *mut cJSON,
    mut item: *mut cJSON,
) -> *mut cJSON {
    if parent.is_null() || item.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let parent_child = match (*parent).child.get() {
        Some(parent_child) => parent_child as *const cJSON as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    };
    if item != parent_child && (*item).prev.get().is_none() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    if item != parent_child {
        if let Some(previous) = (*item).prev.get() {
            previous.next.set((*item).next.get());
        }
    }
    if let Some(next) = (*item).next.get() {
        next.prev.set((*item).prev.get());
    }
    if item == parent_child {
        (*parent).child.set((*item).next.get());
    } else if (*item).next.get().is_none() {
        if let Some(parent_child) = (*parent).child.get() {
            parent_child.prev.set((*item).prev.get());
        }
    }
    (*item).prev.set(None);
    (*item).next.set(None);
    return item;
}
#[export_name = "cJSON_DetachItemFromArray"]
pub unsafe extern "C" fn cJSON_DetachItemFromArray_ffi(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
) -> *mut cJSON {
    if which < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut item = match get_array_item(array.as_ref(), which as size_t) {
        Some(item) => item as *const cJSON as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    };
    return cJSON_DetachItemViaPointer_ffi(array, item);
}
#[export_name = "cJSON_DeleteItemFromArray"]
pub unsafe extern "C" fn cJSON_DeleteItemFromArray_ffi(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
) {
    if which < 0 as ::core::ffi::c_int {
        return;
    }
    let mut item = match get_array_item(array.as_ref(), which as size_t) {
        Some(item) => item as *const cJSON as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    };
    cJSON_Delete(cJSON_DetachItemViaPointer_ffi(array, item).as_ref());
}
#[export_name = "cJSON_DetachItemFromObject"]
pub unsafe extern "C" fn cJSON_DetachItemFromObject_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let name = if string.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(string).to_bytes_with_nul())
    };
    let mut to_detach = match get_object_item(object.as_ref(), name, false_0) {
        Some(item) => item as *const cJSON as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    };
    return cJSON_DetachItemViaPointer_ffi(object, to_detach);
}
#[export_name = "cJSON_DetachItemFromObjectCaseSensitive"]
pub unsafe extern "C" fn cJSON_DetachItemFromObjectCaseSensitive_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let name = if string.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(string).to_bytes_with_nul())
    };
    let mut to_detach = match get_object_item(object.as_ref(), name, true_0) {
        Some(item) => item as *const cJSON as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    };
    return cJSON_DetachItemViaPointer_ffi(object, to_detach);
}
#[export_name = "cJSON_DeleteItemFromObject"]
pub unsafe extern "C" fn cJSON_DeleteItemFromObject_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
) {
    let name = if string.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(string).to_bytes_with_nul())
    };
    let mut to_detach = match get_object_item(object.as_ref(), name, false_0) {
        Some(item) => item as *const cJSON as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    };
    cJSON_Delete(cJSON_DetachItemViaPointer_ffi(object, to_detach).as_ref());
}
#[export_name = "cJSON_DeleteItemFromObjectCaseSensitive"]
pub unsafe extern "C" fn cJSON_DeleteItemFromObjectCaseSensitive_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
) {
    let name = if string.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(string).to_bytes_with_nul())
    };
    let mut to_detach = match get_object_item(object.as_ref(), name, true_0) {
        Some(item) => item as *const cJSON as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    };
    cJSON_Delete(cJSON_DetachItemViaPointer_ffi(object, to_detach).as_ref());
}
#[export_name = "cJSON_InsertItemInArray"]
pub unsafe extern "C" fn cJSON_InsertItemInArray_ffi(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
    mut newitem: *mut cJSON,
) -> cJSON_bool {
    let mut after_inserted: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if which < 0 as ::core::ffi::c_int || newitem.is_null() {
        return false_0;
    }
    after_inserted = match get_array_item(array.as_ref(), which as size_t) {
        Some(item) => item as *const cJSON as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    };
    if after_inserted.is_null() {
        if array.is_null() || array == newitem {
            return false_0;
        }
        return add_item_to_array(&mut *array, &mut *newitem);
    }
    let array_child = match (*array).child.get() {
        Some(array_child) => array_child as *const cJSON as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    };
    if after_inserted != array_child && (*after_inserted).prev.get().is_none() {
        return false_0;
    }
    let newitem_ref: &'static cJSON = &*newitem;
    let after_inserted_ref: &'static cJSON = &*after_inserted;
    (*newitem).next.set(Some(after_inserted_ref));
    (*newitem).prev.set((*after_inserted).prev.get());
    (*after_inserted).prev.set(Some(newitem_ref));
    if after_inserted == array_child {
        (*array).child.set(Some(newitem_ref));
    } else {
        if let Some(previous) = (*newitem).prev.get() {
            previous.next.set(Some(newitem_ref));
        }
    }
    return true_0;
}
#[export_name = "cJSON_ReplaceItemViaPointer"]
pub unsafe extern "C" fn cJSON_ReplaceItemViaPointer_ffi(
    mut parent: *mut cJSON,
    item: *mut cJSON,
    mut replacement: *mut cJSON,
) -> cJSON_bool {
    let Some(parent_ref) = parent.as_mut() else {
        return false_0;
    };
    let Some(replacement_ref) = replacement.as_mut() else {
        return false_0;
    };
    let Some(parent_child_ref) = parent_ref.child.get() else {
        return false_0;
    };
    let parent_child = parent_child_ref as *const cJSON as *mut cJSON;
    if item.is_null() {
        return false_0;
    }
    if replacement == item {
        return true_0;
    }
    let item_ref = &mut *item;
    let replacement_shared: &'static cJSON = &*replacement;

    replacement_ref.next.set(item_ref.next.get());
    replacement_ref.prev.set(item_ref.prev.get());
    if let Some(next) = replacement_ref.next.get() {
        next.prev.set(Some(replacement_shared));
    }
    if parent_child == item {
        if item_ref
            .prev
            .get()
            .is_some_and(|prev| ::core::ptr::eq(prev, parent_child_ref))
        {
            replacement_ref.prev.set(Some(replacement_shared));
        }
        parent_ref.child.set(Some(replacement_shared));
    } else {
        if let Some(previous) = replacement_ref.prev.get() {
            previous.next.set(Some(replacement_shared));
        }
        if replacement_ref.next.get().is_none() {
            if let Some(parent_child) = parent_ref.child.get() {
                parent_child.prev.set(Some(replacement_shared));
            }
        }
    }
    item_ref.next.set(None);
    item_ref.prev.set(None);
    cJSON_Delete(item.as_ref());
    return true_0;
}
#[export_name = "cJSON_ReplaceItemInArray"]
pub unsafe extern "C" fn cJSON_ReplaceItemInArray_ffi(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
    mut newitem: *mut cJSON,
) -> cJSON_bool {
    if which < 0 as ::core::ffi::c_int {
        return false_0;
    }
    return cJSON_ReplaceItemViaPointer_ffi(
        array,
        match get_array_item(array.as_ref(), which as size_t) {
            Some(item) => item as *const cJSON as *mut cJSON,
            None => ::core::ptr::null_mut::<cJSON>(),
        },
        newitem,
    );
}
#[export_name = "cJSON_ReplaceItemInObject"]
pub unsafe extern "C" fn cJSON_ReplaceItemInObject_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
    mut newitem: *mut cJSON,
) -> cJSON_bool {
    if newitem.is_null() || string.is_null() {
        return false_0;
    }
    let replacement_key = ::std::ffi::CStr::from_ptr(string);
    let Some(replacement_key) = cJSON_strdup(replacement_key.to_bytes_with_nul()) else {
        return false_0;
    };
    *(*newitem).string.borrow_mut() = Some(replacement_key.clone());
    (*newitem).type_0 &= !cJSON_StringIsConst;
    let item = match get_object_item(object.as_ref(), Some(&replacement_key), false_0) {
        Some(item) => item as *const cJSON as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    };
    return cJSON_ReplaceItemViaPointer_ffi(object, item, newitem);
}
#[export_name = "cJSON_ReplaceItemInObjectCaseSensitive"]
pub unsafe extern "C" fn cJSON_ReplaceItemInObjectCaseSensitive_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
    mut newitem: *mut cJSON,
) -> cJSON_bool {
    if newitem.is_null() || string.is_null() {
        return false_0;
    }
    let replacement_key = ::std::ffi::CStr::from_ptr(string);
    let Some(replacement_key) = cJSON_strdup(replacement_key.to_bytes_with_nul()) else {
        return false_0;
    };
    *(*newitem).string.borrow_mut() = Some(replacement_key.clone());
    (*newitem).type_0 &= !cJSON_StringIsConst;
    let item = match get_object_item(object.as_ref(), Some(&replacement_key), true_0) {
        Some(item) => item as *const cJSON as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    };
    return cJSON_ReplaceItemViaPointer_ffi(object, item, newitem);
}
pub fn cJSON_CreateNull() -> *mut cJSON {
    let hooks = global_hooks_snapshot();
    return match cJSON_New_Item(
        &hooks,
        cJSON_NULL,
        0 as ::core::ffi::c_int,
        0.0f64,
        ValueStringInit::None,
    ) {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    };
}
#[export_name = "cJSON_CreateNull"]
pub unsafe extern "C" fn cJSON_CreateNull_ffi() -> *mut cJSON {
    cJSON_CreateNull()
}
pub fn cJSON_CreateTrue() -> *mut cJSON {
    let hooks = global_hooks_snapshot();
    return match cJSON_New_Item(
        &hooks,
        cJSON_True,
        0 as ::core::ffi::c_int,
        0.0f64,
        ValueStringInit::None,
    ) {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    };
}
#[export_name = "cJSON_CreateTrue"]
pub unsafe extern "C" fn cJSON_CreateTrue_ffi() -> *mut cJSON {
    cJSON_CreateTrue()
}
pub fn cJSON_CreateFalse() -> *mut cJSON {
    let hooks = global_hooks_snapshot();
    return match cJSON_New_Item(
        &hooks,
        cJSON_False,
        0 as ::core::ffi::c_int,
        0.0f64,
        ValueStringInit::None,
    ) {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    };
}
#[export_name = "cJSON_CreateFalse"]
pub unsafe extern "C" fn cJSON_CreateFalse_ffi() -> *mut cJSON {
    cJSON_CreateFalse()
}
pub fn cJSON_CreateBool(boolean: cJSON_bool) -> *mut cJSON {
    let hooks = global_hooks_snapshot();
    return match cJSON_New_Item(
        &hooks,
        if boolean != 0 {
            cJSON_True
        } else {
            cJSON_False
        },
        0 as ::core::ffi::c_int,
        0.0f64,
        ValueStringInit::None,
    ) {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    };
}
#[export_name = "cJSON_CreateBool"]
pub unsafe extern "C" fn cJSON_CreateBool_ffi(mut boolean: cJSON_bool) -> *mut cJSON {
    cJSON_CreateBool(boolean)
}
pub fn cJSON_CreateNumber(mut num: ::core::ffi::c_double) -> *mut cJSON {
    let hooks = global_hooks_snapshot();
    return match cJSON_New_Item(
        &hooks,
        cJSON_Number,
        cJSON_NumberValueInt(num),
        num,
        ValueStringInit::None,
    ) {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    };
}
#[export_name = "cJSON_CreateNumber"]
pub unsafe extern "C" fn cJSON_CreateNumber_ffi(mut num: ::core::ffi::c_double) -> *mut cJSON {
    cJSON_CreateNumber(num)
}
pub fn cJSON_CreateString(string: Option<&::std::ffi::CStr>) -> *mut cJSON {
    let hooks = global_hooks_snapshot();
    let mut valuestring_failed = false;
    let valuestring = match string {
        Some(string) => ValueStringInit::Copy {
            bytes: string.to_bytes_with_nul(),
            failed: &mut valuestring_failed,
        },
        None => ValueStringInit::Invalid {
            failed: &mut valuestring_failed,
        },
    };
    let item = cJSON_New_Item(
        &hooks,
        cJSON_String,
        0 as ::core::ffi::c_int,
        0.0f64,
        valuestring,
    );
    if valuestring_failed {
        if let Some(item) = item {
            cJSON_Delete(Some(item));
        }
        return ::core::ptr::null_mut::<cJSON>();
    }
    return match item {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    };
}
#[export_name = "cJSON_CreateString"]
pub unsafe extern "C" fn cJSON_CreateString_ffi(
    mut string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let string = if string.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(string))
    };
    cJSON_CreateString(string)
}
#[export_name = "cJSON_CreateStringReference"]
pub unsafe extern "C" fn cJSON_CreateStringReference_ffi(
    mut string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let hooks = global_hooks_snapshot();
    let valuestring: Option<Vec<::core::ffi::c_uchar>> = if string.is_null() {
        None
    } else {
        Some(
            ::std::ffi::CStr::from_ptr(string)
                .to_bytes_with_nul()
                .to_vec(),
        )
    };
    let item = cJSON_New_Item(
        &hooks,
        cJSON_String | cJSON_IsReference,
        0 as ::core::ffi::c_int,
        0.0f64,
        ValueStringInit::None,
    );
    return match item {
        Some(item) => {
            *item.valuestring.borrow_mut() = valuestring;
            item as *mut cJSON
        }
        None => ::core::ptr::null_mut::<cJSON>(),
    };
}
#[export_name = "cJSON_CreateObjectReference"]
pub unsafe extern "C" fn cJSON_CreateObjectReference_ffi(mut child: *const cJSON) -> *mut cJSON {
    let hooks = global_hooks_snapshot();
    let child_ref: Option<&'static cJSON> = if child.is_null() { None } else { Some(&*child) };
    let item = cJSON_New_Item(
        &hooks,
        cJSON_Object | cJSON_IsReference,
        0 as ::core::ffi::c_int,
        0.0f64,
        ValueStringInit::None,
    );
    return match item {
        Some(item) => {
            item.child.set(child_ref);
            item as *mut cJSON
        }
        None => ::core::ptr::null_mut::<cJSON>(),
    };
}
#[export_name = "cJSON_CreateArrayReference"]
pub unsafe extern "C" fn cJSON_CreateArrayReference_ffi(mut child: *const cJSON) -> *mut cJSON {
    let hooks = global_hooks_snapshot();
    let child_ref: Option<&'static cJSON> = if child.is_null() { None } else { Some(&*child) };
    let item = cJSON_New_Item(
        &hooks,
        cJSON_Array | cJSON_IsReference,
        0 as ::core::ffi::c_int,
        0.0f64,
        ValueStringInit::None,
    );
    return match item {
        Some(item) => {
            item.child.set(child_ref);
            item as *mut cJSON
        }
        None => ::core::ptr::null_mut::<cJSON>(),
    };
}
pub fn cJSON_CreateRaw(raw: Option<&::std::ffi::CStr>) -> *mut cJSON {
    let hooks = global_hooks_snapshot();
    let mut valuestring_failed = false;
    let valuestring = match raw {
        Some(raw) => ValueStringInit::Copy {
            bytes: raw.to_bytes_with_nul(),
            failed: &mut valuestring_failed,
        },
        None => ValueStringInit::Invalid {
            failed: &mut valuestring_failed,
        },
    };
    let item = cJSON_New_Item(
        &hooks,
        cJSON_Raw,
        0 as ::core::ffi::c_int,
        0.0f64,
        valuestring,
    );
    if valuestring_failed {
        if let Some(item) = item {
            cJSON_Delete(Some(item));
        }
        return ::core::ptr::null_mut::<cJSON>();
    }
    return match item {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    };
}
#[export_name = "cJSON_CreateRaw"]
pub unsafe extern "C" fn cJSON_CreateRaw_ffi(mut raw: *const ::core::ffi::c_char) -> *mut cJSON {
    let raw = if raw.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(raw))
    };
    cJSON_CreateRaw(raw)
}
pub fn cJSON_CreateArray() -> *mut cJSON {
    let hooks = global_hooks_snapshot();
    return match cJSON_New_Item(
        &hooks,
        cJSON_Array,
        0 as ::core::ffi::c_int,
        0.0f64,
        ValueStringInit::None,
    ) {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    };
}
#[export_name = "cJSON_CreateArray"]
pub unsafe extern "C" fn cJSON_CreateArray_ffi() -> *mut cJSON {
    cJSON_CreateArray()
}
pub fn cJSON_CreateObject() -> *mut cJSON {
    let hooks = global_hooks_snapshot();
    return match cJSON_New_Item(
        &hooks,
        cJSON_Object,
        0 as ::core::ffi::c_int,
        0.0f64,
        ValueStringInit::None,
    ) {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    };
}
#[export_name = "cJSON_CreateObject"]
pub unsafe extern "C" fn cJSON_CreateObject_ffi() -> *mut cJSON {
    cJSON_CreateObject()
}
#[export_name = "cJSON_CreateIntArray"]
pub unsafe extern "C" fn cJSON_CreateIntArray_ffi(
    mut numbers: *const ::core::ffi::c_int,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let numbers = ::core::slice::from_raw_parts(numbers, count as usize);
    let array: *mut cJSON = cJSON_CreateArray();
    if array.is_null() {
        return array;
    }
    for &number in numbers {
        let item = cJSON_CreateNumber(number as ::core::ffi::c_double);
        if item.is_null() {
            cJSON_Delete(array.as_ref());
            return ::core::ptr::null_mut::<cJSON>();
        }
        add_item_to_array(&mut *array, &mut *item);
    }
    return array;
}
#[export_name = "cJSON_CreateFloatArray"]
pub unsafe extern "C" fn cJSON_CreateFloatArray_ffi(
    mut numbers: *const ::core::ffi::c_float,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let numbers = ::core::slice::from_raw_parts(numbers, count as usize);
    let array: *mut cJSON = cJSON_CreateArray();
    if array.is_null() {
        return array;
    }
    for &number in numbers {
        let item = cJSON_CreateNumber(number as ::core::ffi::c_double);
        if item.is_null() {
            cJSON_Delete(array.as_ref());
            return ::core::ptr::null_mut::<cJSON>();
        }
        add_item_to_array(&mut *array, &mut *item);
    }
    return array;
}
#[export_name = "cJSON_CreateDoubleArray"]
pub unsafe extern "C" fn cJSON_CreateDoubleArray_ffi(
    mut numbers: *const ::core::ffi::c_double,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let numbers = ::core::slice::from_raw_parts(numbers, count as usize);
    let array: *mut cJSON = cJSON_CreateArray();
    if array.is_null() {
        return array;
    }
    for &number in numbers {
        let item = cJSON_CreateNumber(number);
        if item.is_null() {
            cJSON_Delete(array.as_ref());
            return ::core::ptr::null_mut::<cJSON>();
        }
        add_item_to_array(&mut *array, &mut *item);
    }
    return array;
}
#[export_name = "cJSON_CreateStringArray"]
pub unsafe extern "C" fn cJSON_CreateStringArray_ffi(
    mut strings: *const *const ::core::ffi::c_char,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    if count < 0 as ::core::ffi::c_int || strings.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let strings = ::core::slice::from_raw_parts(strings, count as usize);
    let array: *mut cJSON = cJSON_CreateArray();
    if array.is_null() {
        return array;
    }
    for &string in strings {
        let string = if string.is_null() {
            None
        } else {
            Some(::std::ffi::CStr::from_ptr(string))
        };
        let item = cJSON_CreateString(string);
        if item.is_null() {
            cJSON_Delete(array.as_ref());
            return ::core::ptr::null_mut::<cJSON>();
        }
        add_item_to_array(&mut *array, &mut *item);
    }
    return array;
}
#[export_name = "cJSON_Duplicate"]
pub unsafe extern "C" fn cJSON_Duplicate_ffi(
    mut item: *const cJSON,
    mut recurse: cJSON_bool,
) -> *mut cJSON {
    return match cJSON_Duplicate_rec(item.as_ref(), 0 as size_t, recurse) {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    };
}
pub fn cJSON_Duplicate_rec(
    item: Option<&cJSON>,
    depth: size_t,
    recurse: cJSON_bool,
) -> Option<&'static mut cJSON> {
    let item = item?;
    let hooks = global_hooks_snapshot();
    let mut valuestring_failed = false;
    let item_valuestring = item.valuestring.borrow();
    let valuestring = match item_valuestring.as_deref() {
        Some(valuestring) => ValueStringInit::Copy {
            bytes: c_string_bytes_with_nul(valuestring),
            failed: &mut valuestring_failed,
        },
        None => ValueStringInit::None,
    };
    let Some(newitem_ref) = cJSON_New_Item(
        &hooks,
        item.type_0 & !cJSON_IsReference,
        item.valueint,
        item.valuedouble,
        valuestring,
    ) else {
        return None;
    };
    if valuestring_failed {
        cJSON_Delete(Some(newitem_ref));
        return None;
    }
    if let Some(item_key) = item.string.borrow().as_deref() {
        let new_key = if item.type_0 & cJSON_StringIsConst != 0 {
            Some(c_string_bytes_with_nul(item_key).to_vec())
        } else {
            cJSON_strdup(c_string_bytes_with_nul(item_key))
        };
        *newitem_ref.string.borrow_mut() = new_key;
        if newitem_ref.string.borrow().is_none() {
            cJSON_Delete(Some(newitem_ref));
            return None;
        }
    }
    if recurse == 0 {
        return Some(newitem_ref);
    }
    let mut child_index = 0 as size_t;
    loop {
        let Some(child_ref) = get_array_item(Some(item), child_index) else {
            return Some(newitem_ref);
        };
        if depth >= CJSON_CIRCULAR_LIMIT as size_t {
            cJSON_Delete(Some(newitem_ref));
            return None;
        }
        let Some(newchild_ref) =
            cJSON_Duplicate_rec(Some(child_ref), depth.wrapping_add(1 as size_t), true_0)
        else {
            cJSON_Delete(Some(newitem_ref));
            return None;
        };
        add_item_to_array(newitem_ref, newchild_ref);
        child_index = child_index.wrapping_add(1 as size_t);
    }
}
#[export_name = "cJSON_Duplicate_rec"]
pub unsafe extern "C" fn cJSON_Duplicate_rec_ffi(
    mut item: *const cJSON,
    mut depth: size_t,
    mut recurse: cJSON_bool,
) -> *mut cJSON {
    match cJSON_Duplicate_rec(item.as_ref(), depth, recurse) {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
fn skip_oneline_comment(input: &mut usize, bytes: &[::core::ffi::c_uchar]) {
    *input = (*input).wrapping_add(2);
    while bytes.get(*input).copied().is_some_and(|byte| byte != 0) {
        if bytes[*input] == '\n' as ::core::ffi::c_uchar {
            *input = (*input).wrapping_add(1);
            return;
        }
        *input = (*input).wrapping_add(1);
    }
}
fn skip_multiline_comment(input: &mut usize, bytes: &[::core::ffi::c_uchar]) {
    *input = (*input).wrapping_add(2);
    while bytes.get(*input).copied().is_some_and(|byte| byte != 0) {
        if bytes[*input] == '*' as ::core::ffi::c_uchar
            && bytes.get((*input).wrapping_add(1)).copied() == Some('/' as ::core::ffi::c_uchar)
        {
            *input = (*input).wrapping_add(2);
            return;
        }
        *input = (*input).wrapping_add(1);
    }
}
fn minify_string(input: &mut usize, output: &mut usize, bytes: &mut [::core::ffi::c_uchar]) {
    bytes[*output] = bytes[*input];
    *input = (*input).wrapping_add(1);
    *output = (*output).wrapping_add(1);
    while bytes.get(*input).copied().is_some_and(|byte| byte != 0) {
        bytes[*output] = bytes[*input];
        if bytes[*input] == '"' as ::core::ffi::c_uchar {
            *input = (*input).wrapping_add(1);
            *output = (*output).wrapping_add(1);
            return;
        } else if bytes[*input] == '\\' as ::core::ffi::c_uchar
            && bytes.get((*input).wrapping_add(1)).copied() == Some('"' as ::core::ffi::c_uchar)
        {
            bytes[(*output).wrapping_add(1)] = bytes[(*input).wrapping_add(1)];
            *input = (*input).wrapping_add(1);
            *output = (*output).wrapping_add(1);
        }
        *input = (*input).wrapping_add(1);
        *output = (*output).wrapping_add(1);
    }
}
fn cJSON_Minify(json: &mut [::core::ffi::c_uchar]) {
    let mut input = 0 as usize;
    let mut into = 0 as usize;
    while json.get(input).copied().is_some_and(|byte| byte != 0) {
        match json[input] as ::core::ffi::c_int {
            32 | 9 | 13 | 10 => {
                input = input.wrapping_add(1);
            }
            47 => {
                if json.get(input.wrapping_add(1)).copied() == Some('/' as ::core::ffi::c_uchar) {
                    skip_oneline_comment(&mut input, json);
                } else if json.get(input.wrapping_add(1)).copied()
                    == Some('*' as ::core::ffi::c_uchar)
                {
                    skip_multiline_comment(&mut input, json);
                } else {
                    input = input.wrapping_add(1);
                }
            }
            34 => {
                minify_string(&mut input, &mut into, json);
            }
            _ => {
                json[into] = json[input];
                input = input.wrapping_add(1);
                into = into.wrapping_add(1);
            }
        }
    }
    if let Some(byte) = json.get_mut(into) {
        *byte = '\0' as ::core::ffi::c_uchar;
    }
}
#[export_name = "cJSON_Minify"]
pub unsafe extern "C" fn cJSON_Minify_ffi(mut json: *mut ::core::ffi::c_char) {
    if json.is_null() {
        return;
    }
    let length = strlen(json);
    let bytes = ::core::slice::from_raw_parts_mut(
        json as *mut ::core::ffi::c_uchar,
        length.wrapping_add(1 as size_t),
    );
    cJSON_Minify(bytes);
}
pub fn cJSON_IsInvalid(item: Option<&cJSON>) -> cJSON_bool {
    return item
        .map(|item| (item.type_0 & 0xff as ::core::ffi::c_int == cJSON_Invalid) as cJSON_bool)
        .unwrap_or(false_0);
}
#[export_name = "cJSON_IsInvalid"]
pub unsafe extern "C" fn cJSON_IsInvalid_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsInvalid(item.as_ref())
}
pub fn cJSON_IsFalse(item: Option<&cJSON>) -> cJSON_bool {
    return item
        .map(|item| (item.type_0 & 0xff as ::core::ffi::c_int == cJSON_False) as cJSON_bool)
        .unwrap_or(false_0);
}
#[export_name = "cJSON_IsFalse"]
pub unsafe extern "C" fn cJSON_IsFalse_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsFalse(item.as_ref())
}
pub fn cJSON_IsTrue(item: Option<&cJSON>) -> cJSON_bool {
    return item
        .map(|item| (item.type_0 & 0xff as ::core::ffi::c_int == cJSON_True) as cJSON_bool)
        .unwrap_or(false_0);
}
#[export_name = "cJSON_IsTrue"]
pub unsafe extern "C" fn cJSON_IsTrue_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsTrue(item.as_ref())
}
pub fn cJSON_IsBool(item: Option<&cJSON>) -> cJSON_bool {
    return item
        .map(|item| {
            (item.type_0 & (cJSON_True | cJSON_False) != 0 as ::core::ffi::c_int) as cJSON_bool
        })
        .unwrap_or(false_0);
}
#[export_name = "cJSON_IsBool"]
pub unsafe extern "C" fn cJSON_IsBool_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsBool(item.as_ref())
}
pub fn cJSON_IsNull(item: Option<&cJSON>) -> cJSON_bool {
    return item
        .map(|item| (item.type_0 & 0xff as ::core::ffi::c_int == cJSON_NULL) as cJSON_bool)
        .unwrap_or(false_0);
}
#[export_name = "cJSON_IsNull"]
pub unsafe extern "C" fn cJSON_IsNull_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsNull(item.as_ref())
}
pub fn cJSON_IsNumber(item: Option<&cJSON>) -> cJSON_bool {
    return item
        .map(|item| (item.type_0 & 0xff as ::core::ffi::c_int == cJSON_Number) as cJSON_bool)
        .unwrap_or(false_0);
}
#[export_name = "cJSON_IsNumber"]
pub unsafe extern "C" fn cJSON_IsNumber_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsNumber(item.as_ref())
}
pub fn cJSON_IsString(item: Option<&cJSON>) -> cJSON_bool {
    return item
        .map(|item| (item.type_0 & 0xff as ::core::ffi::c_int == cJSON_String) as cJSON_bool)
        .unwrap_or(false_0);
}
#[export_name = "cJSON_IsString"]
pub unsafe extern "C" fn cJSON_IsString_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsString(item.as_ref())
}
pub fn cJSON_IsArray(item: Option<&cJSON>) -> cJSON_bool {
    return item
        .map(|item| (item.type_0 & 0xff as ::core::ffi::c_int == cJSON_Array) as cJSON_bool)
        .unwrap_or(false_0);
}
#[export_name = "cJSON_IsArray"]
pub unsafe extern "C" fn cJSON_IsArray_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsArray(item.as_ref())
}
pub fn cJSON_IsObject(item: Option<&cJSON>) -> cJSON_bool {
    return item
        .map(|item| (item.type_0 & 0xff as ::core::ffi::c_int == cJSON_Object) as cJSON_bool)
        .unwrap_or(false_0);
}
#[export_name = "cJSON_IsObject"]
pub unsafe extern "C" fn cJSON_IsObject_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsObject(item.as_ref())
}
pub fn cJSON_IsRaw(item: Option<&cJSON>) -> cJSON_bool {
    return item
        .map(|item| (item.type_0 & 0xff as ::core::ffi::c_int == cJSON_Raw) as cJSON_bool)
        .unwrap_or(false_0);
}
#[export_name = "cJSON_IsRaw"]
pub unsafe extern "C" fn cJSON_IsRaw_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsRaw(item.as_ref())
}
pub fn cJSON_Compare(
    a: Option<&cJSON>,
    b: Option<&cJSON>,
    case_sensitive: cJSON_bool,
) -> cJSON_bool {
    let (Some(a), Some(b)) = (a, b) else {
        return false_0;
    };
    let item_type = a.type_0 & 0xff as ::core::ffi::c_int;
    if item_type != b.type_0 & 0xff as ::core::ffi::c_int {
        return false_0;
    }
    match item_type {
        cJSON_False | cJSON_True | cJSON_NULL | cJSON_Number | cJSON_String | cJSON_Raw
        | cJSON_Array | cJSON_Object => {}
        _ => return false_0,
    }
    if ::core::ptr::eq(a, b) {
        return true_0;
    }
    match item_type {
        cJSON_False | cJSON_True | cJSON_NULL => return true_0,
        cJSON_Number => {
            if compare_double(a.valuedouble, b.valuedouble) != 0 {
                return true_0;
            }
            return false_0;
        }
        cJSON_String | cJSON_Raw => {
            let a_valuestring = a.valuestring.borrow();
            let b_valuestring = b.valuestring.borrow();
            let (Some(a_string), Some(b_string)) =
                (a_valuestring.as_deref(), b_valuestring.as_deref())
            else {
                return false_0;
            };
            if c_string_bytes(a_string) == c_string_bytes(b_string) {
                return true_0;
            }
            return false_0;
        }
        cJSON_Array => {
            let mut index = 0 as size_t;
            loop {
                let a_element_ref = get_array_item(Some(a), index);
                let b_element_ref = get_array_item(Some(b), index);
                let (Some(a_element_ref), Some(b_element_ref)) = (a_element_ref, b_element_ref)
                else {
                    if a_element_ref.is_some() || b_element_ref.is_some() {
                        return false_0;
                    }
                    return true_0;
                };
                if cJSON_Compare(Some(a_element_ref), Some(b_element_ref), case_sensitive) == 0 {
                    return false_0;
                }
                index = index.wrapping_add(1);
            }
        }
        cJSON_Object => {
            let mut index = 0 as size_t;
            while let Some(a_element_ref) = get_array_item(Some(a), index) {
                let a_key = a_element_ref.string.borrow();
                let Some(b_element_ref) =
                    get_object_item(Some(b), a_key.as_deref(), case_sensitive)
                else {
                    return false_0;
                };
                if cJSON_Compare(Some(a_element_ref), Some(b_element_ref), case_sensitive) == 0 {
                    return false_0;
                }
                index = index.wrapping_add(1);
            }
            let mut index = 0 as size_t;
            while let Some(b_element_ref) = get_array_item(Some(b), index) {
                let b_key = b_element_ref.string.borrow();
                let Some(a_element_ref) =
                    get_object_item(Some(a), b_key.as_deref(), case_sensitive)
                else {
                    return false_0;
                };
                if cJSON_Compare(Some(b_element_ref), Some(a_element_ref), case_sensitive) == 0 {
                    return false_0;
                }
                index = index.wrapping_add(1);
            }
            return true_0;
        }
        _ => return false_0,
    };
}
#[export_name = "cJSON_Compare"]
pub unsafe extern "C" fn cJSON_Compare_ffi(
    a: *const cJSON,
    b: *const cJSON,
    case_sensitive: cJSON_bool,
) -> cJSON_bool {
    cJSON_Compare(a.as_ref(), b.as_ref(), case_sensitive)
}
#[export_name = "cJSON_malloc"]
pub unsafe extern "C" fn cJSON_malloc_ffi(mut size: size_t) -> *mut ::core::ffi::c_void {
    let hooks = global_hooks_snapshot();
    hooks.allocate.expect("non-null function pointer")(size)
}
pub fn cJSON_free<T>(object: ::core::ptr::NonNull<T>, hooks: &internal_hooks) {
    hooks.deallocate.expect("non-null function pointer")(
        object.as_ptr() as *mut ::core::ffi::c_void
    );
}
#[export_name = "cJSON_free"]
pub unsafe extern "C" fn cJSON_free_ffi(mut object: *mut ::core::ffi::c_void) {
    let hooks = global_hooks_snapshot();
    if let Some(object) = ::core::ptr::NonNull::new(object) {
        cJSON_free(object, &hooks);
    } else {
        hooks.deallocate.expect("non-null function pointer")(NULL);
    }
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __DBL_EPSILON__: ::core::ffi::c_double = 2.2204460492503131e-16f64;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const INT_MIN: ::core::ffi::c_int = -2147483647 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
pub const DBL_EPSILON: ::core::ffi::c_double = __DBL_EPSILON__;
