extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
#[repr(C)]
pub struct cJSON {
    pub children: Vec<cJSON>,
    pub reference_child: Option<&'static cJSON>,
    pub type_0: ::core::ffi::c_int,
    pub valuestring: Option<cJSON_ValueString>,
    pub valueint: ::core::ffi::c_int,
    pub valuedouble: ::core::ffi::c_double,
    pub string: Option<cJSON_ValueString>,
}
impl Clone for cJSON {
    fn clone(&self) -> Self {
        Self {
            children: Vec::new(),
            reference_child: None,
            type_0: self.type_0 | cJSON_IsReference,
            valuestring: self.valuestring.clone(),
            valueint: self.valueint,
            valuedouble: self.valuedouble,
            string: self.string.clone(),
        }
    }
}
#[derive(Clone)]
pub enum cJSON_ValueString {
    Owned(::std::rc::Rc<::std::ffi::CString>),
    Reference(&'static ::std::ffi::CStr),
}
impl cJSON_ValueString {
    fn as_c_str(&self) -> &::std::ffi::CStr {
        match self {
            Self::Owned(value) => value.as_c_str(),
            Self::Reference(value) => value,
        }
    }
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
pub fn cJSON_GetStringValue(item: Option<&cJSON>) -> Option<&::std::ffi::CStr> {
    let item = item?;
    if cJSON_IsString(Some(item)) == 0 {
        return None;
    }
    item.valuestring.as_ref().map(cJSON_ValueString::as_c_str)
}
#[export_name = "cJSON_GetStringValue"]
pub unsafe extern "C" fn cJSON_GetStringValue_ffi(item: *const cJSON) -> *mut ::core::ffi::c_char {
    match cJSON_GetStringValue(item.as_ref()) {
        Some(valuestring) => valuestring.as_ptr().cast_mut(),
        None => ::core::ptr::null_mut::<::core::ffi::c_char>(),
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
pub fn cJSON_InitHooks(hooks: Option<&cJSON_Hooks>) {
    let Some(hooks) = hooks else {
        set_global_hooks(internal_hooks {
            allocate: Some(malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void),
            deallocate: Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        });
        return;
    };
    let mut new_hooks = internal_hooks {
        allocate: Some(malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void),
        deallocate: Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    };
    if hooks.malloc_fn.is_some() {
        new_hooks.allocate = hooks.malloc_fn;
    }
    if hooks.free_fn.is_some() {
        new_hooks.deallocate = hooks.free_fn;
    }
    set_global_hooks(new_hooks);
}
#[export_name = "cJSON_InitHooks"]
pub unsafe extern "C" fn cJSON_InitHooks_ffi(mut hooks: *mut cJSON_Hooks) {
    cJSON_InitHooks(hooks.as_ref())
}
fn cJSON_New_Item(initialize: impl FnOnce(&mut cJSON)) -> cJSON {
    let mut item = cJSON {
        children: Vec::new(),
        reference_child: None,
        type_0: 0 as ::core::ffi::c_int,
        valuestring: None,
        valueint: 0 as ::core::ffi::c_int,
        valuedouble: 0.0f64,
        string: None,
    };
    initialize(&mut item);
    item
}
fn cJSON_New_Item_Box(initialize: impl FnOnce(&mut cJSON)) -> Box<cJSON> {
    Box::new(cJSON_New_Item(initialize))
}
pub fn cJSON_Delete(item: Option<Box<cJSON>>) {
    drop(item);
}
#[export_name = "cJSON_Delete"]
pub unsafe extern "C" fn cJSON_Delete_ffi(mut item: *mut cJSON) {
    let item = if item.is_null() {
        None
    } else {
        Some(Box::from_raw(item))
    };
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
pub fn cJSON_SetValuestring<'a>(
    object: &'a mut cJSON,
    valuestring: &::std::ffi::CStr,
) -> Option<&'a ::std::ffi::CStr> {
    if object.type_0 & cJSON_String == 0 || object.type_0 & cJSON_IsReference != 0 {
        return None;
    }
    object.valuestring.as_ref()?;
    let copy = ::std::rc::Rc::new(valuestring.to_owned());
    object.valuestring = Some(cJSON_ValueString::Owned(copy));
    object.valuestring.as_ref().map(cJSON_ValueString::as_c_str)
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
    match cJSON_SetValuestring(object, valuestring) {
        Some(valuestring) => valuestring.as_ptr().cast_mut(),
        None => ::core::ptr::null_mut::<::core::ffi::c_char>(),
    }
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
fn format_number_g(
    number: ::core::ffi::c_double,
    precision: usize,
) -> Option<String> {
    let format = match precision {
        15 => "%1.15g",
        17 => "%1.17g",
        _ => return None,
    };
    sprintf::vsprintf(format, &[&number as &dyn sprintf::Printf])
        .ok()
        .map(normalize_number_exponent)
}
fn print_number(item: &cJSON, output_buffer: &mut printbuffer) -> cJSON_bool {
    let d: ::core::ffi::c_double = item.valuedouble;
    let mut number_bytes = if !d.is_finite() {
        b"null".to_vec()
    } else if d == item.valueint as ::core::ffi::c_double {
        let integer = item.valueint.to_string();
        integer.into_bytes()
    } else {
        let Some(formatted) = format_number_g(d, 15) else {
            return false_0;
        };
        let selected = if formatted
            .parse::<::core::ffi::c_double>()
            .map(|parsed| compare_double(parsed, d))
            .unwrap_or(false_0)
            == 0
        {
            let Some(formatted) = format_number_g(d, 17) else {
                return false_0;
            };
            formatted
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

    let mut output_bytes = ::std::vec::Vec::new();
    if output_bytes.try_reserve_exact(output_size).is_err() {
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
                    input_buffer.offset = input_position;
                    return false_0;
                };
                sequence_length = decoded_length as size_t;
                output_bytes.extend_from_slice(&utf8_bytes[..utf8_length as usize]);
            }
            _ => {
                input_buffer.offset = input_position;
                return false_0;
            }
        }
        input_position = input_position.wrapping_add(sequence_length);
    }

    output_bytes.push(b'\0');
    let Ok(output) = ::std::ffi::CString::from_vec_with_nul(output_bytes) else {
        input_buffer.offset = failure_offset;
        return false_0;
    };
    item.type_0 = cJSON_String;
    item.valuestring = Some(cJSON_ValueString::Owned(::std::rc::Rc::new(output)));
    input_buffer.offset = input_end.wrapping_add(1 as size_t);
    return true_0;
}
fn print_string_ptr(
    input: Option<&::std::ffi::CStr>,
    output_buffer: &mut printbuffer,
) -> cJSON_bool {
    let input_bytes = input.map(::std::ffi::CStr::to_bytes);
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
    let valuestring = item.valuestring.as_ref().map(cJSON_ValueString::as_c_str);
    return print_string_ptr(valuestring, p);
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
) -> (Option<Box<cJSON>>, Option<size_t>) {
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
    let (item, parse_end) = cJSON_ParseWithOpts(value, require_null_terminated);
    if let (Some(offset), Some(return_parse_end)) = (parse_end, return_parse_end.as_mut()) {
        *return_parse_end = value
            .map(::std::ffi::CStr::as_ptr)
            .unwrap_or_else(::core::ptr::null::<::core::ffi::c_char>)
            .wrapping_add(offset);
    }
    item.map(Box::into_raw)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
fn cJSON_ParseWithLengthOpts(
    value: Option<&[::core::ffi::c_uchar]>,
    value_addr: size_t,
    require_null_terminated: cJSON_bool,
) -> (Option<Box<cJSON>>, Option<size_t>) {
    let mut c2rust_current_block: u64;
    let mut buffer: parse_buffer = parse_buffer {
        content: &[],
        length: 0 as size_t,
        offset: 0 as size_t,
        depth: 0 as size_t,
    };
    let mut item: Option<Box<cJSON>> = None;
    GLOBAL_ERROR_JSON_ADDR.store(0 as size_t, ::core::sync::atomic::Ordering::Relaxed);
    GLOBAL_ERROR_POSITION.store(0 as size_t, ::core::sync::atomic::Ordering::Relaxed);
    if let Some(value) = value {
        buffer.content = value;
        buffer.length = value.len();
        buffer.offset = 0 as size_t;
        if !buffer.content.is_empty() {
            let mut parsed = false;
            item = Some(cJSON_New_Item_Box(|item| {
                skip_utf8_bom(buffer.content, &mut buffer.offset);
                buffer_skip_whitespace(&mut buffer);
                parsed = parse_value(item, &mut buffer) != 0;
            }));
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
        drop(item);
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
    let (item, parse_end) =
        cJSON_ParseWithLengthOpts(value_slice, value.addr(), require_null_terminated);
    if let (Some(offset), Some(return_parse_end)) = (parse_end, return_parse_end.as_mut()) {
        *return_parse_end = value.wrapping_add(offset);
    }
    item.map(Box::into_raw)
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
        .map(Box::into_raw)
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
        .map(Box::into_raw)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
fn owned_c_string(input: &::std::ffi::CStr) -> Option<::std::ffi::CString> {
    let mut owned = ::std::vec::Vec::new();
    let input = input.to_bytes_with_nul();
    if owned.try_reserve_exact(input.len()).is_err() {
        return None;
    }
    owned.extend_from_slice(input);
    ::std::ffi::CString::from_vec_with_nul(owned).ok()
}
fn print(item: Option<&cJSON>, format: cJSON_bool) -> Option<::std::ffi::CString> {
    const DEFAULT_BUFFER_SIZE: size_t = 256 as size_t;
    let mut backing = ::std::vec::Vec::new();
    if backing.try_reserve_exact(DEFAULT_BUFFER_SIZE).is_err() {
        return None;
    }
    backing.resize(DEFAULT_BUFFER_SIZE, 0);
    let mut buffer = printbuffer {
        buffer: printbuffer_buffer::Dynamic(backing),
        offset: 0,
        depth: 0,
        format,
    };
    let Some(item) = item else {
        return None;
    };
    if print_value(item, &mut buffer) == 0 {
        return None;
    }
    update_offset(&mut buffer);
    let Some(end) = buffer.offset.checked_add(1) else {
        return None;
    };
    let Some(bytes) = printbuffer_bytes(&buffer).get(..end) else {
        return None;
    };
    let Ok(printed) = ::std::ffi::CStr::from_bytes_with_nul(bytes) else {
        return None;
    };
    owned_c_string(printed)
}
#[export_name = "cJSON_Print"]
pub unsafe extern "C" fn cJSON_Print_ffi(mut item: *const cJSON) -> *mut ::core::ffi::c_char {
    let hooks = get_global_hooks();
    let Some(printed) = print(item.as_ref(), true_0) else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    let bytes = printed.as_bytes_with_nul();
    let copy =
        hooks.allocate.expect("non-null function pointer")(bytes.len()) as *mut ::core::ffi::c_char;
    let Some(copy) = ::core::ptr::NonNull::new(copy) else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    ::core::ptr::copy_nonoverlapping(
        bytes.as_ptr().cast::<::core::ffi::c_char>(),
        copy.as_ptr(),
        bytes.len(),
    );
    copy.as_ptr()
}
#[export_name = "cJSON_PrintUnformatted"]
pub unsafe extern "C" fn cJSON_PrintUnformatted_ffi(
    mut item: *const cJSON,
) -> *mut ::core::ffi::c_char {
    let hooks = get_global_hooks();
    let Some(printed) = print(item.as_ref(), false_0) else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    let bytes = printed.as_bytes_with_nul();
    let copy =
        hooks.allocate.expect("non-null function pointer")(bytes.len()) as *mut ::core::ffi::c_char;
    let Some(copy) = ::core::ptr::NonNull::new(copy) else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    ::core::ptr::copy_nonoverlapping(
        bytes.as_ptr().cast::<::core::ffi::c_char>(),
        copy.as_ptr(),
        bytes.len(),
    );
    copy.as_ptr()
}
pub fn cJSON_PrintBuffered(
    item: Option<&cJSON>,
    prebuffer: ::core::ffi::c_int,
    fmt: cJSON_bool,
) -> Option<::std::ffi::CString> {
    if prebuffer < 0 as ::core::ffi::c_int {
        return None;
    }
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
    owned_c_string(printed)
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
    let hooks = get_global_hooks();
    let Some(printed) = cJSON_PrintBuffered(item.as_ref(), prebuffer, fmt) else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    let bytes = printed.as_bytes_with_nul();
    let copy =
        hooks.allocate.expect("non-null function pointer")(bytes.len()) as *mut ::core::ffi::c_char;
    let Some(copy) = ::core::ptr::NonNull::new(copy) else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    ::core::ptr::copy_nonoverlapping(
        bytes.as_ptr().cast::<::core::ffi::c_char>(),
        copy.as_ptr(),
        bytes.len(),
    );
    copy.as_ptr()
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
            let Some(valuestring) = item.valuestring.as_ref() else {
                return false_0;
            };
            let raw = valuestring.as_c_str().to_bytes_with_nul();
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
#[inline(never)]
fn adopt_child_values(parent: &mut cJSON, children: Vec<cJSON>) {
    parent.children = children;
    parent.reference_child = None;
}
#[inline(never)]
fn delete_child_values(children: Vec<cJSON>) {
    drop(children);
}
fn parse_array(item: &mut cJSON, input_buffer: &mut parse_buffer<'_>) -> cJSON_bool {
    let mut c2rust_current_block: u64;
    let mut children = Vec::new();
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
                let mut new_item = cJSON_New_Item(|_| {});
                let new_item_ref = &mut new_item;
                input_buffer.offset = input_buffer.offset.wrapping_add(1);
                buffer_skip_whitespace(input_buffer);
                if parse_value(new_item_ref, input_buffer) == 0 {
                    c2rust_current_block = 1336238348363633231;
                    break;
                }
                children.push(new_item);
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
                item.type_0 = cJSON_Array;
                adopt_child_values(item, children);
                input_buffer.offset = input_buffer.offset.wrapping_add(1);
                return true_0;
            }
        }
    }
    delete_child_values(children);
    return false_0;
}
fn print_array(item: &cJSON, output_buffer: &mut printbuffer) -> cJSON_bool {
    let mut length: size_t = 0 as size_t;
    let Some(output) = ensure(output_buffer, 1 as size_t) else {
        return false_0;
    };
    output[0] = b'[';
    output_buffer.offset = output_buffer.offset.wrapping_add(1);
    output_buffer.depth = output_buffer.depth.wrapping_add(1);
    let mut index = 0 as size_t;
    while let Some(current) = get_array_item(Some(item), index) {
        if print_value(current, output_buffer) == 0 {
            return false_0;
        }
        update_offset(output_buffer);
        if get_array_item(Some(item), index.wrapping_add(1 as size_t)).is_some() {
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
        index = index.wrapping_add(1 as size_t);
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
    let mut children = Vec::new();
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
                let mut new_item = cJSON_New_Item(|_| {});
                let new_item_ref = &mut new_item;
                if !(input_buffer.offset.wrapping_add(1 as size_t) < input_buffer.length) {
                    c2rust_current_block = 9990476168629568694;
                    break;
                } else {
                    input_buffer.offset = input_buffer.offset.wrapping_add(1);
                    buffer_skip_whitespace(input_buffer);
                    if parse_string(new_item_ref, input_buffer) == 0 {
                        c2rust_current_block = 9990476168629568694;
                        break;
                    } else {
                        buffer_skip_whitespace(input_buffer);
                        new_item_ref.string = new_item_ref.valuestring.take();
                        if !(input_buffer.offset.wrapping_add(0 as size_t) < input_buffer.length)
                            || input_buffer.content[input_buffer.offset] as ::core::ffi::c_int
                                != ':' as i32
                        {
                            c2rust_current_block = 9990476168629568694;
                            break;
                        } else {
                            input_buffer.offset = input_buffer.offset.wrapping_add(1);
                            buffer_skip_whitespace(input_buffer);
                            if parse_value(new_item_ref, input_buffer) == 0 {
                                c2rust_current_block = 9990476168629568694;
                                break;
                            } else {
                                children.push(new_item);
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
                item.type_0 = cJSON_Object;
                adopt_child_values(item, children);
                input_buffer.offset = input_buffer.offset.wrapping_add(1);
                return true_0;
            }
        }
    }
    delete_child_values(children);
    return false_0;
}
fn print_object(item: &cJSON, output_buffer: &mut printbuffer) -> cJSON_bool {
    let mut length: size_t = 0 as size_t;
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
    let mut index = 0 as size_t;
    while let Some(current) = get_array_item(Some(item), index) {
        if output_buffer.format != 0 {
            let depth = output_buffer.depth;
            let Some(output) = ensure(output_buffer, depth) else {
                return false_0;
            };
            output.fill(b'\t');
            output_buffer.offset = output_buffer.offset.wrapping_add(depth);
        }
        let current_string = current.string.as_ref().map(cJSON_ValueString::as_c_str);
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
        if print_value(current, output_buffer) == 0 {
            return false_0;
        }
        update_offset(output_buffer);
        let has_next = get_array_item(Some(item), index.wrapping_add(1 as size_t)).is_some();
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
        output_buffer.offset = output_buffer.offset.wrapping_add(length);
        index = index.wrapping_add(1 as size_t);
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
    while get_array_item(Some(array), size).is_some() {
        size = size.wrapping_add(1 as size_t);
    }
    return size as ::core::ffi::c_int;
}
#[export_name = "cJSON_GetArraySize"]
pub unsafe extern "C" fn cJSON_GetArraySize_ffi(mut array: *const cJSON) -> ::core::ffi::c_int {
    cJSON_GetArraySize(array.as_ref())
}
fn first_child(item: &cJSON) -> Option<&cJSON> {
    if item.type_0 & cJSON_IsReference != 0 {
        item.reference_child.or_else(|| item.children.first())
    } else {
        item.children.first()
    }
}
fn get_array_item<'a>(array: Option<&'a cJSON>, mut index: size_t) -> Option<&'a cJSON> {
    let array = array?;
    if array.type_0 & cJSON_IsReference != 0 {
        if index == 0 {
            return array.reference_child.or_else(|| array.children.first());
        }
        return None;
    }
    array.children.get(index as usize)
}
pub fn cJSON_GetArrayItem(array: Option<&cJSON>, index: ::core::ffi::c_int) -> Option<&cJSON> {
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
    match cJSON_GetArrayItem(array.as_ref(), index) {
        Some(item) => ::core::ptr::from_ref(item).cast_mut(),
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
enum ObjectItemName<'a> {
    CStr(&'a ::std::ffi::CStr),
    ItemString(&'a cJSON),
}
fn get_object_item<'a>(
    object: &'a cJSON,
    name: Option<ObjectItemName<'_>>,
    case_sensitive: cJSON_bool,
) -> Option<&'a cJSON> {
    let name = name?;
    let name = match name {
        ObjectItemName::CStr(name) => name,
        ObjectItemName::ItemString(item) => item.string.as_ref()?.as_c_str(),
    };
    let mut index = 0 as size_t;
    while let Some(current) = get_array_item(Some(object), index) {
        let current_string = current.string.as_ref();
        if current_string.is_none() {
            if case_sensitive != 0 {
                return None;
            }
        } else {
            let current_name = current_string
                .expect("checked non-null object key")
                .as_c_str();
            if if case_sensitive != 0 {
                name.to_bytes_with_nul() == current_name.to_bytes_with_nul()
            } else {
                case_insensitive_strcmp(name, current_name) == 0 as ::core::ffi::c_int
            } {
                return Some(current);
            }
        }
        index = index.wrapping_add(1 as size_t);
    }
    None
}
pub fn cJSON_GetObjectItem<'a>(
    object: Option<&'a cJSON>,
    string: Option<&::std::ffi::CStr>,
) -> Option<&'a cJSON> {
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
    match cJSON_GetObjectItem(object.as_ref(), string) {
        Some(item) => ::core::ptr::from_ref(item).cast_mut(),
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_GetObjectItemCaseSensitive<'a>(
    object: Option<&'a cJSON>,
    string: Option<&::std::ffi::CStr>,
) -> Option<&'a cJSON> {
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
    match cJSON_GetObjectItemCaseSensitive(object.as_ref(), string) {
        Some(item) => ::core::ptr::from_ref(item).cast_mut(),
        None => ::core::ptr::null_mut::<cJSON>(),
    }
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
fn create_reference_value(item: Option<&cJSON>) -> Option<cJSON> {
    let item = item?;
    Some(cJSON_New_Item(|reference| {
        *reference = item.clone();
        reference.string = None;
        reference.type_0 |= cJSON_IsReference;
    }))
}
fn add_item_to_array_value<'a>(array: &'a mut cJSON, item: cJSON) -> &'a mut cJSON {
    array.children.push(item);
    array.children.last_mut().expect("inserted array item")
}
pub fn cJSON_AddItemToArray(array: &mut cJSON, item: cJSON) -> cJSON_bool {
    add_item_to_array_value(array, item);
    true_0
}
#[export_name = "cJSON_AddItemToArray"]
pub unsafe extern "C" fn cJSON_AddItemToArray_ffi(
    mut array: *mut cJSON,
    mut item: *mut cJSON,
) -> cJSON_bool {
    if array.is_null() || item.is_null() || array == item {
        return false_0;
    }
    let array = array.as_mut().expect("checked non-null array pointer");
    let item = *Box::from_raw(item);
    cJSON_AddItemToArray(array, item)
}
enum ObjectKey<'a> {
    Owned(&'a ::std::ffi::CStr),
    Reference(&'static ::std::ffi::CStr),
}
fn add_item_to_object_value<'a>(
    object: &'a mut cJSON,
    string: ObjectKey<'_>,
    mut item: cJSON,
) -> &'a mut cJSON {
    let (new_key, new_type) = match string {
        ObjectKey::Reference(string) => (
            cJSON_ValueString::Reference(string),
            item.type_0 | cJSON_StringIsConst,
        ),
        ObjectKey::Owned(string) => (
            cJSON_ValueString::Owned(::std::rc::Rc::new(string.to_owned())),
            item.type_0 & !cJSON_StringIsConst,
        ),
    };
    item.string = Some(new_key);
    item.type_0 = new_type;
    add_item_to_array_value(object, item)
}
pub fn cJSON_AddItemToObject(
    object: &mut cJSON,
    string: &::std::ffi::CStr,
    item: cJSON,
) -> cJSON_bool {
    add_item_to_object_value(object, ObjectKey::Owned(string), item);
    true_0
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
    let object = object.as_mut().expect("checked non-null object pointer");
    let item = *Box::from_raw(item);
    cJSON_AddItemToObject(object, ::std::ffi::CStr::from_ptr(string), item)
}
pub fn cJSON_AddItemToObjectCS(
    object: &mut cJSON,
    string: &'static ::std::ffi::CStr,
    item: cJSON,
) -> cJSON_bool {
    add_item_to_object_value(object, ObjectKey::Reference(string), item);
    true_0
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
    let object = object.as_mut().expect("checked non-null object pointer");
    let item = *Box::from_raw(item);
    cJSON_AddItemToObjectCS(object, ::std::ffi::CStr::from_ptr(string), item)
}
pub fn cJSON_AddItemReferenceToArray(array: &mut cJSON, item: &cJSON) -> cJSON_bool {
    let Some(reference) = create_reference_value(Some(item)) else {
        return false_0;
    };
    cJSON_AddItemToArray(array, reference)
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
    let Some(array) = array.as_mut() else {
        return false_0;
    };
    cJSON_AddItemReferenceToArray(array, item)
}
pub fn cJSON_AddItemReferenceToObject(
    object: &mut cJSON,
    string: &::std::ffi::CStr,
    item: &cJSON,
) -> cJSON_bool {
    let Some(reference) = create_reference_value(Some(item)) else {
        return false_0;
    };
    add_item_to_object_value(object, ObjectKey::Owned(string), reference);
    true_0
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
    let Some(object) = object.as_mut() else {
        return false_0;
    };
    cJSON_AddItemReferenceToObject(object, ::std::ffi::CStr::from_ptr(string), item)
}
pub fn cJSON_AddNullToObject<'a>(
    object: Option<&'a mut cJSON>,
    name: Option<&::std::ffi::CStr>,
) -> Option<&'a mut cJSON> {
    let null = cJSON_New_Item(|item| cjson_init_item_type(item, cJSON_NULL));
    let (Some(object), Some(name)) = (object, name) else {
        return None;
    };
    Some(add_item_to_object_value(
        object,
        ObjectKey::Owned(name),
        null,
    ))
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
    match cJSON_AddNullToObject(object, name) {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_AddTrueToObject<'a>(
    object: Option<&'a mut cJSON>,
    name: Option<&::std::ffi::CStr>,
) -> Option<&'a mut cJSON> {
    let true_item = cJSON_New_Item(|item| cjson_init_item_type(item, cJSON_True));
    let (Some(object), Some(name)) = (object, name) else {
        return None;
    };
    Some(add_item_to_object_value(
        object,
        ObjectKey::Owned(name),
        true_item,
    ))
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
    match cJSON_AddTrueToObject(object, name) {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_AddFalseToObject<'a>(
    object: Option<&'a mut cJSON>,
    name: Option<&::std::ffi::CStr>,
) -> Option<&'a mut cJSON> {
    let false_item = cJSON_New_Item(|item| cjson_init_item_type(item, cJSON_False));
    let (Some(object), Some(name)) = (object, name) else {
        return None;
    };
    Some(add_item_to_object_value(
        object,
        ObjectKey::Owned(name),
        false_item,
    ))
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
    match cJSON_AddFalseToObject(object, name) {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_AddBoolToObject<'a>(
    object: Option<&'a mut cJSON>,
    name: Option<&::std::ffi::CStr>,
    boolean: cJSON_bool,
) -> Option<&'a mut cJSON> {
    let bool_item = cJSON_New_Item(|item| {
        cjson_init_item_type(
            item,
            if boolean != 0 {
                cJSON_True
            } else {
                cJSON_False
            },
        );
    });
    let (Some(object), Some(name)) = (object, name) else {
        return None;
    };
    Some(add_item_to_object_value(
        object,
        ObjectKey::Owned(name),
        bool_item,
    ))
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
    match cJSON_AddBoolToObject(object, name, boolean) {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_AddNumberToObject<'a>(
    object: Option<&'a mut cJSON>,
    name: Option<&::std::ffi::CStr>,
    number: ::core::ffi::c_double,
) -> Option<&'a mut cJSON> {
    let number_item = cJSON_New_Item(|item| cjson_init_number_item(item, number));
    let (Some(object), Some(name)) = (object, name) else {
        return None;
    };
    Some(add_item_to_object_value(
        object,
        ObjectKey::Owned(name),
        number_item,
    ))
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
    match cJSON_AddNumberToObject(object, name, number) {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_AddStringToObject<'a>(
    object: Option<&'a mut cJSON>,
    name: Option<&::std::ffi::CStr>,
    string: Option<&::std::ffi::CStr>,
) -> Option<&'a mut cJSON> {
    let string = string?;
    let string_item = cJSON_New_Item(|item| {
        item.type_0 = cJSON_String;
        item.valuestring = Some(cJSON_ValueString::Owned(::std::rc::Rc::new(
            string.to_owned(),
        )));
    });
    let (Some(object), Some(name)) = (object, name) else {
        return None;
    };
    Some(add_item_to_object_value(
        object,
        ObjectKey::Owned(name),
        string_item,
    ))
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
    match cJSON_AddStringToObject(object, name, string) {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_AddRawToObject<'a>(
    object: Option<&'a mut cJSON>,
    name: Option<&::std::ffi::CStr>,
    raw: Option<&::std::ffi::CStr>,
) -> Option<&'a mut cJSON> {
    let raw = raw?;
    let raw_item = cJSON_New_Item(|item| {
        item.type_0 = cJSON_Raw;
        item.valuestring = Some(cJSON_ValueString::Owned(::std::rc::Rc::new(raw.to_owned())));
    });
    let (Some(object), Some(name)) = (object, name) else {
        return None;
    };
    Some(add_item_to_object_value(
        object,
        ObjectKey::Owned(name),
        raw_item,
    ))
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
    match cJSON_AddRawToObject(object, name, raw) {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_AddObjectToObject<'a>(
    object: Option<&'a mut cJSON>,
    name: Option<&::std::ffi::CStr>,
) -> Option<&'a mut cJSON> {
    let object_item = cJSON_New_Item(|item| cjson_init_item_type(item, cJSON_Object));
    let (Some(object), Some(name)) = (object, name) else {
        return None;
    };
    Some(add_item_to_object_value(
        object,
        ObjectKey::Owned(name),
        object_item,
    ))
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
    match cJSON_AddObjectToObject(object, name) {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_AddArrayToObject<'a>(
    object: Option<&'a mut cJSON>,
    name: Option<&::std::ffi::CStr>,
) -> Option<&'a mut cJSON> {
    let array = cJSON_New_Item(|item| cjson_init_item_type(item, cJSON_Array));
    let (Some(object), Some(name)) = (object, name) else {
        return None;
    };
    Some(add_item_to_object_value(
        object,
        ObjectKey::Owned(name),
        array,
    ))
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
    match cJSON_AddArrayToObject(object, name) {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_DetachItemViaPointer(parent: &mut cJSON, item: Option<&cJSON>) -> Option<cJSON> {
    let item = item?;
    let index = parent
        .children
        .iter()
        .position(|current| ::core::ptr::eq(current, item))?;
    Some(parent.children.remove(index))
}
#[export_name = "cJSON_DetachItemViaPointer"]
pub unsafe extern "C" fn cJSON_DetachItemViaPointer_ffi(
    mut parent: *mut cJSON,
    item: *mut cJSON,
) -> *mut cJSON {
    let Some(parent) = parent.as_mut() else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let item = item.as_ref();
    cJSON_DetachItemViaPointer(parent, item)
        .map(Box::new)
        .map(Box::into_raw)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
fn detach_child_at(parent: &mut cJSON, index: size_t) -> Option<cJSON> {
    if index >= parent.children.len() {
        return None;
    }
    Some(parent.children.remove(index))
}
pub fn cJSON_DetachItemFromArray(
    array: Option<&mut cJSON>,
    which: ::core::ffi::c_int,
) -> Option<cJSON> {
    if which < 0 as ::core::ffi::c_int {
        return None;
    }
    let Some(array) = array else {
        return None;
    };
    detach_child_at(array, which as size_t)
}
#[export_name = "cJSON_DetachItemFromArray"]
pub unsafe extern "C" fn cJSON_DetachItemFromArray_ffi(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
) -> *mut cJSON {
    cJSON_DetachItemFromArray(array.as_mut(), which)
        .map(Box::new)
        .map(Box::into_raw)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_DeleteItemFromArray(array: Option<&mut cJSON>, which: ::core::ffi::c_int) {
    if let Some(item) = cJSON_DetachItemFromArray(array, which) {
        drop(item);
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
) -> Option<cJSON> {
    let object = object?;
    let string = string?;
    let mut index = 0 as size_t;
    while let Some(current) = get_array_item(Some(&*object), index) {
        let current_name = current.string.as_ref().map(cJSON_ValueString::as_c_str);
        if current_name
            .map(|current_name| case_insensitive_strcmp(string, current_name) == 0)
            .unwrap_or(false)
        {
            return detach_child_at(object, index);
        }
        index = index.wrapping_add(1);
    }
    None
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
        .map(Box::new)
        .map(Box::into_raw)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_DetachItemFromObjectCaseSensitive(
    object: Option<&mut cJSON>,
    string: Option<&::std::ffi::CStr>,
) -> Option<cJSON> {
    let object = object?;
    let string = string?;
    let mut index = 0 as size_t;
    while let Some(current) = get_array_item(Some(&*object), index) {
        let current_name = current.string.as_ref().map(cJSON_ValueString::as_c_str);
        if current_name
            .map(|current_name| current_name.to_bytes_with_nul() == string.to_bytes_with_nul())
            .unwrap_or(false)
        {
            return detach_child_at(object, index);
        }
        index = index.wrapping_add(1);
    }
    None
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
        .map(Box::new)
        .map(Box::into_raw)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_DeleteItemFromObject(object: Option<&mut cJSON>, string: Option<&::std::ffi::CStr>) {
    if let Some(item) = cJSON_DetachItemFromObject(object, string) {
        drop(item);
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
        drop(item);
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
    newitem: cJSON,
) -> cJSON_bool {
    if which < 0 as ::core::ffi::c_int {
        return false_0;
    }
    if which as size_t >= cJSON_GetArraySize(Some(array)) as size_t {
        return cJSON_AddItemToArray(array, newitem);
    }
    array.children.insert(which as usize, newitem);
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
    let array = array.as_mut().expect("checked non-null array pointer");
    let newitem = *Box::from_raw(newitem);
    if cJSON_InsertItemInArray(array, which, newitem) != 0 {
        true_0
    } else {
        false_0
    }
}
fn cJSON_ReplaceItemViaPointer(
    parent: Option<&mut cJSON>,
    item: Option<&cJSON>,
    replacement: cJSON,
) -> Result<(), cJSON> {
    let Some(parent) = parent else {
        return Err(replacement);
    };
    if parent.children.is_empty() {
        return Err(replacement);
    }
    let Some(item) = item else {
        return Err(replacement);
    };
    if let Some(index) = parent
        .children
        .iter()
        .position(|current| ::core::ptr::eq(current, item))
    {
        parent.children[index] = replacement;
        Ok(())
    } else {
        Err(replacement)
    }
}
#[export_name = "cJSON_ReplaceItemViaPointer"]
pub unsafe extern "C" fn cJSON_ReplaceItemViaPointer_ffi(
    mut parent: *mut cJSON,
    item: *mut cJSON,
    mut replacement: *mut cJSON,
) -> cJSON_bool {
    let replacement = if replacement.is_null() {
        return false_0;
    } else {
        *Box::from_raw(replacement)
    };
    match cJSON_ReplaceItemViaPointer(parent.as_mut(), item.as_ref(), replacement) {
        Ok(()) => true_0,
        Err(replacement) => {
            let _ = Box::into_raw(Box::new(replacement));
            false_0
        }
    }
}
fn replace_child_at(parent: &mut cJSON, index: size_t, replacement: cJSON) -> cJSON_bool {
    if index >= parent.children.len() {
        return false_0;
    }
    parent.children[index] = replacement;
    true_0
}
pub fn cJSON_ReplaceItemInArray(
    array: &mut cJSON,
    which: ::core::ffi::c_int,
    newitem: cJSON,
) -> cJSON_bool {
    if which < 0 as ::core::ffi::c_int {
        return false_0;
    }
    replace_child_at(array, which as size_t, newitem)
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
    let array = array.as_mut().expect("checked non-null array pointer");
    let newitem = *Box::from_raw(newitem);
    cJSON_ReplaceItemInArray(array, which, newitem)
}
fn replace_item_in_object(
    object: Option<&mut cJSON>,
    string: Option<&::std::ffi::CStr>,
    replacement: Option<cJSON>,
    mut case_sensitive: cJSON_bool,
) -> cJSON_bool {
    let (Some(string), Some(mut replacement)) = (string, replacement) else {
        return false_0;
    };
    replacement.string = Some(cJSON_ValueString::Owned(::std::rc::Rc::new(
        string.to_owned(),
    )));
    replacement.type_0 &= !cJSON_StringIsConst;
    let Some(object) = object else {
        return false_0;
    };
    let mut index = 0 as size_t;
    while let Some(current) = get_array_item(Some(&*object), index) {
        let current_name = current.string.as_ref().map(cJSON_ValueString::as_c_str);
        let matches = current_name
            .map(|current_name| {
                if case_sensitive != 0 {
                    current_name.to_bytes_with_nul() == string.to_bytes_with_nul()
                } else {
                    case_insensitive_strcmp(string, current_name) == 0
                }
            })
            .unwrap_or(false);
        if matches {
            return replace_child_at(object, index, replacement);
        }
        index = index.wrapping_add(1);
    }
    false_0
}
fn cJSON_ReplaceItemInObject(
    object: Option<&mut cJSON>,
    string: Option<&::std::ffi::CStr>,
    newitem: Option<cJSON>,
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
    let newitem = if newitem.is_null() {
        None
    } else {
        Some(*Box::from_raw(newitem))
    };
    cJSON_ReplaceItemInObject(object.as_mut(), string, newitem)
}
fn cJSON_ReplaceItemInObjectCaseSensitive(
    object: Option<&mut cJSON>,
    string: Option<&::std::ffi::CStr>,
    newitem: Option<cJSON>,
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
    let newitem = if newitem.is_null() {
        None
    } else {
        Some(*Box::from_raw(newitem))
    };
    cJSON_ReplaceItemInObjectCaseSensitive(object.as_mut(), string, newitem)
}
fn cjson_init_item_type(item: &mut cJSON, type_0: ::core::ffi::c_int) {
    item.type_0 = type_0;
}
fn cjson_init_number_item(item: &mut cJSON, number: ::core::ffi::c_double) {
    cjson_init_item_type(item, cJSON_Number);
    cJSON_SetNumberHelper(item, number);
}
fn cJSON_CreateNull_Box() -> Box<cJSON> {
    cJSON_New_Item_Box(|item| cjson_init_item_type(item, cJSON_NULL))
}
pub fn cJSON_CreateNull() -> Option<Box<cJSON>> {
    Some(cJSON_CreateNull_Box())
}
#[export_name = "cJSON_CreateNull"]
pub unsafe extern "C" fn cJSON_CreateNull_ffi() -> *mut cJSON {
    cJSON_CreateNull()
        .map(Box::into_raw)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
fn cJSON_CreateTrue_Box() -> Box<cJSON> {
    cJSON_New_Item_Box(|item| cjson_init_item_type(item, cJSON_True))
}
pub fn cJSON_CreateTrue() -> Option<Box<cJSON>> {
    Some(cJSON_CreateTrue_Box())
}
#[export_name = "cJSON_CreateTrue"]
pub unsafe extern "C" fn cJSON_CreateTrue_ffi() -> *mut cJSON {
    cJSON_CreateTrue()
        .map(Box::into_raw)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
fn cJSON_CreateFalse_Box() -> Box<cJSON> {
    cJSON_New_Item_Box(|item| cjson_init_item_type(item, cJSON_False))
}
pub fn cJSON_CreateFalse() -> Option<Box<cJSON>> {
    Some(cJSON_CreateFalse_Box())
}
#[export_name = "cJSON_CreateFalse"]
pub unsafe extern "C" fn cJSON_CreateFalse_ffi() -> *mut cJSON {
    cJSON_CreateFalse()
        .map(Box::into_raw)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
fn cJSON_CreateBool_Box(boolean: cJSON_bool) -> Box<cJSON> {
    cJSON_New_Item_Box(|item| {
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
pub fn cJSON_CreateBool(boolean: cJSON_bool) -> Option<Box<cJSON>> {
    Some(cJSON_CreateBool_Box(boolean))
}
#[export_name = "cJSON_CreateBool"]
pub unsafe extern "C" fn cJSON_CreateBool_ffi(mut boolean: cJSON_bool) -> *mut cJSON {
    cJSON_CreateBool(boolean)
        .map(Box::into_raw)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
fn cJSON_CreateNumber_Box(num: ::core::ffi::c_double) -> Box<cJSON> {
    cJSON_New_Item_Box(|item| cjson_init_number_item(item, num))
}
pub fn cJSON_CreateNumber(num: ::core::ffi::c_double) -> Option<Box<cJSON>> {
    Some(cJSON_CreateNumber_Box(num))
}
#[export_name = "cJSON_CreateNumber"]
pub unsafe extern "C" fn cJSON_CreateNumber_ffi(mut num: ::core::ffi::c_double) -> *mut cJSON {
    cJSON_CreateNumber(num)
        .map(Box::into_raw)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
fn cJSON_CreateString_Box(string: Option<&::std::ffi::CStr>) -> Option<Box<cJSON>> {
    let string = string?;
    Some(cJSON_New_Item_Box(|item| {
        item.type_0 = cJSON_String;
        item.valuestring = Some(cJSON_ValueString::Owned(::std::rc::Rc::new(
            string.to_owned(),
        )));
    }))
}
pub fn cJSON_CreateString(string: Option<&::std::ffi::CStr>) -> Option<Box<cJSON>> {
    cJSON_CreateString_Box(string)
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
        .map(Box::into_raw)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_CreateStringReference(
    string: Option<&'static ::std::ffi::CStr>,
) -> Option<Box<cJSON>> {
    Some(cJSON_New_Item_Box(|item| {
        item.type_0 = cJSON_String | cJSON_IsReference;
        item.valuestring = string.map(cJSON_ValueString::Reference);
    }))
}
#[export_name = "cJSON_CreateStringReference"]
pub unsafe extern "C" fn cJSON_CreateStringReference_ffi(
    mut string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let string: Option<&'static ::std::ffi::CStr> = if string.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(string))
    };
    cJSON_CreateStringReference(string)
        .map(Box::into_raw)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_CreateObjectReference(child: Option<&'static cJSON>) -> Option<Box<cJSON>> {
    Some(cJSON_New_Item_Box(|item| {
        item.type_0 = cJSON_Object | cJSON_IsReference;
        item.reference_child = child;
    }))
}
#[export_name = "cJSON_CreateObjectReference"]
pub unsafe extern "C" fn cJSON_CreateObjectReference_ffi(mut child: *const cJSON) -> *mut cJSON {
    let child = child.as_ref();
    cJSON_CreateObjectReference(child)
        .map(Box::into_raw)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_CreateArrayReference(child: Option<&'static cJSON>) -> Option<Box<cJSON>> {
    Some(cJSON_New_Item_Box(|item| {
        item.type_0 = cJSON_Array | cJSON_IsReference;
        item.reference_child = child;
    }))
}
#[export_name = "cJSON_CreateArrayReference"]
pub unsafe extern "C" fn cJSON_CreateArrayReference_ffi(mut child: *const cJSON) -> *mut cJSON {
    let child = child.as_ref();
    cJSON_CreateArrayReference(child)
        .map(Box::into_raw)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
fn cJSON_CreateRaw_Box(raw: Option<&::std::ffi::CStr>) -> Option<Box<cJSON>> {
    let raw = raw?;
    Some(cJSON_New_Item_Box(|item| {
        item.type_0 = cJSON_Raw;
        item.valuestring = Some(cJSON_ValueString::Owned(::std::rc::Rc::new(raw.to_owned())));
    }))
}
pub fn cJSON_CreateRaw(raw: Option<&::std::ffi::CStr>) -> Option<Box<cJSON>> {
    cJSON_CreateRaw_Box(raw)
}
#[export_name = "cJSON_CreateRaw"]
pub unsafe extern "C" fn cJSON_CreateRaw_ffi(mut raw: *const ::core::ffi::c_char) -> *mut cJSON {
    let raw = if raw.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(raw))
    };
    cJSON_CreateRaw(raw)
        .map(Box::into_raw)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_CreateArray() -> Option<Box<cJSON>> {
    Some(cJSON_New_Item_Box(|item| {
        cjson_init_item_type(item, cJSON_Array)
    }))
}
#[export_name = "cJSON_CreateArray"]
pub unsafe extern "C" fn cJSON_CreateArray_ffi() -> *mut cJSON {
    cJSON_CreateArray()
        .map(Box::into_raw)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_CreateObject() -> Option<Box<cJSON>> {
    Some(cJSON_New_Item_Box(|item| {
        cjson_init_item_type(item, cJSON_Object)
    }))
}
#[export_name = "cJSON_CreateObject"]
pub unsafe extern "C" fn cJSON_CreateObject_ffi() -> *mut cJSON {
    cJSON_CreateObject()
        .map(Box::into_raw)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
fn transfer_item_to_array(array: &mut cJSON, item: cJSON) -> cJSON_bool {
    cJSON_AddItemToArray(array, item)
}
pub fn cJSON_CreateIntArray(numbers: &[::core::ffi::c_int]) -> Option<Box<cJSON>> {
    let mut a = cJSON_CreateArray()?;
    for number in numbers {
        let n =
            cJSON_New_Item(|item| cjson_init_number_item(item, *number as ::core::ffi::c_double));
        if transfer_item_to_array(a.as_mut(), n) == 0 {
            return None;
        }
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
        .map(Box::into_raw)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_CreateFloatArray(numbers: &[::core::ffi::c_float]) -> Option<Box<cJSON>> {
    let mut a = cJSON_CreateArray()?;
    for number in numbers {
        let n =
            cJSON_New_Item(|item| cjson_init_number_item(item, *number as ::core::ffi::c_double));
        if transfer_item_to_array(a.as_mut(), n) == 0 {
            return None;
        }
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
        .map(Box::into_raw)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_CreateDoubleArray(numbers: &[::core::ffi::c_double]) -> Option<Box<cJSON>> {
    let mut a = cJSON_CreateArray()?;
    for number in numbers {
        let n = cJSON_New_Item(|item| cjson_init_number_item(item, *number));
        if transfer_item_to_array(a.as_mut(), n) == 0 {
            return None;
        }
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
        .map(Box::into_raw)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_CreateStringArray(strings: &[Option<&::std::ffi::CStr>]) -> Option<Box<cJSON>> {
    let mut a = cJSON_CreateArray()?;
    for string in strings.iter().copied() {
        let string = string?;
        let n = cJSON_New_Item(|item| {
            item.type_0 = cJSON_String;
            item.valuestring = Some(cJSON_ValueString::Owned(::std::rc::Rc::new(
                string.to_owned(),
            )));
        });
        if transfer_item_to_array(a.as_mut(), n) == 0 {
            return None;
        }
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
        .map(Box::into_raw)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_Duplicate(item: Option<&cJSON>, recurse: cJSON_bool) -> Option<Box<cJSON>> {
    cJSON_Duplicate_rec(item, 0 as size_t, recurse)
}
#[export_name = "cJSON_Duplicate"]
pub unsafe extern "C" fn cJSON_Duplicate_ffi(
    mut item: *const cJSON,
    mut recurse: cJSON_bool,
) -> *mut cJSON {
    cJSON_Duplicate(item.as_ref(), recurse)
        .map(Box::into_raw)
        .unwrap_or_else(::core::ptr::null_mut::<cJSON>)
}
pub fn cJSON_Duplicate_rec(
    item: Option<&cJSON>,
    depth: size_t,
    recurse: cJSON_bool,
) -> Option<Box<cJSON>> {
    cJSON_Duplicate_rec_value(item, depth, recurse).map(Box::new)
}
fn cJSON_Duplicate_rec_value(
    item: Option<&cJSON>,
    depth: size_t,
    recurse: cJSON_bool,
) -> Option<cJSON> {
    let item = item?;
    let valuestring = match item.valuestring.as_ref() {
        Some(cJSON_ValueString::Owned(valuestring)) => Some(cJSON_ValueString::Owned(
            ::std::rc::Rc::new((**valuestring).clone()),
        )),
        Some(cJSON_ValueString::Reference(valuestring)) => Some(cJSON_ValueString::Owned(
            ::std::rc::Rc::new((*valuestring).to_owned()),
        )),
        None => None,
    };
    let string = match item.string.as_ref() {
        Some(cJSON_ValueString::Owned(string)) if item.type_0 & cJSON_StringIsConst != 0 => {
            Some(cJSON_ValueString::Owned(::std::rc::Rc::clone(string)))
        }
        Some(cJSON_ValueString::Owned(string)) => Some(cJSON_ValueString::Owned(
            ::std::rc::Rc::new((**string).clone()),
        )),
        Some(cJSON_ValueString::Reference(string)) if item.type_0 & cJSON_StringIsConst != 0 => {
            Some(cJSON_ValueString::Reference(*string))
        }
        Some(cJSON_ValueString::Reference(string)) => Some(cJSON_ValueString::Owned(
            ::std::rc::Rc::new((*string).to_owned()),
        )),
        None => None,
    };
    let mut newitem = cJSON_New_Item(|newitem| {
        newitem.type_0 = item.type_0 & !cJSON_IsReference;
        newitem.valueint = item.valueint;
        newitem.valuedouble = item.valuedouble;
        newitem.valuestring = valuestring;
        newitem.string = string;
    });
    if recurse == 0 {
        return Some(newitem);
    }
    let mut child_index = 0 as size_t;
    while let Some(child) = get_array_item(Some(item), child_index) {
        if depth >= CJSON_CIRCULAR_LIMIT as size_t {
            return None;
        }
        let Some(newchild) =
            cJSON_Duplicate_rec_value(Some(child), depth.wrapping_add(1 as size_t), true_0)
        else {
            return None;
        };
        if transfer_item_to_array(&mut newitem, newchild) == 0 {
            return None;
        }
        child_index = child_index.wrapping_add(1 as size_t);
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
        .map(Box::into_raw)
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
            let (Some(a_valuestring), Some(b_valuestring)) =
                (a_item.valuestring.as_ref(), b_item.valuestring.as_ref())
            else {
                return false_0;
            };
            if a_valuestring.as_c_str().to_bytes_with_nul()
                == b_valuestring.as_c_str().to_bytes_with_nul()
            {
                return true_0;
            }
            return false_0;
        }
        cJSON_Array => {
            let mut index = 0 as size_t;
            loop {
                let a_child = get_array_item(Some(a_item), index);
                let b_child = get_array_item(Some(b_item), index);
                let (Some(a_child), Some(b_child)) = (a_child, b_child) else {
                    return (a_child.is_none() && b_child.is_none()) as cJSON_bool;
                };
                if cJSON_Compare(Some(a_child), Some(b_child), case_sensitive) == 0 {
                    return false_0;
                }
                index = index.wrapping_add(1 as size_t);
            }
        }
        cJSON_Object => {
            let mut index = 0 as size_t;
            while let Some(a_child) = get_array_item(Some(a_item), index) {
                let Some(b_child) = get_object_item(
                    b_item,
                    Some(ObjectItemName::ItemString(a_child)),
                    case_sensitive,
                ) else {
                    return false_0;
                };
                if cJSON_Compare(Some(a_child), Some(b_child), case_sensitive) == 0 {
                    return false_0;
                }
                index = index.wrapping_add(1 as size_t);
            }
            let mut index = 0 as size_t;
            while let Some(b_child) = get_array_item(Some(b_item), index) {
                let Some(a_child) = get_object_item(
                    a_item,
                    Some(ObjectItemName::ItemString(b_child)),
                    case_sensitive,
                ) else {
                    return false_0;
                };
                if cJSON_Compare(Some(b_child), Some(a_child), case_sensitive) == 0 {
                    return false_0;
                }
                index = index.wrapping_add(1 as size_t);
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
    get_global_hooks()
        .allocate
        .expect("non-null function pointer")(size)
}
#[export_name = "cJSON_free"]
pub unsafe extern "C" fn cJSON_free_ffi(mut object: *mut ::core::ffi::c_void) {
    get_global_hooks()
        .deallocate
        .expect("non-null function pointer")(object);
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __DBL_EPSILON__: ::core::ffi::c_double = 2.2204460492503131e-16f64;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const INT_MIN: ::core::ffi::c_int = -2147483647 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
pub const DBL_EPSILON: ::core::ffi::c_double = __DBL_EPSILON__;
