unsafe extern "C" {
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    safe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    safe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    safe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
#[derive(Clone, Default)]
#[repr(C)]
pub struct cJSON {
    pub next: ::core::cell::Cell<Option<&'static cJSON>>,
    pub prev: ::core::cell::Cell<Option<&'static cJSON>>,
    pub child: ::core::cell::Cell<Option<&'static cJSON>>,
    pub type_0: ::core::ffi::c_int,
    pub valuestring: Option<&'static ::core::ffi::CStr>,
    pub valuestring_len: size_t,
    pub valuestring_storage: Option<Vec<::core::ffi::c_uchar>>,
    pub valuestring_buffer: Option<Vec<::core::ffi::c_char>>,
    pub valueint: ::core::ffi::c_int,
    pub valuedouble: ::core::ffi::c_double,
    pub string: Option<()>,
    pub string_storage: Option<Vec<::core::ffi::c_uchar>>,
    pub string_buffer: Option<Vec<::core::ffi::c_char>>,
}

fn same_item(left: &cJSON, right: &cJSON) -> bool {
    ::core::ptr::eq(left, right)
}

fn same_item_option(left: Option<&cJSON>, right: Option<&cJSON>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => same_item(left, right),
        (None, None) => true,
        _ => false,
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cJSON_Hooks {
    pub malloc_fn: Option<extern "C" fn(size_t) -> *mut ::core::ffi::c_void>,
    pub free_fn: Option<extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
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
pub struct error {
    pub json: ::std::sync::atomic::AtomicPtr<::core::ffi::c_uchar>,
    pub position: ::std::sync::atomic::AtomicUsize,
}
#[derive(Copy, Clone)]
pub struct parse_buffer<'a> {
    pub content: &'a [::core::ffi::c_uchar],
    pub length: size_t,
    pub offset: size_t,
    pub depth: size_t,
    pub hooks: internal_hooks,
}

impl<'a> parse_buffer<'a> {
    fn can_access_at(&self, index: size_t) -> bool {
        self.offset.wrapping_add(index) < self.length
    }

    fn current_byte(&self) -> Option<::core::ffi::c_uchar> {
        if self.can_access_at(0 as size_t) {
            self.content.get(self.offset).copied()
        } else {
            None
        }
    }

    fn byte_at(&self, index: size_t) -> Option<::core::ffi::c_uchar> {
        if self.can_access_at(index) {
            self.content.get(self.offset.wrapping_add(index)).copied()
        } else {
            None
        }
    }

    fn remaining(&self) -> &[::core::ffi::c_uchar] {
        let end = self.length.min(self.content.len());
        if self.offset >= end {
            &[]
        } else {
            &self.content[self.offset..end]
        }
    }
}
#[derive(Clone)]
#[repr(C)]
pub struct printbuffer {
    pub buffer: ::std::vec::Vec<::core::ffi::c_uchar>,
    pub length: size_t,
    pub offset: size_t,
    pub depth: size_t,
    pub noalloc: cJSON_bool,
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
static global_error: error = error {
    json: ::std::sync::atomic::AtomicPtr::new(::core::ptr::null_mut::<::core::ffi::c_uchar>()),
    position: ::std::sync::atomic::AtomicUsize::new(0 as size_t),
};

fn clear_global_error() {
    global_error.json.store(
        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        ::std::sync::atomic::Ordering::Relaxed,
    );
    global_error
        .position
        .store(0 as size_t, ::std::sync::atomic::Ordering::Relaxed);
}

pub fn cJSON_GetErrorPtr() -> Option<::core::ptr::NonNull<::core::ffi::c_char>> {
    let json = global_error
        .json
        .load(::std::sync::atomic::Ordering::Relaxed);
    let position = global_error
        .position
        .load(::std::sync::atomic::Ordering::Relaxed);
    ::core::ptr::NonNull::new(json.wrapping_add(position)).map(::core::ptr::NonNull::cast)
}
#[export_name = "cJSON_GetErrorPtr"]
pub unsafe extern "C" fn cJSON_GetErrorPtr_ffi() -> *const ::core::ffi::c_char {
    match cJSON_GetErrorPtr() {
        Some(error) => error.as_ptr() as *const ::core::ffi::c_char,
        None => ::core::ptr::null(),
    }
}
pub fn cJSON_GetStringValue(item: Option<&cJSON>) -> Option<&[::core::ffi::c_uchar]> {
    let Some(item) = item else {
        return None;
    };
    if cJSON_IsString(Some(item)) == 0 {
        return None;
    }
    item.valuestring_storage.as_deref()
}
#[export_name = "cJSON_GetStringValue"]
pub unsafe extern "C" fn cJSON_GetStringValue_ffi(item: *const cJSON) -> *mut ::core::ffi::c_char {
    let Some(item) = item.as_ref() else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    if cJSON_GetStringValue(Some(item)).is_none() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if let Some(valuestring) = item.valuestring {
        valuestring.as_ptr().cast_mut()
    } else if let Some(buffer) = item.valuestring_buffer.as_ref() {
        buffer.as_ptr().cast_mut()
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
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
pub fn cJSON_Version() -> &'static ::core::ffi::CStr {
    c"1.7.19"
}
#[export_name = "cJSON_Version"]
pub unsafe extern "C" fn cJSON_Version_ffi() -> *const ::core::ffi::c_char {
    cJSON_Version().as_ptr()
}
fn c_string_visible_bytes(bytes: &[::core::ffi::c_uchar]) -> &[::core::ffi::c_uchar] {
    match bytes.iter().position(|byte| *byte == b'\0') {
        Some(end) => &bytes[..end],
        None => bytes,
    }
}

fn case_insensitive_bytes_cmp(
    string1: Option<&[::core::ffi::c_uchar]>,
    string2: Option<&[::core::ffi::c_uchar]>,
) -> ::core::ffi::c_int {
    let (Some(string1), Some(string2)) = (string1, string2) else {
        return 1 as ::core::ffi::c_int;
    };
    for (char1, char2) in string1.iter().zip(string2.iter()) {
        let char1 = char1.to_ascii_lowercase();
        let char2 = char2.to_ascii_lowercase();
        if char1 != char2 {
            return char1 as ::core::ffi::c_int - char2 as ::core::ffi::c_int;
        }
    }
    string1.len() as ::core::ffi::c_int - string2.len() as ::core::ffi::c_int
}
static global_hooks: ::std::sync::RwLock<internal_hooks> =
    ::std::sync::RwLock::new(internal_hooks {
        allocate: Some(malloc as extern "C" fn(size_t) -> *mut ::core::ffi::c_void),
        deallocate: Some(free as extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        reallocate: Some(
            realloc as extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void,
        ),
    });

fn global_hooks_snapshot() -> internal_hooks {
    *global_hooks
        .read()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}
macro_rules! copy_bytes_with_hooks_for_ffi {
    ($bytes:expr, $allocation_size:expr, $hooks:expr) => {{
        let source_bytes = $bytes;
        let allocation_size = $allocation_size;
        let hooks = $hooks;
        if source_bytes.len() > allocation_size {
            None
        } else {
            let copy = hooks.allocate.expect("non-null function pointer")(allocation_size)
                as *mut ::core::ffi::c_uchar;
            match ::core::ptr::NonNull::new(copy) {
                Some(copy) => {
                    unsafe {
                        ::core::ptr::copy_nonoverlapping(
                            source_bytes.as_ptr(),
                            copy.as_ptr(),
                            source_bytes.len(),
                        );
                    }
                    Some(copy)
                }
                None => None,
            }
        }
    }};
}
pub fn cJSON_InitHooks(hooks: Option<&cJSON_Hooks>) {
    let mut new_hooks = internal_hooks {
        allocate: Some(malloc as extern "C" fn(size_t) -> *mut ::core::ffi::c_void),
        deallocate: Some(free as extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        reallocate: Some(
            realloc as extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void,
        ),
    };
    let Some(hooks) = hooks else {
        *global_hooks
            .write()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = new_hooks;
        return;
    };
    if hooks.malloc_fn.is_some() {
        new_hooks.allocate = hooks.malloc_fn;
    }
    if hooks.free_fn.is_some() {
        new_hooks.deallocate = hooks.free_fn;
    }
    new_hooks.reallocate = None;
    if new_hooks.allocate == Some(malloc as extern "C" fn(size_t) -> *mut ::core::ffi::c_void)
        && new_hooks.deallocate == Some(free as extern "C" fn(*mut ::core::ffi::c_void) -> ())
    {
        new_hooks.reallocate = Some(
            realloc as extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void,
        )
            as Option<extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>;
    }
    *global_hooks
        .write()
        .unwrap_or_else(|poisoned| poisoned.into_inner()) = new_hooks;
}
#[export_name = "cJSON_InitHooks"]
pub unsafe extern "C" fn cJSON_InitHooks_ffi(mut hooks: *mut cJSON_Hooks) {
    cJSON_InitHooks(hooks.as_ref())
}
fn cJSON_New_Item(
    hooks: internal_hooks,
    initialize_item: impl FnOnce(&mut cJSON),
) -> Option<&'static mut cJSON> {
    let node: *mut cJSON = hooks.allocate.expect("non-null function pointer")(
        ::core::mem::size_of::<cJSON>() as size_t,
    ) as *mut cJSON;
    let node = ::core::ptr::NonNull::new(node)?;
    let mut item = cJSON::default();
    initialize_item(&mut item);
    let node_ref: &'static mut ::core::mem::MaybeUninit<cJSON> =
        unsafe { ::core::mem::transmute(node) };
    Some(node_ref.write(item))
}
pub fn cJSON_Delete(item: Option<&cJSON>) {
    let mut item = item;
    let hooks = global_hooks_snapshot();
    while let Some(item_ref) = item {
        let item_ptr = ::core::ptr::NonNull::from(item_ref);
        let item_ref: &'static mut cJSON = unsafe { ::core::mem::transmute(item_ptr) };
        let next = item_ref.next.get();
        if item_ref.type_0 & cJSON_IsReference == 0 && item_ref.child.get().is_some() {
            cJSON_Delete(item_ref.child.get());
        }
        item_ref.valuestring = None;
        item_ref.valuestring_len = 0;
        item_ref.valuestring_storage = None;
        item_ref.valuestring_buffer = None;
        item_ref.string = None;
        item_ref.string_storage = None;
        item_ref.string_buffer = None;
        hooks.deallocate.expect("non-null function pointer")(
            item_ptr.as_ptr() as *mut ::core::ffi::c_void
        );
        item = next;
    }
}
#[export_name = "cJSON_Delete"]
pub unsafe extern "C" fn cJSON_Delete_ffi(mut item: *mut cJSON) {
    cJSON_Delete(item.as_ref())
}
fn parse_number(item: &mut cJSON, input_buffer: &mut parse_buffer) -> cJSON_bool {
    let mut i: size_t = 0 as size_t;
    let mut number_string_length: size_t = 0 as size_t;
    i = 0 as size_t;
    while input_buffer.offset.wrapping_add(i) < input_buffer.length {
        match input_buffer.byte_at(i).unwrap_or(0) as ::core::ffi::c_int {
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
    let number_end = input_buffer.offset.wrapping_add(number_string_length);
    let Some(number_bytes) = input_buffer.content.get(input_buffer.offset..number_end) else {
        return false_0;
    };
    let Ok(number_string) = ::core::str::from_utf8(number_bytes) else {
        return false_0;
    };
    let mut number: Option<(::core::ffi::c_double, size_t)> = None;
    let mut consumed = number_string.len();
    while consumed > 0 {
        if let Ok(parsed) = number_string[..consumed].parse::<::core::ffi::c_double>() {
            number = Some((parsed, consumed as size_t));
            break;
        }
        consumed = consumed.wrapping_sub(1);
    }
    let Some((number, consumed)) = number else {
        return false_0;
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
    input_buffer.offset = input_buffer.offset.wrapping_add(consumed);
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
    let Some(object) = object.as_mut() else {
        return 0.0f64 / 0.0f64;
    };
    cJSON_SetNumberHelper(object, number)
}
fn set_valuestring_storage(
    object: &mut cJSON,
    value_storage: Vec<::core::ffi::c_uchar>,
    mut value_buffer: Vec<::core::ffi::c_char>,
) -> cJSON_bool {
    if value_buffer.is_empty() {
        return false_0;
    }
    object.valuestring = None;
    object.valuestring_len = value_storage.len();
    object.valuestring_storage = Some(value_storage);
    object.valuestring_buffer = Some(value_buffer);
    return true_0;
}
struct OwnedValueString {
    storage: Vec<::core::ffi::c_uchar>,
    buffer: Vec<::core::ffi::c_char>,
}
fn make_owned_valuestring_from_bytes(
    value_bytes: &[::core::ffi::c_uchar],
) -> Option<OwnedValueString> {
    let mut value_storage = Vec::new();
    value_storage.try_reserve_exact(value_bytes.len()).ok()?;
    value_storage.extend_from_slice(value_bytes);

    let mut value_buffer = Vec::new();
    value_buffer
        .try_reserve_exact(value_bytes.len().checked_add(1)?)
        .ok()?;
    value_buffer.extend(value_bytes.iter().map(|byte| *byte as ::core::ffi::c_char));
    value_buffer.push(0);

    Some(OwnedValueString {
        storage: value_storage,
        buffer: value_buffer,
    })
}
fn make_owned_valuestring(value: &::core::ffi::CStr) -> Option<OwnedValueString> {
    make_owned_valuestring_from_bytes(value.to_bytes())
}
struct OwnedStringKey {
    storage: Vec<::core::ffi::c_uchar>,
    buffer: Vec<::core::ffi::c_char>,
}
fn make_owned_string_key_from_bytes(key_bytes: &[::core::ffi::c_uchar]) -> Option<OwnedStringKey> {
    let mut key_storage = Vec::new();
    key_storage.try_reserve_exact(key_bytes.len()).ok()?;
    key_storage.extend_from_slice(key_bytes);

    let mut key_buffer = Vec::new();
    key_buffer
        .try_reserve_exact(key_bytes.len().checked_add(1)?)
        .ok()?;
    key_buffer.extend(key_bytes.iter().map(|byte| *byte as ::core::ffi::c_char));
    key_buffer.push(0);

    Some(OwnedStringKey {
        storage: key_storage,
        buffer: key_buffer,
    })
}
fn make_owned_string_key(string: &::core::ffi::CStr) -> Option<OwnedStringKey> {
    make_owned_string_key_from_bytes(string.to_bytes())
}
#[export_name = "cJSON_SetValuestring"]
pub unsafe extern "C" fn cJSON_SetValuestring_ffi(
    mut object: *mut cJSON,
    mut valuestring: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let valuestring = if valuestring.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(valuestring))
    };
    let (Some(object), Some(valuestring)) = (object.as_mut(), valuestring) else {
        return ::core::ptr::null_mut();
    };
    if object.type_0 & cJSON_String == 0 || object.type_0 & cJSON_IsReference != 0 {
        return ::core::ptr::null_mut();
    }
    let current_valuestring = if let Some(valuestring) = object.valuestring {
        valuestring.as_ptr().cast_mut()
    } else if let Some(buffer) = object.valuestring_buffer.as_mut() {
        buffer.as_mut_ptr()
    } else {
        return ::core::ptr::null_mut();
    };
    let new_len = valuestring.to_bytes().len();
    let current_len = object.valuestring_len;
    let new_start = valuestring.as_ptr().addr();
    let new_end = new_start.wrapping_add(new_len);
    let current_start = current_valuestring.addr();
    let current_end = current_start.wrapping_add(current_len);
    if new_len <= current_len && !(new_end < current_start || current_end < new_start) {
        return ::core::ptr::null_mut();
    }
    let old_value_was_buffered = object.valuestring_buffer.is_some();
    let new_bytes = valuestring.to_bytes_with_nul();
    let mut value_buffer: Vec<::core::ffi::c_char> = Vec::with_capacity(new_bytes.len());
    for byte in new_bytes {
        value_buffer.push(*byte as ::core::ffi::c_char);
    }
    let mut value_storage: Vec<::core::ffi::c_uchar> = Vec::with_capacity(new_len);
    for byte in &new_bytes[..new_len] {
        value_storage.push(*byte);
    }
    if set_valuestring_storage(object, value_storage, value_buffer) == 0 {
        return ::core::ptr::null_mut();
    }
    if !old_value_was_buffered {
        cJSON_free(::core::ptr::NonNull::new(
            current_valuestring.cast::<::core::ffi::c_void>(),
        ));
    }
    if let Some(valuestring) = object.valuestring {
        valuestring.as_ptr().cast_mut()
    } else if let Some(buffer) = object.valuestring_buffer.as_mut() {
        buffer.as_mut_ptr()
    } else {
        ::core::ptr::null_mut()
    }
}
fn ensure<'a>(
    p: &'a mut printbuffer,
    mut needed: size_t,
) -> Option<&'a mut [::core::ffi::c_uchar]> {
    let mut newsize: size_t = 0 as size_t;
    let requested = needed;
    if p.length > 0 as size_t && p.offset >= p.length {
        return None;
    }
    if needed > INT_MAX as size_t {
        return None;
    }
    needed = needed.checked_add(p.offset.checked_add(1 as size_t)?)?;
    if needed > p.length {
        if p.noalloc != 0 {
            return None;
        }
        if needed > (INT_MAX / 2 as ::core::ffi::c_int) as size_t {
            if needed <= INT_MAX as size_t {
                newsize = INT_MAX as size_t;
            } else {
                return None;
            }
        } else {
            newsize = needed.wrapping_mul(2 as size_t);
        }
        p.buffer.resize(newsize, 0);
        p.length = newsize;
    }
    p.buffer.get_mut(p.offset..p.offset.checked_add(requested)?)
}
fn compare_double(mut a: ::core::ffi::c_double, mut b: ::core::ffi::c_double) -> cJSON_bool {
    let mut maxVal: ::core::ffi::c_double = if a.abs() > b.abs() { a.abs() } else { b.abs() };
    return ((a - b).abs() <= maxVal * DBL_EPSILON) as ::core::ffi::c_int;
}
fn format_c_g_parts(formatted: String, precision: usize) -> String {
    let precision = precision.max(1);
    let formatted_bytes = formatted.as_bytes();
    let Some(exponent_offset) = formatted_bytes.iter().position(|byte| *byte == b'e') else {
        return formatted;
    };
    let mantissa = &formatted_bytes[..exponent_offset];
    let exponent_bytes = &formatted_bytes[exponent_offset.wrapping_add(1)..];
    let exponent_negative = exponent_bytes.first() == Some(&b'-');
    let exponent_digits = if exponent_negative || exponent_bytes.first() == Some(&b'+') {
        &exponent_bytes[1..]
    } else {
        exponent_bytes
    };
    let mut exponent = 0 as i32;
    for digit in exponent_digits {
        if !digit.is_ascii_digit() {
            return formatted;
        }
        exponent = exponent
            .wrapping_mul(10)
            .wrapping_add((digit - b'0') as i32);
    }
    if exponent_negative {
        exponent = -exponent;
    }
    let sign = if mantissa.first() == Some(&b'-') {
        "-"
    } else {
        ""
    };
    let mut digits: Vec<u8> = mantissa
        .iter()
        .copied()
        .filter(|byte| *byte != b'.' && *byte != b'-')
        .collect();
    if digits.is_empty() {
        return formatted;
    }
    while digits.len() > 1 && digits.last() == Some(&b'0') {
        digits.pop();
    }
    if exponent < -4 || exponent >= precision as i32 {
        let mut output = String::new();
        output.push_str(sign);
        output.push(char::from(digits[0]));
        if digits.len() > 1 {
            output.push('.');
            for digit in digits.iter().skip(1) {
                output.push(char::from(*digit));
            }
        }
        output.push('e');
        if exponent >= 0 {
            output.push('+');
        } else {
            output.push('-');
        }
        let absolute_exponent = exponent.abs();
        if absolute_exponent < 10 {
            output.push('0');
        }
        output.push_str(&absolute_exponent.to_string());
        return output;
    }
    let decimal_position = exponent + 1;
    let mut output = String::new();
    output.push_str(sign);
    if decimal_position <= 0 {
        output.push('0');
        output.push('.');
        for _ in 0..-decimal_position {
            output.push('0');
        }
        for digit in &digits {
            output.push(char::from(*digit));
        }
    } else if decimal_position as usize >= digits.len() {
        for digit in &digits {
            output.push(char::from(*digit));
        }
        for _ in digits.len()..decimal_position as usize {
            output.push('0');
        }
    } else {
        let decimal_position = decimal_position as usize;
        for digit in &digits[..decimal_position] {
            output.push(char::from(*digit));
        }
        output.push('.');
        for digit in &digits[decimal_position..] {
            output.push(char::from(*digit));
        }
    }
    output
}
fn format_scientific_number(
    number: ::core::ffi::c_double,
    fractional_digits: usize,
) -> Option<String> {
    let sign = if number < 0.0 { "-" } else { "" };
    let absolute = number.abs();
    let (mantissa, binary_exponent) = decompose_positive_f64(absolute)?;
    let mut exponent = if absolute == 0.0 {
        0
    } else {
        absolute.log10().floor() as i32
    };
    while compare_positive_f64_to_power10(mantissa, binary_exponent, exponent.checked_add(1)?)
        != ::core::cmp::Ordering::Less
    {
        exponent = exponent.checked_add(1)?;
    }
    while compare_positive_f64_to_power10(mantissa, binary_exponent, exponent)
        == ::core::cmp::Ordering::Less
    {
        exponent = exponent.checked_sub(1)?;
    }

    let total_digits = fractional_digits.checked_add(1)?;
    let decimal_scale = (fractional_digits as i32).checked_sub(exponent)?;
    let mut rounded = rounded_scaled_decimal(mantissa, binary_exponent, decimal_scale)?;
    let limit = decimal_power(total_digits);
    if rounded >= limit {
        rounded /= 10u8;
        exponent = exponent.checked_add(1)?;
    }

    let mut digits = rounded.to_string();
    if digits.len() < total_digits {
        let mut padded = String::new();
        padded.extend(::core::iter::repeat_n('0', total_digits - digits.len()));
        padded.push_str(&digits);
        digits = padded;
    }

    let mut output = String::new();
    output.push_str(sign);
    output.push(digits.as_bytes().first().copied().map(char::from)?);
    output.push('.');
    for digit in digits.as_bytes().iter().skip(1).take(fractional_digits) {
        output.push(char::from(*digit));
    }
    output.push('e');
    output.push_str(&exponent.to_string());
    Some(output)
}
fn decompose_positive_f64(number: ::core::ffi::c_double) -> Option<(u64, i32)> {
    let bits = number.to_bits();
    let exponent_bits = ((bits >> 52) & 0x7ff) as i32;
    let fraction = bits & ((1_u64 << 52) - 1);
    if exponent_bits == 0 {
        if fraction == 0 {
            return Some((0, 0));
        }
        Some((fraction, 1 - 1023 - 52))
    } else {
        Some(((1_u64 << 52) | fraction, exponent_bits - 1023 - 52))
    }
}
fn decimal_power(exponent: usize) -> num_bigint::BigUint {
    num_bigint::BigUint::from(10u8).pow(exponent as u32)
}
fn scaled_binary_integer(mantissa: u64, binary_exponent: i32) -> num_bigint::BigUint {
    let mut value = num_bigint::BigUint::from(mantissa);
    if binary_exponent > 0 {
        value <<= binary_exponent as usize;
    }
    value
}
fn compare_positive_f64_to_power10(
    mantissa: u64,
    binary_exponent: i32,
    decimal_exponent: i32,
) -> ::core::cmp::Ordering {
    let mut left = scaled_binary_integer(mantissa, binary_exponent);
    let mut right = if decimal_exponent >= 0 {
        decimal_power(decimal_exponent as usize)
    } else {
        num_bigint::BigUint::from(1u8)
    };
    if binary_exponent < 0 {
        right <<= (-binary_exponent) as usize;
    }
    if decimal_exponent < 0 {
        left *= decimal_power((-decimal_exponent) as usize);
    }
    left.cmp(&right)
}
fn rounded_scaled_decimal(
    mantissa: u64,
    binary_exponent: i32,
    decimal_scale: i32,
) -> Option<num_bigint::BigUint> {
    let mut numerator = num_bigint::BigUint::from(mantissa);
    let mut denominator = num_bigint::BigUint::from(1u8);
    if binary_exponent >= 0 {
        numerator <<= binary_exponent as usize;
    } else {
        denominator <<= (-binary_exponent) as usize;
    }
    if decimal_scale >= 0 {
        numerator *= decimal_power(decimal_scale as usize);
    } else {
        denominator *= decimal_power((-decimal_scale) as usize);
    }

    let mut quotient = &numerator / &denominator;
    let remainder = numerator % &denominator;
    let twice_remainder = remainder << 1usize;
    match twice_remainder.cmp(&denominator) {
        ::core::cmp::Ordering::Greater => quotient += 1u8,
        ::core::cmp::Ordering::Equal if &quotient % 2u8 == num_bigint::BigUint::from(1u8) => {
            quotient += 1u8
        }
        _ => {}
    }
    Some(quotient)
}
fn write_number_buffer(
    number_buffer: &mut [::core::ffi::c_uchar; 26],
    number: &str,
) -> Option<::core::ffi::c_int> {
    let number_bytes = number.as_bytes();
    if number_bytes.len().wrapping_add(1) > number_buffer.len() {
        return None;
    }
    number_buffer[..number_bytes.len()].copy_from_slice(number_bytes);
    number_buffer[number_bytes.len()] = '\0' as i32 as ::core::ffi::c_uchar;
    Some(number_bytes.len() as ::core::ffi::c_int)
}
fn parse_printed_number(
    number_buffer: &[::core::ffi::c_uchar],
    length: ::core::ffi::c_int,
) -> Option<::core::ffi::c_double> {
    if length < 0 {
        return None;
    }
    let number_bytes = number_buffer.get(..length as usize)?;
    let number_string = ::core::str::from_utf8(number_bytes).ok()?;
    number_string.parse::<::core::ffi::c_double>().ok()
}
fn print_number(item: &cJSON, output_buffer: &mut printbuffer) -> cJSON_bool {
    let mut d: ::core::ffi::c_double = item.valuedouble;
    let mut length: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: size_t = 0 as size_t;
    let mut number_buffer: [::core::ffi::c_uchar; 26] = [
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    if d != d || d - d != d - d && !(d != d) {
        let null = b"null\0";
        number_buffer[..null.len()].copy_from_slice(null);
        length = (null.len() - 1) as ::core::ffi::c_int;
    } else if d == item.valueint as ::core::ffi::c_double {
        let integer = item.valueint.to_string();
        let integer_bytes = integer.as_bytes();
        if integer_bytes.len().wrapping_add(1) > number_buffer.len() {
            return false_0;
        }
        number_buffer[..integer_bytes.len()].copy_from_slice(integer_bytes);
        number_buffer[integer_bytes.len()] = '\0' as i32 as ::core::ffi::c_uchar;
        length = integer_bytes.len() as ::core::ffi::c_int;
    } else {
        let Some(scientific) = format_scientific_number(d, 14) else {
            return false_0;
        };
        let printed = format_c_g_parts(scientific, 15);
        let Some(printed_length) = write_number_buffer(&mut number_buffer, &printed) else {
            return false_0;
        };
        length = printed_length;
        if parse_printed_number(&number_buffer, length)
            .is_none_or(|test| compare_double(test, d) == 0)
        {
            let Some(scientific) = format_scientific_number(d, 16) else {
                return false_0;
            };
            let printed = format_c_g_parts(scientific, 17);
            let Some(printed_length) = write_number_buffer(&mut number_buffer, &printed) else {
                return false_0;
            };
            length = printed_length;
        }
    }
    if length < 0 as ::core::ffi::c_int
        || length
            > (::core::mem::size_of::<[::core::ffi::c_uchar; 26]>() as usize)
                .wrapping_sub(1 as usize) as ::core::ffi::c_int
    {
        return false_0;
    }
    let Some(output) = ensure(
        output_buffer,
        (length as size_t)
            .wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 1]>() as size_t),
    ) else {
        return false_0;
    };
    i = 0 as size_t;
    while i < length as size_t {
        output[i as usize] = number_buffer[i as usize];
        i = i.wrapping_add(1);
    }
    output[i as usize] = '\0' as i32 as ::core::ffi::c_uchar;
    output_buffer.offset = output_buffer.offset.wrapping_add(length as size_t);
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
) -> Option<(size_t, [::core::ffi::c_uchar; 4], size_t)> {
    let mut c2rust_current_block: u64;
    let mut codepoint: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    let mut first_code: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut utf8_length: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
    let mut utf8_position: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
    let mut sequence_length: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
    let mut first_byte_mark: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
    let mut utf8_bytes = [0 as ::core::ffi::c_uchar; 4];
    if !(input.len() < 6 as size_t) {
        first_code = parse_hex4(&input[2 as size_t..]);
        if !(first_code >= 0xdc00 as ::core::ffi::c_uint
            && first_code <= 0xdfff as ::core::ffi::c_uint)
        {
            if first_code >= 0xd800 as ::core::ffi::c_uint
                && first_code <= 0xdbff as ::core::ffi::c_uint
            {
                let mut second_code: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                sequence_length = 12 as ::core::ffi::c_uchar;
                if input.len().wrapping_sub(6 as size_t) < 6 as size_t {
                    c2rust_current_block = 2136517548508416331;
                } else if input[6 as size_t] as ::core::ffi::c_int != '\\' as i32
                    || input[7 as size_t] as ::core::ffi::c_int != 'u' as i32
                {
                    c2rust_current_block = 2136517548508416331;
                } else {
                    second_code = parse_hex4(&input[8 as size_t..]);
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
                        first_byte_mark = 0xc0 as ::core::ffi::c_uchar;
                        c2rust_current_block = 3437258052017859086;
                    } else if codepoint < 0x10000 as ::core::ffi::c_ulong {
                        utf8_length = 3 as ::core::ffi::c_uchar;
                        first_byte_mark = 0xe0 as ::core::ffi::c_uchar;
                        c2rust_current_block = 3437258052017859086;
                    } else if codepoint <= 0x10ffff as ::core::ffi::c_ulong {
                        utf8_length = 4 as ::core::ffi::c_uchar;
                        first_byte_mark = 0xf0 as ::core::ffi::c_uchar;
                        c2rust_current_block = 3437258052017859086;
                    } else {
                        c2rust_current_block = 2136517548508416331;
                    }
                    match c2rust_current_block {
                        2136517548508416331 => {}
                        _ => {
                            utf8_position = (utf8_length as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_uchar;
                            while utf8_position as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                                utf8_bytes[utf8_position as usize] = ((codepoint
                                    | 0x80 as ::core::ffi::c_ulong)
                                    & 0xbf as ::core::ffi::c_ulong)
                                    as ::core::ffi::c_uchar;
                                codepoint >>= 6 as ::core::ffi::c_int;
                                utf8_position = utf8_position.wrapping_sub(1);
                            }
                            if utf8_length as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                utf8_bytes[0 as usize] = ((codepoint
                                    | first_byte_mark as ::core::ffi::c_ulong)
                                    & 0xff as ::core::ffi::c_ulong)
                                    as ::core::ffi::c_uchar;
                            } else {
                                utf8_bytes[0 as usize] = (codepoint & 0x7f as ::core::ffi::c_ulong)
                                    as ::core::ffi::c_uchar;
                            }
                            return Some((
                                sequence_length as size_t,
                                utf8_bytes,
                                utf8_length as size_t,
                            ));
                        }
                    }
                }
            }
        }
    }
    return None;
}
fn parse_string(item: &mut cJSON, input_buffer: &mut parse_buffer) -> cJSON_bool {
    let input_start = input_buffer.offset.wrapping_add(1 as size_t);
    let mut input_pointer = input_start;
    let mut input_end = input_start;
    let content_end = input_buffer.length.min(input_buffer.content.len());
    if input_buffer.current_byte().unwrap_or(0) as ::core::ffi::c_int != '"' as i32 {
        input_buffer.offset = input_pointer;
        return false_0;
    }

    let mut skipped_bytes: size_t = 0 as size_t;
    let mut scan_failed = false;
    while input_end < content_end
        && input_buffer.content[input_end] as ::core::ffi::c_int != '"' as i32
    {
        if input_buffer.content[input_end] as ::core::ffi::c_int == '\\' as i32 {
            if input_end.wrapping_add(1 as size_t) >= input_buffer.length {
                scan_failed = true;
                break;
            }
            skipped_bytes = skipped_bytes.wrapping_add(1);
            input_end = input_end.wrapping_add(1 as size_t);
        }
        input_end = input_end.wrapping_add(1 as size_t);
    }

    if scan_failed
        || input_end >= input_buffer.length
        || input_end >= content_end
        || input_buffer.content[input_end] as ::core::ffi::c_int != '"' as i32
    {
        input_buffer.offset = input_pointer;
        return false_0;
    }

    let allocation_length = input_end
        .wrapping_sub(input_buffer.offset)
        .wrapping_sub(skipped_bytes);
    let mut decoded = Vec::<::core::ffi::c_uchar>::new();
    let mut decode_failed = false;
    while input_pointer < input_end {
        if input_buffer.content[input_pointer] as ::core::ffi::c_int != '\\' as i32 {
            decoded.push(input_buffer.content[input_pointer]);
            input_pointer = input_pointer.wrapping_add(1 as size_t);
            continue;
        }

        let mut sequence_length: ::core::ffi::c_uchar = 2 as ::core::ffi::c_uchar;
        if input_end.wrapping_sub(input_pointer) < 1 as size_t
            || input_pointer.wrapping_add(1 as size_t) >= content_end
        {
            decode_failed = true;
            break;
        }
        match input_buffer.content[input_pointer.wrapping_add(1 as size_t)] as ::core::ffi::c_int {
            98 => {
                decoded.push('\u{8}' as i32 as ::core::ffi::c_uchar);
            }
            102 => {
                decoded.push('\u{c}' as i32 as ::core::ffi::c_uchar);
            }
            110 => {
                decoded.push('\n' as i32 as ::core::ffi::c_uchar);
            }
            114 => {
                decoded.push('\r' as i32 as ::core::ffi::c_uchar);
            }
            116 => {
                decoded.push('\t' as i32 as ::core::ffi::c_uchar);
            }
            34 | 92 | 47 => {
                decoded.push(input_buffer.content[input_pointer.wrapping_add(1 as size_t)]);
            }
            117 => match utf16_literal_to_utf8(&input_buffer.content[input_pointer..input_end]) {
                Some((length, utf8_bytes, utf8_length)) => {
                    sequence_length = length as ::core::ffi::c_uchar;
                    decoded.extend_from_slice(&utf8_bytes[..utf8_length as usize]);
                }
                None => {
                    decode_failed = true;
                    break;
                }
            },
            _ => {
                decode_failed = true;
                break;
            }
        }
        input_pointer = input_pointer.wrapping_add(sequence_length as size_t);
    }

    if !decode_failed && decoded.len() <= allocation_length {
        let Some(value_parts) = make_owned_valuestring_from_bytes(&decoded) else {
            input_buffer.offset = input_start;
            return false_0;
        };
        item.type_0 = cJSON_String;
        if set_valuestring_storage(item, value_parts.storage, value_parts.buffer) == 0 {
            input_buffer.offset = input_start;
            return false_0;
        }
        input_buffer.offset = input_end.wrapping_add(1 as size_t);
        return true_0;
    }
    input_buffer.offset = input_pointer;
    return false_0;
}
fn print_string_bytes(
    input_bytes: &[::core::ffi::c_uchar],
    output_buffer: &mut printbuffer,
) -> cJSON_bool {
    let Some(escape_characters) = input_bytes.iter().try_fold(0 as size_t, |count, byte| {
        let extra = match *byte {
            b'"' | b'\\' | 8 | 12 | b'\n' | b'\r' | b'\t' => 1,
            0..=31 => 5,
            _ => 0,
        };
        count.checked_add(extra)
    }) else {
        return false_0;
    };
    let Some(output_length) = input_bytes.len().checked_add(escape_characters) else {
        return false_0;
    };
    let Some(required_length) = output_length.checked_add(3) else {
        return false_0;
    };
    let Some(output) = ensure(output_buffer, required_length) else {
        return false_0;
    };
    if escape_characters == 0 {
        output[0] = b'"';
        output[1..input_bytes.len().wrapping_add(1)].copy_from_slice(input_bytes);
        output[output_length.wrapping_add(1)] = b'"';
        output[output_length.wrapping_add(2)] = b'\0';
        output_buffer.offset = output_buffer
            .offset
            .wrapping_add(output_length.wrapping_add(2));
        return true_0;
    }
    const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut output_offset = 0 as size_t;
    output[output_offset] = b'"';
    output_offset = output_offset.wrapping_add(1);
    for byte in input_bytes {
        if *byte > 31 && *byte != b'"' && *byte != b'\\' {
            output[output_offset] = *byte;
            output_offset = output_offset.wrapping_add(1);
        } else {
            output[output_offset] = b'\\';
            output_offset = output_offset.wrapping_add(1);
            match *byte {
                b'\\' => output[output_offset] = b'\\',
                b'"' => output[output_offset] = b'"',
                8 => output[output_offset] = b'b',
                12 => output[output_offset] = b'f',
                b'\n' => output[output_offset] = b'n',
                b'\r' => output[output_offset] = b'r',
                b'\t' => output[output_offset] = b't',
                _ => {
                    output[output_offset] = b'u';
                    output_offset = output_offset.wrapping_add(1);
                    output[output_offset] = b'0';
                    output_offset = output_offset.wrapping_add(1);
                    output[output_offset] = b'0';
                    output_offset = output_offset.wrapping_add(1);
                    output[output_offset] = HEX_DIGITS[(*byte >> 4) as usize];
                    output_offset = output_offset.wrapping_add(1);
                    output[output_offset] = HEX_DIGITS[(*byte & 0x0f) as usize];
                }
            }
            output_offset = output_offset.wrapping_add(1);
        }
    }
    output[output_offset] = b'"';
    output_offset = output_offset.wrapping_add(1);
    output[output_offset] = b'\0';
    output_buffer.offset = output_buffer.offset.wrapping_add(output_offset);
    return true_0;
}
fn print_string_ptr(
    input: Option<&::core::ffi::CStr>,
    output_buffer: &mut printbuffer,
) -> cJSON_bool {
    let input_bytes = input.map_or(&[][..], ::core::ffi::CStr::to_bytes);
    print_string_bytes(input_bytes, output_buffer)
}
fn print_string(item: &cJSON, p: &mut printbuffer) -> cJSON_bool {
    print_string_bytes(item.valuestring_storage.as_deref().unwrap_or(&[]), p)
}
fn buffer_skip_whitespace<'buffer, 'input>(
    buffer: Option<&'buffer mut parse_buffer<'input>>,
) -> Option<&'buffer mut parse_buffer<'input>> {
    let Some(buffer) = buffer else {
        return None;
    };
    if !buffer.can_access_at(0 as size_t) {
        return Some(buffer);
    }
    while buffer.can_access_at(0 as size_t)
        && buffer.current_byte().unwrap_or(0) as ::core::ffi::c_int <= 32 as ::core::ffi::c_int
    {
        buffer.offset = buffer.offset.wrapping_add(1);
    }
    if buffer.offset == buffer.length {
        buffer.offset = buffer.offset.wrapping_sub(1);
    }
    Some(buffer)
}
fn skip_utf8_bom<'buffer, 'input>(
    buffer: Option<&'buffer mut parse_buffer<'input>>,
) -> Option<&'buffer mut parse_buffer<'input>> {
    let Some(buffer) = buffer else {
        return None;
    };
    if buffer.offset != 0 as size_t {
        return None;
    }
    if buffer.offset.wrapping_add(4 as size_t) < buffer.length
        && buffer.remaining().starts_with(b"\xEF\xBB\xBF")
    {
        buffer.offset = buffer.offset.wrapping_add(3 as size_t);
    }
    Some(buffer)
}
pub fn cJSON_ParseWithOpts(
    value: &::core::ffi::CStr,
    mut require_null_terminated: cJSON_bool,
) -> (Option<::core::ptr::NonNull<cJSON>>, size_t) {
    return cJSON_ParseWithLengthOpts(Some(value.to_bytes_with_nul()), require_null_terminated);
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
    let result = cJSON_ParseWithOpts(::core::ffi::CStr::from_ptr(value), require_null_terminated);
    if let Some(return_parse_end) = return_parse_end.as_mut() {
        *return_parse_end = value.wrapping_add(result.1);
    }
    result.0.map_or(
        ::core::ptr::null_mut::<cJSON>(),
        ::core::ptr::NonNull::as_ptr,
    )
}
pub fn cJSON_ParseWithLengthOpts(
    value: Option<&[::core::ffi::c_uchar]>,
    mut require_null_terminated: cJSON_bool,
) -> (Option<::core::ptr::NonNull<cJSON>>, size_t) {
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
    let mut item: Option<&'static mut cJSON> = None;
    clear_global_error();
    if let Some(value) = value {
        buffer.length = value.len();
        if 0 as size_t != buffer.length {
            buffer.content = value;
            buffer.offset = 0 as size_t;
            buffer.hooks = global_hooks_snapshot();
            let mut parsed_value = false_0;
            item = cJSON_New_Item(buffer.hooks, |item| {
                skip_utf8_bom(Some(&mut buffer));
                buffer_skip_whitespace(Some(&mut buffer));
                parsed_value = parse_value(item, &mut buffer);
            });
            if item.is_some() {
                if !(parsed_value == 0) {
                    if require_null_terminated != 0 {
                        buffer_skip_whitespace(Some(&mut buffer));
                        if buffer.offset >= buffer.length
                            || buffer.current_byte().unwrap_or(0) as ::core::ffi::c_int
                                != '\0' as i32
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
                            return (item.map(::core::ptr::NonNull::from), buffer.offset);
                        }
                    }
                }
            }
        }
        cJSON_Delete(item.map(|item| &*item));
        let mut local_error_position: size_t = 0 as size_t;
        if buffer.offset < buffer.length {
            local_error_position = buffer.offset;
        } else if buffer.length > 0 as size_t {
            local_error_position = buffer.length.wrapping_sub(1 as size_t);
        }
        global_error.json.store(
            value.as_ptr().cast_mut().cast::<::core::ffi::c_uchar>(),
            ::std::sync::atomic::Ordering::Relaxed,
        );
        global_error
            .position
            .store(local_error_position, ::std::sync::atomic::Ordering::Relaxed);
        return (None, local_error_position);
    }
    return (None, 0 as size_t);
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
    } else {
        Some(::core::slice::from_raw_parts(
            value.cast::<::core::ffi::c_uchar>(),
            buffer_length,
        ))
    };
    let result = cJSON_ParseWithLengthOpts(value_slice, require_null_terminated);
    if !value.is_null() {
        if let Some(return_parse_end) = return_parse_end.as_mut() {
            *return_parse_end = value.wrapping_add(result.1);
        }
    }
    result.0.map_or(
        ::core::ptr::null_mut::<cJSON>(),
        ::core::ptr::NonNull::as_ptr,
    )
}
#[export_name = "cJSON_Parse"]
pub unsafe extern "C" fn cJSON_Parse_ffi(mut value: *const ::core::ffi::c_char) -> *mut cJSON {
    if value.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_ParseWithOpts(::core::ffi::CStr::from_ptr(value), 0 as cJSON_bool)
        .0
        .map_or(
            ::core::ptr::null_mut::<cJSON>(),
            ::core::ptr::NonNull::as_ptr,
        )
}
#[export_name = "cJSON_ParseWithLength"]
pub unsafe extern "C" fn cJSON_ParseWithLength_ffi(
    mut value: *const ::core::ffi::c_char,
    mut buffer_length: size_t,
) -> *mut cJSON {
    let value_slice = if value.is_null() {
        None
    } else {
        Some(::core::slice::from_raw_parts(
            value.cast::<::core::ffi::c_uchar>(),
            buffer_length,
        ))
    };
    cJSON_ParseWithLengthOpts(value_slice, 0 as cJSON_bool)
        .0
        .map_or(
            ::core::ptr::null_mut::<cJSON>(),
            ::core::ptr::NonNull::as_ptr,
        )
}
fn print(
    item: Option<&cJSON>,
    mut format: cJSON_bool,
) -> Option<::std::vec::Vec<::core::ffi::c_uchar>> {
    let default_buffer_size: size_t = 256 as size_t;
    let mut buffer = printbuffer {
        buffer: ::std::vec![0; default_buffer_size],
        length: default_buffer_size,
        offset: 0,
        depth: 0,
        noalloc: 0,
        format,
    };
    if print_value(item, &mut buffer) != 0 {
        let printed_length = buffer.offset.checked_add(1 as size_t)?;
        buffer.buffer.truncate(printed_length);
        return Some(buffer.buffer);
    }
    return None;
}
#[export_name = "cJSON_Print"]
pub unsafe extern "C" fn cJSON_Print_ffi(mut item: *const cJSON) -> *mut ::core::ffi::c_char {
    let hooks = global_hooks_snapshot();
    let Some(bytes) = print(item.as_ref(), true_0) else {
        return ::core::ptr::null_mut();
    };
    copy_bytes_with_hooks_for_ffi!(&bytes, bytes.len(), hooks)
        .map_or(::core::ptr::null_mut(), ::core::ptr::NonNull::as_ptr)
        as *mut ::core::ffi::c_char
}
#[export_name = "cJSON_PrintUnformatted"]
pub unsafe extern "C" fn cJSON_PrintUnformatted_ffi(
    mut item: *const cJSON,
) -> *mut ::core::ffi::c_char {
    let hooks = global_hooks_snapshot();
    let Some(bytes) = print(item.as_ref(), false_0) else {
        return ::core::ptr::null_mut();
    };
    copy_bytes_with_hooks_for_ffi!(&bytes, bytes.len(), hooks)
        .map_or(::core::ptr::null_mut(), ::core::ptr::NonNull::as_ptr)
        as *mut ::core::ffi::c_char
}
pub fn cJSON_PrintBuffered(
    item: Option<&cJSON>,
    mut prebuffer: ::core::ffi::c_int,
    mut fmt: cJSON_bool,
) -> Option<::std::vec::Vec<::core::ffi::c_uchar>> {
    let mut p: printbuffer = printbuffer {
        buffer: ::std::vec::Vec::new(),
        length: 0 as size_t,
        offset: 0 as size_t,
        depth: 0 as size_t,
        noalloc: 0 as cJSON_bool,
        format: 0 as cJSON_bool,
    };
    if prebuffer < 0 as ::core::ffi::c_int {
        return None;
    }
    p.buffer = ::std::vec![0; prebuffer as size_t];
    p.length = prebuffer as size_t;
    p.offset = 0 as size_t;
    p.noalloc = false_0;
    p.format = fmt;
    if print_value(item, &mut p) == 0 {
        return None;
    }
    let printed_length = p.offset.checked_add(1 as size_t)?;
    p.buffer.truncate(printed_length);
    return Some(p.buffer);
}
#[export_name = "cJSON_PrintBuffered"]
pub unsafe extern "C" fn cJSON_PrintBuffered_ffi(
    mut item: *const cJSON,
    mut prebuffer: ::core::ffi::c_int,
    mut fmt: cJSON_bool,
) -> *mut ::core::ffi::c_char {
    let hooks = global_hooks_snapshot();
    let Some(bytes) = cJSON_PrintBuffered(item.as_ref(), prebuffer, fmt) else {
        return ::core::ptr::null_mut();
    };
    copy_bytes_with_hooks_for_ffi!(&bytes, bytes.len(), hooks)
        .map_or(::core::ptr::null_mut(), ::core::ptr::NonNull::as_ptr)
        as *mut ::core::ffi::c_char
}
pub fn cJSON_PrintPreallocated(
    item: Option<&cJSON>,
    buffer: &mut [::core::ffi::c_char],
    format: cJSON_bool,
) -> cJSON_bool {
    let mut p: printbuffer = printbuffer {
        buffer: ::std::vec![0; buffer.len()],
        length: buffer.len(),
        offset: 0 as size_t,
        depth: 0 as size_t,
        noalloc: true_0,
        format,
    };
    let printed = print_value(item, &mut p);
    let copy_length = if p.offset == 0 {
        0
    } else {
        p.offset
            .checked_add(1 as size_t)
            .unwrap_or(buffer.len())
            .min(buffer.len())
            .min(p.buffer.len())
    };
    for (destination, source) in buffer.iter_mut().zip(p.buffer.iter()).take(copy_length) {
        *destination = *source as ::core::ffi::c_char;
    }
    return printed;
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
    let buffer = ::core::slice::from_raw_parts_mut(buffer, length as size_t);
    cJSON_PrintPreallocated(item.as_ref(), buffer, format)
}
fn parse_value(item: &mut cJSON, input_buffer: &mut parse_buffer) -> cJSON_bool {
    if input_buffer.offset.wrapping_add(4 as size_t) <= input_buffer.length
        && input_buffer.remaining().starts_with(b"null")
    {
        item.type_0 = cJSON_NULL;
        input_buffer.offset = input_buffer.offset.wrapping_add(4 as size_t);
        return true_0;
    }
    if input_buffer.offset.wrapping_add(5 as size_t) <= input_buffer.length
        && input_buffer.remaining().starts_with(b"false")
    {
        item.type_0 = cJSON_False;
        input_buffer.offset = input_buffer.offset.wrapping_add(5 as size_t);
        return true_0;
    }
    if input_buffer.offset.wrapping_add(4 as size_t) <= input_buffer.length
        && input_buffer.remaining().starts_with(b"true")
    {
        item.type_0 = cJSON_True;
        item.valueint = 1 as ::core::ffi::c_int;
        input_buffer.offset = input_buffer.offset.wrapping_add(4 as size_t);
        return true_0;
    }
    let current = input_buffer.current_byte().unwrap_or(0);
    if input_buffer.can_access_at(0 as size_t) && current as ::core::ffi::c_int == '"' as i32 {
        return parse_string(item, input_buffer);
    }
    if input_buffer.can_access_at(0 as size_t)
        && (current as ::core::ffi::c_int == '-' as i32
            || current as ::core::ffi::c_int >= '0' as i32
                && current as ::core::ffi::c_int <= '9' as i32)
    {
        return parse_number(item, input_buffer);
    }
    if input_buffer.can_access_at(0 as size_t) && current as ::core::ffi::c_int == '[' as i32 {
        return parse_array(item, input_buffer);
    }
    if input_buffer.can_access_at(0 as size_t) && current as ::core::ffi::c_int == '{' as i32 {
        return parse_object(item, input_buffer);
    }
    return false_0;
}
fn print_value(item: Option<&cJSON>, output_buffer: &mut printbuffer) -> cJSON_bool {
    let Some(item) = item else {
        return false_0;
    };
    match item.type_0 & 0xff as ::core::ffi::c_int {
        cJSON_NULL => {
            let bytes = b"null\0";
            let Some(output) = ensure(output_buffer, bytes.len()) else {
                return false_0;
            };
            output.copy_from_slice(bytes);
            output_buffer.offset = output_buffer
                .offset
                .wrapping_add(bytes.len().wrapping_sub(1));
            return true_0;
        }
        cJSON_False => {
            let bytes = b"false\0";
            let Some(output) = ensure(output_buffer, bytes.len()) else {
                return false_0;
            };
            output.copy_from_slice(bytes);
            output_buffer.offset = output_buffer
                .offset
                .wrapping_add(bytes.len().wrapping_sub(1));
            return true_0;
        }
        cJSON_True => {
            let bytes = b"true\0";
            let Some(output) = ensure(output_buffer, bytes.len()) else {
                return false_0;
            };
            output.copy_from_slice(bytes);
            output_buffer.offset = output_buffer
                .offset
                .wrapping_add(bytes.len().wrapping_sub(1));
            return true_0;
        }
        cJSON_Number => return print_number(item, output_buffer),
        cJSON_Raw => {
            let Some(bytes) = item.valuestring_storage.as_deref() else {
                return false_0;
            };
            let Some(output_length) = bytes.len().checked_add(1 as size_t) else {
                return false_0;
            };
            let Some(output) = ensure(output_buffer, output_length) else {
                return false_0;
            };
            output[..bytes.len()].copy_from_slice(bytes);
            output[bytes.len()] = b'\0';
            output_buffer.offset = output_buffer
                .offset
                .wrapping_add(output_length.wrapping_sub(1));
            return true_0;
        }
        cJSON_String => return print_string(item, output_buffer),
        cJSON_Array => return print_array(item, output_buffer),
        cJSON_Object => return print_object(item, output_buffer),
        _ => return false_0,
    };
}
fn parse_array(item: &mut cJSON, input_buffer: &mut parse_buffer) -> cJSON_bool {
    let mut c2rust_current_block: u64;
    let mut parsed_children = cJSON::default();
    if input_buffer.depth >= CJSON_NESTING_LIMIT as size_t {
        return false_0;
    }
    input_buffer.depth = input_buffer.depth.wrapping_add(1);
    if !(input_buffer.current_byte().unwrap_or(0) as ::core::ffi::c_int != '[' as i32) {
        input_buffer.offset = input_buffer.offset.wrapping_add(1);
        buffer_skip_whitespace(Some(&mut *input_buffer));
        if input_buffer.can_access_at(0 as size_t)
            && input_buffer.current_byte().unwrap_or(0) as ::core::ffi::c_int == ']' as i32
        {
            c2rust_current_block = 6773356538935231690;
        } else if !(input_buffer.offset.wrapping_add(0 as size_t) < input_buffer.length) {
            input_buffer.offset = input_buffer.offset.wrapping_sub(1);
            c2rust_current_block = 1336238348363633231;
        } else {
            input_buffer.offset = input_buffer.offset.wrapping_sub(1);
            loop {
                let mut parse_result = false_0;
                let new_item = cJSON_New_Item(input_buffer.hooks, |new_item| {
                    input_buffer.offset = input_buffer.offset.wrapping_add(1);
                    buffer_skip_whitespace(Some(input_buffer));
                    parse_result = parse_value(new_item, input_buffer);
                });
                let Some(new_item) = new_item else {
                    c2rust_current_block = 1336238348363633231;
                    break;
                };
                if parse_result == 0 {
                    cJSON_Delete(Some(&*new_item));
                    c2rust_current_block = 1336238348363633231;
                    break;
                }
                if add_item_to_array(&mut parsed_children, new_item) == 0 {
                    cJSON_Delete(Some(&*new_item));
                    c2rust_current_block = 1336238348363633231;
                    break;
                }
                buffer_skip_whitespace(Some(input_buffer));
                if !(input_buffer.can_access_at(0 as size_t)
                    && input_buffer.current_byte().unwrap_or(0) as ::core::ffi::c_int == ',' as i32)
                {
                    c2rust_current_block = 15089075282327824602;
                    break;
                }
            }
            match c2rust_current_block {
                1336238348363633231 => {}
                _ => {
                    if !input_buffer.can_access_at(0 as size_t)
                        || input_buffer.current_byte().unwrap_or(0) as ::core::ffi::c_int
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
                item.child.set(parsed_children.child.get());
                input_buffer.offset = input_buffer.offset.wrapping_add(1);
                return true_0;
            }
        }
    }
    cJSON_Delete(parsed_children.child.get());
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
    let mut element_index = 0 as size_t;
    while let Some(current) = get_array_item(Some(item), element_index) {
        if print_value(Some(current), output_buffer) == 0 {
            return false_0;
        }
        if get_array_item(Some(item), element_index.wrapping_add(1 as size_t)).is_some() {
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
            if formatted {
                output[1] = ' ' as i32 as ::core::ffi::c_uchar;
            }
            output[length as usize] = '\0' as i32 as ::core::ffi::c_uchar;
            output_buffer.offset = output_buffer.offset.wrapping_add(length);
        }
        element_index = element_index.wrapping_add(1 as size_t);
    }
    let Some(output) = ensure(output_buffer, 2 as size_t) else {
        return false_0;
    };
    output[0] = ']' as i32 as ::core::ffi::c_uchar;
    output[1] = '\0' as i32 as ::core::ffi::c_uchar;
    output_buffer.depth = output_buffer.depth.wrapping_sub(1);
    output_buffer.offset = output_buffer.offset.wrapping_add(1);
    return true_0;
}
fn parse_object(item: &mut cJSON, input_buffer: &mut parse_buffer) -> cJSON_bool {
    let mut c2rust_current_block: u64;
    let mut parsed_children = cJSON::default();
    if input_buffer.depth >= CJSON_NESTING_LIMIT as size_t {
        return false_0;
    }
    input_buffer.depth = input_buffer.depth.wrapping_add(1);
    if !(!input_buffer.can_access_at(0 as size_t)
        || input_buffer.current_byte().unwrap_or(0) as ::core::ffi::c_int != '{' as i32)
    {
        input_buffer.offset = input_buffer.offset.wrapping_add(1);
        buffer_skip_whitespace(Some(&mut *input_buffer));
        if input_buffer.can_access_at(0 as size_t)
            && input_buffer.current_byte().unwrap_or(0) as ::core::ffi::c_int == '}' as i32
        {
            c2rust_current_block = 4359236900545362719;
        } else if !(input_buffer.offset.wrapping_add(0 as size_t) < input_buffer.length) {
            input_buffer.offset = input_buffer.offset.wrapping_sub(1);
            c2rust_current_block = 9990476168629568694;
        } else {
            input_buffer.offset = input_buffer.offset.wrapping_sub(1);
            loop {
                let mut parsed_item = false;
                let new_item = cJSON_New_Item(input_buffer.hooks, |new_item| {
                    if !(input_buffer.offset.wrapping_add(1 as size_t) < input_buffer.length) {
                        return;
                    }
                    input_buffer.offset = input_buffer.offset.wrapping_add(1);
                    buffer_skip_whitespace(Some(&mut *input_buffer));
                    let parsed_key = parse_string(new_item, input_buffer);
                    if parsed_key == 0 {
                        return;
                    }
                    buffer_skip_whitespace(Some(&mut *input_buffer));
                    new_item.string = if new_item.valuestring_storage.is_some() {
                        Some(())
                    } else {
                        None
                    };
                    new_item.valuestring = None;
                    new_item.string_buffer = new_item.valuestring_buffer.take();
                    new_item.valuestring_len = 0;
                    new_item.string_storage = new_item
                        .valuestring_storage
                        .as_deref()
                        .map(|bytes| c_string_visible_bytes(bytes).to_vec());
                    new_item.valuestring_storage = None;
                    if !input_buffer.can_access_at(0 as size_t)
                        || input_buffer.current_byte().unwrap_or(0) as ::core::ffi::c_int
                            != ':' as i32
                    {
                        return;
                    }
                    input_buffer.offset = input_buffer.offset.wrapping_add(1);
                    buffer_skip_whitespace(Some(&mut *input_buffer));
                    let parsed_value = parse_value(new_item, input_buffer);
                    if parsed_value == 0 {
                        return;
                    }
                    buffer_skip_whitespace(Some(&mut *input_buffer));
                    parsed_item = true;
                });
                let Some(new_item) = new_item else {
                    c2rust_current_block = 9990476168629568694;
                    break;
                };
                if !parsed_item {
                    cJSON_Delete(Some(&*new_item));
                    c2rust_current_block = 9990476168629568694;
                    break;
                }
                if add_item_to_array(&mut parsed_children, new_item) == 0 {
                    cJSON_Delete(Some(&*new_item));
                    c2rust_current_block = 9990476168629568694;
                    break;
                }
                if !(input_buffer.can_access_at(0 as size_t)
                    && input_buffer.current_byte().unwrap_or(0) as ::core::ffi::c_int == ',' as i32)
                {
                    c2rust_current_block = 14359455889292382949;
                    break;
                }
            }
            match c2rust_current_block {
                9990476168629568694 => {}
                _ => {
                    if !input_buffer.can_access_at(0 as size_t)
                        || input_buffer.current_byte().unwrap_or(0) as ::core::ffi::c_int
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
                item.child.set(parsed_children.child.get());
                input_buffer.offset = input_buffer.offset.wrapping_add(1);
                return true_0;
            }
        }
    }
    cJSON_Delete(parsed_children.child.get());
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
    let mut item_index = 0 as size_t;
    while let Some(current) = get_array_item(Some(item), item_index) {
        if output_buffer.format != 0 {
            let Some(output) = ensure(output_buffer, output_buffer.depth) else {
                return false_0;
            };
            output.fill('\t' as i32 as ::core::ffi::c_uchar);
            output_buffer.offset = output_buffer.offset.wrapping_add(output_buffer.depth);
        }
        if print_string_bytes(
            current.string_storage.as_deref().unwrap_or(&[]),
            output_buffer,
        ) == 0
        {
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
        if print_value(Some(current), output_buffer) == 0 {
            return false_0;
        }
        let formatted = output_buffer.format != 0;
        length = ((if formatted {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as size_t)
            .wrapping_add(
                (if get_array_item(Some(item), item_index.wrapping_add(1 as size_t)).is_some() {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as size_t,
            );
        let Some(output) = ensure(output_buffer, length.wrapping_add(1 as size_t)) else {
            return false_0;
        };
        let mut output_offset = 0 as size_t;
        if get_array_item(Some(item), item_index.wrapping_add(1 as size_t)).is_some() {
            output[output_offset as usize] = ',' as i32 as ::core::ffi::c_uchar;
            output_offset = output_offset.wrapping_add(1);
        }
        if formatted {
            output[output_offset as usize] = '\n' as i32 as ::core::ffi::c_uchar;
            output_offset = output_offset.wrapping_add(1);
        }
        output[output_offset as usize] = '\0' as i32 as ::core::ffi::c_uchar;
        output_buffer.offset = output_buffer.offset.wrapping_add(length);
        item_index = item_index.wrapping_add(1 as size_t);
    }
    let formatted = output_buffer.format != 0;
    let depth = output_buffer.depth;
    let Some(output) = ensure(
        output_buffer,
        if formatted {
            depth.wrapping_add(1 as size_t)
        } else {
            2 as size_t
        },
    ) else {
        return false_0;
    };
    let mut output_offset = 0 as size_t;
    if formatted {
        let mut i_0: size_t = 0;
        i_0 = 0 as size_t;
        while i_0 < depth.wrapping_sub(1 as size_t) {
            output[output_offset as usize] = '\t' as i32 as ::core::ffi::c_uchar;
            output_offset = output_offset.wrapping_add(1);
            i_0 = i_0.wrapping_add(1);
        }
    }
    output[output_offset as usize] = '}' as i32 as ::core::ffi::c_uchar;
    output_offset = output_offset.wrapping_add(1);
    output[output_offset as usize] = '\0' as i32 as ::core::ffi::c_uchar;
    output_buffer.depth = output_buffer.depth.wrapping_sub(1);
    output_buffer.offset = output_buffer.offset.wrapping_add(output_offset);
    return true_0;
}
pub fn cJSON_GetArraySize(array: Option<&cJSON>) -> ::core::ffi::c_int {
    let mut size: size_t = 0 as size_t;
    let Some(array) = array else {
        return 0 as ::core::ffi::c_int;
    };
    while get_array_item(Some(array), size).is_some() {
        size = size.wrapping_add(1);
    }
    return size as ::core::ffi::c_int;
}
#[export_name = "cJSON_GetArraySize"]
pub unsafe extern "C" fn cJSON_GetArraySize_ffi(mut array: *const cJSON) -> ::core::ffi::c_int {
    cJSON_GetArraySize(array.as_ref())
}
#[inline(never)]
fn get_array_item(array: Option<&cJSON>, mut index: size_t) -> Option<&cJSON> {
    let Some(array) = array else {
        return None;
    };
    let mut current_child = array.child.get();
    while let Some(child) = current_child {
        if index == 0 as size_t {
            return Some(child);
        }
        index = index.wrapping_sub(1);
        current_child = child.next.get();
    }
    None
}
#[export_name = "cJSON_GetArrayItem"]
pub unsafe extern "C" fn cJSON_GetArrayItem_ffi(
    mut array: *const cJSON,
    mut index: ::core::ffi::c_int,
) -> *mut cJSON {
    if index < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<cJSON>();
    }
    get_array_item(array.as_ref(), index as size_t)
        .map(::core::ptr::NonNull::from)
        .map_or(
            ::core::ptr::null_mut::<cJSON>(),
            ::core::ptr::NonNull::as_ptr,
        )
}
fn get_object_item<'a>(
    object: Option<&'a cJSON>,
    name: Option<&[::core::ffi::c_uchar]>,
    case_sensitive: cJSON_bool,
) -> Option<&'a cJSON> {
    let (Some(object), Some(name)) = (object, name) else {
        return None;
    };
    let mut element_index = 0 as size_t;
    while let Some(element) = get_array_item(Some(object), element_index) {
        if case_sensitive != 0 {
            let Some(element_name) = element.string_storage.as_deref() else {
                return None;
            };
            if element_name == name {
                return Some(element);
            }
        } else {
            if case_insensitive_bytes_cmp(Some(name), element.string_storage.as_deref())
                == 0 as ::core::ffi::c_int
            {
                return Some(element);
            }
        }
        element_index = element_index.wrapping_add(1 as size_t);
    }
    None
}
#[export_name = "cJSON_GetObjectItem"]
pub unsafe extern "C" fn cJSON_GetObjectItem_ffi(
    object: *const cJSON,
    string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let string = if string.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(string))
    };
    get_object_item(
        object.as_ref(),
        string.map(::core::ffi::CStr::to_bytes),
        false_0,
    )
    .map(::core::ptr::NonNull::from)
    .map_or(
        ::core::ptr::null_mut::<cJSON>(),
        ::core::ptr::NonNull::as_ptr,
    )
}
#[export_name = "cJSON_GetObjectItemCaseSensitive"]
pub unsafe extern "C" fn cJSON_GetObjectItemCaseSensitive_ffi(
    object: *const cJSON,
    string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let string = if string.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(string))
    };
    get_object_item(
        object.as_ref(),
        string.map(::core::ffi::CStr::to_bytes),
        true_0,
    )
    .map(::core::ptr::NonNull::from)
    .map_or(
        ::core::ptr::null_mut::<cJSON>(),
        ::core::ptr::NonNull::as_ptr,
    )
}
#[export_name = "cJSON_HasObjectItem"]
pub unsafe extern "C" fn cJSON_HasObjectItem_ffi(
    mut object: *const cJSON,
    mut string: *const ::core::ffi::c_char,
) -> cJSON_bool {
    let string = if string.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(string))
    };
    if get_object_item(
        object.as_ref(),
        string.map(::core::ffi::CStr::to_bytes),
        false_0,
    )
    .is_some()
    {
        1 as cJSON_bool
    } else {
        0 as cJSON_bool
    }
}
fn suffix_object(prev: &'static cJSON, item: &'static cJSON) {
    prev.next.set(Some(item));
    item.prev.set(Some(prev));
}

fn create_reference(item: Option<&cJSON>, hooks: internal_hooks) -> Option<&'static mut cJSON> {
    let Some(item) = item else {
        return None;
    };
    cJSON_New_Item(hooks, |reference| {
        *reference = cJSON {
            next: ::core::cell::Cell::new(None),
            prev: ::core::cell::Cell::new(None),
            child: ::core::cell::Cell::new(item.child.get()),
            type_0: item.type_0 | cJSON_IsReference,
            valuestring: item.valuestring,
            valuestring_len: item.valuestring_len,
            valuestring_storage: item.valuestring_storage.clone(),
            valuestring_buffer: None,
            valueint: item.valueint,
            valuedouble: item.valuedouble,
            string: None,
            string_storage: None,
            string_buffer: None,
        };
    })
}
fn add_item_to_array(array: &cJSON, item: &'static cJSON) -> cJSON_bool {
    if same_item(array, item) {
        return false_0;
    }
    let child = array.child.get();
    if child.is_none() {
        array.child.set(Some(item));
        item.prev.set(Some(item));
        item.next.set(None);
    } else {
        let child = child.expect("child is present");
        let previous = child.prev.get();
        if let Some(previous) = previous {
            if same_item(previous, child) {
                suffix_object(child, item);
                child.prev.set(Some(item));
            } else {
                suffix_object(previous, item);
                child.prev.set(Some(item));
            }
        }
    }
    return true_0;
}
#[export_name = "cJSON_AddItemToArray"]
pub unsafe extern "C" fn cJSON_AddItemToArray_ffi(
    mut array: *mut cJSON,
    mut item: *mut cJSON,
) -> cJSON_bool {
    let (Some(mut array), Some(mut item)) = (
        ::core::ptr::NonNull::new(array),
        ::core::ptr::NonNull::new(item),
    ) else {
        return false_0;
    };
    add_item_to_array(array.as_ref(), item.as_ref())
}
fn add_item_to_object(
    object: &mut cJSON,
    string: Option<&::core::ffi::CStr>,
    item: &'static mut cJSON,
    _hooks: internal_hooks,
    constant_key: cJSON_bool,
) -> cJSON_bool {
    let Some(string) = string else {
        return false_0;
    };
    if same_item(object, item) {
        return false_0;
    }
    let (new_key_storage, new_key_buffer, new_type) = if constant_key != 0 {
        (
            string.to_bytes().to_vec(),
            None,
            item.type_0 | cJSON_StringIsConst,
        )
    } else {
        let Some(key_parts) = make_owned_string_key(string) else {
            return false_0;
        };
        (
            key_parts.storage,
            Some(key_parts.buffer),
            item.type_0 & !cJSON_StringIsConst,
        )
    };
    item.string = Some(());
    item.string_storage = Some(new_key_storage);
    item.string_buffer = new_key_buffer;
    item.type_0 = new_type;
    return add_item_to_array(object, &*item);
}
#[export_name = "cJSON_AddItemToObject"]
pub unsafe extern "C" fn cJSON_AddItemToObject_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
    mut item: *mut cJSON,
) -> cJSON_bool {
    let string = if string.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(string))
    };
    let (Some(object), Some(mut item)) = (object.as_mut(), ::core::ptr::NonNull::new(item)) else {
        return false_0;
    };
    add_item_to_object(
        object,
        string,
        item.as_mut(),
        global_hooks_snapshot(),
        false_0,
    )
}
#[export_name = "cJSON_AddItemToObjectCS"]
pub unsafe extern "C" fn cJSON_AddItemToObjectCS_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
    mut item: *mut cJSON,
) -> cJSON_bool {
    let string = if string.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(string))
    };
    let (Some(object), Some(mut item)) = (object.as_mut(), ::core::ptr::NonNull::new(item)) else {
        return false_0;
    };
    add_item_to_object(
        object,
        string,
        item.as_mut(),
        global_hooks_snapshot(),
        true_0,
    )
}
#[export_name = "cJSON_AddItemReferenceToArray"]
pub unsafe extern "C" fn cJSON_AddItemReferenceToArray_ffi(
    mut array: *mut cJSON,
    mut item: *mut cJSON,
) -> cJSON_bool {
    if array.is_null() {
        return false_0;
    }
    let array = array.as_mut().expect("array was checked for null");
    let Some(reference) = create_reference(item.as_ref(), global_hooks_snapshot()) else {
        return false_0;
    };
    return add_item_to_array(array, reference);
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
    let object = object.as_mut().expect("object was checked for null");
    let hooks = global_hooks_snapshot();
    let string = Some(::core::ffi::CStr::from_ptr(string));
    let Some(mut reference) = create_reference(item.as_ref(), hooks) else {
        return false_0;
    };
    return add_item_to_object(object, string, &mut *reference, hooks, false_0);
}
#[export_name = "cJSON_AddNullToObject"]
pub unsafe extern "C" fn cJSON_AddNullToObject_ffi(
    mut object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let Some(mut null) = cJSON_CreateNull() else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let null_ptr = null as *mut cJSON;
    let name = if name.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(name))
    };
    let Some(object) = object.as_mut() else {
        cJSON_Delete(Some(&*null));
        return ::core::ptr::null_mut::<cJSON>();
    };
    if add_item_to_object(object, name, &mut *null, global_hooks_snapshot(), false_0) != 0 {
        return null_ptr;
    }
    cJSON_Delete_ffi(null_ptr);
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddTrueToObject"]
pub unsafe extern "C" fn cJSON_AddTrueToObject_ffi(
    mut object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let Some(mut true_item) = cJSON_CreateTrue() else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let true_item_ptr = true_item as *mut cJSON;
    let name = if name.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(name))
    };
    let Some(object) = object.as_mut() else {
        cJSON_Delete(Some(&*true_item));
        return ::core::ptr::null_mut::<cJSON>();
    };
    if add_item_to_object(
        object,
        name,
        &mut *true_item,
        global_hooks_snapshot(),
        false_0,
    ) != 0
    {
        return true_item_ptr;
    }
    cJSON_Delete_ffi(true_item_ptr);
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddFalseToObject"]
pub unsafe extern "C" fn cJSON_AddFalseToObject_ffi(
    mut object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let Some(mut false_item) = cJSON_CreateFalse() else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let false_item_ptr = false_item as *mut cJSON;
    let name = if name.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(name))
    };
    let Some(object) = object.as_mut() else {
        cJSON_Delete(Some(&*false_item));
        return ::core::ptr::null_mut::<cJSON>();
    };
    if add_item_to_object(
        object,
        name,
        &mut *false_item,
        global_hooks_snapshot(),
        false_0,
    ) != 0
    {
        return false_item_ptr;
    }
    cJSON_Delete_ffi(false_item_ptr);
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddBoolToObject"]
pub unsafe extern "C" fn cJSON_AddBoolToObject_ffi(
    mut object: *mut cJSON,
    name: *const ::core::ffi::c_char,
    boolean: cJSON_bool,
) -> *mut cJSON {
    let Some(mut bool_item) = cJSON_CreateBool(boolean) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let bool_item_ptr = bool_item as *mut cJSON;
    let name = if name.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(name))
    };
    let Some(object) = object.as_mut() else {
        cJSON_Delete(Some(&*bool_item));
        return ::core::ptr::null_mut::<cJSON>();
    };
    if add_item_to_object(
        object,
        name,
        &mut *bool_item,
        global_hooks_snapshot(),
        false_0,
    ) != 0
    {
        return bool_item_ptr;
    }
    cJSON_Delete_ffi(bool_item_ptr);
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddNumberToObject"]
pub unsafe extern "C" fn cJSON_AddNumberToObject_ffi(
    mut object: *mut cJSON,
    name: *const ::core::ffi::c_char,
    number: ::core::ffi::c_double,
) -> *mut cJSON {
    let Some(mut number_item) = cJSON_CreateNumber(number) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let number_item_ptr = number_item as *mut cJSON;
    let name = if name.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(name))
    };
    let Some(object) = object.as_mut() else {
        cJSON_Delete(Some(&*number_item));
        return ::core::ptr::null_mut::<cJSON>();
    };
    if add_item_to_object(
        object,
        name,
        &mut *number_item,
        global_hooks_snapshot(),
        false_0,
    ) != 0
    {
        return number_item_ptr;
    }
    cJSON_Delete_ffi(number_item_ptr);
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddStringToObject"]
pub unsafe extern "C" fn cJSON_AddStringToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
    string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let name = if name.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(name))
    };
    let string = if string.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(string))
    };
    let Some(mut string_item) = cJSON_CreateString(string) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let string_item_ptr = string_item as *mut cJSON;
    let Some(object) = object.as_mut() else {
        cJSON_Delete(Some(&*string_item));
        return ::core::ptr::null_mut::<cJSON>();
    };
    if add_item_to_object(
        object,
        name,
        &mut *string_item,
        global_hooks_snapshot(),
        false_0,
    ) != 0
    {
        return string_item_ptr;
    }
    cJSON_Delete_ffi(string_item_ptr);
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddRawToObject"]
pub unsafe extern "C" fn cJSON_AddRawToObject_ffi(
    mut object: *mut cJSON,
    name: *const ::core::ffi::c_char,
    raw: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let name = if name.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(name))
    };
    let raw = if raw.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(raw))
    };
    let Some(mut raw_item) = cJSON_CreateRaw(raw) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let raw_item_ptr = raw_item as *mut cJSON;
    let Some(object) = object.as_mut() else {
        cJSON_Delete(Some(&*raw_item));
        return ::core::ptr::null_mut::<cJSON>();
    };
    if add_item_to_object(
        object,
        name,
        &mut *raw_item,
        global_hooks_snapshot(),
        false_0,
    ) != 0
    {
        return raw_item_ptr;
    }
    cJSON_Delete_ffi(raw_item_ptr);
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddObjectToObject"]
pub unsafe extern "C" fn cJSON_AddObjectToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let Some(mut object_item) = cJSON_CreateObject() else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let object_item_ptr = object_item as *mut cJSON;
    let name = if name.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(name))
    };
    let Some(object) = object.as_mut() else {
        cJSON_Delete(Some(&*object_item));
        return ::core::ptr::null_mut::<cJSON>();
    };
    if add_item_to_object(
        object,
        name,
        &mut *object_item,
        global_hooks_snapshot(),
        false_0,
    ) != 0
    {
        return object_item_ptr;
    }
    cJSON_Delete_ffi(object_item_ptr);
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddArrayToObject"]
pub unsafe extern "C" fn cJSON_AddArrayToObject_ffi(
    mut object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let Some(mut array) = cJSON_CreateArray() else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let array_ptr = array as *mut cJSON;
    let name = if name.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(name))
    };
    let Some(object) = object.as_mut() else {
        cJSON_Delete(Some(&*array));
        return ::core::ptr::null_mut::<cJSON>();
    };
    if add_item_to_object(object, name, &mut *array, global_hooks_snapshot(), false_0) != 0 {
        return array_ptr;
    }
    cJSON_Delete_ffi(array_ptr);
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_DetachItemViaPointer"]
pub unsafe extern "C" fn cJSON_DetachItemViaPointer_ffi(
    mut parent: *mut cJSON,
    mut item: *mut cJSON,
) -> *mut cJSON {
    if parent == item {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let (Some(parent), Some(item)) = (parent.as_mut(), item.as_mut()) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let item_link = Some(&*item);
    if !same_item_option(item_link, parent.child.get()) && item.prev.get().is_none() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    if !same_item_option(item_link, parent.child.get()) {
        if let Some(prev) = item.prev.get() {
            prev.next.set(item.next.get());
        }
    }
    if let Some(next) = item.next.get() {
        next.prev.set(item.prev.get());
    }
    if same_item_option(item_link, parent.child.get()) {
        parent.child.set(item.next.get());
    } else if item.next.get().is_none() {
        if let Some(child) = parent.child.get() {
            child.prev.set(item.prev.get());
        }
    }
    item.prev.set(None);
    item.next.set(None);
    item as *mut cJSON
}
#[export_name = "cJSON_DetachItemFromArray"]
pub unsafe extern "C" fn cJSON_DetachItemFromArray_ffi(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
) -> *mut cJSON {
    if which < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut to_detach = get_array_item(array.as_ref(), which as size_t)
        .map(::core::ptr::NonNull::from)
        .map_or(
            ::core::ptr::null_mut::<cJSON>(),
            ::core::ptr::NonNull::as_ptr,
        );
    if array == to_detach {
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_DetachItemViaPointer_ffi(array, to_detach)
}
#[export_name = "cJSON_DeleteItemFromArray"]
pub unsafe extern "C" fn cJSON_DeleteItemFromArray_ffi(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
) {
    if which >= 0 as ::core::ffi::c_int {
        let mut to_detach = get_array_item(array.as_ref(), which as size_t)
            .map(::core::ptr::NonNull::from)
            .map_or(
                ::core::ptr::null_mut::<cJSON>(),
                ::core::ptr::NonNull::as_ptr,
            );
        let detached = if array == to_detach {
            None
        } else {
            cJSON_DetachItemViaPointer_ffi(array, to_detach).as_ref()
        };
        cJSON_Delete(detached);
    }
}
#[export_name = "cJSON_DetachItemFromObject"]
pub unsafe extern "C" fn cJSON_DetachItemFromObject_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let string = if string.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(string))
    };
    let mut to_detach = get_object_item(
        object.as_ref(),
        string.map(::core::ffi::CStr::to_bytes),
        false_0,
    )
    .map(::core::ptr::NonNull::from)
    .map_or(
        ::core::ptr::null_mut::<cJSON>(),
        ::core::ptr::NonNull::as_ptr,
    );
    if object == to_detach {
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_DetachItemViaPointer_ffi(object, to_detach)
}
#[export_name = "cJSON_DetachItemFromObjectCaseSensitive"]
pub unsafe extern "C" fn cJSON_DetachItemFromObjectCaseSensitive_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let string = if string.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(string))
    };
    let mut to_detach = get_object_item(
        object.as_ref(),
        string.map(::core::ffi::CStr::to_bytes),
        true_0,
    )
    .map(::core::ptr::NonNull::from)
    .map_or(
        ::core::ptr::null_mut::<cJSON>(),
        ::core::ptr::NonNull::as_ptr,
    );
    if object == to_detach {
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_DetachItemViaPointer_ffi(object, to_detach)
}
#[export_name = "cJSON_DeleteItemFromObject"]
pub unsafe extern "C" fn cJSON_DeleteItemFromObject_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
) {
    let string = if string.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(string))
    };
    let mut to_detach = get_object_item(
        object.as_ref(),
        string.map(::core::ffi::CStr::to_bytes),
        false_0,
    )
    .map(::core::ptr::NonNull::from)
    .map_or(
        ::core::ptr::null_mut::<cJSON>(),
        ::core::ptr::NonNull::as_ptr,
    );
    let detached = if object == to_detach {
        None
    } else {
        cJSON_DetachItemViaPointer_ffi(object, to_detach).as_ref()
    };
    cJSON_Delete(detached);
}
#[export_name = "cJSON_DeleteItemFromObjectCaseSensitive"]
pub unsafe extern "C" fn cJSON_DeleteItemFromObjectCaseSensitive_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
) {
    let string = if string.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(string))
    };
    let mut to_detach = get_object_item(
        object.as_ref(),
        string.map(::core::ffi::CStr::to_bytes),
        true_0,
    )
    .map(::core::ptr::NonNull::from)
    .map_or(
        ::core::ptr::null_mut::<cJSON>(),
        ::core::ptr::NonNull::as_ptr,
    );
    let detached = if object == to_detach {
        None
    } else {
        cJSON_DetachItemViaPointer_ffi(object, to_detach).as_ref()
    };
    cJSON_Delete(detached);
}
#[export_name = "cJSON_InsertItemInArray"]
pub unsafe extern "C" fn cJSON_InsertItemInArray_ffi(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
    mut newitem: *mut cJSON,
) -> cJSON_bool {
    if array == newitem {
        return false_0;
    }
    let (Some(array), Some(newitem)) = (array.as_mut(), newitem.as_mut()) else {
        return false_0;
    };
    if which < 0 as ::core::ffi::c_int {
        return false_0;
    }
    let Some(after_inserted) = get_array_item(Some(&*array), which as size_t) else {
        return add_item_to_array(array, newitem);
    };
    if !same_item_option(Some(after_inserted), array.child.get())
        && after_inserted.prev.get().is_none()
    {
        return false_0;
    }
    newitem.next.set(Some(after_inserted));
    newitem.prev.set(after_inserted.prev.get());
    after_inserted.prev.set(Some(newitem));
    if same_item_option(Some(after_inserted), array.child.get()) {
        array.child.set(Some(newitem));
    } else {
        let previous = newitem
            .prev
            .get()
            .expect("non-head insert has previous link");
        previous.next.set(Some(newitem));
    }
    return true_0;
}
#[export_name = "cJSON_ReplaceItemViaPointer"]
pub unsafe extern "C" fn cJSON_ReplaceItemViaPointer_ffi(
    parent: *mut cJSON,
    item: *mut cJSON,
    mut replacement: *mut cJSON,
) -> cJSON_bool {
    if parent.is_null() || replacement.is_null() || item.is_null() {
        return false_0;
    }
    let (Some(parent), Some(item), Some(replacement)) =
        (parent.as_mut(), item.as_mut(), replacement.as_mut())
    else {
        return false_0;
    };
    if parent.child.get().is_none() {
        return false_0;
    }
    if same_item(replacement, item) {
        return true_0;
    }
    if same_item(parent, item) || same_item(parent, replacement) {
        return false_0;
    }
    replacement.next.set(item.next.get());
    replacement.prev.set(item.prev.get());
    if let Some(next) = replacement.next.get() {
        next.prev.set(Some(replacement));
    }
    if same_item_option(parent.child.get(), Some(item)) {
        if same_item_option(item.prev.get(), Some(item)) {
            replacement.prev.set(Some(replacement));
        }
        parent.child.set(Some(replacement));
    } else {
        if let Some(prev) = replacement.prev.get() {
            prev.next.set(Some(replacement));
        }
        if replacement.next.get().is_none() {
            let child = parent.child.get().expect("parent child checked above");
            child.prev.set(Some(replacement));
        }
    }
    item.next.set(None);
    item.prev.set(None);
    cJSON_Delete(Some(item));
    true_0
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
        get_array_item(array.as_ref(), which as size_t)
            .map(::core::ptr::NonNull::from)
            .map_or(
                ::core::ptr::null_mut::<cJSON>(),
                ::core::ptr::NonNull::as_ptr,
            ),
        newitem,
    );
}
#[export_name = "cJSON_ReplaceItemInObject"]
pub unsafe extern "C" fn cJSON_ReplaceItemInObject_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
    mut newitem: *mut cJSON,
) -> cJSON_bool {
    if string.is_null() || newitem.is_null() {
        return false_0;
    }
    let string = ::core::ffi::CStr::from_ptr(string);
    let replacement = newitem.as_mut().expect("newitem was checked for null");
    let Some(key_parts) = make_owned_string_key(string) else {
        return false_0;
    };
    replacement.string = Some(());
    replacement.string_storage = Some(key_parts.storage);
    replacement.string_buffer = Some(key_parts.buffer);
    replacement.type_0 &= !cJSON_StringIsConst;
    let mut item = if object == newitem {
        ::core::ptr::null_mut::<cJSON>()
    } else {
        get_object_item(object.as_ref(), Some(string.to_bytes()), false_0)
            .map(::core::ptr::NonNull::from)
            .map_or(
                ::core::ptr::null_mut::<cJSON>(),
                ::core::ptr::NonNull::as_ptr,
            )
    };
    cJSON_ReplaceItemViaPointer_ffi(object, item, newitem)
}
#[export_name = "cJSON_ReplaceItemInObjectCaseSensitive"]
pub unsafe extern "C" fn cJSON_ReplaceItemInObjectCaseSensitive_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
    mut newitem: *mut cJSON,
) -> cJSON_bool {
    if string.is_null() || newitem.is_null() {
        return false_0;
    }
    let string = ::core::ffi::CStr::from_ptr(string);
    let replacement = newitem.as_mut().expect("newitem was checked for null");
    let Some(key_parts) = make_owned_string_key(string) else {
        return false_0;
    };
    replacement.string = Some(());
    replacement.string_storage = Some(key_parts.storage);
    replacement.string_buffer = Some(key_parts.buffer);
    replacement.type_0 &= !cJSON_StringIsConst;
    let mut item = if object == newitem {
        ::core::ptr::null_mut::<cJSON>()
    } else {
        get_object_item(object.as_ref(), Some(string.to_bytes()), true_0)
            .map(::core::ptr::NonNull::from)
            .map_or(
                ::core::ptr::null_mut::<cJSON>(),
                ::core::ptr::NonNull::as_ptr,
            )
    };
    cJSON_ReplaceItemViaPointer_ffi(object, item, newitem)
}
pub fn cJSON_CreateNull() -> Option<&'static mut cJSON> {
    cJSON_New_Item(global_hooks_snapshot(), |item| {
        *item = cJSON {
            type_0: cJSON_NULL,
            ..cJSON::default()
        }
    })
}
#[export_name = "cJSON_CreateNull"]
pub unsafe extern "C" fn cJSON_CreateNull_ffi() -> *mut cJSON {
    match cJSON_CreateNull() {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_CreateTrue() -> Option<&'static mut cJSON> {
    cJSON_New_Item(global_hooks_snapshot(), |item| {
        *item = cJSON {
            type_0: cJSON_True,
            ..cJSON::default()
        }
    })
}
#[export_name = "cJSON_CreateTrue"]
pub unsafe extern "C" fn cJSON_CreateTrue_ffi() -> *mut cJSON {
    match cJSON_CreateTrue() {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_CreateFalse() -> Option<&'static mut cJSON> {
    cJSON_New_Item(global_hooks_snapshot(), |item| {
        *item = cJSON {
            type_0: cJSON_False,
            ..cJSON::default()
        }
    })
}
#[export_name = "cJSON_CreateFalse"]
pub unsafe extern "C" fn cJSON_CreateFalse_ffi() -> *mut cJSON {
    match cJSON_CreateFalse() {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_CreateBool(mut boolean: cJSON_bool) -> Option<&'static mut cJSON> {
    cJSON_New_Item(global_hooks_snapshot(), |item| {
        *item = cJSON {
            type_0: if boolean != 0 {
                cJSON_True
            } else {
                cJSON_False
            },
            ..cJSON::default()
        }
    })
}
#[export_name = "cJSON_CreateBool"]
pub unsafe extern "C" fn cJSON_CreateBool_ffi(mut boolean: cJSON_bool) -> *mut cJSON {
    match cJSON_CreateBool(boolean) {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_CreateNumber(mut num: ::core::ffi::c_double) -> Option<&'static mut cJSON> {
    let valueint = if num >= INT_MAX as ::core::ffi::c_double {
        INT_MAX
    } else if num <= INT_MIN as ::core::ffi::c_double {
        INT_MIN
    } else {
        num as ::core::ffi::c_int
    };
    cJSON_New_Item(global_hooks_snapshot(), |item| {
        *item = cJSON {
            type_0: cJSON_Number,
            valueint,
            valuedouble: num,
            ..cJSON::default()
        }
    })
}
#[export_name = "cJSON_CreateNumber"]
pub unsafe extern "C" fn cJSON_CreateNumber_ffi(mut num: ::core::ffi::c_double) -> *mut cJSON {
    match cJSON_CreateNumber(num) {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_CreateString(string: Option<&::core::ffi::CStr>) -> Option<&'static mut cJSON> {
    let hooks = global_hooks_snapshot();
    let value_parts = string.and_then(make_owned_valuestring)?;
    let mut stored_value = false;
    let item = cJSON_New_Item(hooks, |item| {
        *item = cJSON {
            type_0: cJSON_String,
            ..cJSON::default()
        };
        stored_value = set_valuestring_storage(item, value_parts.storage, value_parts.buffer) != 0;
    });
    let Some(item) = item else {
        return None;
    };
    if !stored_value {
        cJSON_Delete(Some(&*item));
        return None;
    }
    return Some(item);
}
#[export_name = "cJSON_CreateString"]
pub unsafe extern "C" fn cJSON_CreateString_ffi(
    mut string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let string = if string.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(string))
    };
    match cJSON_CreateString(string) {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_CreateStringReference(
    string: Option<&'static ::core::ffi::CStr>,
) -> Option<&'static mut cJSON> {
    let valuestring = string;
    let valuestring_len = string.map_or(0 as size_t, |string| string.to_bytes().len());
    let valuestring_storage = string.map(|string| string.to_bytes().to_vec());
    cJSON_New_Item(global_hooks_snapshot(), |item| {
        *item = cJSON {
            type_0: cJSON_String | cJSON_IsReference,
            valuestring,
            valuestring_len,
            valuestring_storage,
            ..cJSON::default()
        }
    })
}
#[export_name = "cJSON_CreateStringReference"]
pub unsafe extern "C" fn cJSON_CreateStringReference_ffi(
    mut string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let string: Option<&'static ::core::ffi::CStr> = if string.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(string))
    };
    match cJSON_CreateStringReference(string) {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_CreateObjectReference(child: Option<&'static cJSON>) -> Option<&'static mut cJSON> {
    return cJSON_New_Item(global_hooks_snapshot(), |item| {
        *item = cJSON {
            type_0: cJSON_Object | cJSON_IsReference,
            child: ::core::cell::Cell::new(child),
            ..cJSON::default()
        }
    });
}
#[export_name = "cJSON_CreateObjectReference"]
pub unsafe extern "C" fn cJSON_CreateObjectReference_ffi(mut child: *const cJSON) -> *mut cJSON {
    let child = if child.is_null() { None } else { Some(&*child) };
    match cJSON_CreateObjectReference(child) {
        Some(reference) => reference as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_CreateArrayReference(child: Option<&'static cJSON>) -> Option<&'static mut cJSON> {
    return cJSON_New_Item(global_hooks_snapshot(), |item| {
        *item = cJSON {
            type_0: cJSON_Array | cJSON_IsReference,
            child: ::core::cell::Cell::new(child),
            ..cJSON::default()
        }
    });
}
#[export_name = "cJSON_CreateArrayReference"]
pub unsafe extern "C" fn cJSON_CreateArrayReference_ffi(mut child: *const cJSON) -> *mut cJSON {
    let child = if child.is_null() { None } else { Some(&*child) };
    match cJSON_CreateArrayReference(child) {
        Some(reference) => reference as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_CreateRaw(raw: Option<&::core::ffi::CStr>) -> Option<&'static mut cJSON> {
    let hooks = global_hooks_snapshot();
    let value_parts = raw.and_then(make_owned_valuestring)?;
    let mut stored_value = false;
    let item = cJSON_New_Item(hooks, |item| {
        *item = cJSON {
            type_0: cJSON_Raw,
            ..cJSON::default()
        };
        stored_value = set_valuestring_storage(item, value_parts.storage, value_parts.buffer) != 0;
    });
    let Some(item) = item else {
        return None;
    };
    if !stored_value {
        cJSON_Delete(Some(&*item));
        return None;
    }
    return Some(item);
}
#[export_name = "cJSON_CreateRaw"]
pub unsafe extern "C" fn cJSON_CreateRaw_ffi(mut raw: *const ::core::ffi::c_char) -> *mut cJSON {
    let raw = if raw.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(raw))
    };
    match cJSON_CreateRaw(raw) {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_CreateArray() -> Option<&'static mut cJSON> {
    cJSON_New_Item(global_hooks_snapshot(), |item| {
        *item = cJSON {
            type_0: cJSON_Array,
            ..cJSON::default()
        }
    })
}
#[export_name = "cJSON_CreateArray"]
pub unsafe extern "C" fn cJSON_CreateArray_ffi() -> *mut cJSON {
    match cJSON_CreateArray() {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_CreateObject() -> Option<&'static mut cJSON> {
    cJSON_New_Item(global_hooks_snapshot(), |item| {
        *item = cJSON {
            type_0: cJSON_Object,
            ..cJSON::default()
        }
    })
}
#[export_name = "cJSON_CreateObject"]
pub unsafe extern "C" fn cJSON_CreateObject_ffi() -> *mut cJSON {
    match cJSON_CreateObject() {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_CreateIntArray(numbers: &[::core::ffi::c_int]) -> Option<&'static mut cJSON> {
    let Some(a) = cJSON_CreateArray() else {
        return None;
    };
    for number in numbers {
        let Some(n) = cJSON_CreateNumber(*number as ::core::ffi::c_double) else {
            cJSON_Delete(Some(&*a));
            return None;
        };
        add_item_to_array(&mut *a, n);
    }
    return Some(a);
}
#[export_name = "cJSON_CreateIntArray"]
pub unsafe extern "C" fn cJSON_CreateIntArray_ffi(
    mut numbers: *const ::core::ffi::c_int,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    match cJSON_CreateIntArray(::core::slice::from_raw_parts(numbers, count as usize)) {
        Some(array) => array as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_CreateFloatArray(numbers: &[::core::ffi::c_float]) -> Option<&'static mut cJSON> {
    let Some(a) = cJSON_CreateArray() else {
        return None;
    };
    for number in numbers {
        let Some(n) = cJSON_CreateNumber(*number as ::core::ffi::c_double) else {
            cJSON_Delete(Some(&*a));
            return None;
        };
        add_item_to_array(&mut *a, n);
    }
    return Some(a);
}
#[export_name = "cJSON_CreateFloatArray"]
pub unsafe extern "C" fn cJSON_CreateFloatArray_ffi(
    mut numbers: *const ::core::ffi::c_float,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    match cJSON_CreateFloatArray(::core::slice::from_raw_parts(numbers, count as usize)) {
        Some(array) => array as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_CreateDoubleArray(numbers: &[::core::ffi::c_double]) -> Option<&'static mut cJSON> {
    let Some(a) = cJSON_CreateArray() else {
        return None;
    };
    for number in numbers {
        let Some(n) = cJSON_CreateNumber(*number) else {
            cJSON_Delete(Some(&*a));
            return None;
        };
        add_item_to_array(&mut *a, n);
    }
    return Some(a);
}
#[export_name = "cJSON_CreateDoubleArray"]
pub unsafe extern "C" fn cJSON_CreateDoubleArray_ffi(
    mut numbers: *const ::core::ffi::c_double,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    match cJSON_CreateDoubleArray(::core::slice::from_raw_parts(numbers, count as usize)) {
        Some(array) => array as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_CreateStringArray(
    strings: &[Option<&::core::ffi::CStr>],
) -> Option<&'static mut cJSON> {
    let Some(a) = cJSON_CreateArray() else {
        return None;
    };
    for string in strings {
        let Some(string) = string else {
            cJSON_Delete(Some(&*a));
            return None;
        };
        let Some(n) = cJSON_CreateString(Some(string)) else {
            cJSON_Delete(Some(&*a));
            return None;
        };
        add_item_to_array(&mut *a, n);
    }
    return Some(a);
}
#[export_name = "cJSON_CreateStringArray"]
pub unsafe extern "C" fn cJSON_CreateStringArray_ffi(
    mut strings: *const *const ::core::ffi::c_char,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    if count < 0 as ::core::ffi::c_int || strings.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let raw_strings = ::core::slice::from_raw_parts(strings, count as usize);
    let mut strings: ::std::vec::Vec<Option<&::core::ffi::CStr>> =
        ::std::vec::Vec::with_capacity(raw_strings.len());
    for raw_string in raw_strings {
        if raw_string.is_null() {
            strings.push(None);
        } else {
            strings.push(Some(::core::ffi::CStr::from_ptr(*raw_string)));
        }
    }
    match cJSON_CreateStringArray(&strings) {
        Some(array) => array as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
#[export_name = "cJSON_Duplicate"]
pub unsafe extern "C" fn cJSON_Duplicate_ffi(
    mut item: *const cJSON,
    mut recurse: cJSON_bool,
) -> *mut cJSON {
    match cJSON_Duplicate_rec(item.as_ref(), 0 as size_t, recurse) {
        Some(duplicate) => duplicate as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_Duplicate_rec(
    item: Option<&cJSON>,
    mut depth: size_t,
    mut recurse: cJSON_bool,
) -> Option<&'static mut cJSON> {
    let Some(item) = item else {
        return None;
    };
    let hooks = global_hooks_snapshot();
    let valuestring = item.valuestring_storage.as_deref();
    let string = if item.string.is_some() && item.type_0 & cJSON_StringIsConst == 0 {
        item.string_storage.as_deref()
    } else {
        None
    };
    let mut copied_valuestring = valuestring.is_none();
    let mut copied_string = string.is_none();

    let newitem = cJSON_New_Item(hooks, |duplicate| {
        *duplicate = cJSON {
            type_0: item.type_0 & !cJSON_IsReference,
            valueint: item.valueint,
            valuedouble: item.valuedouble,
            string: if item.string.is_some() && item.type_0 & cJSON_StringIsConst != 0 {
                item.string
            } else {
                None
            },
            string_storage: if item.string.is_some() && item.type_0 & cJSON_StringIsConst != 0 {
                item.string_storage.clone()
            } else {
                None
            },
            ..cJSON::default()
        };
        if let Some(valuestring) = valuestring {
            let Some(value_parts) = make_owned_valuestring_from_bytes(valuestring) else {
                return;
            };
            copied_valuestring =
                set_valuestring_storage(duplicate, value_parts.storage, value_parts.buffer) != 0;
        }
        if let Some(string) = string {
            let Some(string_key) = make_owned_string_key_from_bytes(string) else {
                return;
            };
            duplicate.string = Some(());
            duplicate.string_storage = Some(string_key.storage);
            duplicate.string_buffer = Some(string_key.buffer);
            copied_string = true;
        }
    });
    let Some(newitem) = newitem else {
        return None;
    };
    if !copied_valuestring || !copied_string {
        cJSON_Delete(Some(&*newitem));
        return None;
    }
    if recurse == 0 {
        return Some(newitem);
    }

    let mut child_index = 0 as size_t;
    while let Some(child_ref) = get_array_item(Some(item), child_index) {
        if depth >= CJSON_CIRCULAR_LIMIT as size_t {
            cJSON_Delete(Some(&*newitem));
            return None;
        }
        let newchild =
            cJSON_Duplicate_rec(Some(child_ref), depth.wrapping_add(1 as size_t), true_0);
        let Some(newchild) = newchild else {
            cJSON_Delete(Some(&*newitem));
            return None;
        };
        add_item_to_array(&mut *newitem, newchild);
        child_index = child_index.wrapping_add(1 as size_t);
    }
    return Some(newitem);
}
#[export_name = "cJSON_Duplicate_rec"]
pub unsafe extern "C" fn cJSON_Duplicate_rec_ffi(
    mut item: *const cJSON,
    mut depth: size_t,
    mut recurse: cJSON_bool,
) -> *mut cJSON {
    match cJSON_Duplicate_rec(item.as_ref(), depth, recurse) {
        Some(duplicate) => duplicate as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
fn minify_byte_at(json: &[u8], index: usize) -> u8 {
    json.get(index).copied().unwrap_or(0)
}

fn minify_write_byte(json: &mut [u8], index: usize, value: u8) {
    if let Some(slot) = json.get_mut(index) {
        *slot = value;
    }
}

fn skip_oneline_comment(input: &mut usize, json: &[u8]) {
    *input = input.wrapping_add(2);
    while minify_byte_at(json, *input) != 0 {
        if minify_byte_at(json, *input) == b'\n' {
            *input = input.wrapping_add(1);
            return;
        }
        *input = input.wrapping_add(1);
    }
}

fn skip_multiline_comment(input: &mut usize, json: &[u8]) {
    *input = input.wrapping_add(2);
    while minify_byte_at(json, *input) != 0 {
        if minify_byte_at(json, *input) == b'*'
            && minify_byte_at(json, input.wrapping_add(1)) == b'/'
        {
            *input = input.wrapping_add(2);
            return;
        }
        *input = input.wrapping_add(1);
    }
}

fn minify_string(json: &mut [u8], input: &mut usize, output: &mut usize) {
    minify_write_byte(json, *output, minify_byte_at(json, *input));
    *input = input.wrapping_add(1);
    *output = output.wrapping_add(1);
    while minify_byte_at(json, *input) != 0 {
        let current = minify_byte_at(json, *input);
        minify_write_byte(json, *output, current);
        if current == b'"' {
            minify_write_byte(json, *output, b'"');
            *input = input.wrapping_add(1);
            *output = output.wrapping_add(1);
            return;
        } else if current == b'\\' && minify_byte_at(json, input.wrapping_add(1)) == b'"' {
            minify_write_byte(
                json,
                output.wrapping_add(1),
                minify_byte_at(json, input.wrapping_add(1)),
            );
            *input = input.wrapping_add(1);
            *output = output.wrapping_add(1);
        }
        *input = input.wrapping_add(1);
        *output = output.wrapping_add(1);
    }
}

pub fn cJSON_Minify(json: &mut [u8]) {
    let mut input: usize = 0;
    let mut output: usize = 0;
    while minify_byte_at(json, input) != 0 {
        match minify_byte_at(json, input) {
            b' ' | b'\t' | b'\r' | b'\n' => {
                input = input.wrapping_add(1);
            }
            b'/' => {
                if minify_byte_at(json, input.wrapping_add(1)) == b'/' {
                    skip_oneline_comment(&mut input, json);
                } else if minify_byte_at(json, input.wrapping_add(1)) == b'*' {
                    skip_multiline_comment(&mut input, json);
                } else {
                    input = input.wrapping_add(1);
                }
            }
            b'"' => {
                minify_string(json, &mut input, &mut output);
            }
            _ => {
                minify_write_byte(json, output, minify_byte_at(json, input));
                input = input.wrapping_add(1);
                output = output.wrapping_add(1);
            }
        }
    }
    minify_write_byte(json, output, b'\0');
}
#[export_name = "cJSON_Minify"]
pub unsafe extern "C" fn cJSON_Minify_ffi(mut json: *mut ::core::ffi::c_char) {
    if json.is_null() {
        return;
    }
    let length = strlen(json);
    let json_bytes = ::core::slice::from_raw_parts_mut(
        json.cast::<u8>(),
        length.wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 1]>() as size_t),
    );
    cJSON_Minify(json_bytes)
}
pub fn cJSON_IsInvalid(item: Option<&cJSON>) -> cJSON_bool {
    let Some(item) = item else {
        return false_0;
    };
    return (item.type_0 & 0xff as ::core::ffi::c_int == cJSON_Invalid) as ::core::ffi::c_int;
}
#[export_name = "cJSON_IsInvalid"]
pub unsafe extern "C" fn cJSON_IsInvalid_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsInvalid(item.as_ref())
}
pub fn cJSON_IsFalse(item: Option<&cJSON>) -> cJSON_bool {
    let Some(item) = item else {
        return false_0;
    };
    return (item.type_0 & 0xff as ::core::ffi::c_int == cJSON_False) as ::core::ffi::c_int;
}
#[export_name = "cJSON_IsFalse"]
pub unsafe extern "C" fn cJSON_IsFalse_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsFalse(item.as_ref())
}
pub fn cJSON_IsTrue(item: Option<&cJSON>) -> cJSON_bool {
    let Some(item) = item else {
        return false_0;
    };
    return (item.type_0 & 0xff as ::core::ffi::c_int == cJSON_True) as ::core::ffi::c_int;
}
#[export_name = "cJSON_IsTrue"]
pub unsafe extern "C" fn cJSON_IsTrue_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsTrue(item.as_ref())
}
pub fn cJSON_IsBool(item: Option<&cJSON>) -> cJSON_bool {
    let Some(item) = item else {
        return false_0;
    };
    return (item.type_0 & (cJSON_True | cJSON_False) != 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[export_name = "cJSON_IsBool"]
pub unsafe extern "C" fn cJSON_IsBool_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsBool(item.as_ref())
}
pub fn cJSON_IsNull(item: Option<&cJSON>) -> cJSON_bool {
    let Some(item) = item else {
        return false_0;
    };
    return (item.type_0 & 0xff as ::core::ffi::c_int == cJSON_NULL) as ::core::ffi::c_int;
}
#[export_name = "cJSON_IsNull"]
pub unsafe extern "C" fn cJSON_IsNull_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsNull(item.as_ref())
}
pub fn cJSON_IsNumber(item: Option<&cJSON>) -> cJSON_bool {
    let Some(item) = item else {
        return false_0;
    };
    return (item.type_0 & 0xff as ::core::ffi::c_int == cJSON_Number) as ::core::ffi::c_int;
}
#[export_name = "cJSON_IsNumber"]
pub unsafe extern "C" fn cJSON_IsNumber_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsNumber(item.as_ref())
}
pub fn cJSON_IsString(item: Option<&cJSON>) -> cJSON_bool {
    let Some(item) = item else {
        return false_0;
    };
    return (item.type_0 & 0xff as ::core::ffi::c_int == cJSON_String) as ::core::ffi::c_int;
}
#[export_name = "cJSON_IsString"]
pub unsafe extern "C" fn cJSON_IsString_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsString(item.as_ref())
}
pub fn cJSON_IsArray(item: Option<&cJSON>) -> cJSON_bool {
    let Some(item) = item else {
        return false_0;
    };
    return (item.type_0 & 0xff as ::core::ffi::c_int == cJSON_Array) as ::core::ffi::c_int;
}
#[export_name = "cJSON_IsArray"]
pub unsafe extern "C" fn cJSON_IsArray_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsArray(item.as_ref())
}
pub fn cJSON_IsObject(item: Option<&cJSON>) -> cJSON_bool {
    let Some(item) = item else {
        return false_0;
    };
    return (item.type_0 & 0xff as ::core::ffi::c_int == cJSON_Object) as ::core::ffi::c_int;
}
#[export_name = "cJSON_IsObject"]
pub unsafe extern "C" fn cJSON_IsObject_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsObject(item.as_ref())
}
pub fn cJSON_IsRaw(item: Option<&cJSON>) -> cJSON_bool {
    let Some(item) = item else {
        return false_0;
    };
    return (item.type_0 & 0xff as ::core::ffi::c_int == cJSON_Raw) as ::core::ffi::c_int;
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
            let (Some(a_valuestring), Some(b_valuestring)) = (
                a.valuestring_storage.as_deref(),
                b.valuestring_storage.as_deref(),
            ) else {
                return false_0;
            };
            if a_valuestring == b_valuestring {
                return true_0;
            }
            return false_0;
        }
        cJSON_Array => {
            let mut element_index = 0 as size_t;
            loop {
                match (
                    get_array_item(Some(a), element_index),
                    get_array_item(Some(b), element_index),
                ) {
                    (Some(a_element), Some(b_element)) => {
                        if cJSON_Compare(Some(a_element), Some(b_element), case_sensitive) == 0 {
                            return false_0;
                        }
                    }
                    (None, None) => return true_0,
                    _ => return false_0,
                }
                element_index = element_index.wrapping_add(1 as size_t);
            }
        }
        cJSON_Object => {
            let mut a_element_index = 0 as size_t;
            while let Some(a_element) = get_array_item(Some(a), a_element_index) {
                let a_key = a_element.string_storage.as_deref();
                let Some(b_element_0) = get_object_item(Some(b), a_key, case_sensitive) else {
                    return false_0;
                };
                if cJSON_Compare(Some(a_element), Some(b_element_0), case_sensitive) == 0 {
                    return false_0;
                }
                a_element_index = a_element_index.wrapping_add(1 as size_t);
            }
            let mut b_element_index = 0 as size_t;
            while let Some(b_element) = get_array_item(Some(b), b_element_index) {
                let b_key = b_element.string_storage.as_deref();
                let Some(a_element_0) = get_object_item(Some(a), b_key, case_sensitive) else {
                    return false_0;
                };
                if cJSON_Compare(Some(b_element), Some(a_element_0), case_sensitive) == 0 {
                    return false_0;
                }
                b_element_index = b_element_index.wrapping_add(1 as size_t);
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
    global_hooks_snapshot()
        .allocate
        .expect("non-null function pointer")(size)
}
fn cJSON_free(object: Option<::core::ptr::NonNull<::core::ffi::c_void>>) {
    let Some(object) = object else {
        return;
    };
    global_hooks_snapshot()
        .deallocate
        .expect("non-null function pointer")(object.as_ptr());
}
#[export_name = "cJSON_free"]
pub unsafe extern "C" fn cJSON_free_ffi(mut object: *mut ::core::ffi::c_void) {
    cJSON_free(::core::ptr::NonNull::new(object))
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __DBL_EPSILON__: ::core::ffi::c_double = 2.2204460492503131e-16f64;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const INT_MIN: ::core::ffi::c_int = -2147483647 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
pub const DBL_EPSILON: ::core::ffi::c_double = __DBL_EPSILON__;
