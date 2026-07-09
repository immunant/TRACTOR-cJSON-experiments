unsafe extern "C" {
    safe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    safe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    safe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    safe fn free(__ptr: *mut ::core::ffi::c_void);
}
use ::core::cell::{Cell, RefCell};
use ::core::num::{NonZeroI32, NonZeroUsize};
use ::std::rc::Rc;

use lexical_write_float::{NumberFormatBuilder, Options, ToLexicalWithOptions};

pub type size_t = usize;
#[derive(Clone)]
pub enum CjsonString {
    Null,
    Owned(Rc<RefCell<Vec<::core::ffi::c_uchar>>>),
    Borrowed(&'static ::std::ffi::CStr),
}
impl CjsonString {
    fn copied(bytes: Option<&[::core::ffi::c_uchar]>) -> Option<Self> {
        let bytes = bytes?;
        let mut copy = Vec::new();
        if copy.try_reserve_exact(bytes.len()).is_err() {
            return None;
        }
        copy.extend_from_slice(bytes);
        Some(Self::Owned(Rc::new(RefCell::new(copy))))
    }

    fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    fn as_cstr(&self) -> Option<CjsonStringRef> {
        match self {
            Self::Null => None,
            Self::Owned(bytes) => Some(CjsonStringRef::Owned(Rc::clone(bytes))),
            Self::Borrowed(value) => Some(CjsonStringRef::Borrowed(value)),
        }
    }
}
pub enum CjsonStringRef {
    Owned(Rc<RefCell<Vec<::core::ffi::c_uchar>>>),
    Borrowed(&'static ::std::ffi::CStr),
}
impl CjsonStringRef {
    fn with_cstr<R>(&self, f: impl FnOnce(&::std::ffi::CStr) -> R) -> Option<R> {
        match self {
            Self::Owned(bytes) => {
                let bytes = bytes.borrow();
                let Ok(value) = ::std::ffi::CStr::from_bytes_until_nul(&bytes) else {
                    return None;
                };
                Some(f(value))
            }
            Self::Borrowed(value) => Some(f(value)),
        }
    }

    fn to_bytes(&self) -> Option<Vec<::core::ffi::c_uchar>> {
        self.with_cstr(|value| value.to_bytes().to_vec())
    }

    fn to_bytes_with_nul(&self) -> Option<Vec<::core::ffi::c_uchar>> {
        self.with_cstr(|value| value.to_bytes_with_nul().to_vec())
    }
}
#[repr(C)]
pub struct cJSON {
    pub next: Cell<Option<&'static cJSON>>,
    pub prev: Cell<Option<&'static cJSON>>,
    pub child: Cell<Option<&'static cJSON>>,
    pub type_0: ::core::ffi::c_int,
    pub valuestring: RefCell<CjsonString>,
    pub valueint: ::core::ffi::c_int,
    pub valuedouble: ::core::ffi::c_double,
    pub string: RefCell<CjsonString>,
}
impl Clone for cJSON {
    fn clone(&self) -> Self {
        Self {
            next: Cell::new(self.next.get()),
            prev: Cell::new(self.prev.get()),
            child: Cell::new(self.child.get()),
            type_0: self.type_0,
            valuestring: RefCell::new(self.valuestring.borrow().clone()),
            valueint: self.valueint,
            valuedouble: self.valuedouble,
            string: RefCell::new(self.string.borrow().clone()),
        }
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct error {
    pub json: usize,
    pub position: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parse_buffer<'a> {
    pub content: &'a [::core::ffi::c_uchar],
    pub length: size_t,
    pub offset: size_t,
    pub depth: size_t,
    pub hooks: internal_hooks,
}
impl parse_buffer<'_> {
    fn current_byte(&self) -> Option<::core::ffi::c_uchar> {
        if self.offset < self.length {
            self.content.get(self.offset).copied()
        } else {
            None
        }
    }

    fn starts_with_at_offset(&self, expected: &[u8]) -> bool {
        let end = self.offset.wrapping_add(expected.len());
        end <= self.length && self.content.get(self.offset..end) == Some(expected)
    }
}
pub enum printbuffer_storage<'a> {
    Dynamic(Vec<::core::ffi::c_uchar>),
    Fixed(&'a mut [::core::ffi::c_uchar]),
}
pub struct printbuffer<'a> {
    pub storage: printbuffer_storage<'a>,
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
const CJSON_FLOAT_FORMAT: u128 = NumberFormatBuilder::new()
    .required_exponent_sign(true)
    .build_strict();
const CJSON_FLOAT_OPTIONS_15: Options = Options::builder()
    .max_significant_digits(NonZeroUsize::new(15))
    .positive_exponent_break(NonZeroI32::new(15))
    .negative_exponent_break(NonZeroI32::new(-4))
    .trim_floats(true)
    .build_strict();
const CJSON_FLOAT_OPTIONS_17: Options = Options::builder()
    .max_significant_digits(NonZeroUsize::new(17))
    .positive_exponent_break(NonZeroI32::new(17))
    .negative_exponent_break(NonZeroI32::new(-4))
    .trim_floats(true)
    .build_strict();
static global_error_json: ::std::sync::atomic::AtomicUsize =
    ::std::sync::atomic::AtomicUsize::new(0);
static global_error_position: ::std::sync::atomic::AtomicUsize =
    ::std::sync::atomic::AtomicUsize::new(0);
pub fn cJSON_GetErrorPtr() -> Option<error> {
    let json = global_error_json.load(::std::sync::atomic::Ordering::Relaxed);
    if json == 0 {
        return None;
    }
    Some(error {
        json,
        position: global_error_position.load(::std::sync::atomic::Ordering::Relaxed),
    })
}
#[export_name = "cJSON_GetErrorPtr"]
pub unsafe extern "C" fn cJSON_GetErrorPtr_ffi() -> *const ::core::ffi::c_char {
    match cJSON_GetErrorPtr() {
        Some(error) => ::core::ptr::with_exposed_provenance::<::core::ffi::c_uchar>(error.json)
            .wrapping_add(error.position) as *const ::core::ffi::c_char,
        None => ::core::ptr::null::<::core::ffi::c_char>(),
    }
}
pub fn cJSON_GetStringValue(item: Option<&cJSON>) -> Option<CjsonStringRef> {
    match item {
        Some(item) if cJSON_IsString(Some(item)) != 0 => print_string(item, CjsonStringSlot::Value),
        _ => None,
    }
}
#[export_name = "cJSON_GetStringValue"]
pub unsafe extern "C" fn cJSON_GetStringValue_ffi(item: *const cJSON) -> *mut ::core::ffi::c_char {
    match cJSON_GetStringValue(unsafe { item.as_ref() }) {
        Some(CjsonStringRef::Owned(value)) => value.borrow().as_ptr().cast_mut().cast(),
        Some(CjsonStringRef::Borrowed(value)) => value.as_ptr().cast_mut(),
        None => ::core::ptr::null_mut::<::core::ffi::c_char>(),
    }
}
pub fn cJSON_GetNumberValue(item: Option<&cJSON>) -> ::core::ffi::c_double {
    match item {
        Some(item) if cJSON_IsNumber(Some(item)) != 0 => item.valuedouble,
        _ => 0.0f64 / 0.0f64,
    }
}
#[export_name = "cJSON_GetNumberValue"]
pub unsafe extern "C" fn cJSON_GetNumberValue_ffi(item: *const cJSON) -> ::core::ffi::c_double {
    cJSON_GetNumberValue(unsafe { item.as_ref() })
}
pub fn cJSON_Version() -> &'static [u8] {
    b"1.7.19\0"
}
#[export_name = "cJSON_Version"]
pub unsafe extern "C" fn cJSON_Version_ffi() -> *const ::core::ffi::c_char {
    cJSON_Version().as_ptr().cast::<::core::ffi::c_char>()
}
fn case_insensitive_cstr_cmp(
    string1: &::std::ffi::CStr,
    string2: &::std::ffi::CStr,
) -> ::core::ffi::c_int {
    let string1 = string1.to_bytes_with_nul();
    let string2 = string2.to_bytes_with_nul();
    let mut index = 0usize;
    loop {
        let character1 = string1[index];
        let character2 = string2[index];
        let lower1 = character1.to_ascii_lowercase() as ::core::ffi::c_int;
        let lower2 = character2.to_ascii_lowercase() as ::core::ffi::c_int;
        if lower1 != lower2 {
            return lower1 - lower2;
        }
        if character1 as ::core::ffi::c_int == '\0' as i32 {
            return 0 as ::core::ffi::c_int;
        }
        index = index.wrapping_add(1);
    }
}
static global_hooks: ::std::sync::Mutex<internal_hooks> = ::std::sync::Mutex::new(internal_hooks {
    allocate: Some(malloc as extern "C" fn(size_t) -> *mut ::core::ffi::c_void),
    deallocate: Some(free as extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    reallocate: Some(
        realloc as extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void,
    ),
});
fn global_hooks_guard() -> ::std::sync::MutexGuard<'static, internal_hooks> {
    global_hooks
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}
fn global_hooks_snapshot() -> internal_hooks {
    *global_hooks_guard()
}
pub fn cJSON_InitHooks(hooks: Option<&cJSON_Hooks>) {
    let mut hooks_guard = global_hooks_guard();
    let Some(hooks) = hooks else {
        hooks_guard.allocate = Some(malloc as extern "C" fn(size_t) -> *mut ::core::ffi::c_void)
            as Option<extern "C" fn(size_t) -> *mut ::core::ffi::c_void>;
        hooks_guard.deallocate = Some(free as extern "C" fn(*mut ::core::ffi::c_void) -> ())
            as Option<extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
        hooks_guard.reallocate = Some(
            realloc as extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void,
        )
            as Option<extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>;
        return;
    };
    hooks_guard.allocate = Some(malloc as extern "C" fn(size_t) -> *mut ::core::ffi::c_void)
        as Option<extern "C" fn(size_t) -> *mut ::core::ffi::c_void>;
    if hooks.malloc_fn.is_some() {
        hooks_guard.allocate = hooks.malloc_fn;
    }
    hooks_guard.deallocate = Some(free as extern "C" fn(*mut ::core::ffi::c_void) -> ())
        as Option<extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    if hooks.free_fn.is_some() {
        hooks_guard.deallocate = hooks.free_fn;
    }
    hooks_guard.reallocate = None;
    if hooks_guard.allocate == Some(malloc as extern "C" fn(size_t) -> *mut ::core::ffi::c_void)
        && hooks_guard.deallocate == Some(free as extern "C" fn(*mut ::core::ffi::c_void) -> ())
    {
        hooks_guard.reallocate = Some(
            realloc as extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void,
        )
            as Option<extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>;
    }
}
#[export_name = "cJSON_InitHooks"]
pub unsafe extern "C" fn cJSON_InitHooks_ffi(hooks: *mut cJSON_Hooks) {
    cJSON_InitHooks(unsafe { hooks.as_ref() })
}
fn cJSON_New_Item(
    _hooks: &internal_hooks,
    item_type: ::core::ffi::c_int,
) -> Option<&'static mut cJSON> {
    let mut node = Vec::new();
    if node.try_reserve_exact(1).is_err() {
        return None;
    }
    node.push(cJSON {
        next: Cell::new(None),
        prev: Cell::new(None),
        child: Cell::new(None),
        type_0: item_type,
        valuestring: RefCell::new(CjsonString::Null),
        valueint: 0 as ::core::ffi::c_int,
        valuedouble: 0 as ::core::ffi::c_double,
        string: RefCell::new(CjsonString::Null),
    });
    Box::leak(node.into_boxed_slice()).first_mut()
}
pub fn cJSON_Delete(mut item: Option<&cJSON>) {
    while let Some(item_ref) = item {
        let next = item_ref.next.get();
        if item_ref.type_0 & cJSON_IsReference == 0 {
            while item_ref.child.get().is_some() {
                let child = cJSON_DetachItemFromArray(Some(item_ref), 0 as size_t);
                cJSON_Delete(child);
            }
        }
        item_ref.valuestring.replace(CjsonString::Null);
        item_ref.string.replace(CjsonString::Null);
        item_ref.child.set(None);
        item_ref.prev.set(None);
        item_ref.next.set(None);
        item = next;
    }
}
#[export_name = "cJSON_Delete"]
pub unsafe extern "C" fn cJSON_Delete_ffi(mut item: *mut cJSON) {
    cJSON_Delete(unsafe { item.as_ref() })
}
fn decimal_number_prefix_length(input: &[::core::ffi::c_uchar]) -> Option<size_t> {
    let mut index = 0usize;
    if input
        .get(index)
        .is_some_and(|byte| *byte == b'+' || *byte == b'-')
    {
        index = index.wrapping_add(1);
    }

    let mut digits_before_decimal = 0usize;
    while input.get(index).is_some_and(|byte| byte.is_ascii_digit()) {
        digits_before_decimal = digits_before_decimal.wrapping_add(1);
        index = index.wrapping_add(1);
    }

    let mut digits_after_decimal = 0usize;
    if input.get(index) == Some(&b'.') {
        index = index.wrapping_add(1);
        while input.get(index).is_some_and(|byte| byte.is_ascii_digit()) {
            digits_after_decimal = digits_after_decimal.wrapping_add(1);
            index = index.wrapping_add(1);
        }
    }

    if digits_before_decimal == 0 && digits_after_decimal == 0 {
        return None;
    }

    let mantissa_end = index;
    if input
        .get(index)
        .is_some_and(|byte| *byte == b'e' || *byte == b'E')
    {
        index = index.wrapping_add(1);
        if input
            .get(index)
            .is_some_and(|byte| *byte == b'+' || *byte == b'-')
        {
            index = index.wrapping_add(1);
        }

        let exponent_start = index;
        while input.get(index).is_some_and(|byte| byte.is_ascii_digit()) {
            index = index.wrapping_add(1);
        }
        if index == exponent_start {
            return Some(mantissa_end);
        }
    }

    Some(index)
}
fn parse_number(item: &mut cJSON, input_buffer: &mut parse_buffer<'_>) -> cJSON_bool {
    let number_bytes = &input_buffer.content[input_buffer.offset..input_buffer.length];
    let Some(number_string_length) = decimal_number_prefix_length(number_bytes) else {
        return false_0;
    };
    let number_string = match ::core::str::from_utf8(&number_bytes[..number_string_length as usize])
    {
        Ok(number_string) => number_string,
        Err(_) => return false_0,
    };
    let number = match number_string.parse::<::core::ffi::c_double>() {
        Ok(number) => number,
        Err(_) => return false_0,
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
    input_buffer.offset = input_buffer
        .offset
        .wrapping_add(number_string_length as size_t);
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
    cJSON_SetNumberHelper(&mut *object, number)
}
pub fn cJSON_SetValuestring<'a>(
    object: Option<&'a mut cJSON>,
    valuestring: Option<&::std::ffi::CStr>,
) -> cJSON_bool {
    let Some(object) = object else {
        return false_0;
    };
    let Some(valuestring) = valuestring else {
        return false_0;
    };
    if object.type_0 & cJSON_String == 0 || object.type_0 & cJSON_IsReference != 0 {
        return false_0;
    }
    if object.valuestring.borrow().is_null() {
        return false_0;
    }

    let value_bytes = valuestring.to_bytes_with_nul();
    let v1_len = value_bytes.len().wrapping_sub(1);
    let v2_len = match print_string(object, CjsonStringSlot::Value) {
        Some(current_value) => match current_value.to_bytes() {
            Some(bytes) => bytes.len(),
            None => return false_0,
        },
        None => return false_0,
    };
    if v1_len <= v2_len {
        let current_value = object.valuestring.borrow().clone();
        let CjsonString::Owned(destination_buffer) = current_value else {
            return false_0;
        };
        let destination = destination_buffer.borrow();
        let source_start = valuestring.as_ptr() as usize;
        let source_end = source_start.wrapping_add(v1_len);
        let destination_start = destination.as_ptr() as usize;
        let destination_end = destination_start.wrapping_add(v2_len);
        if !(source_end < destination_start || destination_end < source_start) {
            return false_0;
        }
        if destination.len() < value_bytes.len() {
            return false_0;
        }
        drop(destination);

        destination_buffer.borrow_mut()[..value_bytes.len()].copy_from_slice(value_bytes);
        return true_0;
    }

    let Some(copy) = CjsonString::copied(Some(valuestring.to_bytes_with_nul())) else {
        return false_0;
    };
    object.valuestring.replace(copy);
    true_0
}
#[export_name = "cJSON_SetValuestring"]
pub unsafe extern "C" fn cJSON_SetValuestring_ffi(
    mut object: *mut cJSON,
    mut valuestring: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let valuestring = if valuestring.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(valuestring) })
    };
    let Some(object) = (unsafe { object.as_mut() }) else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    if cJSON_SetValuestring(Some(&mut *object), valuestring) != 0 {
        match &*object.valuestring.borrow() {
            CjsonString::Null => ::core::ptr::null_mut::<::core::ffi::c_char>(),
            CjsonString::Owned(value) => value.borrow().as_ptr().cast_mut().cast(),
            CjsonString::Borrowed(value) => value.as_ptr().cast_mut(),
        }
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    }
}
impl printbuffer_storage<'_> {
    fn len(&self) -> size_t {
        match self {
            printbuffer_storage::Dynamic(buffer) => buffer.len(),
            printbuffer_storage::Fixed(buffer) => buffer.len(),
        }
    }
}
fn ensure<'p>(
    p: &'p mut printbuffer<'_>,
    needed: size_t,
) -> Option<&'p mut [::core::ffi::c_uchar]> {
    let requested = needed;
    let current_length = p.storage.len();
    if current_length > 0 as size_t && p.offset >= current_length {
        return None;
    }
    if needed > INT_MAX as size_t {
        return None;
    }
    let needed = needed.wrapping_add(p.offset.wrapping_add(1 as size_t));
    match &mut p.storage {
        printbuffer_storage::Fixed(buffer) => {
            if needed > buffer.len() {
                return None;
            }
            buffer.get_mut(p.offset..p.offset.wrapping_add(requested))
        }
        printbuffer_storage::Dynamic(buffer) => {
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
                if newsize > buffer.len() {
                    if buffer
                        .try_reserve_exact(newsize.wrapping_sub(buffer.len()))
                        .is_err()
                    {
                        buffer.clear();
                        return None;
                    }
                    buffer.resize(newsize, 0);
                }
            }
            buffer.get_mut(p.offset..p.offset.wrapping_add(requested))
        }
    }
}
fn compare_double(a: ::core::ffi::c_double, b: ::core::ffi::c_double) -> cJSON_bool {
    let max_val: ::core::ffi::c_double = if a.abs() > b.abs() { a.abs() } else { b.abs() };
    return ((a - b).abs() <= max_val * DBL_EPSILON) as ::core::ffi::c_int;
}
fn copy_with_c_exponent_padding(source: &[u8], destination: &mut [u8]) -> Option<usize> {
    let exponent_index = source.iter().position(|byte| *byte == b'e')?;
    if exponent_index.wrapping_add(2) >= source.len() {
        return None;
    }

    let exponent_digits = source.len().wrapping_sub(exponent_index.wrapping_add(2));
    let needs_padding = exponent_digits == 1;
    let length = source.len().wrapping_add(needs_padding as usize);
    if length > destination.len() {
        return None;
    }

    let exponent_prefix_end = exponent_index.wrapping_add(2);
    destination[..exponent_prefix_end].copy_from_slice(&source[..exponent_prefix_end]);
    let mut output_index = exponent_prefix_end;
    if needs_padding {
        destination[output_index] = b'0';
        output_index = output_index.wrapping_add(1);
    }
    destination[output_index..length].copy_from_slice(&source[exponent_prefix_end..]);
    Some(length)
}
fn copy_float_c_g(
    value: ::core::ffi::c_double,
    precision: usize,
    destination: &mut [u8],
) -> Option<usize> {
    let options = match precision {
        15 => &CJSON_FLOAT_OPTIONS_15,
        17 => &CJSON_FLOAT_OPTIONS_17,
        _ => return None,
    };
    let mut scratch = [0u8; 128];
    let digits = value.to_lexical_with_options::<CJSON_FLOAT_FORMAT>(&mut scratch, options);
    if digits.contains(&b'e') {
        return copy_with_c_exponent_padding(digits, destination);
    }
    if digits.len() > destination.len() {
        return None;
    }
    destination[..digits.len()].copy_from_slice(digits);
    Some(digits.len())
}
fn print_number(item: &cJSON, output_buffer: &mut printbuffer<'_>) -> cJSON_bool {
    let d: ::core::ffi::c_double = item.valuedouble;
    let mut number_buffer = [0u8; 26];
    let length = if !d.is_finite() {
        number_buffer[..4].copy_from_slice(b"null");
        4
    } else if d == item.valueint as ::core::ffi::c_double {
        let integer = item.valueint.to_string();
        let integer = integer.as_bytes();
        if integer.len() > number_buffer.len().wrapping_sub(1) {
            return false_0;
        }
        number_buffer[..integer.len()].copy_from_slice(integer);
        integer.len()
    } else {
        let mut short_number = [0u8; 26];
        let Some(short_length) = copy_float_c_g(d, 15, &mut short_number) else {
            return false_0;
        };
        let test = ::core::str::from_utf8(&short_number[..short_length])
            .ok()
            .and_then(|number| number.parse::<::core::ffi::c_double>().ok());
        if test.is_some_and(|test| compare_double(test, d) != 0) {
            number_buffer[..short_length].copy_from_slice(&short_number[..short_length]);
            short_length
        } else {
            let Some(long_length) = copy_float_c_g(d, 17, &mut number_buffer) else {
                return false_0;
            };
            long_length
        }
    };
    if length > (::core::mem::size_of::<[::core::ffi::c_uchar; 26]>() as usize).wrapping_sub(1) {
        return false_0;
    }
    let output = match ensure(
        output_buffer,
        length.wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 1]>() as size_t),
    ) {
        Some(output) => output,
        None => return false_0,
    };
    output[..length].copy_from_slice(&number_buffer[..length]);
    output[length] = '\0' as i32 as ::core::ffi::c_uchar;
    output_buffer.offset = output_buffer.offset.wrapping_add(length);
    return true_0;
}
fn parse_hex4(input: &[::core::ffi::c_uchar]) -> ::core::ffi::c_uint {
    let Some(input) = input.get(..4) else {
        return 0 as ::core::ffi::c_uint;
    };
    let mut h: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut i: size_t = 0 as size_t;
    i = 0 as size_t;
    while i < 4 as size_t {
        let byte = input[i as usize];
        if byte as ::core::ffi::c_int >= '0' as i32 && byte as ::core::ffi::c_int <= '9' as i32 {
            h = h.wrapping_add(
                (byte as ::core::ffi::c_uint).wrapping_sub('0' as i32 as ::core::ffi::c_uint),
            );
        } else if byte as ::core::ffi::c_int >= 'A' as i32
            && byte as ::core::ffi::c_int <= 'F' as i32
        {
            h = h.wrapping_add(
                (10 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_add(byte as ::core::ffi::c_uint)
                    .wrapping_sub('A' as i32 as ::core::ffi::c_uint),
            );
        } else if byte as ::core::ffi::c_int >= 'a' as i32
            && byte as ::core::ffi::c_int <= 'f' as i32
        {
            h = h.wrapping_add(
                (10 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_add(byte as ::core::ffi::c_uint)
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
    output: &mut [::core::ffi::c_uchar],
) -> Option<(::core::ffi::c_uchar, usize)> {
    let mut codepoint: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    let mut first_code: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut utf8_length: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
    let mut utf8_position: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
    let mut sequence_length: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
    let mut first_byte_mark: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
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
                if input.len() < 12 as usize
                    || input[6 as usize] as ::core::ffi::c_int != '\\' as i32
                    || input[7 as usize] as ::core::ffi::c_int != 'u' as i32
                {
                    return None;
                } else {
                    second_code = parse_hex4(&input[8 as usize..]);
                    if second_code < 0xdc00 as ::core::ffi::c_uint
                        || second_code > 0xdfff as ::core::ffi::c_uint
                    {
                        return None;
                    } else {
                        codepoint = (0x10000 as ::core::ffi::c_int as ::core::ffi::c_uint)
                            .wrapping_add(
                                (first_code & 0x3ff as ::core::ffi::c_uint)
                                    << 10 as ::core::ffi::c_int
                                    | second_code & 0x3ff as ::core::ffi::c_uint,
                            ) as ::core::ffi::c_ulong;
                    }
                }
            } else {
                sequence_length = 6 as ::core::ffi::c_uchar;
                codepoint = first_code as ::core::ffi::c_ulong;
            }
            if codepoint < 0x80 as ::core::ffi::c_ulong {
                utf8_length = 1 as ::core::ffi::c_uchar;
            } else if codepoint < 0x800 as ::core::ffi::c_ulong {
                utf8_length = 2 as ::core::ffi::c_uchar;
                first_byte_mark = 0xc0 as ::core::ffi::c_uchar;
            } else if codepoint < 0x10000 as ::core::ffi::c_ulong {
                utf8_length = 3 as ::core::ffi::c_uchar;
                first_byte_mark = 0xe0 as ::core::ffi::c_uchar;
            } else if codepoint <= 0x10ffff as ::core::ffi::c_ulong {
                utf8_length = 4 as ::core::ffi::c_uchar;
                first_byte_mark = 0xf0 as ::core::ffi::c_uchar;
            } else {
                return None;
            }
            if output.len() < utf8_length as usize {
                return None;
            }
            utf8_position = (utf8_length as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
            while utf8_position as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                output[utf8_position as usize] = ((codepoint | 0x80 as ::core::ffi::c_ulong)
                    & 0xbf as ::core::ffi::c_ulong)
                    as ::core::ffi::c_uchar;
                codepoint >>= 6 as ::core::ffi::c_int;
                utf8_position = utf8_position.wrapping_sub(1);
            }
            if utf8_length as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                output[0 as usize] = ((codepoint | first_byte_mark as ::core::ffi::c_ulong)
                    & 0xff as ::core::ffi::c_ulong)
                    as ::core::ffi::c_uchar;
            } else {
                output[0 as usize] =
                    (codepoint & 0x7f as ::core::ffi::c_ulong) as ::core::ffi::c_uchar;
            }
            return Some((sequence_length, utf8_length as usize));
        }
    }
    return None;
}
fn parse_string(item: &mut cJSON, input_buffer: &mut parse_buffer<'_>) -> cJSON_bool {
    let input = input_buffer.content;
    let start_offset = input_buffer.offset;
    let mut failure_offset = start_offset.wrapping_add(1);

    if input.get(start_offset).copied() != Some(b'"') {
        input_buffer.offset = failure_offset;
        return false_0;
    }

    let mut input_end = start_offset.wrapping_add(1);
    let mut skipped_bytes: size_t = 0 as size_t;
    while input_end < input_buffer.length && input[input_end] as ::core::ffi::c_int != '"' as i32 {
        if input[input_end] as ::core::ffi::c_int == '\\' as i32 {
            if input_end.wrapping_add(1 as usize) >= input_buffer.length {
                input_buffer.offset = failure_offset;
                return false_0;
            }
            skipped_bytes = skipped_bytes.wrapping_add(1);
            input_end = input_end.wrapping_add(1);
        }
        input_end = input_end.wrapping_add(1);
    }

    if input_end >= input_buffer.length || input[input_end] as ::core::ffi::c_int != '"' as i32 {
        input_buffer.offset = failure_offset;
        return false_0;
    }

    let allocation_length = input_end
        .wrapping_sub(start_offset)
        .wrapping_sub(skipped_bytes);
    let output_allocation_length = allocation_length
        .wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 1]>() as size_t);
    let allocation_failure_offset = failure_offset;
    let mut output_buffer = Vec::new();
    if output_buffer
        .try_reserve_exact(output_allocation_length)
        .is_err()
    {
        input_buffer.offset = failure_offset;
        return false_0;
    }
    output_buffer.resize(output_allocation_length, 0);

    let mut input_pointer = start_offset.wrapping_add(1);
    let mut output_offset = 0usize;
    let mut parse_failed = false;

    macro_rules! write_output_byte {
        ($byte:expr) => {{
            if output_offset >= output_buffer.len() {
                parse_failed = true;
                break;
            }
            output_buffer[output_offset] = $byte;
            output_offset = output_offset.wrapping_add(1);
        }};
    }

    while input_pointer < input_end {
        if input[input_pointer] as ::core::ffi::c_int != '\\' as i32 {
            write_output_byte!(input[input_pointer]);
            input_pointer = input_pointer.wrapping_add(1);
            continue;
        }

        failure_offset = input_pointer;
        let mut sequence_length: ::core::ffi::c_uchar = 2 as ::core::ffi::c_uchar;
        if input_end.wrapping_sub(input_pointer) < 1 as size_t {
            parse_failed = true;
            break;
        }

        match input.get(input_pointer.wrapping_add(1)).copied() {
            Some(b'b') => {
                write_output_byte!('\u{8}' as i32 as ::core::ffi::c_uchar);
            }
            Some(b'f') => {
                write_output_byte!('\u{c}' as i32 as ::core::ffi::c_uchar);
            }
            Some(b'n') => {
                write_output_byte!('\n' as i32 as ::core::ffi::c_uchar);
            }
            Some(b'r') => {
                write_output_byte!('\r' as i32 as ::core::ffi::c_uchar);
            }
            Some(b't') => {
                write_output_byte!('\t' as i32 as ::core::ffi::c_uchar);
            }
            Some(escaped @ (b'"' | b'\\' | b'/')) => {
                write_output_byte!(escaped);
            }
            Some(b'u') => {
                if output_buffer.len().saturating_sub(output_offset) < 4 {
                    parse_failed = true;
                    break;
                }
                let unicode_escape = &input[input_pointer..input_end];
                let output_escape = &mut output_buffer[output_offset..output_offset + 4];
                match utf16_literal_to_utf8(unicode_escape, output_escape) {
                    Some((parsed_sequence_length, utf8_length)) => {
                        sequence_length = parsed_sequence_length;
                        output_offset = output_offset.wrapping_add(utf8_length);
                    }
                    None => {
                        parse_failed = true;
                        break;
                    }
                }
            }
            _ => {
                parse_failed = true;
                break;
            }
        }
        input_pointer = input_pointer.wrapping_add(sequence_length as usize);
    }

    if parse_failed || output_offset >= output_buffer.len() {
        input_buffer.offset = failure_offset;
        return false_0;
    }
    output_buffer[output_offset] = '\0' as i32 as ::core::ffi::c_uchar;
    let Some(output) = CjsonString::copied(Some(&output_buffer)) else {
        input_buffer.offset = allocation_failure_offset;
        return false_0;
    };
    item.type_0 = cJSON_String;
    item.valuestring.replace(output);
    input_buffer.offset = input_end.wrapping_add(1);
    return true_0;
}
fn print_string_ptr(
    input: Option<CjsonStringRef>,
    output_buffer: &mut printbuffer<'_>,
) -> cJSON_bool {
    let mut output_length: size_t = 0 as size_t;
    let mut escape_characters: size_t = 0 as size_t;
    let Some(input) = input else {
        let Some(output) = ensure(
            output_buffer,
            ::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t,
        ) else {
            return false_0;
        };
        output.copy_from_slice(b"\"\"\0");
        output_buffer.offset = output_buffer.offset.wrapping_add(2 as size_t);
        return true_0;
    };
    let Some(input) = input.to_bytes() else {
        return false_0;
    };
    for &character in &input {
        match character as ::core::ffi::c_int {
            34 | 92 | 8 | 12 | 10 | 13 | 9 => {
                escape_characters = escape_characters.wrapping_add(1);
            }
            _ => {
                if (character as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
                    escape_characters = escape_characters.wrapping_add(5 as size_t);
                }
            }
        }
    }
    output_length = (input.len() as size_t).wrapping_add(escape_characters);
    let Some(output) = ensure(
        output_buffer,
        output_length.wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t),
    ) else {
        return false_0;
    };
    if escape_characters == 0 as size_t {
        output[0] = '"' as i32 as ::core::ffi::c_uchar;
        output[1..1 + input.len()].copy_from_slice(&input);
        output[output_length.wrapping_add(1 as size_t) as usize] =
            '"' as i32 as ::core::ffi::c_uchar;
        output[output_length.wrapping_add(2 as size_t) as usize] =
            '\0' as i32 as ::core::ffi::c_uchar;
        output_buffer.offset = output_buffer
            .offset
            .wrapping_add(output_length.wrapping_add(2 as size_t));
        return true_0;
    }
    output[0] = '"' as i32 as ::core::ffi::c_uchar;
    let mut output_index = 1usize;
    for &character in &input {
        if character as ::core::ffi::c_int > 31 as ::core::ffi::c_int
            && character as ::core::ffi::c_int != '"' as i32
            && character as ::core::ffi::c_int != '\\' as i32
        {
            output[output_index] = character;
            output_index = output_index.wrapping_add(1);
        } else {
            output[output_index] = '\\' as i32 as ::core::ffi::c_uchar;
            output_index = output_index.wrapping_add(1);
            match character as ::core::ffi::c_int {
                92 => {
                    output[output_index] = '\\' as i32 as ::core::ffi::c_uchar;
                    output_index = output_index.wrapping_add(1);
                }
                34 => {
                    output[output_index] = '"' as i32 as ::core::ffi::c_uchar;
                    output_index = output_index.wrapping_add(1);
                }
                8 => {
                    output[output_index] = 'b' as i32 as ::core::ffi::c_uchar;
                    output_index = output_index.wrapping_add(1);
                }
                12 => {
                    output[output_index] = 'f' as i32 as ::core::ffi::c_uchar;
                    output_index = output_index.wrapping_add(1);
                }
                10 => {
                    output[output_index] = 'n' as i32 as ::core::ffi::c_uchar;
                    output_index = output_index.wrapping_add(1);
                }
                13 => {
                    output[output_index] = 'r' as i32 as ::core::ffi::c_uchar;
                    output_index = output_index.wrapping_add(1);
                }
                9 => {
                    output[output_index] = 't' as i32 as ::core::ffi::c_uchar;
                    output_index = output_index.wrapping_add(1);
                }
                _ => {
                    const HEX: &[u8; 16] = b"0123456789abcdef";
                    output[output_index] = 'u' as i32 as ::core::ffi::c_uchar;
                    output[output_index + 1] = '0' as i32 as ::core::ffi::c_uchar;
                    output[output_index + 2] = '0' as i32 as ::core::ffi::c_uchar;
                    output[output_index + 3] = HEX[(character >> 4) as usize];
                    output[output_index + 4] = HEX[(character & 0xf) as usize];
                    output_index = output_index.wrapping_add(5);
                }
            }
        }
    }
    output[output_length.wrapping_add(1 as size_t) as usize] = '"' as i32 as ::core::ffi::c_uchar;
    output[output_length.wrapping_add(2 as size_t) as usize] = '\0' as i32 as ::core::ffi::c_uchar;
    output_buffer.offset = output_buffer
        .offset
        .wrapping_add(output_length.wrapping_add(2 as size_t));
    return true_0;
}
enum CjsonStringSlot {
    Value,
    ObjectKey,
}

fn print_string(item: &cJSON, slot: CjsonStringSlot) -> Option<CjsonStringRef> {
    match slot {
        CjsonStringSlot::Value => item.valuestring.borrow().as_cstr(),
        CjsonStringSlot::ObjectKey => item.string.borrow().as_cstr(),
    }
}
fn print_item_string(item: &cJSON, p: &mut printbuffer<'_>) -> cJSON_bool {
    return print_string_ptr(print_string(item, CjsonStringSlot::Value), p);
}
fn buffer_skip_whitespace(buffer: &mut parse_buffer<'_>) {
    if buffer.offset >= buffer.length {
        return;
    }
    while buffer.offset < buffer.length
        && buffer.content[buffer.offset] as ::core::ffi::c_int <= 32 as ::core::ffi::c_int
    {
        buffer.offset = buffer.offset.wrapping_add(1);
    }
    if buffer.offset == buffer.length {
        buffer.offset = buffer.offset.wrapping_sub(1);
    }
}
fn skip_utf8_bom(buffer: &mut parse_buffer<'_>) {
    if buffer.offset != 0 as size_t {
        return;
    }
    if buffer.offset.wrapping_add(4 as size_t) < buffer.length
        && buffer.starts_with_at_offset(b"\xEF\xBB\xBF")
    {
        buffer.offset = buffer.offset.wrapping_add(3 as size_t);
    }
}
pub fn cJSON_ParseWithOpts(
    value: Option<&::std::ffi::CStr>,
    return_parse_end: Option<&mut *const ::core::ffi::c_char>,
    require_null_terminated: cJSON_bool,
) -> *mut cJSON {
    let Some(value) = value else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    cJSON_ParseWithLengthOpts(
        Some(value.to_bytes_with_nul()),
        return_parse_end,
        require_null_terminated,
    )
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
        Some(unsafe { ::std::ffi::CStr::from_ptr(value) })
    };
    let return_parse_end = unsafe { return_parse_end.as_mut() };
    cJSON_ParseWithOpts(value, return_parse_end, require_null_terminated)
}
pub fn cJSON_ParseWithLengthOpts(
    value: Option<&[::core::ffi::c_uchar]>,
    mut return_parse_end: Option<&mut *const ::core::ffi::c_char>,
    require_null_terminated: cJSON_bool,
) -> *mut cJSON {
    let mut buffer: parse_buffer<'_> = parse_buffer {
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
    global_error_json.store(0, ::std::sync::atomic::Ordering::Relaxed);
    global_error_position.store(0 as size_t, ::std::sync::atomic::Ordering::Relaxed);

    let Some(value) = value else {
        return ::core::ptr::null_mut::<cJSON>();
    };

    if !value.is_empty() {
        buffer.content = value;
        buffer.length = value.len() as size_t;
        buffer.offset = 0 as size_t;
        let hooks = global_hooks_snapshot();
        buffer.hooks = hooks;
        let mut item = cJSON_New_Item(&hooks, cJSON_Invalid);
        if let Some(item_ref) = item.as_deref_mut() {
            skip_utf8_bom(&mut buffer);
            buffer_skip_whitespace(&mut buffer);
            if parse_value(item_ref, &mut buffer) != 0 {
                let parse_success = if require_null_terminated != 0 {
                    buffer_skip_whitespace(&mut buffer);
                    buffer.current_byte() == Some('\0' as i32 as ::core::ffi::c_uchar)
                } else {
                    true
                };
                if parse_success {
                    if let Some(return_parse_end) = &mut return_parse_end {
                        **return_parse_end = value.as_ptr().wrapping_add(buffer.offset)
                            as *const ::core::ffi::c_char;
                    }
                    return ::core::ptr::from_mut(item_ref);
                }
            }
        }

        if let Some(item_ref) = item {
            cJSON_Delete(Some(item_ref));
        }
    }

    let local_error = error {
        json: value.as_ptr().expose_provenance(),
        position: if buffer.offset < buffer.length {
            buffer.offset
        } else if buffer.length > 0 as size_t {
            buffer.length.wrapping_sub(1 as size_t)
        } else {
            0 as size_t
        },
    };
    if let Some(return_parse_end) = &mut return_parse_end {
        **return_parse_end =
            ::core::ptr::with_exposed_provenance::<::core::ffi::c_uchar>(local_error.json)
                .wrapping_add(local_error.position) as *const ::core::ffi::c_char;
    }
    global_error_json.store(local_error.json, ::std::sync::atomic::Ordering::Relaxed);
    global_error_position.store(local_error.position, ::std::sync::atomic::Ordering::Relaxed);

    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_ParseWithLengthOpts"]
pub unsafe extern "C" fn cJSON_ParseWithLengthOpts_ffi(
    mut value: *const ::core::ffi::c_char,
    mut buffer_length: size_t,
    mut return_parse_end: *mut *const ::core::ffi::c_char,
    mut require_null_terminated: cJSON_bool,
) -> *mut cJSON {
    let value = if value.is_null() {
        None
    } else {
        Some(unsafe {
            ::core::slice::from_raw_parts(value as *const ::core::ffi::c_uchar, buffer_length)
        })
    };
    let return_parse_end = unsafe { return_parse_end.as_mut() };
    cJSON_ParseWithLengthOpts(value, return_parse_end, require_null_terminated)
}
pub fn cJSON_Parse(value: Option<&::std::ffi::CStr>) -> *mut cJSON {
    return cJSON_ParseWithOpts(value, None, 0 as cJSON_bool);
}
#[export_name = "cJSON_Parse"]
pub unsafe extern "C" fn cJSON_Parse_ffi(mut value: *const ::core::ffi::c_char) -> *mut cJSON {
    let value = if value.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(value) })
    };
    cJSON_Parse(value)
}
pub fn cJSON_ParseWithLength(value: Option<&[::core::ffi::c_uchar]>) -> *mut cJSON {
    cJSON_ParseWithLengthOpts(value, None, 0 as cJSON_bool)
}
#[export_name = "cJSON_ParseWithLength"]
pub unsafe extern "C" fn cJSON_ParseWithLength_ffi(
    mut value: *const ::core::ffi::c_char,
    mut buffer_length: size_t,
) -> *mut cJSON {
    let value = if value.is_null() {
        None
    } else {
        Some(unsafe {
            ::core::slice::from_raw_parts(value as *const ::core::ffi::c_uchar, buffer_length)
        })
    };
    cJSON_ParseWithLength(value)
}
fn dynamic_printbuffer(initial_size: size_t, format: cJSON_bool) -> Option<printbuffer<'static>> {
    let mut storage = Vec::new();
    if storage.try_reserve_exact(initial_size).is_err() {
        return None;
    }
    storage.resize(initial_size, 0);
    Some(printbuffer {
        storage: printbuffer_storage::Dynamic(storage),
        offset: 0 as size_t,
        depth: 0 as size_t,
        format,
    })
}
fn printbuffer_cstr<'a>(buffer: &'a printbuffer<'_>) -> Option<&'a ::std::ffi::CStr> {
    let bytes = match &buffer.storage {
        printbuffer_storage::Dynamic(storage) => storage.get(..buffer.offset.wrapping_add(1)),
        printbuffer_storage::Fixed(storage) => storage.get(..buffer.offset.wrapping_add(1)),
    };
    let Some(bytes) = bytes else {
        return None;
    };
    let Ok(output) = ::std::ffi::CStr::from_bytes_with_nul(bytes) else {
        return None;
    };
    Some(output)
}
macro_rules! return_print_result_to_c {
    ($result:expr) => {{
        match $result {
            Some(output) => {
                let hooks = global_hooks_snapshot();
                let copy = hooks.allocate.expect("non-null function pointer")(
                    output.len() as size_t
                ) as *mut ::core::ffi::c_uchar;
                if copy.is_null() {
                    ::core::ptr::null_mut::<::core::ffi::c_char>()
                } else {
                    unsafe {
                        ::core::ptr::copy_nonoverlapping(output.as_ptr(), copy, output.len());
                    }
                    copy.cast::<::core::ffi::c_char>()
                }
            }
            None => ::core::ptr::null_mut::<::core::ffi::c_char>(),
        }
    }};
}
fn print(item: Option<&cJSON>, format: cJSON_bool) -> Option<Vec<::core::ffi::c_uchar>> {
    const DEFAULT_BUFFER_SIZE: size_t = 256 as size_t;
    let Some(mut buffer) = dynamic_printbuffer(DEFAULT_BUFFER_SIZE, format) else {
        return None;
    };
    if print_value(item, &mut buffer) != 0 {
        if let Some(output) = printbuffer_cstr(&buffer) {
            return Some(output.to_bytes_with_nul().to_vec());
        }
    }
    return None;
}
pub fn cJSON_Print(item: Option<&cJSON>) -> Option<Vec<::core::ffi::c_uchar>> {
    return print(item, true_0);
}
#[export_name = "cJSON_Print"]
pub unsafe extern "C" fn cJSON_Print_ffi(mut item: *const cJSON) -> *mut ::core::ffi::c_char {
    return_print_result_to_c!(cJSON_Print(unsafe { item.as_ref() }))
}
pub fn cJSON_PrintUnformatted(item: Option<&cJSON>) -> Option<Vec<::core::ffi::c_uchar>> {
    return print(item, false_0);
}
#[export_name = "cJSON_PrintUnformatted"]
pub unsafe extern "C" fn cJSON_PrintUnformatted_ffi(
    mut item: *const cJSON,
) -> *mut ::core::ffi::c_char {
    return_print_result_to_c!(cJSON_PrintUnformatted(unsafe { item.as_ref() }))
}
pub fn cJSON_PrintBuffered(
    item: Option<&cJSON>,
    mut prebuffer: ::core::ffi::c_int,
    mut fmt: cJSON_bool,
) -> Option<Vec<::core::ffi::c_uchar>> {
    if prebuffer < 0 as ::core::ffi::c_int {
        return None;
    }
    let Some(mut p) = dynamic_printbuffer(prebuffer as size_t, fmt) else {
        return None;
    };
    if print_value(item, &mut p) == 0 {
        return None;
    }
    match printbuffer_cstr(&p) {
        Some(output) => Some(output.to_bytes_with_nul().to_vec()),
        None => None,
    }
}
#[export_name = "cJSON_PrintBuffered"]
pub unsafe extern "C" fn cJSON_PrintBuffered_ffi(
    mut item: *const cJSON,
    mut prebuffer: ::core::ffi::c_int,
    mut fmt: cJSON_bool,
) -> *mut ::core::ffi::c_char {
    return_print_result_to_c!(cJSON_PrintBuffered(
        unsafe { item.as_ref() },
        prebuffer,
        fmt
    ))
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
        storage: printbuffer_storage::Fixed(buffer),
        offset: 0 as size_t,
        depth: 0 as size_t,
        format,
    };
    return print_value(item, &mut p);
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
    let buffer = unsafe {
        ::core::slice::from_raw_parts_mut(buffer as *mut ::core::ffi::c_uchar, length as usize)
    };
    cJSON_PrintPreallocated(unsafe { item.as_ref() }, Some(buffer), format)
}
fn parse_value(item: &mut cJSON, input_buffer: &mut parse_buffer<'_>) -> cJSON_bool {
    if input_buffer.starts_with_at_offset(b"null") {
        item.type_0 = cJSON_NULL;
        input_buffer.offset = input_buffer.offset.wrapping_add(4 as size_t);
        return true_0;
    }
    if input_buffer.starts_with_at_offset(b"false") {
        item.type_0 = cJSON_False;
        input_buffer.offset = input_buffer.offset.wrapping_add(5 as size_t);
        return true_0;
    }
    if input_buffer.starts_with_at_offset(b"true") {
        item.type_0 = cJSON_True;
        item.valueint = 1 as ::core::ffi::c_int;
        input_buffer.offset = input_buffer.offset.wrapping_add(4 as size_t);
        return true_0;
    }
    if input_buffer.current_byte() == Some('"' as i32 as ::core::ffi::c_uchar) {
        return parse_string(item, input_buffer);
    }
    if let Some(current) = input_buffer.current_byte() {
        if current as ::core::ffi::c_int == '-' as i32
            || current as ::core::ffi::c_int >= '0' as i32
                && current as ::core::ffi::c_int <= '9' as i32
        {
            return parse_number(item, input_buffer);
        }
    }
    if input_buffer.current_byte() == Some('[' as i32 as ::core::ffi::c_uchar) {
        return parse_array(item, input_buffer);
    }
    if input_buffer.current_byte() == Some('{' as i32 as ::core::ffi::c_uchar) {
        return parse_object(item, input_buffer);
    }
    return false_0;
}
fn print_value(item: Option<&cJSON>, output_buffer: &mut printbuffer<'_>) -> cJSON_bool {
    let Some(item) = item else {
        return false_0;
    };
    match item.type_0 & 0xff as ::core::ffi::c_int {
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
        cJSON_Number => return print_number(item, output_buffer),
        cJSON_Raw => {
            if item.valuestring.borrow().is_null() {
                return false_0;
            }
            let Some(raw_value) = print_string(item, CjsonStringSlot::Value) else {
                return false_0;
            };
            let Some(raw_value) = raw_value.to_bytes_with_nul() else {
                return false_0;
            };
            let raw_length = raw_value.len();
            let Some(output) = ensure(output_buffer, raw_length) else {
                return false_0;
            };
            output.copy_from_slice(&raw_value);
            output_buffer.offset = output_buffer
                .offset
                .wrapping_add(raw_length.wrapping_sub(1 as size_t));
            return true_0;
        }
        cJSON_String => return print_item_string(item, output_buffer),
        cJSON_Array => return print_array(item, output_buffer),
        cJSON_Object => return print_object(item, output_buffer),
        _ => return false_0,
    };
}
fn parse_array(item: &mut cJSON, input_buffer: &mut parse_buffer<'_>) -> cJSON_bool {
    let hooks = input_buffer.hooks;
    let mut parsed_items = cJSON {
        next: Cell::new(None),
        prev: Cell::new(None),
        child: Cell::new(None),
        type_0: cJSON_Invalid,
        valuestring: RefCell::new(CjsonString::Null),
        valueint: 0 as ::core::ffi::c_int,
        valuedouble: 0 as ::core::ffi::c_double,
        string: RefCell::new(CjsonString::Null),
    };
    if input_buffer.depth >= CJSON_NESTING_LIMIT as size_t {
        return false_0;
    }
    input_buffer.depth = input_buffer.depth.wrapping_add(1);
    if input_buffer.current_byte() == Some('[' as i32 as ::core::ffi::c_uchar) {
        let mut success = false;
        input_buffer.offset = input_buffer.offset.wrapping_add(1);
        buffer_skip_whitespace(input_buffer);
        if input_buffer.current_byte() == Some(']' as i32 as ::core::ffi::c_uchar) {
            success = true;
        } else if input_buffer.current_byte().is_none() {
            input_buffer.offset = input_buffer.offset.wrapping_sub(1);
        } else {
            input_buffer.offset = input_buffer.offset.wrapping_sub(1);
            let mut failed = false;
            loop {
                let Some(new_item) = cJSON_New_Item(&hooks, cJSON_Invalid) else {
                    failed = true;
                    break;
                };
                input_buffer.offset = input_buffer.offset.wrapping_add(1);
                buffer_skip_whitespace(input_buffer);
                if parse_value(new_item, input_buffer) == 0 {
                    cJSON_Delete(Some(new_item));
                    failed = true;
                    break;
                }
                add_item_to_array(&mut parsed_items, new_item);
                buffer_skip_whitespace(input_buffer);
                if input_buffer.current_byte() != Some(',' as i32 as ::core::ffi::c_uchar) {
                    break;
                }
            }
            if !failed && input_buffer.current_byte() == Some(']' as i32 as ::core::ffi::c_uchar) {
                success = true;
            }
        }
        if success {
            input_buffer.depth = input_buffer.depth.wrapping_sub(1);
            item.type_0 = cJSON_Array;
            item.child.set(parsed_items.child.get());
            input_buffer.offset = input_buffer.offset.wrapping_add(1);
            return true_0;
        }
    }
    while parsed_items.child.get().is_some() {
        cJSON_DeleteItemFromArray(Some(&mut parsed_items), 0);
    }
    return false_0;
}
fn print_array(item: &cJSON, output_buffer: &mut printbuffer<'_>) -> cJSON_bool {
    let mut length: size_t = 0 as size_t;
    let mut current_index: size_t = 0 as size_t;
    let Some(output) = ensure(output_buffer, 1 as size_t) else {
        return false_0;
    };
    output[0] = '[' as i32 as ::core::ffi::c_uchar;
    output_buffer.offset = output_buffer.offset.wrapping_add(1);
    output_buffer.depth = output_buffer.depth.wrapping_add(1);
    while let Some(element) = get_array_item(Some(item), current_index) {
        if print_value(Some(element), output_buffer) == 0 {
            return false_0;
        }
        if get_array_item(Some(item), current_index.wrapping_add(1 as size_t)).is_some() {
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
            let mut output_index = 1usize;
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
fn parse_object(item: &mut cJSON, input_buffer: &mut parse_buffer<'_>) -> cJSON_bool {
    let hooks = input_buffer.hooks;
    let mut parsed_items = cJSON {
        next: Cell::new(None),
        prev: Cell::new(None),
        child: Cell::new(None),
        type_0: cJSON_Invalid,
        valuestring: RefCell::new(CjsonString::Null),
        valueint: 0 as ::core::ffi::c_int,
        valuedouble: 0 as ::core::ffi::c_double,
        string: RefCell::new(CjsonString::Null),
    };
    if input_buffer.depth >= CJSON_NESTING_LIMIT as size_t {
        return false_0;
    }
    input_buffer.depth = input_buffer.depth.wrapping_add(1);
    if input_buffer.current_byte() == Some('{' as i32 as ::core::ffi::c_uchar) {
        let mut success = false;
        input_buffer.offset = input_buffer.offset.wrapping_add(1);
        buffer_skip_whitespace(input_buffer);
        if input_buffer.current_byte() == Some('}' as i32 as ::core::ffi::c_uchar) {
            success = true;
        } else if input_buffer.current_byte().is_none() {
            input_buffer.offset = input_buffer.offset.wrapping_sub(1);
        } else {
            input_buffer.offset = input_buffer.offset.wrapping_sub(1);
            let mut failed = false;
            loop {
                let Some(new_item) = cJSON_New_Item(&hooks, cJSON_Invalid) else {
                    failed = true;
                    break;
                };
                if input_buffer.offset.wrapping_add(1 as size_t) >= input_buffer.length {
                    cJSON_Delete(Some(new_item));
                    failed = true;
                    break;
                }
                input_buffer.offset = input_buffer.offset.wrapping_add(1);
                buffer_skip_whitespace(input_buffer);
                if parse_string(new_item, input_buffer) == 0 {
                    cJSON_Delete(Some(new_item));
                    failed = true;
                    break;
                }
                buffer_skip_whitespace(input_buffer);
                let key = new_item.valuestring.replace(CjsonString::Null);
                new_item.string.replace(key);
                if input_buffer.current_byte() != Some(':' as i32 as ::core::ffi::c_uchar) {
                    cJSON_Delete(Some(new_item));
                    failed = true;
                    break;
                }
                input_buffer.offset = input_buffer.offset.wrapping_add(1);
                buffer_skip_whitespace(input_buffer);
                if parse_value(new_item, input_buffer) == 0 {
                    cJSON_Delete(Some(new_item));
                    failed = true;
                    break;
                }
                add_item_to_array(&mut parsed_items, new_item);
                buffer_skip_whitespace(input_buffer);
                if input_buffer.current_byte() != Some(',' as i32 as ::core::ffi::c_uchar) {
                    break;
                }
            }
            if !failed && input_buffer.current_byte() == Some('}' as i32 as ::core::ffi::c_uchar) {
                success = true;
            }
        }
        if success {
            input_buffer.depth = input_buffer.depth.wrapping_sub(1);
            item.type_0 = cJSON_Object;
            item.child.set(parsed_items.child.get());
            input_buffer.offset = input_buffer.offset.wrapping_add(1);
            return true_0;
        }
    }
    while parsed_items.child.get().is_some() {
        cJSON_DeleteItemFromArray(Some(&mut parsed_items), 0);
    }
    return false_0;
}
fn print_object(item: &cJSON, output_buffer: &mut printbuffer<'_>) -> cJSON_bool {
    let mut length: size_t = 0 as size_t;
    let mut current_index: size_t = 0 as size_t;
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
    while let Some(current) = get_array_item(Some(item), current_index) {
        if output_buffer.format != 0 {
            let depth = output_buffer.depth;
            let Some(output) = ensure(output_buffer, depth) else {
                return false_0;
            };
            output.fill('\t' as i32 as ::core::ffi::c_uchar);
            output_buffer.offset = output_buffer.offset.wrapping_add(depth);
        }
        if print_string_ptr(
            print_string(current, CjsonStringSlot::ObjectKey),
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
                (if get_array_item(Some(item), current_index.wrapping_add(1 as size_t)).is_some() {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as size_t,
            );
        let Some(output) = ensure(output_buffer, length.wrapping_add(1 as size_t)) else {
            return false_0;
        };
        let mut output_index = 0usize;
        if current.next.get().is_some() {
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
    let mut output_index = 0usize;
    if formatted {
        while output_index < depth.wrapping_sub(1 as size_t) {
            output[output_index] = '\t' as i32 as ::core::ffi::c_uchar;
            output_index = output_index.wrapping_add(1);
        }
    }
    output[output_index] = '}' as i32 as ::core::ffi::c_uchar;
    output[output_index + 1] = '\0' as i32 as ::core::ffi::c_uchar;
    output_buffer.offset = output_buffer
        .offset
        .wrapping_add(final_length.wrapping_sub(1 as size_t));
    output_buffer.depth = output_buffer.depth.wrapping_sub(1);
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
    cJSON_GetArraySize(unsafe { array.as_ref() })
}
fn get_array_item(mut array: Option<&cJSON>, mut index: size_t) -> Option<&'static cJSON> {
    let mut current_child = match array.take() {
        Some(array) => array.child.get(),
        None => return None,
    };
    while let Some(current) = current_child.filter(|_| index > 0 as size_t) {
        index = index.wrapping_sub(1);
        current_child = current.next.get();
    }
    current_child
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
    match cJSON_GetArrayItem(unsafe { array.as_ref() }, index) {
        Some(item) => item as *const cJSON as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
fn get_object_item<'a>(
    object: Option<&'a cJSON>,
    name: Option<&::std::ffi::CStr>,
    case_sensitive: cJSON_bool,
) -> Option<&'a cJSON> {
    let Some(name) = name else {
        return None;
    };
    let Some(object) = object else {
        return None;
    };
    let mut index = 0 as size_t;
    while let Some(current) = get_array_item(Some(object), index) {
        if current.string.borrow().is_null() {
            if case_sensitive != 0 {
                return None;
            }
        } else {
            let Some(current_name) = print_string(current, CjsonStringSlot::ObjectKey) else {
                return None;
            };
            let matches = current_name
                .with_cstr(|current_name| {
                    if case_sensitive != 0 {
                        name == current_name
                    } else {
                        case_insensitive_cstr_cmp(name, current_name) == 0 as ::core::ffi::c_int
                    }
                })
                .unwrap_or(false);
            if matches {
                return Some(current);
            }
        }
        index = index.wrapping_add(1);
    }
    None
}
pub fn cJSON_GetObjectItem<'a>(
    object: Option<&'a cJSON>,
    string: Option<&::std::ffi::CStr>,
) -> Option<&'a cJSON> {
    return get_object_item(object, string, false_0);
}
#[export_name = "cJSON_GetObjectItem"]
pub unsafe extern "C" fn cJSON_GetObjectItem_ffi(
    object: *const cJSON,
    string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let string = if string.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(string) })
    };
    match cJSON_GetObjectItem(unsafe { object.as_ref() }, string) {
        Some(item) => item as *const cJSON as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_GetObjectItemCaseSensitive<'a>(
    object: Option<&'a cJSON>,
    string: Option<&::std::ffi::CStr>,
) -> Option<&'a cJSON> {
    return get_object_item(object, string, true_0);
}
#[export_name = "cJSON_GetObjectItemCaseSensitive"]
pub unsafe extern "C" fn cJSON_GetObjectItemCaseSensitive_ffi(
    object: *const cJSON,
    string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let string = if string.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(string) })
    };
    match cJSON_GetObjectItemCaseSensitive(unsafe { object.as_ref() }, string) {
        Some(item) => item as *const cJSON as *mut cJSON,
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
        Some(unsafe { ::std::ffi::CStr::from_ptr(string) })
    };
    cJSON_HasObjectItem(unsafe { object.as_ref() }, string)
}
fn create_reference(item: Option<&cJSON>, hooks: &internal_hooks) -> Option<&'static mut cJSON> {
    let Some(item) = item else {
        return None;
    };
    let Some(reference_ref) = cJSON_New_Item(hooks, cJSON_Invalid) else {
        return None;
    };
    *reference_ref = item.clone();
    reference_ref.string.replace(CjsonString::Null);
    reference_ref.type_0 |= cJSON_IsReference;
    reference_ref.prev.set(None);
    reference_ref.next.set(None);
    return Some(reference_ref);
}
fn add_item_to_array(array: &cJSON, item: &'static cJSON) -> cJSON_bool {
    if array.child.get().is_none() {
        array.child.set(Some(item));
        item.prev.set(Some(item));
        item.next.set(None);
    } else {
        let Some(head) = get_array_item(Some(array), 0 as size_t) else {
            return true_0;
        };
        if head.prev.get().is_some() {
            let mut tail_index = 0 as size_t;
            let mut tail = head;
            while let Some(next) = get_array_item(Some(array), tail_index.wrapping_add(1)) {
                tail_index = tail_index.wrapping_add(1);
                tail = next;
            }
            tail.next.set(Some(item));
            item.prev.set(Some(tail));
            item.next.set(None);
            head.prev.set(Some(item));
        }
    }
    return true_0;
}
enum ObjectKey<'a> {
    Copy(&'a ::std::ffi::CStr),
    Constant(&'static ::std::ffi::CStr),
}
pub fn cJSON_AddItemToArray(
    array: Option<&mut cJSON>,
    item: Option<&'static mut cJSON>,
) -> cJSON_bool {
    match (array, item) {
        (Some(array), Some(item)) => add_item_to_array(array, item),
        _ => false_0,
    }
}
#[export_name = "cJSON_AddItemToArray"]
pub unsafe extern "C" fn cJSON_AddItemToArray_ffi(
    mut array: *mut cJSON,
    mut item: *mut cJSON,
) -> cJSON_bool {
    if array == item {
        return false_0;
    }
    cJSON_AddItemToArray(unsafe { array.as_mut() }, unsafe { item.as_mut() })
}
fn add_item_to_object(
    object: &mut cJSON,
    string: Option<ObjectKey<'_>>,
    item: &'static mut cJSON,
    _hooks: &internal_hooks,
) -> cJSON_bool {
    if ::core::ptr::eq::<cJSON>(object, item) {
        return false_0;
    }
    let Some(string) = string else {
        return false_0;
    };

    let (new_key, new_type) = match string {
        ObjectKey::Constant(string) => (
            CjsonString::Borrowed(string),
            item.type_0 | cJSON_StringIsConst,
        ),
        ObjectKey::Copy(string) => {
            let Some(new_key) = CjsonString::copied(Some(string.to_bytes_with_nul())) else {
                return false_0;
            };
            (new_key, item.type_0 & !cJSON_StringIsConst)
        }
    };
    item.string.replace(new_key);
    item.type_0 = new_type;
    return add_item_to_array(object, item);
}
pub fn cJSON_AddItemToObject(
    object: Option<&mut cJSON>,
    string: Option<&::std::ffi::CStr>,
    item: Option<&'static mut cJSON>,
) -> cJSON_bool {
    let (Some(object), Some(item)) = (object, item) else {
        return false_0;
    };
    let hooks = global_hooks_snapshot();
    return add_item_to_object(object, string.map(ObjectKey::Copy), item, &hooks);
}
#[export_name = "cJSON_AddItemToObject"]
pub unsafe extern "C" fn cJSON_AddItemToObject_ffi(
    object: *mut cJSON,
    string: *const ::core::ffi::c_char,
    item: *mut cJSON,
) -> cJSON_bool {
    if object == item {
        return false_0;
    }
    let string = if string.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(string) })
    };
    cJSON_AddItemToObject(unsafe { object.as_mut() }, string, unsafe { item.as_mut() })
}
pub fn cJSON_AddItemToObjectCS(
    object: Option<&mut cJSON>,
    string: Option<&'static ::std::ffi::CStr>,
    item: Option<&'static mut cJSON>,
) -> cJSON_bool {
    let (Some(object), Some(item)) = (object, item) else {
        return false_0;
    };
    let hooks = global_hooks_snapshot();
    return add_item_to_object(object, string.map(ObjectKey::Constant), item, &hooks);
}
#[export_name = "cJSON_AddItemToObjectCS"]
pub unsafe extern "C" fn cJSON_AddItemToObjectCS_ffi(
    object: *mut cJSON,
    string: *const ::core::ffi::c_char,
    item: *mut cJSON,
) -> cJSON_bool {
    if object == item {
        return false_0;
    }
    let string: Option<&'static ::std::ffi::CStr> = if string.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(string) })
    };
    cJSON_AddItemToObjectCS(unsafe { object.as_mut() }, string, unsafe { item.as_mut() })
}
pub fn cJSON_AddItemReferenceToArray(
    array: Option<&mut cJSON>,
    item: Option<&cJSON>,
) -> cJSON_bool {
    let Some(array) = array else {
        return false_0;
    };
    let Some(item) = item else {
        return false_0;
    };
    let hooks = global_hooks_snapshot();
    let Some(reference) = create_reference(Some(item), &hooks) else {
        return false_0;
    };
    return add_item_to_array(array, reference);
}
#[export_name = "cJSON_AddItemReferenceToArray"]
pub unsafe extern "C" fn cJSON_AddItemReferenceToArray_ffi(
    mut array: *mut cJSON,
    mut item: *mut cJSON,
) -> cJSON_bool {
    cJSON_AddItemReferenceToArray(unsafe { array.as_mut() }, unsafe { item.as_ref() })
}
pub fn cJSON_AddItemReferenceToObject(
    object: Option<&mut cJSON>,
    string: Option<&::std::ffi::CStr>,
    item: Option<&cJSON>,
) -> cJSON_bool {
    let Some(object) = object else {
        return false_0;
    };
    if string.is_none() {
        return false_0;
    }
    let hooks = global_hooks_snapshot();
    let Some(reference) = create_reference(item, &hooks) else {
        return false_0;
    };
    return add_item_to_object(object, string.map(ObjectKey::Copy), reference, &hooks);
}
#[export_name = "cJSON_AddItemReferenceToObject"]
pub unsafe extern "C" fn cJSON_AddItemReferenceToObject_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
    mut item: *mut cJSON,
) -> cJSON_bool {
    let string = if string.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(string) })
    };
    if object == item && !object.is_null() {
        if string.is_none() {
            return false_0;
        }
        let hooks = global_hooks_snapshot();
        let reference = create_reference(unsafe { item.as_ref() }, &hooks);
        let Some(object) = (unsafe { object.as_mut() }) else {
            return false_0;
        };
        let Some(reference) = reference else {
            return false_0;
        };
        return add_item_to_object(object, string.map(ObjectKey::Copy), reference, &hooks);
    }
    cJSON_AddItemReferenceToObject(unsafe { object.as_mut() }, string, unsafe { item.as_ref() })
}
fn add_created_item_to_object(
    object: Option<&mut cJSON>,
    name: Option<&::std::ffi::CStr>,
    item: &'static mut cJSON,
    _hooks: &internal_hooks,
) -> Option<&'static cJSON> {
    let Some(object) = object else {
        cJSON_Delete(Some(item));
        return None;
    };
    let Some(name) = name else {
        cJSON_Delete(Some(item));
        return None;
    };

    let Some(new_key) = CjsonString::copied(Some(name.to_bytes_with_nul())) else {
        cJSON_Delete(Some(item));
        return None;
    };
    item.string.replace(new_key);
    item.type_0 &= !cJSON_StringIsConst;

    let item_link: &'static cJSON = item;
    add_item_to_array(object, item_link);
    Some(item_link)
}
pub fn cJSON_AddNullToObject(
    object: Option<&mut cJSON>,
    name: Option<&::std::ffi::CStr>,
) -> *mut cJSON {
    let item_hooks = global_hooks_snapshot();
    let Some(null) = cJSON_New_Item(&item_hooks, cJSON_NULL) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let hooks = global_hooks_snapshot();
    match add_created_item_to_object(object, name, null, &hooks) {
        Some(item) => ::core::ptr::from_ref(item) as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
#[export_name = "cJSON_AddNullToObject"]
pub unsafe extern "C" fn cJSON_AddNullToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let name = if name.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(name) })
    };
    cJSON_AddNullToObject(unsafe { object.as_mut() }, name)
}
pub fn cJSON_AddTrueToObject(
    object: Option<&mut cJSON>,
    name: Option<&::std::ffi::CStr>,
) -> *mut cJSON {
    let item_hooks = global_hooks_snapshot();
    let Some(true_item) = cJSON_New_Item(&item_hooks, cJSON_True) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let hooks = global_hooks_snapshot();
    match add_created_item_to_object(object, name, true_item, &hooks) {
        Some(item) => ::core::ptr::from_ref(item) as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
#[export_name = "cJSON_AddTrueToObject"]
pub unsafe extern "C" fn cJSON_AddTrueToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let name = if name.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(name) })
    };
    cJSON_AddTrueToObject(unsafe { object.as_mut() }, name)
}
pub fn cJSON_AddFalseToObject(
    object: Option<&mut cJSON>,
    name: Option<&::std::ffi::CStr>,
) -> *mut cJSON {
    let item_hooks = global_hooks_snapshot();
    let Some(false_item) = cJSON_New_Item(&item_hooks, cJSON_False) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let hooks = global_hooks_snapshot();
    match add_created_item_to_object(object, name, false_item, &hooks) {
        Some(item) => ::core::ptr::from_ref(item) as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
#[export_name = "cJSON_AddFalseToObject"]
pub unsafe extern "C" fn cJSON_AddFalseToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let name = if name.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(name) })
    };
    cJSON_AddFalseToObject(unsafe { object.as_mut() }, name)
}
pub fn cJSON_AddBoolToObject(
    object: Option<&mut cJSON>,
    name: Option<&::std::ffi::CStr>,
    boolean: cJSON_bool,
) -> *mut cJSON {
    let item_hooks = global_hooks_snapshot();
    let Some(bool_item) = cJSON_New_Item(
        &item_hooks,
        if boolean != 0 {
            cJSON_True
        } else {
            cJSON_False
        },
    ) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let hooks = global_hooks_snapshot();
    match add_created_item_to_object(object, name, bool_item, &hooks) {
        Some(item) => ::core::ptr::from_ref(item) as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
#[export_name = "cJSON_AddBoolToObject"]
pub unsafe extern "C" fn cJSON_AddBoolToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
    boolean: cJSON_bool,
) -> *mut cJSON {
    let name = if name.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(name) })
    };
    cJSON_AddBoolToObject(unsafe { object.as_mut() }, name, boolean)
}
pub fn cJSON_AddNumberToObject(
    object: Option<&mut cJSON>,
    name: Option<&::std::ffi::CStr>,
    number: ::core::ffi::c_double,
) -> *mut cJSON {
    let item_hooks = global_hooks_snapshot();
    let Some(number_item) = cJSON_New_Item(&item_hooks, cJSON_Number) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    cJSON_SetNumberHelper(number_item, number);
    let hooks = global_hooks_snapshot();
    match add_created_item_to_object(object, name, number_item, &hooks) {
        Some(item) => ::core::ptr::from_ref(item) as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
#[export_name = "cJSON_AddNumberToObject"]
pub unsafe extern "C" fn cJSON_AddNumberToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
    number: ::core::ffi::c_double,
) -> *mut cJSON {
    let name = if name.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(name) })
    };
    cJSON_AddNumberToObject(unsafe { object.as_mut() }, name, number)
}
pub fn cJSON_AddStringToObject(
    object: Option<&mut cJSON>,
    name: Option<&::std::ffi::CStr>,
    string: Option<&::std::ffi::CStr>,
) -> *mut cJSON {
    let item_hooks = global_hooks_snapshot();
    let Some(string_item) = cJSON_New_Item(&item_hooks, cJSON_String) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let hooks = global_hooks_snapshot();
    let Some(valuestring) = CjsonString::copied(string.map(::std::ffi::CStr::to_bytes_with_nul))
    else {
        cJSON_Delete(Some(string_item));
        return ::core::ptr::null_mut::<cJSON>();
    };
    string_item.valuestring.replace(valuestring);
    match add_created_item_to_object(object, name, string_item, &hooks) {
        Some(item) => ::core::ptr::from_ref(item) as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
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
        Some(unsafe { ::std::ffi::CStr::from_ptr(name) })
    };
    let string = if string.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(string) })
    };
    cJSON_AddStringToObject(unsafe { object.as_mut() }, name, string)
}
pub fn cJSON_AddRawToObject(
    object: Option<&mut cJSON>,
    name: Option<&::std::ffi::CStr>,
    raw: Option<&::std::ffi::CStr>,
) -> *mut cJSON {
    let item_hooks = global_hooks_snapshot();
    let Some(raw_item) = cJSON_New_Item(&item_hooks, cJSON_Raw) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let hooks = global_hooks_snapshot();
    let Some(valuestring) = CjsonString::copied(raw.map(::std::ffi::CStr::to_bytes_with_nul))
    else {
        cJSON_Delete(Some(raw_item));
        return ::core::ptr::null_mut::<cJSON>();
    };
    raw_item.valuestring.replace(valuestring);
    match add_created_item_to_object(object, name, raw_item, &hooks) {
        Some(item) => ::core::ptr::from_ref(item) as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
#[export_name = "cJSON_AddRawToObject"]
pub unsafe extern "C" fn cJSON_AddRawToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
    raw: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let name = if name.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(name) })
    };
    let raw = if raw.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(raw) })
    };
    cJSON_AddRawToObject(unsafe { object.as_mut() }, name, raw)
}
pub fn cJSON_AddObjectToObject(
    object: Option<&mut cJSON>,
    name: Option<&::std::ffi::CStr>,
) -> *mut cJSON {
    let item_hooks = global_hooks_snapshot();
    let Some(object_item) = cJSON_New_Item(&item_hooks, cJSON_Object) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let hooks = global_hooks_snapshot();
    match add_created_item_to_object(object, name, object_item, &hooks) {
        Some(item) => ::core::ptr::from_ref(item) as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
#[export_name = "cJSON_AddObjectToObject"]
pub unsafe extern "C" fn cJSON_AddObjectToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let name = if name.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(name) })
    };
    cJSON_AddObjectToObject(unsafe { object.as_mut() }, name)
}
pub fn cJSON_AddArrayToObject(
    object: Option<&mut cJSON>,
    name: Option<&::std::ffi::CStr>,
) -> *mut cJSON {
    let item_hooks = global_hooks_snapshot();
    let Some(array) = cJSON_New_Item(&item_hooks, cJSON_Array) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let hooks = global_hooks_snapshot();
    match add_created_item_to_object(object, name, array, &hooks) {
        Some(item) => ::core::ptr::from_ref(item) as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
#[export_name = "cJSON_AddArrayToObject"]
pub unsafe extern "C" fn cJSON_AddArrayToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let name = if name.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(name) })
    };
    cJSON_AddArrayToObject(unsafe { object.as_mut() }, name)
}
pub fn cJSON_DetachItemViaPointer<'a>(
    parent: Option<&cJSON>,
    item: Option<&'a cJSON>,
) -> Option<&'a cJSON> {
    let Some(parent_ref) = parent else {
        return None;
    };
    let Some(item_ref) = item else {
        return None;
    };
    let mut index = 0 as size_t;
    let item_index = loop {
        match get_array_item(Some(&*parent_ref), index) {
            Some(current) => {
                if ::core::ptr::eq::<cJSON>(current, item_ref) {
                    break Some(index);
                }
            }
            None => break None,
        }
        index = index.wrapping_add(1);
    };
    let Some(index) = item_index else {
        return None;
    };

    let previous_item = if index == 0 {
        None
    } else {
        get_array_item(Some(&*parent_ref), index.wrapping_sub(1))
    };
    let next_item = get_array_item(Some(&*parent_ref), index.wrapping_add(1));

    if let Some(previous_item) = previous_item {
        previous_item.next.set(item_ref.next.get());
    } else {
        parent_ref.child.set(item_ref.next.get());
    }
    if let Some(next_item) = next_item {
        next_item.prev.set(item_ref.prev.get());
    } else if let Some(head) = get_array_item(Some(&*parent_ref), 0 as size_t) {
        head.prev.set(item_ref.prev.get());
    }

    item_ref.prev.set(None);
    item_ref.next.set(None);
    Some(item_ref)
}
#[export_name = "cJSON_DetachItemViaPointer"]
pub unsafe extern "C" fn cJSON_DetachItemViaPointer_ffi(
    mut parent: *mut cJSON,
    item: *mut cJSON,
) -> *mut cJSON {
    if parent.is_null() || item.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    if parent == item {
        let parent_ref = unsafe { &mut *parent };
        let parent_ptr = ::core::ptr::from_mut(parent_ref);
        let parent_is_child = parent_ref
            .child
            .get()
            .is_some_and(|child| ::core::ptr::eq(child, &*parent_ref));
        if !parent_is_child && parent_ref.prev.get().is_none() {
            return ::core::ptr::null_mut::<cJSON>();
        }
        if !parent_is_child {
            if let Some(previous) = parent_ref.prev.get() {
                previous.next.set(parent_ref.next.get());
            }
        }
        if let Some(next) = parent_ref.next.get() {
            next.prev.set(parent_ref.prev.get());
        }
        if parent_is_child {
            parent_ref.child.set(parent_ref.next.get());
        } else if parent_ref.next.get().is_none() {
            if let Some(child) = parent_ref.child.get() {
                child.prev.set(parent_ref.prev.get());
            }
        }
        parent_ref.prev.set(None);
        parent_ref.next.set(None);
        return parent_ptr;
    }
    match cJSON_DetachItemViaPointer(unsafe { parent.as_ref() }, unsafe { item.as_ref() }) {
        Some(item) => item as *const cJSON as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_DetachItemFromArray(array: Option<&cJSON>, which: size_t) -> Option<&cJSON> {
    let Some(array) = array else {
        return None;
    };
    let item = match get_array_item(Some(array), which) {
        Some(item) => item,
        None => return None,
    };
    cJSON_DetachItemViaPointer(Some(array), Some(item))
}
#[export_name = "cJSON_DetachItemFromArray"]
pub unsafe extern "C" fn cJSON_DetachItemFromArray_ffi(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
) -> *mut cJSON {
    if which < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<cJSON>();
    }
    match cJSON_DetachItemFromArray(unsafe { array.as_ref() }, which as size_t) {
        Some(item) => item as *const cJSON as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_DeleteItemFromArray(array: Option<&mut cJSON>, mut which: ::core::ffi::c_int) {
    let item = if which < 0 as ::core::ffi::c_int {
        None
    } else {
        cJSON_DetachItemFromArray(array.map(|array| &*array), which as size_t)
    };
    cJSON_Delete(item);
}
#[export_name = "cJSON_DeleteItemFromArray"]
pub unsafe extern "C" fn cJSON_DeleteItemFromArray_ffi(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
) {
    cJSON_DeleteItemFromArray(unsafe { array.as_mut() }, which)
}
pub fn cJSON_DetachItemFromObject<'a>(
    object: Option<&'a mut cJSON>,
    string: Option<&::std::ffi::CStr>,
) -> Option<&'a cJSON> {
    detach_item_from_object(object, string, false_0)
}
#[export_name = "cJSON_DetachItemFromObject"]
pub unsafe extern "C" fn cJSON_DetachItemFromObject_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let string = if string.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(string) })
    };
    match cJSON_DetachItemFromObject(unsafe { object.as_mut() }, string) {
        Some(item) => item as *const cJSON as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_DetachItemFromObjectCaseSensitive<'a>(
    object: Option<&'a mut cJSON>,
    string: Option<&::std::ffi::CStr>,
) -> Option<&'a cJSON> {
    detach_item_from_object(object, string, true_0)
}
fn detach_item_from_object<'a>(
    object: Option<&'a mut cJSON>,
    string: Option<&::std::ffi::CStr>,
    case_sensitive: cJSON_bool,
) -> Option<&'a cJSON> {
    let Some(object) = object else {
        return None;
    };
    let to_detach = match if case_sensitive != 0 {
        cJSON_GetObjectItemCaseSensitive(Some(&*object), string)
    } else {
        cJSON_GetObjectItem(Some(&*object), string)
    } {
        Some(item) => item as *const cJSON,
        None => return None,
    };
    let mut index = 0 as size_t;
    let mut to_detach_index = None;
    while let Some(item) = get_array_item(Some(&*object), index) {
        if ::core::ptr::eq(item, to_detach) {
            to_detach_index = Some(index);
            break;
        }
        index = index.wrapping_add(1);
    }
    match to_detach_index {
        Some(index) => cJSON_DetachItemFromArray(Some(&*object), index),
        None => None,
    }
}
#[export_name = "cJSON_DetachItemFromObjectCaseSensitive"]
pub unsafe extern "C" fn cJSON_DetachItemFromObjectCaseSensitive_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let string = if string.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(string) })
    };
    match cJSON_DetachItemFromObjectCaseSensitive(unsafe { object.as_mut() }, string) {
        Some(item) => item as *const cJSON as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_DeleteItemFromObject(object: Option<&mut cJSON>, string: Option<&::std::ffi::CStr>) {
    let item = cJSON_DetachItemFromObject(object, string);
    cJSON_Delete(item);
}
#[export_name = "cJSON_DeleteItemFromObject"]
pub unsafe extern "C" fn cJSON_DeleteItemFromObject_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
) {
    let string = if string.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(string) })
    };
    cJSON_DeleteItemFromObject(unsafe { object.as_mut() }, string)
}
pub fn cJSON_DeleteItemFromObjectCaseSensitive(
    object: Option<&mut cJSON>,
    string: Option<&::std::ffi::CStr>,
) {
    let item = cJSON_DetachItemFromObjectCaseSensitive(object, string);
    cJSON_Delete(item);
}
#[export_name = "cJSON_DeleteItemFromObjectCaseSensitive"]
pub unsafe extern "C" fn cJSON_DeleteItemFromObjectCaseSensitive_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
) {
    let string = if string.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(string) })
    };
    cJSON_DeleteItemFromObjectCaseSensitive(unsafe { object.as_mut() }, string)
}
pub fn cJSON_InsertItemInArray(
    array: Option<&mut cJSON>,
    which: ::core::ffi::c_int,
    newitem: Option<&'static mut cJSON>,
) -> cJSON_bool {
    if which < 0 as ::core::ffi::c_int {
        return false_0;
    }
    let Some(array_ref) = array else {
        return false_0;
    };
    let Some(newitem_ref) = newitem else {
        return false_0;
    };
    let newitem_link: &'static cJSON = newitem_ref;
    let after_inserted = get_array_item(Some(&*array_ref), which as size_t);
    let Some(after_inserted_ref) = after_inserted else {
        return add_item_to_array(array_ref, newitem_link);
    };
    if ::core::ptr::eq(after_inserted_ref, newitem_link) {
        return false_0;
    }
    if array_ref
        .child
        .get()
        .is_some_and(|child| !::core::ptr::eq(child, after_inserted_ref))
        && after_inserted_ref.prev.get().is_none()
    {
        return false_0;
    }
    newitem_link.next.set(Some(after_inserted_ref));
    newitem_link.prev.set(after_inserted_ref.prev.get());
    after_inserted_ref.prev.set(Some(newitem_link));
    if array_ref
        .child
        .get()
        .is_some_and(|child| ::core::ptr::eq(child, after_inserted_ref))
    {
        array_ref.child.set(Some(newitem_link));
    } else if let Some(previous_item) =
        get_array_item(Some(&*array_ref), (which as size_t).wrapping_sub(1))
    {
        previous_item.next.set(Some(newitem_link));
    } else {
        return false_0;
    }
    return true_0;
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
    cJSON_InsertItemInArray(unsafe { array.as_mut() }, which, unsafe {
        newitem.as_mut()
    })
}
pub fn cJSON_ReplaceItemViaPointer(
    parent: Option<&mut cJSON>,
    item: Option<&mut cJSON>,
    replacement: Option<&'static mut cJSON>,
) -> cJSON_bool {
    let Some(parent_ref) = parent else {
        return false_0;
    };
    let Some(item_ref) = item else {
        return false_0;
    };
    let Some(replacement_ref) = replacement else {
        return false_0;
    };
    if parent_ref.child.get().is_none() {
        return false_0;
    }
    if ::core::ptr::eq::<cJSON>(&*replacement_ref, &*item_ref) {
        return true_0;
    }

    let mut index = 0 as size_t;
    let item_index = loop {
        match get_array_item(Some(&*parent_ref), index) {
            Some(current) => {
                if ::core::ptr::eq::<cJSON>(current, &*item_ref) {
                    break Some(index);
                }
            }
            None => break None,
        }
        index = index.wrapping_add(1);
    };
    let Some(index) = item_index else {
        return false_0;
    };
    let Some(detached) = cJSON_DetachItemFromArray(Some(parent_ref), index) else {
        return false_0;
    };
    cJSON_Delete(Some(detached));
    replacement_ref.next.set(None);
    replacement_ref.prev.set(None);
    return cJSON_InsertItemInArray(
        Some(parent_ref),
        index as ::core::ffi::c_int,
        Some(replacement_ref),
    );
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
    let parent_ref = unsafe { &mut *parent };
    if parent_ref.child.get().is_none() {
        return false_0;
    }
    if replacement == item {
        return true_0;
    }

    if parent == item {
        {
            let replacement_ref = unsafe { &mut *replacement };
            let replacement_link: &'static cJSON = unsafe { &*replacement };
            replacement_ref.next.set(parent_ref.next.get());
            replacement_ref.prev.set(parent_ref.prev.get());
            if let Some(next) = replacement_ref.next.get() {
                next.prev.set(Some(replacement_link));
            }
            if parent_ref
                .child
                .get()
                .is_some_and(|child| ::core::ptr::eq(child, &*parent_ref))
            {
                if parent_ref
                    .prev
                    .get()
                    .is_some_and(|previous| ::core::ptr::eq(previous, &*parent_ref))
                {
                    replacement_ref.prev.set(Some(replacement_link));
                }
                parent_ref.child.set(Some(replacement_link));
            } else {
                if let Some(previous) = replacement_ref.prev.get() {
                    previous.next.set(Some(replacement_link));
                }
                if replacement_ref.next.get().is_none() {
                    if let Some(child) = parent_ref.child.get() {
                        child.prev.set(Some(replacement_link));
                    }
                }
            }
        }
        parent_ref.next.set(None);
        parent_ref.prev.set(None);
        cJSON_Delete(Some(parent_ref));
        return true_0;
    }

    if parent == replacement {
        let parent_link: &'static cJSON = unsafe { &*parent };
        let item_ref = unsafe { &mut *item };
        parent_ref.next.set(item_ref.next.get());
        parent_ref.prev.set(item_ref.prev.get());
        if let Some(next) = parent_ref.next.get() {
            next.prev.set(Some(parent_link));
        }
        if parent_ref
            .child
            .get()
            .is_some_and(|child| ::core::ptr::eq(child, &*item_ref))
        {
            if item_ref
                .prev
                .get()
                .is_some_and(|previous| ::core::ptr::eq(previous, &*item_ref))
            {
                parent_ref.prev.set(Some(parent_link));
            }
            parent_ref.child.set(Some(parent_link));
        } else {
            if let Some(previous) = parent_ref.prev.get() {
                previous.next.set(Some(parent_link));
            }
            if parent_ref.next.get().is_none() {
                if let Some(child) = parent_ref.child.get() {
                    child.prev.set(Some(parent_link));
                }
            }
        }
        item_ref.next.set(None);
        item_ref.prev.set(None);
        cJSON_Delete(Some(item_ref));
        return true_0;
    }

    cJSON_ReplaceItemViaPointer(
        unsafe { parent.as_mut() },
        unsafe { item.as_mut() },
        unsafe { replacement.as_mut() },
    )
}
pub fn cJSON_ReplaceItemInArray(
    array: Option<&mut cJSON>,
    mut which: ::core::ffi::c_int,
    newitem: Option<&'static mut cJSON>,
) -> cJSON_bool {
    if which < 0 as ::core::ffi::c_int {
        return false_0;
    }
    let Some(array) = array else {
        return false_0;
    };
    let Some(newitem) = newitem else {
        return false_0;
    };
    let item = match get_array_item(Some(&*array), which as size_t) {
        Some(item) => item,
        None => return false_0,
    };
    if ::core::ptr::eq(item, &*newitem) {
        return true_0;
    }
    let Some(item) = cJSON_DetachItemFromArray(Some(array), which as size_t) else {
        return false_0;
    };
    cJSON_Delete(Some(item));
    return cJSON_InsertItemInArray(Some(array), which, Some(newitem));
}
#[export_name = "cJSON_ReplaceItemInArray"]
pub unsafe extern "C" fn cJSON_ReplaceItemInArray_ffi(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
    mut newitem: *mut cJSON,
) -> cJSON_bool {
    if array == newitem {
        if which < 0 as ::core::ffi::c_int {
            return false_0;
        }
        let item = match get_array_item(unsafe { array.as_ref() }, which as size_t) {
            Some(item) => item as *const cJSON as *mut cJSON,
            None => ::core::ptr::null_mut::<cJSON>(),
        };
        return cJSON_ReplaceItemViaPointer_ffi(array, item, newitem);
    }
    cJSON_ReplaceItemInArray(unsafe { array.as_mut() }, which, unsafe {
        newitem.as_mut()
    })
}
fn prepare_replacement_object_key(
    replacement: &mut cJSON,
    string: &::std::ffi::CStr,
) -> cJSON_bool {
    let Some(new_key) = CjsonString::copied(Some(string.to_bytes_with_nul())) else {
        return false_0;
    };
    replacement.string.replace(new_key);
    replacement.type_0 &= !cJSON_StringIsConst;
    true_0
}
fn replace_item_in_object(
    object: Option<&mut cJSON>,
    string: Option<&::std::ffi::CStr>,
    replacement: Option<&'static mut cJSON>,
    case_sensitive: cJSON_bool,
) -> cJSON_bool {
    let Some(string) = string else { return false_0 };
    let Some(replacement) = replacement else {
        return false_0;
    };
    if prepare_replacement_object_key(replacement, string) == false_0 {
        return false_0;
    }
    let Some(object) = object else {
        return false_0;
    };
    let (to_replace_index, replacing_self) = {
        let to_replace = match get_object_item(Some(&*object), Some(string), case_sensitive) {
            Some(item) => item,
            None => return false_0,
        };
        if ::core::ptr::eq(to_replace, &*replacement) {
            (None, true)
        } else {
            let mut index = 0 as size_t;
            let mut to_replace_index = None;
            while let Some(item) = get_array_item(Some(&*object), index) {
                if ::core::ptr::eq(item, to_replace) {
                    to_replace_index = Some(index);
                    break;
                }
                index = index.wrapping_add(1);
            }
            (to_replace_index, false)
        }
    };
    if replacing_self {
        return true_0;
    }
    let Some(to_replace_index) = to_replace_index else {
        return false_0;
    };
    let Some(item) = cJSON_DetachItemFromArray(Some(object), to_replace_index) else {
        return false_0;
    };
    cJSON_Delete(Some(item));
    return cJSON_InsertItemInArray(
        Some(object),
        to_replace_index as ::core::ffi::c_int,
        Some(replacement),
    );
}
pub fn cJSON_ReplaceItemInObject(
    object: Option<&mut cJSON>,
    string: Option<&::std::ffi::CStr>,
    newitem: Option<&'static mut cJSON>,
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
        Some(unsafe { ::std::ffi::CStr::from_ptr(string) })
    };
    if object == newitem {
        let Some(string) = string else { return false_0 };
        let Some(replacement) = (unsafe { object.as_mut() }) else {
            return false_0;
        };
        if prepare_replacement_object_key(replacement, string) == false_0 {
            return false_0;
        }
        let to_replace = match get_object_item(Some(&*replacement), Some(string), false_0) {
            Some(item) => item as *const cJSON as *mut cJSON,
            None => ::core::ptr::null_mut::<cJSON>(),
        };
        if to_replace == newitem {
            return true_0;
        }
        return cJSON_ReplaceItemViaPointer_ffi(object, to_replace, newitem);
    }
    cJSON_ReplaceItemInObject(unsafe { object.as_mut() }, string, unsafe {
        newitem.as_mut()
    })
}
pub fn cJSON_ReplaceItemInObjectCaseSensitive(
    object: Option<&mut cJSON>,
    string: Option<&::std::ffi::CStr>,
    newitem: Option<&'static mut cJSON>,
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
        Some(unsafe { ::std::ffi::CStr::from_ptr(string) })
    };
    if object == newitem {
        let Some(string) = string else { return false_0 };
        let Some(replacement) = (unsafe { object.as_mut() }) else {
            return false_0;
        };
        if prepare_replacement_object_key(replacement, string) == false_0 {
            return false_0;
        }
        let to_replace = match get_object_item(Some(&*replacement), Some(string), true_0) {
            Some(item) => item as *const cJSON as *mut cJSON,
            None => ::core::ptr::null_mut::<cJSON>(),
        };
        if to_replace == newitem {
            return true_0;
        }
        return cJSON_ReplaceItemViaPointer_ffi(object, to_replace, newitem);
    }
    cJSON_ReplaceItemInObjectCaseSensitive(unsafe { object.as_mut() }, string, unsafe {
        newitem.as_mut()
    })
}
pub fn cJSON_CreateNull() -> *mut cJSON {
    let hooks = global_hooks_snapshot();
    let Some(item) = cJSON_New_Item(&hooks, cJSON_NULL) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    return ::core::ptr::from_mut(item);
}
#[export_name = "cJSON_CreateNull"]
pub unsafe extern "C" fn cJSON_CreateNull_ffi() -> *mut cJSON {
    cJSON_CreateNull()
}
pub fn cJSON_CreateTrue() -> *mut cJSON {
    let hooks = global_hooks_snapshot();
    let Some(item) = cJSON_New_Item(&hooks, cJSON_True) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    return ::core::ptr::from_mut(item);
}
#[export_name = "cJSON_CreateTrue"]
pub unsafe extern "C" fn cJSON_CreateTrue_ffi() -> *mut cJSON {
    cJSON_CreateTrue()
}
pub fn cJSON_CreateFalse() -> *mut cJSON {
    let hooks = global_hooks_snapshot();
    let Some(item) = cJSON_New_Item(&hooks, cJSON_False) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    return ::core::ptr::from_mut(item);
}
#[export_name = "cJSON_CreateFalse"]
pub unsafe extern "C" fn cJSON_CreateFalse_ffi() -> *mut cJSON {
    cJSON_CreateFalse()
}
pub fn cJSON_CreateBool(mut boolean: cJSON_bool) -> *mut cJSON {
    let hooks = global_hooks_snapshot();
    let Some(item) = cJSON_New_Item(
        &hooks,
        if boolean != 0 {
            cJSON_True
        } else {
            cJSON_False
        },
    ) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    return ::core::ptr::from_mut(item);
}
#[export_name = "cJSON_CreateBool"]
pub unsafe extern "C" fn cJSON_CreateBool_ffi(mut boolean: cJSON_bool) -> *mut cJSON {
    cJSON_CreateBool(boolean)
}
pub fn cJSON_CreateNumber(mut num: ::core::ffi::c_double) -> *mut cJSON {
    let hooks = global_hooks_snapshot();
    let Some(item) = cJSON_New_Item(&hooks, cJSON_Number) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    cJSON_SetNumberHelper(item, num);
    return ::core::ptr::from_mut(item);
}
#[export_name = "cJSON_CreateNumber"]
pub unsafe extern "C" fn cJSON_CreateNumber_ffi(mut num: ::core::ffi::c_double) -> *mut cJSON {
    cJSON_CreateNumber(num)
}
pub fn cJSON_CreateString(string: Option<&::std::ffi::CStr>) -> *mut cJSON {
    let hooks = global_hooks_snapshot();
    let Some(item) = cJSON_New_Item(&hooks, cJSON_String) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let Some(valuestring) = CjsonString::copied(string.map(::std::ffi::CStr::to_bytes_with_nul))
    else {
        cJSON_Delete(Some(item));
        return ::core::ptr::null_mut::<cJSON>();
    };
    item.valuestring.replace(valuestring);
    return ::core::ptr::from_mut(item);
}
#[export_name = "cJSON_CreateString"]
pub unsafe extern "C" fn cJSON_CreateString_ffi(
    mut string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let string = if string.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(string) })
    };
    cJSON_CreateString(string)
}
pub fn cJSON_CreateStringReference(string: Option<&'static ::std::ffi::CStr>) -> *mut cJSON {
    let hooks = global_hooks_snapshot();
    let Some(item) = cJSON_New_Item(&hooks, cJSON_String | cJSON_IsReference) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    item.valuestring.replace(match string {
        Some(string) => CjsonString::Borrowed(string),
        None => CjsonString::Null,
    });
    return ::core::ptr::from_mut(item);
}
#[export_name = "cJSON_CreateStringReference"]
pub unsafe extern "C" fn cJSON_CreateStringReference_ffi(
    mut string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let string: Option<&'static ::std::ffi::CStr> = if string.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(string) })
    };
    cJSON_CreateStringReference(string)
}
pub fn cJSON_CreateObjectReference(child: Option<&'static cJSON>) -> *mut cJSON {
    let hooks = global_hooks_snapshot();
    let Some(item) = cJSON_New_Item(&hooks, cJSON_Object | cJSON_IsReference) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    item.child.set(child);
    return ::core::ptr::from_mut(item);
}
#[export_name = "cJSON_CreateObjectReference"]
pub unsafe extern "C" fn cJSON_CreateObjectReference_ffi(mut child: *const cJSON) -> *mut cJSON {
    cJSON_CreateObjectReference(unsafe { child.as_ref() })
}
pub fn cJSON_CreateArrayReference(child: Option<&'static cJSON>) -> *mut cJSON {
    let hooks = global_hooks_snapshot();
    let Some(item) = cJSON_New_Item(&hooks, cJSON_Array | cJSON_IsReference) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    item.child.set(child);
    return ::core::ptr::from_mut(item);
}
#[export_name = "cJSON_CreateArrayReference"]
pub unsafe extern "C" fn cJSON_CreateArrayReference_ffi(mut child: *const cJSON) -> *mut cJSON {
    cJSON_CreateArrayReference(unsafe { child.as_ref() })
}
pub fn cJSON_CreateRaw(raw: Option<&::std::ffi::CStr>) -> *mut cJSON {
    let hooks = global_hooks_snapshot();
    let Some(item) = cJSON_New_Item(&hooks, cJSON_Raw) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let Some(valuestring) = CjsonString::copied(raw.map(::std::ffi::CStr::to_bytes_with_nul))
    else {
        cJSON_Delete(Some(item));
        return ::core::ptr::null_mut::<cJSON>();
    };
    item.valuestring.replace(valuestring);
    return ::core::ptr::from_mut(item);
}
#[export_name = "cJSON_CreateRaw"]
pub unsafe extern "C" fn cJSON_CreateRaw_ffi(mut raw: *const ::core::ffi::c_char) -> *mut cJSON {
    let raw = if raw.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(raw) })
    };
    cJSON_CreateRaw(raw)
}
pub fn cJSON_CreateArray() -> *mut cJSON {
    let hooks = global_hooks_snapshot();
    let Some(item) = cJSON_New_Item(&hooks, cJSON_Array) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    return ::core::ptr::from_mut(item);
}
#[export_name = "cJSON_CreateArray"]
pub unsafe extern "C" fn cJSON_CreateArray_ffi() -> *mut cJSON {
    cJSON_CreateArray()
}
pub fn cJSON_CreateObject() -> *mut cJSON {
    let hooks = global_hooks_snapshot();
    let Some(item) = cJSON_New_Item(&hooks, cJSON_Object) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    return ::core::ptr::from_mut(item);
}
#[export_name = "cJSON_CreateObject"]
pub unsafe extern "C" fn cJSON_CreateObject_ffi() -> *mut cJSON {
    cJSON_CreateObject()
}
pub fn cJSON_CreateIntArray(numbers: &[::core::ffi::c_int]) -> *mut cJSON {
    let array_hooks = global_hooks_snapshot();
    let Some(array_ref) = cJSON_New_Item(&array_hooks, cJSON_Array) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    for &number in numbers {
        let item_hooks = global_hooks_snapshot();
        let Some(number_ref) = cJSON_New_Item(&item_hooks, cJSON_Number) else {
            cJSON_Delete(Some(array_ref));
            return ::core::ptr::null_mut::<cJSON>();
        };
        cJSON_SetNumberHelper(number_ref, number as ::core::ffi::c_double);
        add_item_to_array(array_ref, number_ref);
    }
    return ::core::ptr::from_mut(array_ref);
}
#[export_name = "cJSON_CreateIntArray"]
pub unsafe extern "C" fn cJSON_CreateIntArray_ffi(
    mut numbers: *const ::core::ffi::c_int,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let numbers = if count == 0 as ::core::ffi::c_int {
        &[]
    } else {
        unsafe { ::core::slice::from_raw_parts(numbers, count as usize) }
    };
    cJSON_CreateIntArray(numbers)
}
pub fn cJSON_CreateFloatArray(numbers: &[::core::ffi::c_float]) -> *mut cJSON {
    let array_hooks = global_hooks_snapshot();
    let Some(array_ref) = cJSON_New_Item(&array_hooks, cJSON_Array) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    for &number in numbers {
        let item_hooks = global_hooks_snapshot();
        let Some(number_ref) = cJSON_New_Item(&item_hooks, cJSON_Number) else {
            cJSON_Delete(Some(array_ref));
            return ::core::ptr::null_mut::<cJSON>();
        };
        cJSON_SetNumberHelper(number_ref, number as ::core::ffi::c_double);
        add_item_to_array(array_ref, number_ref);
    }
    return ::core::ptr::from_mut(array_ref);
}
#[export_name = "cJSON_CreateFloatArray"]
pub unsafe extern "C" fn cJSON_CreateFloatArray_ffi(
    mut numbers: *const ::core::ffi::c_float,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let numbers = if count == 0 as ::core::ffi::c_int {
        &[]
    } else {
        unsafe { ::core::slice::from_raw_parts(numbers, count as usize) }
    };
    cJSON_CreateFloatArray(numbers)
}
pub fn cJSON_CreateDoubleArray(numbers: &[::core::ffi::c_double]) -> *mut cJSON {
    let array_hooks = global_hooks_snapshot();
    let Some(array_ref) = cJSON_New_Item(&array_hooks, cJSON_Array) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    for &number in numbers {
        let item_hooks = global_hooks_snapshot();
        let Some(number_ref) = cJSON_New_Item(&item_hooks, cJSON_Number) else {
            cJSON_Delete(Some(array_ref));
            return ::core::ptr::null_mut::<cJSON>();
        };
        cJSON_SetNumberHelper(number_ref, number);
        add_item_to_array(array_ref, number_ref);
    }
    return ::core::ptr::from_mut(array_ref);
}
#[export_name = "cJSON_CreateDoubleArray"]
pub unsafe extern "C" fn cJSON_CreateDoubleArray_ffi(
    mut numbers: *const ::core::ffi::c_double,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let numbers = if count == 0 as ::core::ffi::c_int {
        &[]
    } else {
        unsafe { ::core::slice::from_raw_parts(numbers, count as usize) }
    };
    cJSON_CreateDoubleArray(numbers)
}
pub fn cJSON_CreateStringArray(strings: &[Option<&::std::ffi::CStr>]) -> *mut cJSON {
    let array_hooks = global_hooks_snapshot();
    let Some(array_ref) = cJSON_New_Item(&array_hooks, cJSON_Array) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let mut failed = false;
    for &string in strings {
        let item_hooks = global_hooks_snapshot();
        let Some(string_ref) = cJSON_New_Item(&item_hooks, cJSON_String) else {
            failed = true;
            break;
        };
        let Some(valuestring) =
            CjsonString::copied(string.map(::std::ffi::CStr::to_bytes_with_nul))
        else {
            failed = true;
            break;
        };
        string_ref.valuestring.replace(valuestring);
        add_item_to_array(array_ref, string_ref);
    }
    if failed {
        cJSON_Delete(Some(array_ref));
        return ::core::ptr::null_mut::<cJSON>();
    }
    return ::core::ptr::from_mut(array_ref);
}
#[export_name = "cJSON_CreateStringArray"]
pub unsafe extern "C" fn cJSON_CreateStringArray_ffi(
    mut strings: *const *const ::core::ffi::c_char,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    if count < 0 as ::core::ffi::c_int || strings.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    if count == 0 as ::core::ffi::c_int {
        return cJSON_CreateStringArray(&[]);
    }
    let raw_strings = unsafe { ::core::slice::from_raw_parts(strings, count as usize) };
    let mut strings = Vec::with_capacity(raw_strings.len());
    for &string in raw_strings {
        let string = if string.is_null() {
            None
        } else {
            Some(unsafe { ::std::ffi::CStr::from_ptr(string) })
        };
        strings.push(string);
    }
    cJSON_CreateStringArray(&strings)
}
pub fn cJSON_Duplicate(item: Option<&cJSON>, recurse: cJSON_bool) -> *mut cJSON {
    match cJSON_Duplicate_rec(item, 0 as size_t, recurse) {
        Some(item) => ::core::ptr::from_mut(item),
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
#[export_name = "cJSON_Duplicate"]
pub unsafe extern "C" fn cJSON_Duplicate_ffi(
    mut item: *const cJSON,
    mut recurse: cJSON_bool,
) -> *mut cJSON {
    cJSON_Duplicate(unsafe { item.as_ref() }, recurse)
}
pub fn cJSON_Duplicate_rec(
    item: Option<&cJSON>,
    depth: size_t,
    recurse: cJSON_bool,
) -> Option<&'static mut cJSON> {
    let Some(item) = item else {
        return None;
    };
    let hooks = global_hooks_snapshot();
    let Some(newitem) = cJSON_New_Item(&hooks, cJSON_Invalid) else {
        return None;
    };
    newitem.type_0 = item.type_0 & !cJSON_IsReference;
    newitem.valueint = item.valueint;
    newitem.valuedouble = item.valuedouble;
    let duplicated = 'copy: {
        if !item.valuestring.borrow().is_null() {
            let Some(valuestring) = print_string(item, CjsonStringSlot::Value) else {
                break 'copy false;
            };
            let Some(valuestring_bytes) = valuestring.to_bytes_with_nul() else {
                break 'copy false;
            };
            let Some(valuestring) = CjsonString::copied(Some(&valuestring_bytes)) else {
                break 'copy false;
            };
            newitem.valuestring.replace(valuestring);
        }
        if !item.string.borrow().is_null() {
            let new_string = if item.type_0 & cJSON_StringIsConst != 0 {
                item.string.borrow().clone()
            } else {
                let Some(string) = print_string(item, CjsonStringSlot::ObjectKey) else {
                    break 'copy false;
                };
                let Some(string_bytes) = string.to_bytes_with_nul() else {
                    break 'copy false;
                };
                let Some(string) = CjsonString::copied(Some(&string_bytes)) else {
                    break 'copy false;
                };
                string
            };
            newitem.string.replace(new_string);
        }
        if recurse == 0 {
            break 'copy true;
        }
        let mut child_index = 0 as size_t;
        while let Some(child_ref) = get_array_item(Some(item), child_index) {
            if depth >= CJSON_CIRCULAR_LIMIT as size_t {
                break 'copy false;
            }
            let Some(newchild) =
                cJSON_Duplicate_rec(Some(child_ref), depth.wrapping_add(1 as size_t), true_0)
            else {
                break 'copy false;
            };
            add_item_to_array(newitem, newchild);
            child_index = child_index.wrapping_add(1);
        }
        true
    };
    if !duplicated {
        cJSON_Delete(Some(newitem));
        return None;
    }
    Some(newitem)
}
#[export_name = "cJSON_Duplicate_rec"]
pub unsafe extern "C" fn cJSON_Duplicate_rec_ffi(
    mut item: *const cJSON,
    mut depth: size_t,
    mut recurse: cJSON_bool,
) -> *mut cJSON {
    match cJSON_Duplicate_rec(unsafe { item.as_ref() }, depth, recurse) {
        Some(item) => ::core::ptr::from_mut(item),
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
fn skip_oneline_comment(input: &mut usize, json: &[::core::ffi::c_char]) {
    *input = (*input).wrapping_add(2 as usize);
    while json
        .get(*input)
        .copied()
        .is_some_and(|character| character as ::core::ffi::c_int != '\0' as i32)
    {
        if json[*input] as ::core::ffi::c_int == '\n' as i32 {
            *input = (*input).wrapping_add(1 as usize);
            return;
        }
        *input = (*input).wrapping_add(1 as usize);
    }
}
fn skip_multiline_comment(input: &mut usize, json: &[::core::ffi::c_char]) {
    *input = (*input).wrapping_add(2 as usize);
    while json
        .get(*input)
        .copied()
        .is_some_and(|character| character as ::core::ffi::c_int != '\0' as i32)
    {
        if json[*input] as ::core::ffi::c_int == '*' as i32
            && json
                .get((*input).wrapping_add(1 as usize))
                .copied()
                .is_some_and(|character| character as ::core::ffi::c_int == '/' as i32)
        {
            *input = (*input).wrapping_add(2 as usize);
            return;
        }
        *input = (*input).wrapping_add(1 as usize);
    }
}
fn minify_string(input: &mut usize, output: &mut usize, json: &mut [::core::ffi::c_char]) {
    if json.get(*input).is_none() || json.get(*output).is_none() {
        return;
    }
    json[*output] = json[*input];
    *input = (*input).wrapping_add(1 as usize);
    *output = (*output).wrapping_add(1 as usize);
    while json
        .get(*input)
        .copied()
        .is_some_and(|character| character as ::core::ffi::c_int != '\0' as i32)
    {
        if json.get(*output).is_none() {
            return;
        }
        json[*output] = json[*input];
        if json[*input] as ::core::ffi::c_int == '"' as i32 {
            json[*output] = '"' as i32 as ::core::ffi::c_char;
            *input = (*input).wrapping_add(1 as usize);
            *output = (*output).wrapping_add(1 as usize);
            return;
        } else if json[*input] as ::core::ffi::c_int == '\\' as i32
            && json
                .get((*input).wrapping_add(1 as usize))
                .copied()
                .is_some_and(|character| character as ::core::ffi::c_int == '"' as i32)
        {
            if (*output).wrapping_add(1 as usize) >= json.len() {
                return;
            }
            json[(*output).wrapping_add(1 as usize)] = json[(*input).wrapping_add(1 as usize)];
            *input = (*input).wrapping_add(1 as usize);
            *output = (*output).wrapping_add(1 as usize);
        }
        *input = (*input).wrapping_add(1 as usize);
        *output = (*output).wrapping_add(1 as usize);
    }
}
pub fn cJSON_Minify(json: Option<&mut [::core::ffi::c_char]>) {
    let Some(json) = json else {
        return;
    };
    let mut input = 0usize;
    let mut output = 0usize;
    while json
        .get(input)
        .copied()
        .is_some_and(|character| character as ::core::ffi::c_int != '\0' as i32)
    {
        match json[input] as ::core::ffi::c_int {
            32 | 9 | 13 | 10 => {
                input = input.wrapping_add(1 as usize);
            }
            47 => {
                if json
                    .get(input.wrapping_add(1 as usize))
                    .copied()
                    .is_some_and(|character| character as ::core::ffi::c_int == '/' as i32)
                {
                    skip_oneline_comment(&mut input, json);
                } else if json
                    .get(input.wrapping_add(1 as usize))
                    .copied()
                    .is_some_and(|character| character as ::core::ffi::c_int == '*' as i32)
                {
                    skip_multiline_comment(&mut input, json);
                } else {
                    input = input.wrapping_add(1 as usize);
                }
            }
            34 => {
                minify_string(&mut input, &mut output, json);
            }
            _ => {
                json[output] = json[input];
                input = input.wrapping_add(1 as usize);
                output = output.wrapping_add(1 as usize);
            }
        }
    }
    if let Some(terminator) = json.get_mut(output) {
        *terminator = '\0' as i32 as ::core::ffi::c_char;
    }
}
#[export_name = "cJSON_Minify"]
pub unsafe extern "C" fn cJSON_Minify_ffi(mut json: *mut ::core::ffi::c_char) {
    let json = if json.is_null() {
        None
    } else {
        let length = strlen(json);
        Some(unsafe { ::core::slice::from_raw_parts_mut(json, length.wrapping_add(1)) })
    };
    cJSON_Minify(json)
}
fn cjson_is_type(item: Option<&cJSON>, item_type: ::core::ffi::c_int) -> cJSON_bool {
    match item {
        Some(item) => {
            ((item.type_0 & 0xff as ::core::ffi::c_int) == item_type) as ::core::ffi::c_int
        }
        None => false_0,
    }
}
pub fn cJSON_IsInvalid(item: Option<&cJSON>) -> cJSON_bool {
    cjson_is_type(item, cJSON_Invalid)
}
#[export_name = "cJSON_IsInvalid"]
pub unsafe extern "C" fn cJSON_IsInvalid_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsInvalid(unsafe { item.as_ref() })
}
pub fn cJSON_IsFalse(item: Option<&cJSON>) -> cJSON_bool {
    cjson_is_type(item, cJSON_False)
}
#[export_name = "cJSON_IsFalse"]
pub unsafe extern "C" fn cJSON_IsFalse_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsFalse(unsafe { item.as_ref() })
}
pub fn cJSON_IsTrue(item: Option<&cJSON>) -> cJSON_bool {
    cjson_is_type(item, cJSON_True)
}
#[export_name = "cJSON_IsTrue"]
pub unsafe extern "C" fn cJSON_IsTrue_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsTrue(unsafe { item.as_ref() })
}
pub fn cJSON_IsBool(item: Option<&cJSON>) -> cJSON_bool {
    match item {
        Some(item) => {
            (item.type_0 & (cJSON_True | cJSON_False) != 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int
        }
        None => false_0,
    }
}
#[export_name = "cJSON_IsBool"]
pub unsafe extern "C" fn cJSON_IsBool_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsBool(unsafe { item.as_ref() })
}
pub fn cJSON_IsNull(item: Option<&cJSON>) -> cJSON_bool {
    cjson_is_type(item, cJSON_NULL)
}
#[export_name = "cJSON_IsNull"]
pub unsafe extern "C" fn cJSON_IsNull_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsNull(unsafe { item.as_ref() })
}
pub fn cJSON_IsNumber(item: Option<&cJSON>) -> cJSON_bool {
    cjson_is_type(item, cJSON_Number)
}
#[export_name = "cJSON_IsNumber"]
pub unsafe extern "C" fn cJSON_IsNumber_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsNumber(unsafe { item.as_ref() })
}
pub fn cJSON_IsString(item: Option<&cJSON>) -> cJSON_bool {
    cjson_is_type(item, cJSON_String)
}
#[export_name = "cJSON_IsString"]
pub unsafe extern "C" fn cJSON_IsString_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsString(unsafe { item.as_ref() })
}
pub fn cJSON_IsArray(item: Option<&cJSON>) -> cJSON_bool {
    cjson_is_type(item, cJSON_Array)
}
#[export_name = "cJSON_IsArray"]
pub unsafe extern "C" fn cJSON_IsArray_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsArray(unsafe { item.as_ref() })
}
pub fn cJSON_IsObject(item: Option<&cJSON>) -> cJSON_bool {
    cjson_is_type(item, cJSON_Object)
}
#[export_name = "cJSON_IsObject"]
pub unsafe extern "C" fn cJSON_IsObject_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsObject(unsafe { item.as_ref() })
}
pub fn cJSON_IsRaw(item: Option<&cJSON>) -> cJSON_bool {
    cjson_is_type(item, cJSON_Raw)
}
#[export_name = "cJSON_IsRaw"]
pub unsafe extern "C" fn cJSON_IsRaw_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsRaw(unsafe { item.as_ref() })
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
            if a.valuestring.borrow().is_null() || b.valuestring.borrow().is_null() {
                return false_0;
            }
            let Some(a_value) = print_string(a, CjsonStringSlot::Value) else {
                return false_0;
            };
            let Some(b_value) = print_string(b, CjsonStringSlot::Value) else {
                return false_0;
            };
            if a_value.to_bytes() == b_value.to_bytes() {
                return true_0;
            }
            return false_0;
        }
        cJSON_Array => {
            let mut index = 0 as size_t;
            loop {
                match (
                    get_array_item(Some(a), index),
                    get_array_item(Some(b), index),
                ) {
                    (Some(a_element), Some(b_element)) => {
                        if cJSON_Compare(Some(a_element), Some(b_element), case_sensitive) == 0 {
                            return false_0;
                        }
                    }
                    (None, None) => return true_0,
                    _ => return false_0,
                }
                index = index.wrapping_add(1);
            }
        }
        cJSON_Object => {
            let mut index = 0 as size_t;
            while let Some(a_element) = get_array_item(Some(a), index) {
                let Some(a_key) = print_string(a_element, CjsonStringSlot::ObjectKey) else {
                    return false_0;
                };
                let Some(b_element) = a_key
                    .with_cstr(|a_key| get_object_item(Some(b), Some(a_key), case_sensitive))
                    .flatten()
                else {
                    return false_0;
                };
                if cJSON_Compare(Some(a_element), Some(b_element), case_sensitive) == 0 {
                    return false_0;
                }
                index = index.wrapping_add(1);
            }

            index = 0 as size_t;
            while let Some(b_element) = get_array_item(Some(b), index) {
                let Some(b_key) = print_string(b_element, CjsonStringSlot::ObjectKey) else {
                    return false_0;
                };
                let Some(a_element) = b_key
                    .with_cstr(|b_key| get_object_item(Some(a), Some(b_key), case_sensitive))
                    .flatten()
                else {
                    return false_0;
                };
                if cJSON_Compare(Some(b_element), Some(a_element), case_sensitive) == 0 {
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
    cJSON_Compare(unsafe { a.as_ref() }, unsafe { b.as_ref() }, case_sensitive)
}
#[export_name = "cJSON_malloc"]
pub unsafe extern "C" fn cJSON_malloc_ffi(mut size: size_t) -> *mut ::core::ffi::c_void {
    let hooks = global_hooks_snapshot();
    return hooks.allocate.expect("non-null function pointer")(size);
}
#[export_name = "cJSON_free"]
pub unsafe extern "C" fn cJSON_free_ffi(mut object: *mut ::core::ffi::c_void) {
    let hooks = global_hooks_snapshot();
    hooks.deallocate.expect("non-null function pointer")(object);
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __DBL_EPSILON__: ::core::ffi::c_double = 2.2204460492503131e-16f64;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const INT_MIN: ::core::ffi::c_int = -2147483647 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
pub const DBL_EPSILON: ::core::ffi::c_double = __DBL_EPSILON__;
