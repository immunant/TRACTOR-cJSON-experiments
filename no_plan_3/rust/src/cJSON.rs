extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cJSON {
    pub next: *mut cJSON,
    pub prev: *mut cJSON,
    pub child: *mut cJSON,
    pub type_0: ::core::ffi::c_int,
    pub valuestring: *mut ::core::ffi::c_char,
    pub valueint: ::core::ffi::c_int,
    pub valuedouble: ::core::ffi::c_double,
    pub string: *mut ::core::ffi::c_char,
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
    pub allocate: Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>,
    pub deallocate: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub reallocate:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct error {
    pub json_addr: size_t,
    pub position: size_t,
}
#[derive(Copy, Clone)]
pub struct parse_buffer<'a> {
    pub content: &'a [::core::ffi::c_uchar],
    pub length: size_t,
    pub offset: size_t,
    pub depth: size_t,
    pub hooks: internal_hooks,
}
pub enum printbuffer_buffer<'a> {
    Dynamic(::std::vec::Vec<::core::ffi::c_uchar>),
    Preallocated(&'a mut [::core::ffi::c_uchar]),
}
pub struct printbuffer<'a> {
    pub buffer: printbuffer_buffer<'a>,
    pub offset: size_t,
    pub depth: size_t,
    pub format: cJSON_bool,
}
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
static GLOBAL_ERROR_JSON_ADDR: ::core::sync::atomic::AtomicUsize =
    ::core::sync::atomic::AtomicUsize::new(0 as size_t);
static GLOBAL_ERROR_POSITION: ::core::sync::atomic::AtomicUsize =
    ::core::sync::atomic::AtomicUsize::new(0 as size_t);
pub fn cJSON_GetErrorLocation() -> error {
    let json_addr = GLOBAL_ERROR_JSON_ADDR.load(::core::sync::atomic::Ordering::Relaxed);
    let position = GLOBAL_ERROR_POSITION.load(::core::sync::atomic::Ordering::Relaxed);
    error {
        json_addr,
        position,
    }
}
#[export_name = "cJSON_GetErrorPtr"]
pub unsafe extern "C" fn cJSON_GetErrorPtr_ffi() -> *const ::core::ffi::c_char {
    let location = cJSON_GetErrorLocation();
    ::core::ptr::with_exposed_provenance::<::core::ffi::c_char>(location.json_addr)
        .wrapping_add(location.position)
}
pub enum cJSON_StringValue {
    Present,
}
pub fn cJSON_GetStringValue(item: Option<&cJSON>) -> Option<cJSON_StringValue> {
    let item = item?;
    if cJSON_IsString(Some(item)) == 0 {
        return None;
    }
    if item.valuestring.is_null() {
        return None;
    }
    Some(cJSON_StringValue::Present)
}
#[export_name = "cJSON_GetStringValue"]
pub unsafe extern "C" fn cJSON_GetStringValue_ffi(item: *const cJSON) -> *mut ::core::ffi::c_char {
    let item = item.as_ref();
    match (item, cJSON_GetStringValue(item)) {
        (Some(item), Some(cJSON_StringValue::Present)) => item.valuestring,
        _ => ::core::ptr::null_mut::<::core::ffi::c_char>(),
    }
}
pub fn cJSON_GetNumberValue(item: Option<&cJSON>) -> ::core::ffi::c_double {
    let Some(item) = item else {
        return 0.0f64 / 0.0f64;
    };
    if cJSON_IsNumber(Some(item)) == 0 {
        return 0.0f64 / 0.0f64;
    }
    return item.valuedouble;
}
#[export_name = "cJSON_GetNumberValue"]
pub unsafe extern "C" fn cJSON_GetNumberValue_ffi(item: *const cJSON) -> ::core::ffi::c_double {
    cJSON_GetNumberValue(item.as_ref())
}
pub fn cJSON_Version() -> &'static ::std::ffi::CStr {
    c"1.7.19"
}
#[export_name = "cJSON_Version"]
pub unsafe extern "C" fn cJSON_Version_ffi() -> *const ::core::ffi::c_char {
    cJSON_Version().as_ptr()
}
fn case_insensitive_strcmp(
    string1: &::std::ffi::CStr,
    string2: &::std::ffi::CStr,
) -> ::core::ffi::c_int {
    for (left, right) in string1
        .to_bytes_with_nul()
        .iter()
        .copied()
        .zip(string2.to_bytes_with_nul().iter().copied())
    {
        let left = left.to_ascii_lowercase();
        let right = right.to_ascii_lowercase();
        if left != right {
            return left as ::core::ffi::c_int - right as ::core::ffi::c_int;
        }
        if left == b'\0' {
            return 0 as ::core::ffi::c_int;
        }
    }
    0 as ::core::ffi::c_int
}
static global_hooks: ::std::sync::Mutex<internal_hooks> = ::std::sync::Mutex::new(internal_hooks {
    allocate: Some(malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void),
    deallocate: Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    reallocate: Some(
        realloc
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void,
    ),
});
fn get_global_hooks() -> internal_hooks {
    *global_hooks
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}
fn set_global_hooks(hooks: internal_hooks) {
    *global_hooks
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner()) = hooks;
}
fn cJSON_strdup(
    string: Option<&::std::ffi::CStr>,
    hooks: &internal_hooks,
) -> Option<::core::ptr::NonNull<::core::ffi::c_char>> {
    let string = string?;
    let bytes = string.to_bytes_with_nul();
    let copy = unsafe { hooks.allocate.expect("non-null function pointer")(bytes.len()) }
        as *mut ::core::ffi::c_char;
    let copy = ::core::ptr::NonNull::new(copy)?;
    unsafe {
        ::core::ptr::copy_nonoverlapping(
            bytes.as_ptr().cast::<::core::ffi::c_char>(),
            copy.as_ptr(),
            bytes.len(),
        );
    }
    Some(copy)
}
pub fn cJSON_InitHooks(hooks: Option<&cJSON_Hooks>) {
    let Some(hooks) = hooks else {
        set_global_hooks(internal_hooks {
            allocate: Some(malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void),
            deallocate: Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
            reallocate: Some(
                realloc
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        size_t,
                    ) -> *mut ::core::ffi::c_void,
            ),
        });
        return;
    };
    let mut new_hooks = internal_hooks {
        allocate: Some(malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void),
        deallocate: Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        reallocate: None,
    };
    let default_allocate = new_hooks
        .allocate
        .expect("default allocate hook must be present");
    let default_deallocate = new_hooks
        .deallocate
        .expect("default deallocate hook must be present");
    if hooks.malloc_fn.is_some() {
        new_hooks.allocate = hooks.malloc_fn;
    }
    if hooks.free_fn.is_some() {
        new_hooks.deallocate = hooks.free_fn;
    }
    let uses_default_allocate = if let Some(allocate) = new_hooks.allocate {
        ::core::ptr::fn_addr_eq(allocate, default_allocate)
    } else {
        false
    };
    let uses_default_deallocate = if let Some(deallocate) = new_hooks.deallocate {
        ::core::ptr::fn_addr_eq(deallocate, default_deallocate)
    } else {
        false
    };
    if uses_default_allocate && uses_default_deallocate {
        new_hooks.reallocate = Some(
            realloc
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                ) -> *mut ::core::ffi::c_void,
        );
    }
    set_global_hooks(new_hooks);
}
#[export_name = "cJSON_InitHooks"]
pub unsafe extern "C" fn cJSON_InitHooks_ffi(mut hooks: *mut cJSON_Hooks) {
    cJSON_InitHooks(hooks.as_ref())
}
fn cJSON_New_Item(
    hooks: &internal_hooks,
    initialize: impl FnOnce(&mut cJSON),
) -> Option<::core::ptr::NonNull<cJSON>> {
    let node = unsafe {
        hooks.allocate.expect("non-null function pointer")(::core::mem::size_of::<cJSON>() as size_t)
    } as *mut cJSON;
    let node = ::core::ptr::NonNull::new(node)?;
    let mut item = cJSON {
        next: ::core::ptr::null_mut::<cJSON>(),
        prev: ::core::ptr::null_mut::<cJSON>(),
        child: ::core::ptr::null_mut::<cJSON>(),
        type_0: 0 as ::core::ffi::c_int,
        valuestring: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        valueint: 0 as ::core::ffi::c_int,
        valuedouble: 0.0f64,
        string: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    initialize(&mut item);
    unsafe {
        node.as_ptr().write(item);
    }
    Some(node)
}
pub fn cJSON_Delete(mut item: *mut cJSON) {
    let mut next: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    unsafe {
        while !item.is_null() {
            next = (*item).next as *mut cJSON;
            if (*item).type_0 & cJSON_IsReference == 0 && !(*item).child.is_null() {
                cJSON_Delete((*item).child as *mut cJSON);
            }
            if (*item).type_0 & cJSON_IsReference == 0 && !(*item).valuestring.is_null() {
                get_global_hooks()
                    .deallocate
                    .expect("non-null function pointer")(
                    (*item).valuestring as *mut ::core::ffi::c_void,
                );
                (*item).valuestring = ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            if (*item).type_0 & cJSON_StringIsConst == 0 && !(*item).string.is_null() {
                get_global_hooks()
                    .deallocate
                    .expect("non-null function pointer")(
                    (*item).string as *mut ::core::ffi::c_void,
                );
                (*item).string = ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            get_global_hooks()
                .deallocate
                .expect("non-null function pointer")(item as *mut ::core::ffi::c_void);
            item = next;
        }
    }
}
#[export_name = "cJSON_Delete"]
pub unsafe extern "C" fn cJSON_Delete_ffi(mut item: *mut cJSON) {
    cJSON_Delete(item)
}
fn parse_number_prefix(number: &[::core::ffi::c_uchar]) -> Option<(::core::ffi::c_double, size_t)> {
    let number = ::std::str::from_utf8(number).ok()?;
    for end in (1..=number.len()).rev() {
        if let Ok(parsed) = number[..end].parse::<::core::ffi::c_double>() {
            return Some((parsed, end as size_t));
        }
    }
    None
}
fn parse_number(item: &mut cJSON, input_buffer: &mut parse_buffer<'_>) -> cJSON_bool {
    let mut i: size_t = 0 as size_t;
    let mut number_string_length: size_t = 0 as size_t;
    i = 0 as size_t;
    while input_buffer.offset.wrapping_add(i) < input_buffer.length {
        match input_buffer.content[input_buffer.offset.wrapping_add(i)] as ::core::ffi::c_int {
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 43 | 45 | 101 | 69 => {
                number_string_length = number_string_length.wrapping_add(1);
            }
            46 => {
                number_string_length = number_string_length.wrapping_add(1);
            }
            _ => {
                break;
            }
        }
        i = i.wrapping_add(1);
    }

    let Some(number_bytes) = input_buffer
        .content
        .get(input_buffer.offset..input_buffer.offset.wrapping_add(number_string_length))
    else {
        return false_0;
    };

    let Some((parsed_number, parsed_length)) = parse_number_prefix(number_bytes) else {
        return false_0;
    };
    let number = parsed_number;
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
pub fn cJSON_SetNumberHelper(
    object: &mut cJSON,
    number: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if number >= INT_MAX as ::core::ffi::c_double {
        object.valueint = INT_MAX;
    } else if number <= INT_MIN as ::core::ffi::c_double {
        object.valueint = INT_MIN;
    } else {
        object.valueint = number as ::core::ffi::c_int;
    }
    object.valuedouble = number;
    return object.valuedouble;
}
#[export_name = "cJSON_SetNumberHelper"]
pub unsafe extern "C" fn cJSON_SetNumberHelper_ffi(
    mut object: *mut cJSON,
    mut number: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let object = object
        .as_mut()
        .expect("cJSON_SetNumberHelper received a null object pointer");
    cJSON_SetNumberHelper(object, number)
}
pub fn cJSON_SetValuestring(
    object: &mut cJSON,
    valuestring: &::std::ffi::CStr,
) -> Option<::core::ptr::NonNull<::core::ffi::c_char>> {
    if object.type_0 & cJSON_String == 0 || object.type_0 & cJSON_IsReference != 0 {
        return None;
    }
    let current = ::core::ptr::NonNull::new(object.valuestring)?;
    let v1_len = valuestring.to_bytes().len();
    let current_cstr = unsafe { ::std::ffi::CStr::from_ptr(current.as_ptr()) };
    let v2_len = current_cstr.to_bytes().len();
    if v1_len <= v2_len {
        let valuestring_start = valuestring.as_ptr();
        let valuestring_end = valuestring_start.wrapping_add(v1_len);
        let current_start: *const ::core::ffi::c_char = current.as_ptr();
        let current_end = current_start.wrapping_add(v2_len);
        if !(valuestring_end < current_start || current_end < valuestring_start) {
            return None;
        }
        let bytes = valuestring.to_bytes_with_nul();
        unsafe {
            ::core::ptr::copy_nonoverlapping(
                bytes.as_ptr().cast::<::core::ffi::c_char>(),
                current.as_ptr(),
                bytes.len(),
            );
        }
        return Some(current);
    }
    let hooks = get_global_hooks();
    let copy = cJSON_strdup(Some(valuestring), &hooks)?;
    if !object.valuestring.is_null() {
        cJSON_free(object.valuestring as *mut ::core::ffi::c_void);
    }
    object.valuestring = copy.as_ptr();
    Some(copy)
}
#[export_name = "cJSON_SetValuestring"]
pub unsafe extern "C" fn cJSON_SetValuestring_ffi(
    mut object: *mut cJSON,
    mut valuestring: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if object.is_null() || valuestring.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let object = object
        .as_mut()
        .expect("cJSON_SetValuestring received a null object pointer");
    let valuestring = ::std::ffi::CStr::from_ptr(valuestring);
    cJSON_SetValuestring(object, valuestring)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<::core::ffi::c_char>)
}
fn ensure<'a, 'b>(
    p: &'a mut printbuffer<'b>,
    mut needed: size_t,
) -> Option<&'a mut [::core::ffi::c_uchar]> {
    let requested = needed;
    if needed > INT_MAX as size_t {
        return None;
    }
    needed = needed.wrapping_add(p.offset.wrapping_add(1 as size_t));
    match &mut p.buffer {
        printbuffer_buffer::Dynamic(buffer) => {
            if needed > buffer.len() {
                let newsize = if needed > (INT_MAX / 2 as ::core::ffi::c_int) as size_t {
                    if needed <= INT_MAX as size_t {
                        INT_MAX as size_t
                    } else {
                        return None;
                    }
                } else {
                    needed.wrapping_mul(2 as size_t)
                };
                if buffer
                    .try_reserve_exact(newsize.saturating_sub(buffer.len()))
                    .is_err()
                {
                    return None;
                }
                buffer.resize(newsize, 0);
            }
            buffer.get_mut(p.offset..p.offset.wrapping_add(requested))
        }
        printbuffer_buffer::Preallocated(buffer) => {
            if buffer.len() > 0 && p.offset >= buffer.len() {
                return None;
            }
            if needed > buffer.len() {
                return None;
            }
            buffer.get_mut(p.offset..p.offset.wrapping_add(requested))
        }
    }
}
fn printbuffer_bytes<'a, 'b>(buffer: &'a printbuffer<'b>) -> &'a [::core::ffi::c_uchar] {
    match &buffer.buffer {
        printbuffer_buffer::Dynamic(buffer) => buffer.as_slice(),
        printbuffer_buffer::Preallocated(buffer) => buffer,
    }
}
fn update_offset(buffer: &mut printbuffer<'_>) {
    let bytes = printbuffer_bytes(buffer);
    let Some(remaining) = bytes.get(buffer.offset..) else {
        return;
    };
    let string_length = bytes
        .get(buffer.offset..)
        .unwrap_or(&[])
        .iter()
        .position(|byte| *byte == b'\0')
        .unwrap_or(remaining.len());
    buffer.offset = buffer.offset.wrapping_add(string_length);
}
fn compare_double(mut a: ::core::ffi::c_double, mut b: ::core::ffi::c_double) -> cJSON_bool {
    let mut maxVal: ::core::ffi::c_double = if a.abs() > b.abs() { a.abs() } else { b.abs() };
    return ((a - b).abs() <= maxVal * DBL_EPSILON) as ::core::ffi::c_int;
}
fn trim_number_fraction(mut number: String) -> String {
    if let Some(exponent_start) = number.find('e') {
        let exponent = number.split_off(exponent_start);
        if number.contains('.') {
            while number.ends_with('0') {
                number.pop();
            }
            if number.ends_with('.') {
                number.pop();
            }
        }
        number.push_str(&exponent);
        number
    } else {
        if number.contains('.') {
            while number.ends_with('0') {
                number.pop();
            }
            if number.ends_with('.') {
                number.pop();
            }
        }
        number
    }
}
fn normalize_number_exponent(number: String) -> String {
    let Some(exponent_start) = number.find('e') else {
        return number;
    };
    let mantissa = &number[..exponent_start];
    let exponent = &number[exponent_start + 1..];
    let parsed_exponent = exponent.parse::<::core::ffi::c_int>().unwrap_or(0);
    let abs_exponent = parsed_exponent.abs();
    let mut normalized = String::new();
    normalized.push_str(mantissa);
    normalized.push('e');
    normalized.push(if parsed_exponent < 0 { '-' } else { '+' });
    if abs_exponent < 10 {
        normalized.push('0');
    }
    normalized.push_str(&abs_exponent.to_string());
    normalized
}
fn print_number(item: &cJSON, output_buffer: &mut printbuffer) -> cJSON_bool {
    let mut d: ::core::ffi::c_double = item.valuedouble;
    macro_rules! format_number_g {
        ($number:expr, $precision:expr) => {{
            let number = $number;
            let precision = $precision;
            if number == 0.0 {
                "0".to_string()
            } else {
                let exponent = number.abs().log10().floor() as ::core::ffi::c_int;
                let use_exponent = exponent < -4 || exponent >= precision as ::core::ffi::c_int;
                let mut formatted = if use_exponent {
                    format!("{:.*e}", precision.saturating_sub(1), number)
                } else {
                    let fraction_digits = if exponent >= 0 {
                        precision.saturating_sub(exponent as usize + 1)
                    } else {
                        precision
                            .saturating_sub(1)
                            .wrapping_add((-exponent) as usize)
                    };
                    format!("{:.*}", fraction_digits, number)
                };
                formatted = trim_number_fraction(formatted);
                normalize_number_exponent(formatted)
            }
        }};
    }
    let mut number_bytes = if d != d || d - d != d - d && !(d != d) {
        b"null".to_vec()
    } else if d == item.valueint as ::core::ffi::c_double {
        let integer = item.valueint.to_string();
        integer.into_bytes()
    } else {
        let formatted = format_number_g!(d, 15usize);
        let parsed = formatted.parse::<::core::ffi::c_double>();
        let selected = if parsed
            .map(|parsed| compare_double(parsed, d))
            .unwrap_or(false_0)
            == 0
        {
            format_number_g!(d, 17usize)
        } else {
            formatted
        };
        selected.into_bytes()
    };
    let length = number_bytes.len();
    number_bytes.push(b'\0');
    if length
        > (::core::mem::size_of::<[::core::ffi::c_uchar; 26]>() as usize).wrapping_sub(1 as usize)
    {
        return false_0;
    }
    let Some(output) = ensure(output_buffer, number_bytes.len() as size_t) else {
        return false_0;
    };
    output.copy_from_slice(&number_bytes);
    output_buffer.offset = output_buffer.offset.wrapping_add(length as size_t);
    return true_0;
}
fn parse_hex4(input: &[::core::ffi::c_uchar]) -> ::core::ffi::c_uint {
    let mut h: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut i: size_t = 0 as size_t;
    i = 0 as size_t;
    while i < 4 as size_t {
        let digit = input[i as usize];
        if digit as ::core::ffi::c_int >= '0' as i32 && digit as ::core::ffi::c_int <= '9' as i32 {
            h = h.wrapping_add(
                (digit as ::core::ffi::c_uint).wrapping_sub('0' as i32 as ::core::ffi::c_uint),
            );
        } else if digit as ::core::ffi::c_int >= 'A' as i32
            && digit as ::core::ffi::c_int <= 'F' as i32
        {
            h = h.wrapping_add(
                (10 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_add(digit as ::core::ffi::c_uint)
                    .wrapping_sub('A' as i32 as ::core::ffi::c_uint),
            );
        } else if digit as ::core::ffi::c_int >= 'a' as i32
            && digit as ::core::ffi::c_int <= 'f' as i32
        {
            h = h.wrapping_add(
                (10 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_add(digit as ::core::ffi::c_uint)
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
) -> Option<(
    ::core::ffi::c_uchar,
    [::core::ffi::c_uchar; 4],
    ::core::ffi::c_uchar,
)> {
    if input.len() < 6 {
        return None;
    }
    let first_code = parse_hex4(&input[2..6]);
    if first_code >= 0xdc00 as ::core::ffi::c_uint && first_code <= 0xdfff as ::core::ffi::c_uint {
        return None;
    }
    let (mut codepoint, sequence_length) = if first_code >= 0xd800 as ::core::ffi::c_uint
        && first_code <= 0xdbff as ::core::ffi::c_uint
    {
        if input.len() < 12
            || input[6] as ::core::ffi::c_int != '\\' as i32
            || input[7] as ::core::ffi::c_int != 'u' as i32
        {
            return None;
        }
        let second_code = parse_hex4(&input[8..12]);
        if second_code < 0xdc00 as ::core::ffi::c_uint
            || second_code > 0xdfff as ::core::ffi::c_uint
        {
            return None;
        }
        (
            (0x10000 as ::core::ffi::c_int as ::core::ffi::c_uint).wrapping_add(
                (first_code & 0x3ff as ::core::ffi::c_uint) << 10 as ::core::ffi::c_int
                    | second_code & 0x3ff as ::core::ffi::c_uint,
            ) as ::core::ffi::c_ulong,
            12 as ::core::ffi::c_uchar,
        )
    } else {
        (
            first_code as ::core::ffi::c_ulong,
            6 as ::core::ffi::c_uchar,
        )
    };
    let (utf8_length, first_byte_mark) = if codepoint < 0x80 as ::core::ffi::c_ulong {
        (1 as ::core::ffi::c_uchar, 0 as ::core::ffi::c_uchar)
    } else if codepoint < 0x800 as ::core::ffi::c_ulong {
        (2 as ::core::ffi::c_uchar, 0xc0 as ::core::ffi::c_uchar)
    } else if codepoint < 0x10000 as ::core::ffi::c_ulong {
        (3 as ::core::ffi::c_uchar, 0xe0 as ::core::ffi::c_uchar)
    } else if codepoint <= 0x10ffff as ::core::ffi::c_ulong {
        (4 as ::core::ffi::c_uchar, 0xf0 as ::core::ffi::c_uchar)
    } else {
        return None;
    };
    let mut output = [0 as ::core::ffi::c_uchar; 4];
    let mut utf8_position =
        (utf8_length as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    while utf8_position as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        output[utf8_position as usize] = ((codepoint | 0x80 as ::core::ffi::c_ulong)
            & 0xbf as ::core::ffi::c_ulong)
            as ::core::ffi::c_uchar;
        codepoint >>= 6 as ::core::ffi::c_int;
        utf8_position = utf8_position.wrapping_sub(1);
    }
    if utf8_length as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
        output[0] = ((codepoint | first_byte_mark as ::core::ffi::c_ulong)
            & 0xff as ::core::ffi::c_ulong) as ::core::ffi::c_uchar;
    } else {
        output[0] = (codepoint & 0x7f as ::core::ffi::c_ulong) as ::core::ffi::c_uchar;
    }
    Some((sequence_length, output, utf8_length))
}
fn parse_string(item: &mut cJSON, input_buffer: &mut parse_buffer<'_>) -> cJSON_bool {
    let content = input_buffer.content;
    let start = input_buffer.offset;
    let failure_offset = start.wrapping_add(1 as size_t);
    if content.get(start) != Some(&b'"') {
        input_buffer.offset = failure_offset;
        return false_0;
    }

    let mut input_end = start.wrapping_add(1 as size_t);
    let mut skipped_bytes = 0 as size_t;
    while input_end < input_buffer.length && content[input_end] != b'"' {
        if content[input_end] == b'\\' {
            if input_end.wrapping_add(1 as size_t) >= input_buffer.length {
                input_buffer.offset = failure_offset;
                return false_0;
            }
            skipped_bytes = skipped_bytes.wrapping_add(1 as size_t);
            input_end = input_end.wrapping_add(1 as size_t);
        }
        input_end = input_end.wrapping_add(1 as size_t);
    }
    if input_end >= input_buffer.length || content[input_end] != b'"' {
        input_buffer.offset = failure_offset;
        return false_0;
    }

    let allocation_length = input_end.wrapping_sub(start).wrapping_sub(skipped_bytes);
    let output_size = allocation_length
        .wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 1]>() as size_t);
    let output = unsafe {
        input_buffer
            .hooks
            .allocate
            .expect("non-null function pointer")(output_size)
    } as *mut ::core::ffi::c_uchar;
    let Some(output) = ::core::ptr::NonNull::new(output) else {
        input_buffer.offset = failure_offset;
        return false_0;
    };

    let mut output_bytes = ::std::vec::Vec::new();
    if output_bytes.try_reserve_exact(output_size).is_err() {
        unsafe {
            input_buffer
                .hooks
                .deallocate
                .expect("non-null function pointer")(
                output.as_ptr() as *mut ::core::ffi::c_void
            );
        }
        input_buffer.offset = failure_offset;
        return false_0;
    }

    let mut input_position = start.wrapping_add(1 as size_t);
    while input_position < input_end {
        if content[input_position] != b'\\' {
            output_bytes.push(content[input_position]);
            input_position = input_position.wrapping_add(1 as size_t);
            continue;
        }

        if input_position.wrapping_add(1 as size_t) >= input_end {
            unsafe {
                input_buffer
                    .hooks
                    .deallocate
                    .expect("non-null function pointer")(
                    output.as_ptr() as *mut ::core::ffi::c_void
                );
            }
            input_buffer.offset = input_position;
            return false_0;
        }

        let mut sequence_length = 2 as size_t;
        match content[input_position.wrapping_add(1 as size_t)] {
            b'b' => output_bytes.push('\u{8}' as u8),
            b'f' => output_bytes.push('\u{c}' as u8),
            b'n' => output_bytes.push(b'\n'),
            b'r' => output_bytes.push(b'\r'),
            b't' => output_bytes.push(b'\t'),
            b'"' | b'\\' | b'/' => {
                output_bytes.push(content[input_position.wrapping_add(1 as size_t)]);
            }
            b'u' => {
                let Some((decoded_length, utf8_bytes, utf8_length)) =
                    utf16_literal_to_utf8(&content[input_position..input_end])
                else {
                    unsafe {
                        input_buffer
                            .hooks
                            .deallocate
                            .expect("non-null function pointer")(
                            output.as_ptr() as *mut ::core::ffi::c_void,
                        );
                    }
                    input_buffer.offset = input_position;
                    return false_0;
                };
                sequence_length = decoded_length as size_t;
                output_bytes.extend_from_slice(&utf8_bytes[..utf8_length as usize]);
            }
            _ => {
                unsafe {
                    input_buffer
                        .hooks
                        .deallocate
                        .expect("non-null function pointer")(
                        output.as_ptr() as *mut ::core::ffi::c_void
                    );
                }
                input_buffer.offset = input_position;
                return false_0;
            }
        }
        input_position = input_position.wrapping_add(sequence_length);
    }

    output_bytes.push(b'\0');
    unsafe {
        ::core::ptr::copy_nonoverlapping(
            output_bytes.as_ptr(),
            output.as_ptr(),
            output_bytes.len(),
        );
    }
    item.type_0 = cJSON_String;
    item.valuestring = output.as_ptr() as *mut ::core::ffi::c_char;
    input_buffer.offset = input_end.wrapping_add(1 as size_t);
    return true_0;
}
fn print_string_ptr(
    input: *const ::core::ffi::c_uchar,
    output_buffer: &mut printbuffer,
) -> cJSON_bool {
    let input_bytes = if input.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(input.cast::<::core::ffi::c_char>()) }.to_bytes())
    };
    let output_length = input_bytes
        .map(|input_bytes| {
            input_bytes.iter().fold(3usize, |length, byte| {
                length
                    + match *byte {
                        b'"' | b'\\' | 8 | 12 | b'\n' | b'\r' | b'\t' => 2,
                        0..=31 => 6,
                        _ => 1,
                    }
            })
        })
        .unwrap_or(3usize);
    let mut output_bytes = ::std::vec::Vec::new();
    if output_bytes.try_reserve_exact(output_length).is_err() {
        return false_0;
    }
    output_bytes.push(b'"');
    if let Some(input_bytes) = input_bytes {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        for byte in input_bytes.iter().copied() {
            if byte > 31 && byte != b'"' && byte != b'\\' {
                output_bytes.push(byte);
                continue;
            }
            output_bytes.push(b'\\');
            match byte {
                b'\\' => output_bytes.push(b'\\'),
                b'"' => output_bytes.push(b'"'),
                8 => output_bytes.push(b'b'),
                12 => output_bytes.push(b'f'),
                b'\n' => output_bytes.push(b'n'),
                b'\r' => output_bytes.push(b'r'),
                b'\t' => output_bytes.push(b't'),
                _ => {
                    output_bytes.push(b'u');
                    output_bytes.push(b'0');
                    output_bytes.push(b'0');
                    output_bytes.push(HEX[(byte >> 4) as usize]);
                    output_bytes.push(HEX[(byte & 0x0f) as usize]);
                }
            }
        }
        output_bytes.push(b'"');
    } else {
        output_bytes.push(b'"');
    }
    output_bytes.push(b'\0');
    let Some(output) = ensure(output_buffer, output_bytes.len() as size_t) else {
        return false_0;
    };
    output.copy_from_slice(&output_bytes);
    return true_0;
}
fn print_string(item: &cJSON, p: &mut printbuffer) -> cJSON_bool {
    return print_string_ptr(item.valuestring as *mut ::core::ffi::c_uchar, p);
}
fn buffer_skip_whitespace(buffer: &mut parse_buffer<'_>) {
    if buffer.offset.wrapping_add(0 as size_t) >= buffer.length {
        return;
    }
    while buffer.offset.wrapping_add(0 as size_t) < buffer.length
        && buffer.content[buffer.offset] as ::core::ffi::c_int <= 32 as ::core::ffi::c_int
    {
        buffer.offset = buffer.offset.wrapping_add(1);
    }
    if buffer.offset == buffer.length {
        buffer.offset = buffer.offset.wrapping_sub(1);
    }
}
fn buffer_starts_with(buffer: &parse_buffer<'_>, prefix: &[::core::ffi::c_uchar]) -> bool {
    buffer
        .content
        .get(buffer.offset..)
        .is_some_and(|remaining| remaining.starts_with(prefix))
}
fn skip_utf8_bom(content: &[::core::ffi::c_uchar], offset: &mut size_t) {
    if *offset == 0 as size_t
        && (*offset).wrapping_add(4 as size_t) < content.len()
        && content.get(..3) == Some(b"\xEF\xBB\xBF")
    {
        *offset = (*offset).wrapping_add(3 as size_t);
    }
}
fn cJSON_ParseWithOpts(
    value: Option<&::std::ffi::CStr>,
    require_null_terminated: cJSON_bool,
) -> (Option<::core::ptr::NonNull<cJSON>>, Option<size_t>) {
    let value_addr = value
        .map(|value| value.as_ptr().addr())
        .unwrap_or(0 as size_t);
    let value = value.map(|value| value.to_bytes_with_nul());
    cJSON_ParseWithLengthOpts(value, value_addr, require_null_terminated)
}
#[export_name = "cJSON_ParseWithOpts"]
pub unsafe extern "C" fn cJSON_ParseWithOpts_ffi(
    mut value: *const ::core::ffi::c_char,
    mut return_parse_end: *mut *const ::core::ffi::c_char,
    mut require_null_terminated: cJSON_bool,
) -> *mut cJSON {
    let value = if value.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(value))
    };
    let result = cJSON_ParseWithOpts(value, require_null_terminated);
    if let (Some(offset), Some(return_parse_end)) = (result.1, return_parse_end.as_mut()) {
        *return_parse_end = value
            .map(::std::ffi::CStr::as_ptr)
            .unwrap_or_else(::core::ptr::null::<::core::ffi::c_char>)
            .wrapping_add(offset);
    }
    result
        .0
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
fn cJSON_ParseWithLengthOpts(
    value: Option<&[::core::ffi::c_uchar]>,
    value_addr: size_t,
    require_null_terminated: cJSON_bool,
) -> (Option<::core::ptr::NonNull<cJSON>>, Option<size_t>) {
    let mut c2rust_current_block: u64;
    let mut buffer: parse_buffer = parse_buffer {
        content: &[],
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
    GLOBAL_ERROR_JSON_ADDR.store(0 as size_t, ::core::sync::atomic::Ordering::Relaxed);
    GLOBAL_ERROR_POSITION.store(0 as size_t, ::core::sync::atomic::Ordering::Relaxed);
    if let Some(value) = value {
        buffer.content = value;
        buffer.length = value.len();
        buffer.offset = 0 as size_t;
        buffer.hooks = get_global_hooks();
        if !buffer.content.is_empty() {
            let mut parsed = false;
            let hooks = buffer.hooks;
            item = cJSON_New_Item(&hooks, |item| {
                skip_utf8_bom(buffer.content, &mut buffer.offset);
                buffer_skip_whitespace(&mut buffer);
                parsed = parse_value(item, &mut buffer) != 0;
            });
            if item.is_some() {
                if parsed {
                    if require_null_terminated != 0 {
                        buffer_skip_whitespace(&mut buffer);
                        if buffer.offset >= buffer.length
                            || buffer.content[buffer.offset] as ::core::ffi::c_int != '\0' as i32
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
                            return (item, Some(buffer.offset));
                        }
                    }
                }
            }
        }
    }
    if let Some(item) = item {
        cJSON_Delete(item.as_ptr());
    }
    if value.is_some() {
        let mut local_error: error = error {
            json_addr: value_addr,
            position: 0,
        };
        local_error.position = 0 as size_t;
        if buffer.offset < buffer.length {
            local_error.position = buffer.offset;
        } else if buffer.length > 0 as size_t {
            local_error.position = buffer.length.wrapping_sub(1 as size_t);
        }
        GLOBAL_ERROR_JSON_ADDR.store(
            local_error.json_addr,
            ::core::sync::atomic::Ordering::Relaxed,
        );
        GLOBAL_ERROR_POSITION.store(
            local_error.position,
            ::core::sync::atomic::Ordering::Relaxed,
        );
        return (None, Some(local_error.position));
    }
    (None, None)
}
#[export_name = "cJSON_ParseWithLengthOpts"]
pub unsafe extern "C" fn cJSON_ParseWithLengthOpts_ffi(
    mut value: *const ::core::ffi::c_char,
    mut buffer_length: size_t,
    mut return_parse_end: *mut *const ::core::ffi::c_char,
    mut require_null_terminated: cJSON_bool,
) -> *mut cJSON {
    let value_slice = if value.is_null() {
        None
    } else if buffer_length == 0 {
        Some(&[][..])
    } else {
        Some(::core::slice::from_raw_parts(
            value as *const ::core::ffi::c_uchar,
            buffer_length,
        ))
    };
    let result = cJSON_ParseWithLengthOpts(value_slice, value.addr(), require_null_terminated);
    if let (Some(offset), Some(return_parse_end)) = (result.1, return_parse_end.as_mut()) {
        *return_parse_end = value.wrapping_add(offset);
    }
    result
        .0
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
#[export_name = "cJSON_Parse"]
pub unsafe extern "C" fn cJSON_Parse_ffi(mut value: *const ::core::ffi::c_char) -> *mut cJSON {
    let value = if value.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(value))
    };
    cJSON_ParseWithOpts(value, 0 as cJSON_bool)
        .0
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
#[export_name = "cJSON_ParseWithLength"]
pub unsafe extern "C" fn cJSON_ParseWithLength_ffi(
    mut value: *const ::core::ffi::c_char,
    mut buffer_length: size_t,
) -> *mut cJSON {
    let value_slice = if value.is_null() {
        None
    } else if buffer_length == 0 {
        Some(&[][..])
    } else {
        Some(::core::slice::from_raw_parts(
            value as *const ::core::ffi::c_uchar,
            buffer_length,
        ))
    };
    cJSON_ParseWithLengthOpts(value_slice, value.addr(), 0 as cJSON_bool)
        .0
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
fn print(
    item: Option<&cJSON>,
    mut format: cJSON_bool,
    hooks: &internal_hooks,
) -> *mut ::core::ffi::c_uchar {
    const DEFAULT_BUFFER_SIZE: size_t = 256 as size_t;
    let mut backing = ::std::vec::Vec::new();
    if backing.try_reserve_exact(DEFAULT_BUFFER_SIZE).is_err() {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    backing.resize(DEFAULT_BUFFER_SIZE, 0);
    let mut buffer = printbuffer {
        buffer: printbuffer_buffer::Dynamic(backing),
        offset: 0,
        depth: 0,
        format,
    };
    let Some(item) = item else {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    };
    if print_value(item, &mut buffer) == 0 {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    update_offset(&mut buffer);
    let Some(end) = buffer.offset.checked_add(1) else {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    };
    let Some(bytes) = printbuffer_bytes(&buffer).get(..end) else {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    };
    let Ok(printed) = ::std::ffi::CStr::from_bytes_with_nul(bytes) else {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    };
    match cJSON_strdup(Some(printed), hooks) {
        Some(printed) => printed.as_ptr().cast::<::core::ffi::c_uchar>(),
        None => ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
    }
}
#[export_name = "cJSON_Print"]
pub unsafe extern "C" fn cJSON_Print_ffi(mut item: *const cJSON) -> *mut ::core::ffi::c_char {
    let hooks = get_global_hooks();
    print(item.as_ref(), true_0, &hooks) as *mut ::core::ffi::c_char
}
#[export_name = "cJSON_PrintUnformatted"]
pub unsafe extern "C" fn cJSON_PrintUnformatted_ffi(
    mut item: *const cJSON,
) -> *mut ::core::ffi::c_char {
    let hooks = get_global_hooks();
    print(item.as_ref(), false_0, &hooks) as *mut ::core::ffi::c_char
}
pub fn cJSON_PrintBuffered(
    item: Option<&cJSON>,
    prebuffer: ::core::ffi::c_int,
    fmt: cJSON_bool,
) -> Option<::core::ptr::NonNull<::core::ffi::c_char>> {
    if prebuffer < 0 as ::core::ffi::c_int {
        return None;
    }
    let hooks = get_global_hooks();
    let mut backing = ::std::vec::Vec::new();
    if backing.try_reserve_exact(prebuffer as usize).is_err() {
        return None;
    }
    backing.resize(prebuffer as usize, 0);
    let mut p = printbuffer {
        buffer: printbuffer_buffer::Dynamic(backing),
        offset: 0 as size_t,
        depth: 0 as size_t,
        format: fmt,
    };
    let Some(item) = item else {
        return None;
    };
    if print_value(item, &mut p) == 0 {
        return None;
    }
    update_offset(&mut p);
    let end = p.offset.checked_add(1)?;
    let bytes = printbuffer_bytes(&p).get(..end)?;
    let printed = ::std::ffi::CStr::from_bytes_with_nul(bytes).ok()?;
    cJSON_strdup(Some(printed), &hooks)
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
    cJSON_PrintBuffered(item.as_ref(), prebuffer, fmt)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<::core::ffi::c_char>)
}
pub fn cJSON_PrintPreallocated(
    item: Option<&cJSON>,
    buffer: Option<&mut [::core::ffi::c_uchar]>,
    format: cJSON_bool,
) -> cJSON_bool {
    let Some(buffer) = buffer else {
        return false_0;
    };
    let mut p = printbuffer {
        buffer: printbuffer_buffer::Preallocated(buffer),
        offset: 0 as size_t,
        depth: 0 as size_t,
        format,
    };
    let Some(item) = item else {
        return false_0;
    };
    print_value(item, &mut p)
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
    let buffer =
        ::core::slice::from_raw_parts_mut(buffer.cast::<::core::ffi::c_uchar>(), length as usize);
    cJSON_PrintPreallocated(item.as_ref(), Some(buffer), format)
}
fn parse_value(item: &mut cJSON, input_buffer: &mut parse_buffer<'_>) -> cJSON_bool {
    if buffer_starts_with(input_buffer, b"null") {
        item.type_0 = cJSON_NULL;
        input_buffer.offset = input_buffer.offset.wrapping_add(4 as size_t);
        return true_0;
    }
    if buffer_starts_with(input_buffer, b"false") {
        item.type_0 = cJSON_False;
        input_buffer.offset = input_buffer.offset.wrapping_add(5 as size_t);
        return true_0;
    }
    if buffer_starts_with(input_buffer, b"true") {
        item.type_0 = cJSON_True;
        item.valueint = 1 as ::core::ffi::c_int;
        input_buffer.offset = input_buffer.offset.wrapping_add(4 as size_t);
        return true_0;
    }
    if input_buffer.offset.wrapping_add(0 as size_t) < input_buffer.length
        && input_buffer.content[input_buffer.offset] as ::core::ffi::c_int == '"' as i32
    {
        return parse_string(item, input_buffer);
    }
    if input_buffer.offset.wrapping_add(0 as size_t) < input_buffer.length
        && (input_buffer.content[input_buffer.offset] as ::core::ffi::c_int == '-' as i32
            || input_buffer.content[input_buffer.offset] as ::core::ffi::c_int >= '0' as i32
                && input_buffer.content[input_buffer.offset] as ::core::ffi::c_int <= '9' as i32)
    {
        return parse_number(item, input_buffer);
    }
    if input_buffer.offset.wrapping_add(0 as size_t) < input_buffer.length
        && input_buffer.content[input_buffer.offset] as ::core::ffi::c_int == '[' as i32
    {
        return parse_array(item, input_buffer);
    }
    if input_buffer.offset.wrapping_add(0 as size_t) < input_buffer.length
        && input_buffer.content[input_buffer.offset] as ::core::ffi::c_int == '{' as i32
    {
        return parse_object(item, input_buffer);
    }
    return false_0;
}
fn print_value(item: &cJSON, output_buffer: &mut printbuffer) -> cJSON_bool {
    match item.type_0 & 0xff as ::core::ffi::c_int {
        cJSON_NULL => {
            let Some(output) = ensure(output_buffer, 5 as size_t) else {
                return false_0;
            };
            output.copy_from_slice(b"null\0");
            return true_0;
        }
        cJSON_False => {
            let Some(output) = ensure(output_buffer, 6 as size_t) else {
                return false_0;
            };
            output.copy_from_slice(b"false\0");
            return true_0;
        }
        cJSON_True => {
            let Some(output) = ensure(output_buffer, 5 as size_t) else {
                return false_0;
            };
            output.copy_from_slice(b"true\0");
            return true_0;
        }
        cJSON_Number => return print_number(item, output_buffer),
        cJSON_Raw => {
            if item.valuestring.is_null() {
                return false_0;
            }
            let raw = unsafe { ::std::ffi::CStr::from_ptr(item.valuestring) }.to_bytes_with_nul();
            let Some(output) = ensure(output_buffer, raw.len() as size_t) else {
                return false_0;
            };
            output.copy_from_slice(raw);
            return true_0;
        }
        cJSON_String => return print_string(item, output_buffer),
        cJSON_Array => return print_array(item, output_buffer),
        cJSON_Object => return print_object(item, output_buffer),
        _ => return false_0,
    };
}
fn parse_array(item: &mut cJSON, input_buffer: &mut parse_buffer<'_>) -> cJSON_bool {
    let mut c2rust_current_block: u64;
    let mut head: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut current_item: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if input_buffer.depth >= CJSON_NESTING_LIMIT as size_t {
        return false_0;
    }
    input_buffer.depth = input_buffer.depth.wrapping_add(1);
    if !(input_buffer.content[input_buffer.offset] as ::core::ffi::c_int != '[' as i32) {
        input_buffer.offset = input_buffer.offset.wrapping_add(1);
        buffer_skip_whitespace(input_buffer);
        if input_buffer.offset.wrapping_add(0 as size_t) < input_buffer.length
            && input_buffer.content[input_buffer.offset] as ::core::ffi::c_int == ']' as i32
        {
            c2rust_current_block = 6773356538935231690;
        } else if !(input_buffer.offset.wrapping_add(0 as size_t) < input_buffer.length) {
            input_buffer.offset = input_buffer.offset.wrapping_sub(1);
            c2rust_current_block = 1336238348363633231;
        } else {
            input_buffer.offset = input_buffer.offset.wrapping_sub(1);
            loop {
                let Some(new_item) = cJSON_New_Item(&input_buffer.hooks, |_| {}) else {
                    c2rust_current_block = 1336238348363633231;
                    break;
                };
                let new_item = new_item.as_ptr();
                if head.is_null() {
                    head = new_item;
                    current_item = head;
                } else {
                    unsafe {
                        (*current_item).next = new_item as *mut cJSON;
                        (*new_item).prev = current_item as *mut cJSON;
                    }
                    current_item = new_item;
                }
                input_buffer.offset = input_buffer.offset.wrapping_add(1);
                buffer_skip_whitespace(input_buffer);
                if parse_value(
                    unsafe { current_item.as_mut() }
                        .expect("parse_array received a null current item"),
                    input_buffer,
                ) == 0
                {
                    c2rust_current_block = 1336238348363633231;
                    break;
                }
                buffer_skip_whitespace(input_buffer);
                if !(input_buffer.offset.wrapping_add(0 as size_t) < input_buffer.length
                    && input_buffer.content[input_buffer.offset] as ::core::ffi::c_int
                        == ',' as i32)
                {
                    c2rust_current_block = 15089075282327824602;
                    break;
                }
            }
            match c2rust_current_block {
                1336238348363633231 => {}
                _ => {
                    if !(input_buffer.offset.wrapping_add(0 as size_t) < input_buffer.length)
                        || input_buffer.content[input_buffer.offset] as ::core::ffi::c_int
                            != ']' as i32
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
                input_buffer.depth = input_buffer.depth.wrapping_sub(1);
                if !head.is_null() {
                    unsafe {
                        (*head).prev = current_item as *mut cJSON;
                    }
                }
                item.type_0 = cJSON_Array;
                item.child = head as *mut cJSON;
                input_buffer.offset = input_buffer.offset.wrapping_add(1);
                return true_0;
            }
        }
    }
    if !head.is_null() {
        cJSON_Delete(head);
    }
    return false_0;
}
fn print_array(item: &cJSON, output_buffer: &mut printbuffer) -> cJSON_bool {
    let mut length: size_t = 0 as size_t;
    let mut current_element: *mut cJSON = item.child as *mut cJSON;
    let Some(output) = ensure(output_buffer, 1 as size_t) else {
        return false_0;
    };
    output[0] = b'[';
    output_buffer.offset = output_buffer.offset.wrapping_add(1);
    output_buffer.depth = output_buffer.depth.wrapping_add(1);
    while !current_element.is_null() {
        if print_value(unsafe { &*current_element }, output_buffer) == 0 {
            return false_0;
        }
        update_offset(output_buffer);
        if unsafe { !(*current_element).next.is_null() } {
            length = (if output_buffer.format != 0 {
                2 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as size_t;
            let format = output_buffer.format != 0;
            let Some(output) = ensure(output_buffer, length.wrapping_add(1 as size_t)) else {
                return false_0;
            };
            output[0] = b',';
            let mut written = 1usize;
            if format {
                output[written] = b' ';
                written += 1;
            }
            output[written] = b'\0';
            output_buffer.offset = output_buffer.offset.wrapping_add(length);
        }
        current_element = unsafe { (*current_element).next as *mut cJSON };
    }
    let Some(output) = ensure(output_buffer, 2 as size_t) else {
        return false_0;
    };
    output[0] = b']';
    output[1] = b'\0';
    output_buffer.depth = output_buffer.depth.wrapping_sub(1);
    return true_0;
}
fn parse_object(item: &mut cJSON, input_buffer: &mut parse_buffer<'_>) -> cJSON_bool {
    let mut c2rust_current_block: u64;
    let mut head: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut current_item: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if input_buffer.depth >= CJSON_NESTING_LIMIT as size_t {
        return false_0;
    }
    input_buffer.depth = input_buffer.depth.wrapping_add(1);
    if !(!(input_buffer.offset.wrapping_add(0 as size_t) < input_buffer.length)
        || input_buffer.content[input_buffer.offset] as ::core::ffi::c_int != '{' as i32)
    {
        input_buffer.offset = input_buffer.offset.wrapping_add(1);
        buffer_skip_whitespace(input_buffer);
        if input_buffer.offset.wrapping_add(0 as size_t) < input_buffer.length
            && input_buffer.content[input_buffer.offset] as ::core::ffi::c_int == '}' as i32
        {
            c2rust_current_block = 4359236900545362719;
        } else if !(input_buffer.offset.wrapping_add(0 as size_t) < input_buffer.length) {
            input_buffer.offset = input_buffer.offset.wrapping_sub(1);
            c2rust_current_block = 9990476168629568694;
        } else {
            input_buffer.offset = input_buffer.offset.wrapping_sub(1);
            loop {
                let Some(new_item) = cJSON_New_Item(&input_buffer.hooks, |_| {}) else {
                    c2rust_current_block = 9990476168629568694;
                    break;
                };
                let new_item = new_item.as_ptr();
                {
                    if head.is_null() {
                        head = new_item;
                        current_item = head;
                    } else {
                        unsafe {
                            (*current_item).next = new_item as *mut cJSON;
                            (*new_item).prev = current_item as *mut cJSON;
                        }
                        current_item = new_item;
                    }
                    if !(input_buffer.offset.wrapping_add(1 as size_t) < input_buffer.length) {
                        c2rust_current_block = 9990476168629568694;
                        break;
                    } else {
                        input_buffer.offset = input_buffer.offset.wrapping_add(1);
                        buffer_skip_whitespace(input_buffer);
                        if parse_string(
                            unsafe { current_item.as_mut() }
                                .expect("parse_object received a null current item"),
                            input_buffer,
                        ) == 0
                        {
                            c2rust_current_block = 9990476168629568694;
                            break;
                        } else {
                            buffer_skip_whitespace(input_buffer);
                            unsafe {
                                (*current_item).string = (*current_item).valuestring;
                                (*current_item).valuestring =
                                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                            }
                            if !(input_buffer.offset.wrapping_add(0 as size_t)
                                < input_buffer.length)
                                || input_buffer.content[input_buffer.offset] as ::core::ffi::c_int
                                    != ':' as i32
                            {
                                c2rust_current_block = 9990476168629568694;
                                break;
                            } else {
                                input_buffer.offset = input_buffer.offset.wrapping_add(1);
                                buffer_skip_whitespace(input_buffer);
                                if parse_value(
                                    unsafe { current_item.as_mut() }
                                        .expect("parse_object received a null current item"),
                                    input_buffer,
                                ) == 0
                                {
                                    c2rust_current_block = 9990476168629568694;
                                    break;
                                } else {
                                    buffer_skip_whitespace(input_buffer);
                                    if !(input_buffer.offset.wrapping_add(0 as size_t)
                                        < input_buffer.length
                                        && input_buffer.content[input_buffer.offset]
                                            as ::core::ffi::c_int
                                            == ',' as i32)
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
                    if !(input_buffer.offset.wrapping_add(0 as size_t) < input_buffer.length)
                        || input_buffer.content[input_buffer.offset] as ::core::ffi::c_int
                            != '}' as i32
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
                input_buffer.depth = input_buffer.depth.wrapping_sub(1);
                if !head.is_null() {
                    unsafe {
                        (*head).prev = current_item as *mut cJSON;
                    }
                }
                item.type_0 = cJSON_Object;
                item.child = head as *mut cJSON;
                input_buffer.offset = input_buffer.offset.wrapping_add(1);
                return true_0;
            }
        }
    }
    if !head.is_null() {
        cJSON_Delete(head);
    }
    return false_0;
}
fn print_object(item: &cJSON, output_buffer: &mut printbuffer) -> cJSON_bool {
    let mut length: size_t = 0 as size_t;
    let mut current_item: *mut cJSON = item.child as *mut cJSON;
    length = (if output_buffer.format != 0 {
        2 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    }) as size_t;
    let format = output_buffer.format != 0;
    let Some(output) = ensure(output_buffer, length.wrapping_add(1 as size_t)) else {
        return false_0;
    };
    output[0] = b'{';
    if format {
        output[1] = b'\n';
    }
    output_buffer.depth = output_buffer.depth.wrapping_add(1);
    output_buffer.offset = output_buffer.offset.wrapping_add(length);
    while !current_item.is_null() {
        if output_buffer.format != 0 {
            let depth = output_buffer.depth;
            let Some(output) = ensure(output_buffer, depth) else {
                return false_0;
            };
            output.fill(b'\t');
            output_buffer.offset = output_buffer.offset.wrapping_add(depth);
        }
        let current_string = unsafe { (*current_item).string as *mut ::core::ffi::c_uchar };
        if print_string_ptr(current_string, output_buffer) == 0 {
            return false_0;
        }
        update_offset(output_buffer);
        length = (if output_buffer.format != 0 {
            2 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) as size_t;
        let format = output_buffer.format != 0;
        let Some(output) = ensure(output_buffer, length) else {
            return false_0;
        };
        output[0] = b':';
        if format {
            output[1] = b'\t';
        }
        output_buffer.offset = output_buffer.offset.wrapping_add(length);
        if print_value(unsafe { &*current_item }, output_buffer) == 0 {
            return false_0;
        }
        update_offset(output_buffer);
        let has_next = unsafe { !(*current_item).next.is_null() };
        length = ((if output_buffer.format != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as size_t)
            .wrapping_add((if has_next { 1 } else { 0 }) as size_t);
        let format = output_buffer.format != 0;
        let Some(output) = ensure(output_buffer, length.wrapping_add(1 as size_t)) else {
            return false_0;
        };
        let mut written = 0usize;
        if has_next {
            output[written] = b',';
            written += 1;
        }
        if format {
            output[written] = b'\n';
            written += 1;
        }
        output[written] = b'\0';
        unsafe {
            current_item = (*current_item).next as *mut cJSON;
        }
        output_buffer.offset = output_buffer.offset.wrapping_add(length);
    }
    let format = output_buffer.format != 0;
    let depth = output_buffer.depth;
    let needed = if format {
        depth.wrapping_add(1 as size_t)
    } else {
        2 as size_t
    };
    let Some(output) = ensure(output_buffer, needed) else {
        return false_0;
    };
    let mut written = 0usize;
    if format {
        let indent = depth.wrapping_sub(1 as size_t);
        output[..indent].fill(b'\t');
        written = indent;
    }
    output[written] = b'}';
    output[written + 1] = b'\0';
    output_buffer.depth = output_buffer.depth.wrapping_sub(1);
    return true_0;
}
pub fn cJSON_GetArraySize(array: Option<&cJSON>) -> ::core::ffi::c_int {
    let Some(array) = array else {
        return 0 as ::core::ffi::c_int;
    };
    let mut size: size_t = 0 as size_t;
    let mut child = array.child;
    unsafe {
        while !child.is_null() {
            size = size.wrapping_add(1);
            child = (*child).next as *mut cJSON;
        }
    }
    return size as ::core::ffi::c_int;
}
#[export_name = "cJSON_GetArraySize"]
pub unsafe extern "C" fn cJSON_GetArraySize_ffi(mut array: *const cJSON) -> ::core::ffi::c_int {
    cJSON_GetArraySize(array.as_ref())
}
fn get_array_item(array: Option<&cJSON>, mut index: size_t) -> Option<::core::ptr::NonNull<cJSON>> {
    let array = array?;
    let mut current_child = array.child;
    unsafe {
        while !current_child.is_null() && index > 0 as size_t {
            index = index.wrapping_sub(1);
            current_child = (*current_child).next as *mut cJSON;
        }
    }
    ::core::ptr::NonNull::new(current_child)
}
pub fn cJSON_GetArrayItem(
    array: Option<&cJSON>,
    index: ::core::ffi::c_int,
) -> Option<::core::ptr::NonNull<cJSON>> {
    if index < 0 as ::core::ffi::c_int {
        return None;
    }
    return get_array_item(array, index as size_t);
}
#[export_name = "cJSON_GetArrayItem"]
pub unsafe extern "C" fn cJSON_GetArrayItem_ffi(
    mut array: *const cJSON,
    mut index: ::core::ffi::c_int,
) -> *mut cJSON {
    cJSON_GetArrayItem(array.as_ref(), index)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
enum ObjectItemName<'a> {
    CStr(&'a ::std::ffi::CStr),
    ItemString(&'a cJSON),
}
fn get_object_item(
    object: &cJSON,
    name: Option<ObjectItemName<'_>>,
    case_sensitive: cJSON_bool,
) -> Option<::core::ptr::NonNull<cJSON>> {
    let name = name?;
    let mut current_element = object.child;
    unsafe {
        let name = match name {
            ObjectItemName::CStr(name) => name,
            ObjectItemName::ItemString(item) => {
                if item.string.is_null() {
                    return None;
                }
                ::std::ffi::CStr::from_ptr(item.string)
            }
        };
        while !current_element.is_null() {
            let current_string = (*current_element).string;
            if current_string.is_null() {
                if case_sensitive != 0 {
                    return None;
                }
            } else {
                let current_name = ::std::ffi::CStr::from_ptr(current_string);
                if if case_sensitive != 0 {
                    name.to_bytes_with_nul() == current_name.to_bytes_with_nul()
                } else {
                    case_insensitive_strcmp(name, current_name) == 0 as ::core::ffi::c_int
                } {
                    return ::core::ptr::NonNull::new(current_element);
                }
            }
            current_element = (*current_element).next as *mut cJSON;
        }
    }
    None
}
pub fn cJSON_GetObjectItem(
    object: Option<&cJSON>,
    string: Option<&::std::ffi::CStr>,
) -> Option<::core::ptr::NonNull<cJSON>> {
    get_object_item(object?, string.map(ObjectItemName::CStr), false_0)
}
#[export_name = "cJSON_GetObjectItem"]
pub unsafe extern "C" fn cJSON_GetObjectItem_ffi(
    object: *const cJSON,
    string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let string = if string.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(string))
    };
    cJSON_GetObjectItem(object.as_ref(), string)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_GetObjectItemCaseSensitive(
    object: Option<&cJSON>,
    string: Option<&::std::ffi::CStr>,
) -> Option<::core::ptr::NonNull<cJSON>> {
    get_object_item(object?, string.map(ObjectItemName::CStr), true_0)
}
#[export_name = "cJSON_GetObjectItemCaseSensitive"]
pub unsafe extern "C" fn cJSON_GetObjectItemCaseSensitive_ffi(
    object: *const cJSON,
    string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let string = if string.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(string))
    };
    cJSON_GetObjectItemCaseSensitive(object.as_ref(), string)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_HasObjectItem(
    object: Option<&cJSON>,
    string: Option<&::std::ffi::CStr>,
) -> cJSON_bool {
    return if cJSON_GetObjectItem(object, string).is_some() {
        1 as cJSON_bool
    } else {
        0 as cJSON_bool
    };
}
#[export_name = "cJSON_HasObjectItem"]
pub unsafe extern "C" fn cJSON_HasObjectItem_ffi(
    mut object: *const cJSON,
    mut string: *const ::core::ffi::c_char,
) -> cJSON_bool {
    let string = if string.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(string))
    };
    cJSON_HasObjectItem(object.as_ref(), string)
}
fn suffix_object(prev: ::core::ptr::NonNull<cJSON>, item: ::core::ptr::NonNull<cJSON>) {
    let prev = prev.as_ptr();
    let item = item.as_ptr();
    unsafe {
        (*prev).next = item;
        (*item).prev = prev;
    }
}
fn create_reference(
    item: Option<&cJSON>,
    hooks: &internal_hooks,
) -> Option<::core::ptr::NonNull<cJSON>> {
    let item = item?;
    cJSON_New_Item(hooks, |reference| {
        *reference = *item;
        reference.string = ::core::ptr::null_mut::<::core::ffi::c_char>();
        reference.type_0 |= cJSON_IsReference;
        reference.prev = ::core::ptr::null_mut::<cJSON>();
        reference.next = reference.prev;
    })
}
fn add_item_to_array(
    mut array: ::core::ptr::NonNull<cJSON>,
    mut item: ::core::ptr::NonNull<cJSON>,
) -> cJSON_bool {
    if array == item {
        return false_0;
    }
    let array = array.as_ptr();
    let item_ptr = item.as_ptr();
    unsafe {
        let child = (*array).child;
        if child.is_null() {
            (*array).child = item_ptr;
            (*item_ptr).prev = item_ptr;
            (*item_ptr).next = ::core::ptr::null_mut::<cJSON>();
            return true_0;
        }
        if !(*child).prev.is_null() {
            suffix_object(
                ::core::ptr::NonNull::new((*child).prev).expect("non-null previous array item"),
                item,
            );
            (*child).prev = item_ptr;
        }
    }
    true_0
}
pub fn cJSON_AddItemToArray(array: &mut cJSON, item: &mut cJSON) -> cJSON_bool {
    add_item_to_array(
        ::core::ptr::NonNull::from(array),
        ::core::ptr::NonNull::from(item),
    )
}
#[export_name = "cJSON_AddItemToArray"]
pub unsafe extern "C" fn cJSON_AddItemToArray_ffi(
    mut array: *mut cJSON,
    mut item: *mut cJSON,
) -> cJSON_bool {
    if array.is_null() || item.is_null() || array == item {
        return false_0;
    }
    let array = array
        .as_mut()
        .expect("cJSON_AddItemToArray received a null array pointer");
    let item = item
        .as_mut()
        .expect("cJSON_AddItemToArray received a null item pointer");
    cJSON_AddItemToArray(array, item)
}
fn add_item_to_object(
    object: &mut cJSON,
    string: &::std::ffi::CStr,
    mut item: ::core::ptr::NonNull<cJSON>,
    hooks: &internal_hooks,
    constant_key: bool,
) -> cJSON_bool {
    let mut new_key: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut new_type: ::core::ffi::c_int = cJSON_Invalid;
    if ::core::ptr::eq(object as *const cJSON, item.as_ptr() as *const cJSON) {
        return false_0;
    }
    let item = unsafe { item.as_mut() };
    if constant_key {
        new_key = string.as_ptr().cast_mut();
        new_type = item.type_0 | cJSON_StringIsConst;
    } else {
        new_key = cJSON_strdup(Some(string), hooks)
            .map(::core::ptr::NonNull::as_ptr)
            .unwrap_or_else(::core::ptr::null_mut::<::core::ffi::c_char>);
        if new_key.is_null() {
            return false_0;
        }
        new_type = item.type_0 & !cJSON_StringIsConst;
    }
    if item.type_0 & cJSON_StringIsConst == 0 && !item.string.is_null() {
        unsafe {
            hooks.deallocate.expect("non-null function pointer")(
                item.string as *mut ::core::ffi::c_void,
            );
        }
    }
    item.string = new_key;
    item.type_0 = new_type;
    return cJSON_AddItemToArray(object, item);
}
pub fn cJSON_AddItemToObject(
    object: &mut cJSON,
    string: &::std::ffi::CStr,
    item: &mut cJSON,
) -> cJSON_bool {
    let hooks = get_global_hooks();
    return add_item_to_object(
        object,
        string,
        ::core::ptr::NonNull::from(item),
        &hooks,
        false,
    );
}
#[export_name = "cJSON_AddItemToObject"]
pub unsafe extern "C" fn cJSON_AddItemToObject_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
    mut item: *mut cJSON,
) -> cJSON_bool {
    if object.is_null() || string.is_null() || item.is_null() || object == item {
        return false_0;
    }
    cJSON_AddItemToObject(
        object
            .as_mut()
            .expect("cJSON_AddItemToObject received a null object pointer"),
        ::std::ffi::CStr::from_ptr(string),
        item.as_mut()
            .expect("cJSON_AddItemToObject received a null item pointer"),
    )
}
pub fn cJSON_AddItemToObjectCS(
    object: &mut cJSON,
    string: &::std::ffi::CStr,
    item: &mut cJSON,
) -> cJSON_bool {
    let hooks = get_global_hooks();
    return add_item_to_object(
        object,
        string,
        ::core::ptr::NonNull::from(item),
        &hooks,
        true,
    );
}
#[export_name = "cJSON_AddItemToObjectCS"]
pub unsafe extern "C" fn cJSON_AddItemToObjectCS_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
    mut item: *mut cJSON,
) -> cJSON_bool {
    if object.is_null() || string.is_null() || item.is_null() || object == item {
        return false_0;
    }
    cJSON_AddItemToObjectCS(
        object
            .as_mut()
            .expect("cJSON_AddItemToObjectCS received a null object pointer"),
        ::std::ffi::CStr::from_ptr(string),
        item.as_mut()
            .expect("cJSON_AddItemToObjectCS received a null item pointer"),
    )
}
pub fn cJSON_AddItemReferenceToArray(array: &mut cJSON, item: cJSON) -> cJSON_bool {
    let hooks = get_global_hooks();
    let Some(reference) = create_reference(Some(&item), &hooks) else {
        return false_0;
    };
    return add_item_to_array(::core::ptr::NonNull::from(array), reference);
}
#[export_name = "cJSON_AddItemReferenceToArray"]
pub unsafe extern "C" fn cJSON_AddItemReferenceToArray_ffi(
    mut array: *mut cJSON,
    mut item: *mut cJSON,
) -> cJSON_bool {
    if array.is_null() {
        return false_0;
    }
    let Some(item) = item.as_ref() else {
        return false_0;
    };
    let item = *item;
    let Some(array) = array.as_mut() else {
        return false_0;
    };
    cJSON_AddItemReferenceToArray(array, item)
}
pub fn cJSON_AddItemReferenceToObject(
    object: &mut cJSON,
    string: &::std::ffi::CStr,
    item: cJSON,
) -> cJSON_bool {
    let hooks = get_global_hooks();
    let Some(reference) = create_reference(Some(&item), &hooks) else {
        return false_0;
    };
    return add_item_to_object(object, string, reference, &hooks, false);
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
    let Some(item) = item.as_ref() else {
        return false_0;
    };
    let item = *item;
    let Some(object) = object.as_mut() else {
        return false_0;
    };
    cJSON_AddItemReferenceToObject(object, ::std::ffi::CStr::from_ptr(string), item)
}
pub fn cJSON_AddNullToObject(
    object: Option<&mut cJSON>,
    name: Option<&::std::ffi::CStr>,
) -> Option<::core::ptr::NonNull<cJSON>> {
    let Some(null) = cJSON_CreateNull() else {
        return None;
    };
    let (Some(object), Some(name)) = (object, name) else {
        cJSON_Delete(null.as_ptr());
        return None;
    };
    let hooks = get_global_hooks();
    if add_item_to_object(object, name, null, &hooks, false) != 0 {
        return Some(null);
    }
    cJSON_Delete(null.as_ptr());
    None
}
#[export_name = "cJSON_AddNullToObject"]
pub unsafe extern "C" fn cJSON_AddNullToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let object = object.as_mut();
    let name = if object.is_some() && !name.is_null() {
        Some(::std::ffi::CStr::from_ptr(name))
    } else {
        None
    };
    cJSON_AddNullToObject(object, name)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_AddTrueToObject(
    object: Option<&mut cJSON>,
    name: Option<&::std::ffi::CStr>,
) -> Option<::core::ptr::NonNull<cJSON>> {
    let Some(true_item) = cJSON_CreateTrue() else {
        return None;
    };
    let (Some(object), Some(name)) = (object, name) else {
        cJSON_Delete(true_item.as_ptr());
        return None;
    };
    let hooks = get_global_hooks();
    if add_item_to_object(object, name, true_item, &hooks, false) != 0 {
        return Some(true_item);
    }
    cJSON_Delete(true_item.as_ptr());
    None
}
#[export_name = "cJSON_AddTrueToObject"]
pub unsafe extern "C" fn cJSON_AddTrueToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let object = object.as_mut();
    let name = if object.is_some() && !name.is_null() {
        Some(::std::ffi::CStr::from_ptr(name))
    } else {
        None
    };
    cJSON_AddTrueToObject(object, name)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_AddFalseToObject(
    object: Option<&mut cJSON>,
    name: Option<&::std::ffi::CStr>,
) -> Option<::core::ptr::NonNull<cJSON>> {
    let Some(false_item) = cJSON_CreateFalse() else {
        return None;
    };
    let (Some(object), Some(name)) = (object, name) else {
        cJSON_Delete(false_item.as_ptr());
        return None;
    };
    let hooks = get_global_hooks();
    if add_item_to_object(object, name, false_item, &hooks, false) != 0 {
        return Some(false_item);
    }
    cJSON_Delete(false_item.as_ptr());
    None
}
#[export_name = "cJSON_AddFalseToObject"]
pub unsafe extern "C" fn cJSON_AddFalseToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let object = object.as_mut();
    let name = if object.is_some() && !name.is_null() {
        Some(::std::ffi::CStr::from_ptr(name))
    } else {
        None
    };
    cJSON_AddFalseToObject(object, name)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_AddBoolToObject(
    object: Option<&mut cJSON>,
    name: Option<&::std::ffi::CStr>,
    boolean: cJSON_bool,
) -> Option<::core::ptr::NonNull<cJSON>> {
    let Some(bool_item) = cJSON_CreateBool(boolean) else {
        return None;
    };
    let (Some(object), Some(name)) = (object, name) else {
        cJSON_Delete(bool_item.as_ptr());
        return None;
    };
    let hooks = get_global_hooks();
    if add_item_to_object(object, name, bool_item, &hooks, false) != 0 {
        return Some(bool_item);
    }
    cJSON_Delete(bool_item.as_ptr());
    None
}
#[export_name = "cJSON_AddBoolToObject"]
pub unsafe extern "C" fn cJSON_AddBoolToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
    boolean: cJSON_bool,
) -> *mut cJSON {
    let object = object.as_mut();
    let name = if object.is_some() && !name.is_null() {
        Some(::std::ffi::CStr::from_ptr(name))
    } else {
        None
    };
    cJSON_AddBoolToObject(object, name, boolean)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_AddNumberToObject(
    object: Option<&mut cJSON>,
    name: Option<&::std::ffi::CStr>,
    number: ::core::ffi::c_double,
) -> Option<::core::ptr::NonNull<cJSON>> {
    let Some(number_item) = cJSON_CreateNumber(number) else {
        return None;
    };
    let (Some(object), Some(name)) = (object, name) else {
        cJSON_Delete(number_item.as_ptr());
        return None;
    };
    let hooks = get_global_hooks();
    if add_item_to_object(object, name, number_item, &hooks, false) != 0 {
        return Some(number_item);
    }
    cJSON_Delete(number_item.as_ptr());
    None
}
#[export_name = "cJSON_AddNumberToObject"]
pub unsafe extern "C" fn cJSON_AddNumberToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
    number: ::core::ffi::c_double,
) -> *mut cJSON {
    let object = object.as_mut();
    let name = if object.is_some() && !name.is_null() {
        Some(::std::ffi::CStr::from_ptr(name))
    } else {
        None
    };
    cJSON_AddNumberToObject(object, name, number)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_AddStringToObject(
    object: Option<&mut cJSON>,
    name: Option<&::std::ffi::CStr>,
    string: Option<&::std::ffi::CStr>,
) -> Option<::core::ptr::NonNull<cJSON>> {
    let Some(string_item) = cJSON_CreateString(string) else {
        return None;
    };
    let (Some(object), Some(name)) = (object, name) else {
        cJSON_Delete(string_item.as_ptr());
        return None;
    };
    let hooks = get_global_hooks();
    if add_item_to_object(object, name, string_item, &hooks, false) != 0 {
        return Some(string_item);
    }
    cJSON_Delete(string_item.as_ptr());
    None
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
    let object = object.as_mut();
    let name = if object.is_some() && !name.is_null() {
        Some(::std::ffi::CStr::from_ptr(name))
    } else {
        None
    };
    cJSON_AddStringToObject(object, name, string)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_AddRawToObject(
    object: Option<&mut cJSON>,
    name: Option<&::std::ffi::CStr>,
    raw: Option<&::std::ffi::CStr>,
) -> Option<::core::ptr::NonNull<cJSON>> {
    let Some(raw_item) = cJSON_CreateRaw(raw) else {
        return None;
    };
    let (Some(object), Some(name)) = (object, name) else {
        cJSON_Delete(raw_item.as_ptr());
        return None;
    };
    let hooks = get_global_hooks();
    if add_item_to_object(object, name, raw_item, &hooks, false) != 0 {
        return Some(raw_item);
    }
    cJSON_Delete(raw_item.as_ptr());
    None
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
    let object = object.as_mut();
    let name = if object.is_some() && !name.is_null() {
        Some(::std::ffi::CStr::from_ptr(name))
    } else {
        None
    };
    cJSON_AddRawToObject(object, name, raw)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_AddObjectToObject(
    object: Option<&mut cJSON>,
    name: Option<&::std::ffi::CStr>,
) -> Option<::core::ptr::NonNull<cJSON>> {
    let Some(object_item) = cJSON_CreateObject() else {
        return None;
    };
    let (Some(object), Some(name)) = (object, name) else {
        cJSON_Delete(object_item.as_ptr());
        return None;
    };
    let hooks = get_global_hooks();
    if add_item_to_object(object, name, object_item, &hooks, false) != 0 {
        return Some(object_item);
    }
    cJSON_Delete(object_item.as_ptr());
    None
}
#[export_name = "cJSON_AddObjectToObject"]
pub unsafe extern "C" fn cJSON_AddObjectToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let object = object.as_mut();
    let name = if object.is_some() && !name.is_null() {
        Some(::std::ffi::CStr::from_ptr(name))
    } else {
        None
    };
    cJSON_AddObjectToObject(object, name)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_AddArrayToObject(
    object: Option<&mut cJSON>,
    name: Option<&::std::ffi::CStr>,
) -> Option<::core::ptr::NonNull<cJSON>> {
    let Some(array) = cJSON_CreateArray() else {
        return None;
    };
    let (Some(object), Some(name)) = (object, name) else {
        cJSON_Delete(array.as_ptr());
        return None;
    };
    let hooks = get_global_hooks();
    if add_item_to_object(object, name, array, &hooks, false) != 0 {
        return Some(array);
    }
    cJSON_Delete(array.as_ptr());
    None
}
#[export_name = "cJSON_AddArrayToObject"]
pub unsafe extern "C" fn cJSON_AddArrayToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let object = object.as_mut();
    let name = if object.is_some() && !name.is_null() {
        Some(::std::ffi::CStr::from_ptr(name))
    } else {
        None
    };
    cJSON_AddArrayToObject(object, name)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_DetachItemViaPointer(
    parent: &mut cJSON,
    item: Option<::core::ptr::NonNull<cJSON>>,
) -> Option<::core::ptr::NonNull<cJSON>> {
    let item = item?;
    let item_ptr = item.as_ptr();
    unsafe {
        if item_ptr != parent.child && (*item_ptr).prev.is_null() {
            return None;
        }
        if item_ptr != parent.child {
            (*(*item_ptr).prev).next = (*item_ptr).next;
        }
        if !(*item_ptr).next.is_null() {
            (*(*item_ptr).next).prev = (*item_ptr).prev;
        }
        if item_ptr == parent.child {
            parent.child = (*item_ptr).next;
        } else if (*item_ptr).next.is_null() {
            (*parent.child).prev = (*item_ptr).prev;
        }
        (*item_ptr).prev = ::core::ptr::null_mut::<cJSON>();
        (*item_ptr).next = ::core::ptr::null_mut::<cJSON>();
    }
    Some(item)
}
#[export_name = "cJSON_DetachItemViaPointer"]
pub unsafe extern "C" fn cJSON_DetachItemViaPointer_ffi(
    mut parent: *mut cJSON,
    item: *mut cJSON,
) -> *mut cJSON {
    let Some(parent) = parent.as_mut() else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    cJSON_DetachItemViaPointer(parent, ::core::ptr::NonNull::new(item))
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_DetachItemFromArray(
    array: Option<&mut cJSON>,
    which: ::core::ffi::c_int,
) -> Option<::core::ptr::NonNull<cJSON>> {
    if which < 0 as ::core::ffi::c_int {
        return None;
    }
    let Some(array) = array else {
        return None;
    };
    let to_detach = cJSON_GetArrayItem(Some(&*array), which);
    cJSON_DetachItemViaPointer(array, to_detach)
}
#[export_name = "cJSON_DetachItemFromArray"]
pub unsafe extern "C" fn cJSON_DetachItemFromArray_ffi(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
) -> *mut cJSON {
    cJSON_DetachItemFromArray(array.as_mut(), which)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_DeleteItemFromArray(array: Option<&mut cJSON>, which: ::core::ffi::c_int) {
    if let Some(item) = cJSON_DetachItemFromArray(array, which) {
        cJSON_Delete(item.as_ptr());
    }
}
#[export_name = "cJSON_DeleteItemFromArray"]
pub unsafe extern "C" fn cJSON_DeleteItemFromArray_ffi(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
) {
    cJSON_DeleteItemFromArray(array.as_mut(), which)
}
pub fn cJSON_DetachItemFromObject(
    object: Option<&mut cJSON>,
    string: Option<&::std::ffi::CStr>,
) -> Option<::core::ptr::NonNull<cJSON>> {
    let object = object?;
    let string = string?;
    let to_detach = cJSON_GetObjectItem(Some(&*object), Some(string));
    cJSON_DetachItemViaPointer(object, to_detach)
}
#[export_name = "cJSON_DetachItemFromObject"]
pub unsafe extern "C" fn cJSON_DetachItemFromObject_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let string = if string.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(string))
    };
    cJSON_DetachItemFromObject(object.as_mut(), string)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_DetachItemFromObjectCaseSensitive(
    object: Option<&mut cJSON>,
    string: Option<&::std::ffi::CStr>,
) -> Option<::core::ptr::NonNull<cJSON>> {
    let object = object?;
    let string = string?;
    let to_detach = cJSON_GetObjectItemCaseSensitive(Some(&*object), Some(string));
    cJSON_DetachItemViaPointer(object, to_detach)
}
#[export_name = "cJSON_DetachItemFromObjectCaseSensitive"]
pub unsafe extern "C" fn cJSON_DetachItemFromObjectCaseSensitive_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let string = if string.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(string))
    };
    cJSON_DetachItemFromObjectCaseSensitive(object.as_mut(), string)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_DeleteItemFromObject(object: Option<&mut cJSON>, string: Option<&::std::ffi::CStr>) {
    if let Some(item) = cJSON_DetachItemFromObject(object, string) {
        cJSON_Delete(item.as_ptr());
    }
}
#[export_name = "cJSON_DeleteItemFromObject"]
pub unsafe extern "C" fn cJSON_DeleteItemFromObject_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
) {
    let string = if string.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(string))
    };
    cJSON_DeleteItemFromObject(object.as_mut(), string)
}
pub fn cJSON_DeleteItemFromObjectCaseSensitive(
    object: Option<&mut cJSON>,
    string: Option<&::std::ffi::CStr>,
) {
    if let Some(item) = cJSON_DetachItemFromObjectCaseSensitive(object, string) {
        cJSON_Delete(item.as_ptr());
    }
}
#[export_name = "cJSON_DeleteItemFromObjectCaseSensitive"]
pub unsafe extern "C" fn cJSON_DeleteItemFromObjectCaseSensitive_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
) {
    let string = if string.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(string))
    };
    cJSON_DeleteItemFromObjectCaseSensitive(object.as_mut(), string)
}
pub fn cJSON_InsertItemInArray(
    array: &mut cJSON,
    which: ::core::ffi::c_int,
    newitem: &mut cJSON,
) -> cJSON_bool {
    if which < 0 as ::core::ffi::c_int || ::core::ptr::eq(array, newitem) {
        return false_0;
    }
    let array_item = ::core::ptr::NonNull::from(&mut *array);
    let newitem_item = ::core::ptr::NonNull::from(&mut *newitem);
    let Some(after_inserted) = get_array_item(Some(array), which as size_t) else {
        return add_item_to_array(array_item, newitem_item);
    };
    let after_inserted = after_inserted.as_ptr();
    let newitem_ptr = newitem_item.as_ptr();
    if after_inserted != array.child && unsafe { (*after_inserted).prev.is_null() } {
        return false_0;
    }
    newitem.next = after_inserted as *mut cJSON;
    newitem.prev = unsafe { (*after_inserted).prev };
    unsafe {
        (*after_inserted).prev = newitem_ptr as *mut cJSON;
    }
    if after_inserted == array.child {
        array.child = newitem_ptr as *mut cJSON;
    } else {
        unsafe {
            (*newitem.prev).next = newitem_ptr as *mut cJSON;
        }
    }
    return true_0;
}
#[export_name = "cJSON_InsertItemInArray"]
pub unsafe extern "C" fn cJSON_InsertItemInArray_ffi(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
    mut newitem: *mut cJSON,
) -> cJSON_bool {
    if array.is_null() || newitem.is_null() || array == newitem {
        return false_0;
    }
    let array = array
        .as_mut()
        .expect("cJSON_InsertItemInArray received a null array pointer");
    let newitem = newitem
        .as_mut()
        .expect("cJSON_InsertItemInArray received a null item pointer");
    cJSON_InsertItemInArray(array, which, newitem)
}
fn cJSON_ReplaceItemViaPointer(
    parent: Option<::core::ptr::NonNull<cJSON>>,
    item: Option<::core::ptr::NonNull<cJSON>>,
    replacement: Option<::core::ptr::NonNull<cJSON>>,
) -> cJSON_bool {
    let Some(parent) = parent else {
        return false_0;
    };
    let parent = parent.as_ptr();
    if unsafe { (*parent).child.is_null() } {
        return false_0;
    }
    let (Some(item), Some(replacement)) = (item, replacement) else {
        return false_0;
    };
    let item = item.as_ptr();
    let replacement = replacement.as_ptr();
    if replacement == item {
        return true_0;
    }
    unsafe {
        (*replacement).next = (*item).next;
        (*replacement).prev = (*item).prev;
        if !(*replacement).next.is_null() {
            (*(*replacement).next).prev = replacement as *mut cJSON;
        }
        if (*parent).child == item {
            if (*(*parent).child).prev == (*parent).child {
                (*replacement).prev = replacement as *mut cJSON;
            }
            (*parent).child = replacement as *mut cJSON;
        } else {
            if !(*replacement).prev.is_null() {
                (*(*replacement).prev).next = replacement as *mut cJSON;
            }
            if (*replacement).next.is_null() {
                (*(*parent).child).prev = replacement as *mut cJSON;
            }
        }
        (*item).next = ::core::ptr::null_mut::<cJSON>();
        (*item).prev = ::core::ptr::null_mut::<cJSON>();
    };
    cJSON_Delete(item);
    return true_0;
}
#[export_name = "cJSON_ReplaceItemViaPointer"]
pub unsafe extern "C" fn cJSON_ReplaceItemViaPointer_ffi(
    parent: *mut cJSON,
    item: *mut cJSON,
    mut replacement: *mut cJSON,
) -> cJSON_bool {
    cJSON_ReplaceItemViaPointer(
        ::core::ptr::NonNull::new(parent),
        ::core::ptr::NonNull::new(item),
        ::core::ptr::NonNull::new(replacement),
    )
}
pub fn cJSON_ReplaceItemInArray(
    array: &mut cJSON,
    which: ::core::ffi::c_int,
    newitem: &mut cJSON,
) -> cJSON_bool {
    if which < 0 as ::core::ffi::c_int || ::core::ptr::eq(array, newitem) {
        return false_0;
    }
    let item = get_array_item(Some(array), which as size_t);
    let array = ::core::ptr::NonNull::from(&mut *array);
    let newitem = ::core::ptr::NonNull::from(&mut *newitem);
    return cJSON_ReplaceItemViaPointer(Some(array), item, Some(newitem));
}
#[export_name = "cJSON_ReplaceItemInArray"]
pub unsafe extern "C" fn cJSON_ReplaceItemInArray_ffi(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
    mut newitem: *mut cJSON,
) -> cJSON_bool {
    if array.is_null() || newitem.is_null() || array == newitem {
        return false_0;
    }
    let array = array
        .as_mut()
        .expect("cJSON_ReplaceItemInArray received a null array pointer");
    let newitem = newitem
        .as_mut()
        .expect("cJSON_ReplaceItemInArray received a null item pointer");
    cJSON_ReplaceItemInArray(array, which, newitem)
}
fn replace_item_in_object(
    object: Option<&mut cJSON>,
    string: Option<&::std::ffi::CStr>,
    replacement: Option<&mut cJSON>,
    mut case_sensitive: cJSON_bool,
) -> cJSON_bool {
    let (Some(string), Some(replacement)) = (string, replacement) else {
        return false_0;
    };
    let hooks = get_global_hooks();
    if replacement.type_0 & cJSON_StringIsConst == 0 && !replacement.string.is_null() {
        cJSON_free(replacement.string as *mut ::core::ffi::c_void);
    }
    replacement.string = cJSON_strdup(Some(string), &hooks)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<::core::ffi::c_char>);
    if replacement.string.is_null() {
        return false_0;
    }
    replacement.type_0 &= !cJSON_StringIsConst;
    let Some(object) = object else {
        return false_0;
    };
    let item = if case_sensitive != 0 {
        cJSON_GetObjectItemCaseSensitive(Some(object), Some(string))
    } else {
        cJSON_GetObjectItem(Some(object), Some(string))
    };
    let object = ::core::ptr::NonNull::from(object);
    let replacement = ::core::ptr::NonNull::from(replacement);
    return cJSON_ReplaceItemViaPointer(Some(object), item, Some(replacement));
}
fn cJSON_ReplaceItemInObject(
    object: Option<&mut cJSON>,
    string: Option<&::std::ffi::CStr>,
    newitem: Option<&mut cJSON>,
) -> cJSON_bool {
    return replace_item_in_object(object, string, newitem, false_0);
}
#[export_name = "cJSON_ReplaceItemInObject"]
pub unsafe extern "C" fn cJSON_ReplaceItemInObject_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
    mut newitem: *mut cJSON,
) -> cJSON_bool {
    let string = if string.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(string))
    };
    cJSON_ReplaceItemInObject(object.as_mut(), string, newitem.as_mut())
}
fn cJSON_ReplaceItemInObjectCaseSensitive(
    object: Option<&mut cJSON>,
    string: Option<&::std::ffi::CStr>,
    newitem: Option<&mut cJSON>,
) -> cJSON_bool {
    return replace_item_in_object(object, string, newitem, true_0);
}
#[export_name = "cJSON_ReplaceItemInObjectCaseSensitive"]
pub unsafe extern "C" fn cJSON_ReplaceItemInObjectCaseSensitive_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
    mut newitem: *mut cJSON,
) -> cJSON_bool {
    let string = if string.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(string))
    };
    cJSON_ReplaceItemInObjectCaseSensitive(object.as_mut(), string, newitem.as_mut())
}
fn cjson_init_item_type(item: &mut cJSON, type_0: ::core::ffi::c_int) {
    item.type_0 = type_0;
}
fn cjson_init_number_item(item: &mut cJSON, number: ::core::ffi::c_double) {
    cjson_init_item_type(item, cJSON_Number);
    cJSON_SetNumberHelper(item, number);
}
pub fn cJSON_CreateNull() -> Option<::core::ptr::NonNull<cJSON>> {
    let hooks = get_global_hooks();
    cJSON_New_Item(&hooks, |item| cjson_init_item_type(item, cJSON_NULL))
}
#[export_name = "cJSON_CreateNull"]
pub unsafe extern "C" fn cJSON_CreateNull_ffi() -> *mut cJSON {
    cJSON_CreateNull()
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_CreateTrue() -> Option<::core::ptr::NonNull<cJSON>> {
    let hooks = get_global_hooks();
    cJSON_New_Item(&hooks, |item| cjson_init_item_type(item, cJSON_True))
}
#[export_name = "cJSON_CreateTrue"]
pub unsafe extern "C" fn cJSON_CreateTrue_ffi() -> *mut cJSON {
    cJSON_CreateTrue()
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_CreateFalse() -> Option<::core::ptr::NonNull<cJSON>> {
    let hooks = get_global_hooks();
    cJSON_New_Item(&hooks, |item| cjson_init_item_type(item, cJSON_False))
}
#[export_name = "cJSON_CreateFalse"]
pub unsafe extern "C" fn cJSON_CreateFalse_ffi() -> *mut cJSON {
    cJSON_CreateFalse()
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_CreateBool(boolean: cJSON_bool) -> Option<::core::ptr::NonNull<cJSON>> {
    let hooks = get_global_hooks();
    cJSON_New_Item(&hooks, |item| {
        cjson_init_item_type(
            item,
            if boolean != 0 {
                cJSON_True
            } else {
                cJSON_False
            },
        );
    })
}
#[export_name = "cJSON_CreateBool"]
pub unsafe extern "C" fn cJSON_CreateBool_ffi(mut boolean: cJSON_bool) -> *mut cJSON {
    cJSON_CreateBool(boolean)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_CreateNumber(num: ::core::ffi::c_double) -> Option<::core::ptr::NonNull<cJSON>> {
    let hooks = get_global_hooks();
    cJSON_New_Item(&hooks, |item| cjson_init_number_item(item, num))
}
#[export_name = "cJSON_CreateNumber"]
pub unsafe extern "C" fn cJSON_CreateNumber_ffi(mut num: ::core::ffi::c_double) -> *mut cJSON {
    cJSON_CreateNumber(num)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_CreateString(
    string: Option<&::std::ffi::CStr>,
) -> Option<::core::ptr::NonNull<cJSON>> {
    let hooks = get_global_hooks();
    let mut valuestring = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let item = cJSON_New_Item(&hooks, |item| {
        item.type_0 = cJSON_String;
        valuestring = cJSON_strdup(string, &hooks)
            .map(::core::ptr::NonNull::as_ptr)
            .unwrap_or_else(::core::ptr::null_mut::<::core::ffi::c_char>);
        item.valuestring = valuestring;
    });
    if let Some(item) = item {
        if valuestring.is_null() {
            cJSON_Delete(item.as_ptr());
            return None;
        }
        return Some(item);
    }
    None
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
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_CreateStringReference(
    string: Option<&::std::ffi::CStr>,
) -> Option<::core::ptr::NonNull<cJSON>> {
    let hooks = get_global_hooks();
    cJSON_New_Item(&hooks, |item| {
        item.type_0 = cJSON_String | cJSON_IsReference;
        item.valuestring = string
            .map(::std::ffi::CStr::as_ptr)
            .unwrap_or_else(::core::ptr::null::<::core::ffi::c_char>)
            .cast_mut();
    })
}
#[export_name = "cJSON_CreateStringReference"]
pub unsafe extern "C" fn cJSON_CreateStringReference_ffi(
    mut string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let string = if string.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(string))
    };
    cJSON_CreateStringReference(string)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_CreateObjectReference(child: Option<&cJSON>) -> Option<::core::ptr::NonNull<cJSON>> {
    let hooks = get_global_hooks();
    cJSON_New_Item(&hooks, |item| {
        item.type_0 = cJSON_Object | cJSON_IsReference;
        item.child = match child {
            Some(child) => (child as *const cJSON).cast_mut(),
            None => ::core::ptr::null_mut::<cJSON>(),
        };
    })
}
#[export_name = "cJSON_CreateObjectReference"]
pub unsafe extern "C" fn cJSON_CreateObjectReference_ffi(mut child: *const cJSON) -> *mut cJSON {
    let child = child.as_ref();
    cJSON_CreateObjectReference(child)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_CreateArrayReference(child: Option<&cJSON>) -> Option<::core::ptr::NonNull<cJSON>> {
    let hooks = get_global_hooks();
    cJSON_New_Item(&hooks, |item| {
        item.type_0 = cJSON_Array | cJSON_IsReference;
        item.child = match child {
            Some(child) => (child as *const cJSON).cast_mut(),
            None => ::core::ptr::null_mut::<cJSON>(),
        };
    })
}
#[export_name = "cJSON_CreateArrayReference"]
pub unsafe extern "C" fn cJSON_CreateArrayReference_ffi(mut child: *const cJSON) -> *mut cJSON {
    let child = child.as_ref();
    cJSON_CreateArrayReference(child)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_CreateRaw(raw: Option<&::std::ffi::CStr>) -> Option<::core::ptr::NonNull<cJSON>> {
    let hooks = get_global_hooks();
    let mut valuestring = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let item = cJSON_New_Item(&hooks, |item| {
        item.type_0 = cJSON_Raw;
        valuestring = cJSON_strdup(raw, &hooks)
            .map(::core::ptr::NonNull::as_ptr)
            .unwrap_or_else(::core::ptr::null_mut::<::core::ffi::c_char>);
        item.valuestring = valuestring;
    });
    if let Some(item) = item {
        if valuestring.is_null() {
            cJSON_Delete(item.as_ptr());
            return None;
        }
        return Some(item);
    }
    None
}
#[export_name = "cJSON_CreateRaw"]
pub unsafe extern "C" fn cJSON_CreateRaw_ffi(mut raw: *const ::core::ffi::c_char) -> *mut cJSON {
    let raw = if raw.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(raw))
    };
    cJSON_CreateRaw(raw)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_CreateArray() -> Option<::core::ptr::NonNull<cJSON>> {
    let hooks = get_global_hooks();
    cJSON_New_Item(&hooks, |item| cjson_init_item_type(item, cJSON_Array))
}
#[export_name = "cJSON_CreateArray"]
pub unsafe extern "C" fn cJSON_CreateArray_ffi() -> *mut cJSON {
    cJSON_CreateArray()
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_CreateObject() -> Option<::core::ptr::NonNull<cJSON>> {
    let hooks = get_global_hooks();
    cJSON_New_Item(&hooks, |item| cjson_init_item_type(item, cJSON_Object))
}
#[export_name = "cJSON_CreateObject"]
pub unsafe extern "C" fn cJSON_CreateObject_ffi() -> *mut cJSON {
    cJSON_CreateObject()
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_CreateIntArray(numbers: &[::core::ffi::c_int]) -> Option<::core::ptr::NonNull<cJSON>> {
    let a = cJSON_CreateArray()?;
    for number in numbers {
        let Some(n) = cJSON_CreateNumber(*number as ::core::ffi::c_double) else {
            cJSON_Delete(a.as_ptr());
            return None;
        };
        add_item_to_array(a, n);
    }
    Some(a)
}
#[export_name = "cJSON_CreateIntArray"]
pub unsafe extern "C" fn cJSON_CreateIntArray_ffi(
    mut numbers: *const ::core::ffi::c_int,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let numbers = if count == 0 {
        &[][..]
    } else {
        ::core::slice::from_raw_parts(numbers, count as usize)
    };
    cJSON_CreateIntArray(numbers)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_CreateFloatArray(
    numbers: &[::core::ffi::c_float],
) -> Option<::core::ptr::NonNull<cJSON>> {
    let a = cJSON_CreateArray()?;
    for number in numbers {
        let Some(n) = cJSON_CreateNumber(*number as ::core::ffi::c_double) else {
            cJSON_Delete(a.as_ptr());
            return None;
        };
        add_item_to_array(a, n);
    }
    Some(a)
}
#[export_name = "cJSON_CreateFloatArray"]
pub unsafe extern "C" fn cJSON_CreateFloatArray_ffi(
    mut numbers: *const ::core::ffi::c_float,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let numbers = if count == 0 {
        &[][..]
    } else {
        ::core::slice::from_raw_parts(numbers, count as usize)
    };
    cJSON_CreateFloatArray(numbers)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_CreateDoubleArray(
    numbers: &[::core::ffi::c_double],
) -> Option<::core::ptr::NonNull<cJSON>> {
    let a = cJSON_CreateArray()?;
    for number in numbers {
        let Some(n) = cJSON_CreateNumber(*number) else {
            cJSON_Delete(a.as_ptr());
            return None;
        };
        add_item_to_array(a, n);
    }
    Some(a)
}
#[export_name = "cJSON_CreateDoubleArray"]
pub unsafe extern "C" fn cJSON_CreateDoubleArray_ffi(
    mut numbers: *const ::core::ffi::c_double,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let numbers = if count == 0 {
        &[][..]
    } else {
        ::core::slice::from_raw_parts(numbers, count as usize)
    };
    cJSON_CreateDoubleArray(numbers)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_CreateStringArray(
    strings: &[Option<&::std::ffi::CStr>],
) -> Option<::core::ptr::NonNull<cJSON>> {
    let a = cJSON_CreateArray()?;
    for string in strings.iter().copied() {
        let Some(n) = cJSON_CreateString(string) else {
            cJSON_Delete(a.as_ptr());
            return None;
        };
        add_item_to_array(a, n);
    }
    Some(a)
}
#[export_name = "cJSON_CreateStringArray"]
pub unsafe extern "C" fn cJSON_CreateStringArray_ffi(
    mut strings: *const *const ::core::ffi::c_char,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    if count < 0 as ::core::ffi::c_int || strings.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let strings = if count == 0 {
        &[][..]
    } else {
        ::core::slice::from_raw_parts(strings, count as usize)
    };
    let mut string_refs = ::std::vec::Vec::with_capacity(strings.len());
    for string in strings.iter().copied() {
        if string.is_null() {
            string_refs.push(None);
        } else {
            string_refs.push(Some(::std::ffi::CStr::from_ptr(string)));
        }
    }
    cJSON_CreateStringArray(&string_refs)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_Duplicate(
    item: Option<&cJSON>,
    recurse: cJSON_bool,
) -> Option<::core::ptr::NonNull<cJSON>> {
    cJSON_Duplicate_rec(item, 0 as size_t, recurse)
}
#[export_name = "cJSON_Duplicate"]
pub unsafe extern "C" fn cJSON_Duplicate_ffi(
    mut item: *const cJSON,
    mut recurse: cJSON_bool,
) -> *mut cJSON {
    cJSON_Duplicate(item.as_ref(), recurse)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_Duplicate_rec(
    item: Option<&cJSON>,
    depth: size_t,
    recurse: cJSON_bool,
) -> Option<::core::ptr::NonNull<cJSON>> {
    let item = item?;
    let hooks = get_global_hooks();
    let mut failed = false;
    let valuestring = if item.valuestring.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(item.valuestring) })
    };
    let string = if item.string.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(item.string) })
    };
    let Some(newitem) = cJSON_New_Item(&hooks, |newitem| {
        newitem.type_0 = item.type_0 & !cJSON_IsReference;
        newitem.valueint = item.valueint;
        newitem.valuedouble = item.valuedouble;
        if let Some(valuestring) = valuestring {
            newitem.valuestring = cJSON_strdup(Some(valuestring), &hooks)
                .map(::core::ptr::NonNull::as_ptr)
                .unwrap_or_else(::core::ptr::null_mut::<::core::ffi::c_char>);
            failed = newitem.valuestring.is_null();
        }
        if !failed {
            newitem.string = if item.type_0 & cJSON_StringIsConst != 0 {
                item.string
            } else if let Some(string) = string {
                cJSON_strdup(Some(string), &hooks)
                    .map(::core::ptr::NonNull::as_ptr)
                    .unwrap_or_else(::core::ptr::null_mut::<::core::ffi::c_char>)
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            };
            failed = string.is_some() && newitem.string.is_null();
        }
    }) else {
        return None;
    };
    if failed {
        cJSON_Delete(newitem.as_ptr());
        return None;
    }
    if recurse == 0 {
        return Some(newitem);
    }
    let mut child = item.child;
    while !child.is_null() {
        if depth >= CJSON_CIRCULAR_LIMIT as size_t {
            cJSON_Delete(newitem.as_ptr());
            return None;
        }
        let Some(newchild) = cJSON_Duplicate_rec(
            unsafe { child.as_ref() },
            depth.wrapping_add(1 as size_t),
            true_0,
        ) else {
            cJSON_Delete(newitem.as_ptr());
            return None;
        };
        add_item_to_array(newitem, newchild);
        child = unsafe { (*child).next as *mut cJSON };
    }
    Some(newitem)
}
#[export_name = "cJSON_Duplicate_rec"]
pub unsafe extern "C" fn cJSON_Duplicate_rec_ffi(
    mut item: *const cJSON,
    mut depth: size_t,
    mut recurse: cJSON_bool,
) -> *mut cJSON {
    cJSON_Duplicate_rec(item.as_ref(), depth, recurse)
        .map(::core::ptr::NonNull::as_ptr)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
fn skip_oneline_comment(json: &[u8], input: &mut usize) {
    *input += 2;
    while *input < json.len() && json[*input] != b'\0' {
        if json[*input] == b'\n' {
            *input += 1;
            return;
        }
        *input += 1;
    }
}
fn skip_multiline_comment(json: &[u8], input: &mut usize) {
    *input += 2;
    while *input < json.len() && json[*input] != b'\0' {
        if json[*input] == b'*' && json.get(*input + 1) == Some(&b'/') {
            *input += 2;
            return;
        }
        *input += 1;
    }
}
fn minify_string(json: &mut [u8], input: &mut usize, output: &mut usize) {
    json[*output] = json[*input];
    *input += 1;
    *output += 1;
    while *input < json.len() && json[*input] != b'\0' {
        let current = json[*input];
        json[*output] = current;
        if current == b'"' {
            *input += 1;
            *output += 1;
            return;
        } else if current == b'\\' && json.get(*input + 1) == Some(&b'"') {
            json[*output + 1] = b'"';
            *input += 1;
            *output += 1;
        }
        *input += 1;
        *output += 1;
    }
}
pub fn cJSON_Minify(json: &mut [u8]) {
    let mut input = 0usize;
    let mut output = 0usize;
    while input < json.len() && json[input] != b'\0' {
        match json[input] {
            b' ' | b'\t' | b'\r' | b'\n' => {
                input += 1;
            }
            b'/' => match json.get(input + 1).copied() {
                Some(b'/') => skip_oneline_comment(json, &mut input),
                Some(b'*') => skip_multiline_comment(json, &mut input),
                _ => {
                    input += 1;
                }
            },
            b'"' => minify_string(json, &mut input, &mut output),
            _ => {
                json[output] = json[input];
                input += 1;
                output += 1;
            }
        }
    }
    if output < json.len() {
        json[output] = b'\0';
    }
}
#[export_name = "cJSON_Minify"]
pub unsafe extern "C" fn cJSON_Minify_ffi(mut json: *mut ::core::ffi::c_char) {
    if json.is_null() {
        return;
    }
    let length = ::std::ffi::CStr::from_ptr(json).to_bytes_with_nul().len();
    let json = ::core::slice::from_raw_parts_mut(json.cast::<u8>(), length);
    cJSON_Minify(json)
}
fn cjson_has_type(item: Option<&cJSON>, type_0: ::core::ffi::c_int) -> cJSON_bool {
    item.map(|item| (item.type_0 & 0xff as ::core::ffi::c_int == type_0) as ::core::ffi::c_int)
        .unwrap_or(false_0)
}
pub fn cJSON_IsInvalid(item: Option<&cJSON>) -> cJSON_bool {
    cjson_has_type(item, cJSON_Invalid)
}
#[export_name = "cJSON_IsInvalid"]
pub unsafe extern "C" fn cJSON_IsInvalid_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsInvalid(item.as_ref())
}
pub fn cJSON_IsFalse(item: Option<&cJSON>) -> cJSON_bool {
    cjson_has_type(item, cJSON_False)
}
#[export_name = "cJSON_IsFalse"]
pub unsafe extern "C" fn cJSON_IsFalse_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsFalse(item.as_ref())
}
pub fn cJSON_IsTrue(item: Option<&cJSON>) -> cJSON_bool {
    cjson_has_type(item, cJSON_True)
}
#[export_name = "cJSON_IsTrue"]
pub unsafe extern "C" fn cJSON_IsTrue_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsTrue(item.as_ref())
}
pub fn cJSON_IsBool(item: Option<&cJSON>) -> cJSON_bool {
    item.map(|item| {
        (item.type_0 & (cJSON_True | cJSON_False) != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
    })
    .unwrap_or(false_0)
}
#[export_name = "cJSON_IsBool"]
pub unsafe extern "C" fn cJSON_IsBool_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsBool(item.as_ref())
}
pub fn cJSON_IsNull(item: Option<&cJSON>) -> cJSON_bool {
    cjson_has_type(item, cJSON_NULL)
}
#[export_name = "cJSON_IsNull"]
pub unsafe extern "C" fn cJSON_IsNull_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsNull(item.as_ref())
}
pub fn cJSON_IsNumber(item: Option<&cJSON>) -> cJSON_bool {
    cjson_has_type(item, cJSON_Number)
}
#[export_name = "cJSON_IsNumber"]
pub unsafe extern "C" fn cJSON_IsNumber_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsNumber(item.as_ref())
}
pub fn cJSON_IsString(item: Option<&cJSON>) -> cJSON_bool {
    cjson_has_type(item, cJSON_String)
}
#[export_name = "cJSON_IsString"]
pub unsafe extern "C" fn cJSON_IsString_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsString(item.as_ref())
}
pub fn cJSON_IsArray(item: Option<&cJSON>) -> cJSON_bool {
    cjson_has_type(item, cJSON_Array)
}
#[export_name = "cJSON_IsArray"]
pub unsafe extern "C" fn cJSON_IsArray_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsArray(item.as_ref())
}
pub fn cJSON_IsObject(item: Option<&cJSON>) -> cJSON_bool {
    cjson_has_type(item, cJSON_Object)
}
#[export_name = "cJSON_IsObject"]
pub unsafe extern "C" fn cJSON_IsObject_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsObject(item.as_ref())
}
pub fn cJSON_IsRaw(item: Option<&cJSON>) -> cJSON_bool {
    cjson_has_type(item, cJSON_Raw)
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
    let (Some(a_item), Some(b_item)) = (a, b) else {
        return false_0;
    };
    let a_type = a_item.type_0 & 0xff as ::core::ffi::c_int;
    if a_type != b_item.type_0 & 0xff as ::core::ffi::c_int {
        return false_0;
    }
    match a_type {
        cJSON_False | cJSON_True | cJSON_NULL | cJSON_Number | cJSON_String | cJSON_Raw
        | cJSON_Array | cJSON_Object => {}
        _ => return false_0,
    }
    if ::core::ptr::eq(a_item, b_item) {
        return true_0;
    }
    match a_type {
        cJSON_False | cJSON_True | cJSON_NULL => return true_0,
        cJSON_Number => {
            if compare_double(a_item.valuedouble, b_item.valuedouble) != 0 {
                return true_0;
            }
            return false_0;
        }
        cJSON_String | cJSON_Raw => {
            if a_item.valuestring.is_null() || b_item.valuestring.is_null() {
                return false_0;
            }
            if unsafe { strcmp(a_item.valuestring, b_item.valuestring) } == 0 as ::core::ffi::c_int
            {
                return true_0;
            }
            return false_0;
        }
        cJSON_Array => unsafe {
            let mut a_element: *mut cJSON = a_item.child as *mut cJSON;
            let mut b_element: *mut cJSON = b_item.child as *mut cJSON;
            while !a_element.is_null() && !b_element.is_null() {
                let a_child = &*a_element;
                let b_child = &*b_element;
                if cJSON_Compare(Some(a_child), Some(b_child), case_sensitive) == 0 {
                    return false_0;
                }
                a_element = a_child.next;
                b_element = b_child.next;
            }
            if a_element != b_element {
                return false_0;
            }
            return true_0;
        },
        cJSON_Object => unsafe {
            let mut a_element_0: *mut cJSON = a_item.child as *mut cJSON;
            while !a_element_0.is_null() {
                let a_child = &*a_element_0;
                let Some(b_element_0) = get_object_item(
                    b_item,
                    Some(ObjectItemName::ItemString(a_child)),
                    case_sensitive,
                ) else {
                    return false_0;
                };
                if cJSON_Compare(Some(a_child), Some(&*b_element_0.as_ptr()), case_sensitive) == 0 {
                    return false_0;
                }
                a_element_0 = a_child.next;
            }
            let mut b_element_0 = b_item.child as *mut cJSON;
            while !b_element_0.is_null() {
                let b_child = &*b_element_0;
                let Some(a_element_0) = get_object_item(
                    a_item,
                    Some(ObjectItemName::ItemString(b_child)),
                    case_sensitive,
                ) else {
                    return false_0;
                };
                if cJSON_Compare(Some(b_child), Some(&*a_element_0.as_ptr()), case_sensitive) == 0 {
                    return false_0;
                }
                b_element_0 = b_child.next;
            }
            return true_0;
        },
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
    get_global_hooks()
        .allocate
        .expect("non-null function pointer")(size)
}
pub fn cJSON_free(object: *mut ::core::ffi::c_void) {
    unsafe {
        get_global_hooks()
            .deallocate
            .expect("non-null function pointer")(object);
    }
}
#[export_name = "cJSON_free"]
pub unsafe extern "C" fn cJSON_free_ffi(mut object: *mut ::core::ffi::c_void) {
    cJSON_free(object)
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __DBL_EPSILON__: ::core::ffi::c_double = 2.2204460492503131e-16f64;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const INT_MIN: ::core::ffi::c_int = -2147483647 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
pub const DBL_EPSILON: ::core::ffi::c_double = __DBL_EPSILON__;
