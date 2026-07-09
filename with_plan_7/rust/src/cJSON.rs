use ::core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use ::num_bigint::BigUint;
use ::num_traits::One;
use ::std::sync::RwLock;

unsafe extern "C" {
    safe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    safe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
#[repr(C)]
pub struct cJSON {
    pub next: AtomicPtr<cJSON>,
    pub prev: AtomicPtr<cJSON>,
    pub child: AtomicPtr<cJSON>,
    pub type_0: ::core::ffi::c_int,
    pub valuestring: ValueStringStorage,
    pub valuestring_len: size_t,
    pub valueint: ::core::ffi::c_int,
    pub valuedouble: ::core::ffi::c_double,
    pub string_bytes: Option<Vec<::core::ffi::c_uchar>>,
}
impl Clone for cJSON {
    fn clone(&self) -> Self {
        Self {
            next: AtomicPtr::new(self.next.load(Ordering::Relaxed)),
            prev: AtomicPtr::new(self.prev.load(Ordering::Relaxed)),
            child: AtomicPtr::new(self.child.load(Ordering::Relaxed)),
            type_0: self.type_0,
            valuestring: self.valuestring.clone(),
            valuestring_len: self.valuestring_len,
            valueint: self.valueint,
            valuedouble: self.valuedouble,
            string_bytes: self.string_bytes.clone(),
        }
    }
}
struct AllocatedNode {
    address: usize,
    _node: Box<cJSON>,
}
#[derive(Clone)]
pub enum ValueStringStorage {
    None,
    Owned(Vec<::core::ffi::c_uchar>),
    Borrowed(&'static ::core::ffi::CStr),
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
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parse_buffer<'a> {
    pub content: Option<&'a [::core::ffi::c_uchar]>,
    pub length: size_t,
    pub offset: size_t,
    pub depth: size_t,
    pub hooks: internal_hooks,
}
pub enum printbuffer_storage<'a> {
    Owned(Vec<::core::ffi::c_char>),
    Borrowed(&'a mut [::core::ffi::c_char]),
}
pub struct printbuffer<'a> {
    pub storage: printbuffer_storage<'a>,
    pub offset: size_t,
    pub depth: size_t,
    pub noalloc: cJSON_bool,
    pub format: cJSON_bool,
    pub hooks: internal_hooks,
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
const _: () =
    assert!(CJSON_VERSION_MAJOR == 1 && CJSON_VERSION_MINOR == 7 && CJSON_VERSION_PATCH == 19);
static CJSON_VERSION_STRING: [::core::ffi::c_uchar; 7] = [b'1', b'.', b'7', b'.', b'1', b'9', 0];
static global_error_json: AtomicUsize = AtomicUsize::new(0);
static global_error_position: AtomicUsize = AtomicUsize::new(0);
fn with_allocated_node<T>(address: usize, default: T, read_node: impl FnOnce(&cJSON) -> T) -> T {
    let nodes = match allocated_nodes.read() {
        Ok(nodes) => nodes,
        Err(poisoned) => poisoned.into_inner(),
    };
    let Some(node) = nodes.iter().find(|node| node.address == address) else {
        return default;
    };
    read_node(node._node.as_ref())
}
fn with_allocated_node_mut<T>(
    address: usize,
    default: T,
    mutate_node: impl FnOnce(&mut cJSON) -> T,
) -> T {
    let mut nodes = match allocated_nodes.write() {
        Ok(nodes) => nodes,
        Err(poisoned) => poisoned.into_inner(),
    };
    let Some(node) = nodes.iter_mut().find(|node| node.address == address) else {
        return default;
    };
    mutate_node(node._node.as_mut())
}
fn with_two_allocated_nodes_mut<T>(
    first_address: usize,
    second_address: usize,
    default: T,
    mutate_nodes: impl FnOnce(&mut cJSON, &mut cJSON) -> T,
) -> T {
    if first_address == second_address {
        return default;
    }
    let mut nodes = match allocated_nodes.write() {
        Ok(nodes) => nodes,
        Err(poisoned) => poisoned.into_inner(),
    };
    let Some(first_index) = nodes.iter().position(|node| node.address == first_address) else {
        return default;
    };
    let Some(second_index) = nodes.iter().position(|node| node.address == second_address) else {
        return default;
    };
    if first_index < second_index {
        let (left, right) = nodes.split_at_mut(second_index);
        mutate_nodes(
            left[first_index]._node.as_mut(),
            right[0]._node.as_mut(),
        )
    } else {
        let (left, right) = nodes.split_at_mut(first_index);
        mutate_nodes(
            right[0]._node.as_mut(),
            left[second_index]._node.as_mut(),
        )
    }
}
pub fn cJSON_GetErrorPtr() -> Option<usize> {
    let json = global_error_json.load(Ordering::SeqCst);
    if json == 0 {
        return None;
    }
    let position = global_error_position.load(Ordering::SeqCst);
    Some(json.wrapping_add(position))
}
#[export_name = "cJSON_GetErrorPtr"]
pub unsafe extern "C" fn cJSON_GetErrorPtr_ffi() -> *const ::core::ffi::c_char {
    match cJSON_GetErrorPtr() {
        Some(error_ptr) => error_ptr as *const ::core::ffi::c_char,
        None => ::core::ptr::null::<::core::ffi::c_char>(),
    }
}
pub fn cJSON_GetStringValue(item: Option<&cJSON>) -> Option<&[::core::ffi::c_uchar]> {
    match item {
        Some(item) if cJSON_IsString(Some(item)) != 0 => valuestring_bytes_with_nul(item),
        _ => None,
    }
}
#[export_name = "cJSON_GetStringValue"]
pub unsafe extern "C" fn cJSON_GetStringValue_ffi(item: *const cJSON) -> *mut ::core::ffi::c_char {
    match cJSON_GetStringValue(item.as_ref()) {
        Some(value) => value.as_ptr().cast_mut().cast::<::core::ffi::c_char>(),
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
    cJSON_GetNumberValue(item.as_ref())
}
pub fn cJSON_Version() -> &'static ::core::ffi::CStr {
    ::core::ffi::CStr::from_bytes_with_nul(&CJSON_VERSION_STRING)
        .expect("version string has trailing NUL")
}
#[export_name = "cJSON_Version"]
pub unsafe extern "C" fn cJSON_Version_ffi() -> *const ::core::ffi::c_char {
    cJSON_Version().as_ptr()
}
fn case_insensitive_strcmp(
    string1: Option<&::core::ffi::CStr>,
    string2: Option<&::core::ffi::CStr>,
) -> ::core::ffi::c_int {
    let (Some(string1), Some(string2)) = (string1, string2) else {
        return 1 as ::core::ffi::c_int;
    };
    if string1.as_ptr() == string2.as_ptr() {
        return 0 as ::core::ffi::c_int;
    }
    let bytes1 = string1.to_bytes_with_nul();
    let bytes2 = string2.to_bytes_with_nul();
    for (byte1, byte2) in bytes1.iter().zip(bytes2.iter()) {
        let lower1 = byte1.to_ascii_lowercase() as ::core::ffi::c_int;
        let lower2 = byte2.to_ascii_lowercase() as ::core::ffi::c_int;
        if lower1 != lower2 {
            return lower1 - lower2;
        }
        if *byte1 as ::core::ffi::c_int == '\0' as i32 {
            return 0 as ::core::ffi::c_int;
        }
    }
    0 as ::core::ffi::c_int
}
fn case_insensitive_bytes_cmp(
    string1: Option<&[::core::ffi::c_uchar]>,
    string2: Option<&[::core::ffi::c_uchar]>,
) -> ::core::ffi::c_int {
    let (Some(string1), Some(string2)) = (string1, string2) else {
        return 1 as ::core::ffi::c_int;
    };
    let compare_len = string1.len().max(string2.len()).wrapping_add(1);
    for index in 0..compare_len {
        let byte1 = string1.get(index).copied().unwrap_or(0);
        let byte2 = string2.get(index).copied().unwrap_or(0);
        let lower1 = byte1.to_ascii_lowercase() as ::core::ffi::c_int;
        let lower2 = byte2.to_ascii_lowercase() as ::core::ffi::c_int;
        if lower1 != lower2 {
            return lower1 - lower2;
        }
        if byte1 as ::core::ffi::c_int == '\0' as i32 {
            return 0 as ::core::ffi::c_int;
        }
    }
    0 as ::core::ffi::c_int
}
static global_hooks: RwLock<internal_hooks> = RwLock::new(internal_hooks {
    allocate: Some(malloc),
    deallocate: Some(free),
});
static allocated_nodes: RwLock<Vec<AllocatedNode>> = RwLock::new(Vec::new());
fn current_global_hooks() -> internal_hooks {
    match global_hooks.read() {
        Ok(hooks) => *hooks,
        Err(poisoned) => *poisoned.into_inner(),
    }
}
fn set_global_hooks(hooks: internal_hooks) {
    match global_hooks.write() {
        Ok(mut current_hooks) => *current_hooks = hooks,
        Err(poisoned) => {
            let mut current_hooks = poisoned.into_inner();
            *current_hooks = hooks;
        }
    }
}
pub fn cJSON_InitHooks(hooks: Option<&cJSON_Hooks>) {
    let mut new_hooks = internal_hooks {
        allocate: Some(malloc),
        deallocate: Some(free),
    };
    if let Some(hooks) = hooks {
        if hooks.malloc_fn.is_some() {
            new_hooks.allocate = hooks.malloc_fn;
        }
        if hooks.free_fn.is_some() {
            new_hooks.deallocate = hooks.free_fn;
        }
    }
    set_global_hooks(new_hooks);
}
#[export_name = "cJSON_InitHooks"]
pub unsafe extern "C" fn cJSON_InitHooks_ffi(mut hooks: *mut cJSON_Hooks) {
    cJSON_InitHooks(hooks.as_ref())
}
fn cJSON_New_Item(
    _hooks: &internal_hooks,
    type_0: ::core::ffi::c_int,
    valueint: ::core::ffi::c_int,
    valuedouble: ::core::ffi::c_double,
    configure: impl FnOnce(&mut cJSON),
) -> *mut cJSON {
    let mut item = cJSON {
        next: AtomicPtr::new(::core::ptr::null_mut::<cJSON>()),
        prev: AtomicPtr::new(::core::ptr::null_mut::<cJSON>()),
        child: AtomicPtr::new(::core::ptr::null_mut::<cJSON>()),
        type_0,
        valuestring: ValueStringStorage::None,
        valuestring_len: 0,
        valueint,
        valuedouble,
        string_bytes: None,
    };
    configure(&mut item);

    let mut node = Box::new(item);
    let node_ptr = ::core::ptr::from_mut::<cJSON>(node.as_mut());
    let allocated_node = AllocatedNode {
        address: node_ptr as usize,
        _node: node,
    };
    match allocated_nodes.write() {
        Ok(mut nodes) => nodes.push(allocated_node),
        Err(poisoned) => poisoned.into_inner().push(allocated_node),
    }
    node_ptr
}
fn default_item(_: &mut cJSON) {}
fn c_string_content_bytes(bytes: &[::core::ffi::c_uchar]) -> &[::core::ffi::c_uchar] {
    match bytes.iter().position(|byte| *byte == 0) {
        Some(nul_index) => &bytes[..nul_index],
        None => bytes,
    }
}
fn c_string_bytes_with_nul(bytes: &[::core::ffi::c_uchar]) -> &[::core::ffi::c_uchar] {
    match bytes.iter().position(|byte| *byte == 0) {
        Some(nul_index) => &bytes[..=nul_index],
        None => bytes,
    }
}
fn make_c_string_storage(bytes: &[::core::ffi::c_uchar]) -> Option<Vec<::core::ffi::c_uchar>> {
    let mut storage = Vec::new();
    if storage.try_reserve_exact(bytes.len()).is_err() {
        return None;
    }
    storage.extend_from_slice(bytes);
    Some(storage)
}
fn install_valuestring_storage(item: &mut cJSON, storage: Vec<::core::ffi::c_uchar>) {
    item.valuestring_len = storage.len().saturating_sub(1);
    item.valuestring = ValueStringStorage::Owned(storage);
}
fn install_valuestring_reference(item: &mut cJSON, string: &'static ::core::ffi::CStr) {
    item.valuestring_len = string.to_bytes().len();
    item.valuestring = ValueStringStorage::Borrowed(string);
}
fn set_valuestring_storage(item: &mut cJSON, bytes: &[::core::ffi::c_uchar]) -> cJSON_bool {
    match make_c_string_storage(bytes) {
        Some(storage) => {
            install_valuestring_storage(item, storage);
            true_0
        }
        None => false_0,
    }
}
fn valuestring_bytes(item: &cJSON) -> Option<&[::core::ffi::c_uchar]> {
    match &item.valuestring {
        ValueStringStorage::None => None,
        ValueStringStorage::Owned(storage) => Some(c_string_content_bytes(storage)),
        ValueStringStorage::Borrowed(string) => Some(string.to_bytes()),
    }
}
fn valuestring_bytes_with_nul(item: &cJSON) -> Option<&[::core::ffi::c_uchar]> {
    match &item.valuestring {
        ValueStringStorage::None => None,
        ValueStringStorage::Owned(storage) => Some(c_string_bytes_with_nul(storage)),
        ValueStringStorage::Borrowed(string) => Some(string.to_bytes_with_nul()),
    }
}
fn take_owned_valuestring_storage(item: &mut cJSON) -> Option<Vec<::core::ffi::c_uchar>> {
    item.valuestring_len = 0;
    match ::core::mem::replace(&mut item.valuestring, ValueStringStorage::None) {
        ValueStringStorage::Owned(storage) => Some(storage),
        ValueStringStorage::None | ValueStringStorage::Borrowed(_) => None,
    }
}
fn set_item_key_storage(item: &mut cJSON, bytes: &[::core::ffi::c_uchar]) -> cJSON_bool {
    match make_c_string_storage(bytes) {
        Some(storage) => {
            install_item_key_storage(item, storage);
            true_0
        }
        None => false_0,
    }
}
fn install_item_key_storage(item: &mut cJSON, storage: Vec<::core::ffi::c_uchar>) {
    item.string_bytes = Some(storage);
}
fn item_key_bytes(item: &cJSON) -> Option<&[::core::ffi::c_uchar]> {
    item.string_bytes.as_deref().map(c_string_content_bytes)
}
pub fn cJSON_Delete(item: *mut cJSON) {
    cJSON_Delete_address(item as usize);
}
fn cJSON_Delete_address(mut item_address: usize) {
    while item_address != 0 {
        let address = item_address;
        let (next, child, is_reference) =
            with_allocated_node_mut(address, (0usize, 0usize, true), |item_ref| {
                let next = item_ref.next.load(Ordering::Relaxed) as usize;
                let child = item_ref.child.load(Ordering::Relaxed) as usize;
                let is_reference = item_ref.type_0 & cJSON_IsReference != 0;
                item_ref.valuestring = ValueStringStorage::None;
                item_ref.valuestring_len = 0;
                item_ref.string_bytes = None;
                (next, child, is_reference)
            });
        if !is_reference && child != 0 {
            cJSON_Delete_address(child);
        }
        match allocated_nodes.write() {
            Ok(mut nodes) => {
                if let Some(index) = nodes.iter().position(|node| node.address == address) {
                    nodes.swap_remove(index);
                }
            }
            Err(poisoned) => {
                let mut nodes = poisoned.into_inner();
                if let Some(index) = nodes.iter().position(|node| node.address == address) {
                    nodes.swap_remove(index);
                }
            }
        }
        item_address = next;
    }
}
#[export_name = "cJSON_Delete"]
pub unsafe extern "C" fn cJSON_Delete_ffi(mut item: *mut cJSON) {
    cJSON_Delete(item)
}
fn number_valueint(number: ::core::ffi::c_double) -> ::core::ffi::c_int {
    if number >= INT_MAX as ::core::ffi::c_double {
        INT_MAX
    } else if number <= INT_MIN as ::core::ffi::c_double {
        INT_MIN
    } else {
        number as ::core::ffi::c_int
    }
}

fn parse_number(item: &mut cJSON, input_buffer: &mut parse_buffer<'_>) -> cJSON_bool {
    let mut i: size_t = 0 as size_t;
    let mut number_string_length: size_t = 0 as size_t;
    let content = match input_buffer.content {
        Some(content) => content,
        None => return false_0,
    };
    i = 0 as size_t;
    while input_buffer.offset.wrapping_add(i) < input_buffer.length {
        match content[input_buffer.offset.wrapping_add(i)] as ::core::ffi::c_int {
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
    let number_bytes =
        &content[input_buffer.offset..input_buffer.offset.wrapping_add(number_string_length)];
    let number_string = match ::core::str::from_utf8(number_bytes) {
        Ok(number_string) => number_string,
        Err(_) => return false_0,
    };
    let mut parsed_length = 0usize;
    if matches!(number_bytes.get(parsed_length), Some(b'-' | b'+')) {
        parsed_length = parsed_length.wrapping_add(1);
    }
    let digits_start = parsed_length;
    while matches!(number_bytes.get(parsed_length), Some(b'0'..=b'9')) {
        parsed_length = parsed_length.wrapping_add(1);
    }
    let has_integer_digits = parsed_length > digits_start;
    let mut has_fraction_digits = false;
    if matches!(number_bytes.get(parsed_length), Some(b'.')) {
        parsed_length = parsed_length.wrapping_add(1);
        let fraction_start = parsed_length;
        while matches!(number_bytes.get(parsed_length), Some(b'0'..=b'9')) {
            parsed_length = parsed_length.wrapping_add(1);
        }
        has_fraction_digits = parsed_length > fraction_start;
    }
    if !has_integer_digits && !has_fraction_digits {
        return false_0;
    }
    let significand_end = parsed_length;
    if matches!(number_bytes.get(parsed_length), Some(b'e' | b'E')) {
        parsed_length = parsed_length.wrapping_add(1);
        if matches!(number_bytes.get(parsed_length), Some(b'-' | b'+')) {
            parsed_length = parsed_length.wrapping_add(1);
        }
        let exponent_start = parsed_length;
        while matches!(number_bytes.get(parsed_length), Some(b'0'..=b'9')) {
            parsed_length = parsed_length.wrapping_add(1);
        }
        if parsed_length == exponent_start {
            parsed_length = significand_end;
        }
    }
    let number = match number_string[..parsed_length].parse::<::core::ffi::c_double>() {
        Ok(number) => number,
        Err(_) => return false_0,
    };
    item.valuedouble = number;
    item.valueint = number_valueint(number);
    item.type_0 = cJSON_Number;
    input_buffer.offset = input_buffer.offset.wrapping_add(parsed_length);
    return true_0;
}
pub fn cJSON_SetNumberHelper(
    object: &mut cJSON,
    mut number: ::core::ffi::c_double,
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
    valuestring: Option<&::core::ffi::CStr>,
) -> Option<&'a mut cJSON> {
    let (Some(object), Some(valuestring)) = (object, valuestring) else {
        return None;
    };
    if object.type_0 & cJSON_String == 0 || object.type_0 & cJSON_IsReference != 0 {
        return None;
    }
    let Some(old_valuestring) = valuestring_bytes_with_nul(object) else {
        return None;
    };
    let new_len = valuestring.to_bytes().len();
    let old_len = object.valuestring_len;
    if new_len <= old_len {
        let new_start = valuestring.as_ptr().addr();
        let new_end = new_start.wrapping_add(new_len);
        let old_start = old_valuestring.as_ptr().addr();
        let old_end = old_start.wrapping_add(old_len);
        if !(new_end < old_start || old_end < new_start) {
            return None;
        }
        let new_bytes = valuestring.to_bytes_with_nul();
        if let ValueStringStorage::Owned(storage) = &mut object.valuestring {
            if storage.len() < new_bytes.len() {
                return None;
            }
            storage[..new_bytes.len()].copy_from_slice(new_bytes);
        } else {
            return None;
        }
        object.valuestring_len = new_len;
        return Some(object);
    }
    let new_bytes = valuestring.to_bytes_with_nul();
    let Some(new_storage) = make_c_string_storage(new_bytes) else {
        return None;
    };
    install_valuestring_storage(object, new_storage);
    Some(object)
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
    match cJSON_SetValuestring(object.as_mut(), valuestring) {
        Some(object) => match valuestring_bytes_with_nul(object) {
            Some(value) => value.as_ptr().cast_mut().cast::<::core::ffi::c_char>(),
            None => ::core::ptr::null_mut(),
        },
        None => ::core::ptr::null_mut(),
    }
}
fn ensure<'a>(p: &'a mut printbuffer<'_>, needed: size_t) -> Option<&'a mut [::core::ffi::c_char]> {
    let length = match &p.storage {
        printbuffer_storage::Owned(buffer) => buffer.len(),
        printbuffer_storage::Borrowed(buffer) => buffer.len(),
    };
    if length > 0 as size_t && p.offset >= length {
        return None;
    }
    if needed > INT_MAX as size_t {
        return None;
    }
    let required = needed.wrapping_add(p.offset.wrapping_add(1 as size_t));
    if required > length {
        if p.noalloc != 0 {
            return None;
        }
        let newsize = if required > (INT_MAX / 2 as ::core::ffi::c_int) as size_t {
            if required <= INT_MAX as size_t {
                INT_MAX as size_t
            } else {
                return None;
            }
        } else {
            required.wrapping_mul(2 as size_t)
        };
        match &mut p.storage {
            printbuffer_storage::Owned(buffer) => {
                if buffer
                    .try_reserve_exact(newsize.wrapping_sub(buffer.len()))
                    .is_err()
                {
                    return None;
                }
                buffer.resize(newsize, 0);
            }
            printbuffer_storage::Borrowed(_) => return None,
        }
    }
    let start = p.offset;
    let end = start.wrapping_add(needed);
    match &mut p.storage {
        printbuffer_storage::Owned(buffer) => buffer.get_mut(start..end),
        printbuffer_storage::Borrowed(buffer) => buffer.get_mut(start..end),
    }
}
fn copy_bytes_to_chars(output: &mut [::core::ffi::c_char], bytes: &[u8]) {
    for (out, byte) in output.iter_mut().zip(bytes.iter()) {
        *out = *byte as ::core::ffi::c_char;
    }
}
fn compare_double(a: ::core::ffi::c_double, b: ::core::ffi::c_double) -> cJSON_bool {
    let maxVal: ::core::ffi::c_double = if a.abs() > b.abs() { a.abs() } else { b.abs() };
    return ((a - b).abs() <= maxVal * DBL_EPSILON) as ::core::ffi::c_int;
}
fn trim_c_g_fraction(mut number: String) -> String {
    if let Some(dot) = number.find('.') {
        let mut end = number.len();
        while end > dot + 1 && number.as_bytes()[end - 1] == b'0' {
            end -= 1;
        }
        if end == dot + 1 {
            end -= 1;
        }
        number.truncate(end);
    }
    number
}
fn f64_abs_parts(value: ::core::ffi::c_double) -> Option<(u64, i32)> {
    let bits = value.to_bits() & !(1u64 << 63);
    let exponent_bits = ((bits >> 52) & 0x7ff) as i32;
    let fraction = bits & ((1u64 << 52) - 1);
    if exponent_bits == 0 {
        if fraction == 0 {
            None
        } else {
            Some((fraction, -1074))
        }
    } else {
        Some((fraction | (1u64 << 52), exponent_bits - 1075))
    }
}
fn decimal_power(exponent: u32) -> BigUint {
    BigUint::from(10u8).pow(exponent)
}
fn scaled_decimal_rational(
    mantissa: u64,
    binary_exponent: i32,
    decimal_scale: i32,
) -> (BigUint, BigUint) {
    let mut numerator = BigUint::from(mantissa);
    let mut denominator = BigUint::one();
    if binary_exponent >= 0 {
        numerator <<= binary_exponent as usize;
    } else {
        denominator <<= (-binary_exponent) as usize;
    }
    if decimal_scale >= 0 {
        numerator *= decimal_power(decimal_scale as u32);
    } else {
        denominator *= decimal_power((-decimal_scale) as u32);
    }
    (numerator, denominator)
}
fn compare_abs_f64_to_decimal_power(
    mantissa: u64,
    binary_exponent: i32,
    decimal_exponent: i32,
) -> ::core::cmp::Ordering {
    let (numerator, denominator) =
        scaled_decimal_rational(mantissa, binary_exponent, -decimal_exponent);
    numerator.cmp(&denominator)
}
fn scientific_decimal_exponent(
    value: ::core::ffi::c_double,
    mantissa: u64,
    binary_exponent: i32,
) -> i32 {
    let mut exponent = value.abs().log10().floor() as i32;
    while compare_abs_f64_to_decimal_power(mantissa, binary_exponent, exponent + 1)
        != ::core::cmp::Ordering::Less
    {
        exponent += 1;
    }
    while compare_abs_f64_to_decimal_power(mantissa, binary_exponent, exponent)
        == ::core::cmp::Ordering::Less
    {
        exponent -= 1;
    }
    exponent
}
fn rounded_decimal_digits(mantissa: u64, binary_exponent: i32, decimal_scale: i32) -> BigUint {
    let (numerator, denominator) =
        scaled_decimal_rational(mantissa, binary_exponent, decimal_scale);
    let mut quotient = &numerator / &denominator;
    let remainder = numerator % &denominator;
    let twice_remainder = &remainder << 1usize;
    if twice_remainder > denominator
        || (twice_remainder == denominator && (&quotient % 2u8) == BigUint::one())
    {
        quotient += BigUint::one();
    }
    quotient
}
fn push_left_padded_digits(out: &mut String, digits: &str, width: usize) {
    for _ in digits.len()..width {
        out.push('0');
    }
    out.push_str(digits);
}
fn c_g_scientific(value: ::core::ffi::c_double, precision: usize) -> String {
    let Some((mantissa, binary_exponent)) = f64_abs_parts(value) else {
        return "0e+00".to_string();
    };
    let mut exponent = scientific_decimal_exponent(value, mantissa, binary_exponent);
    let decimal_scale = precision as i32 - 1 - exponent;
    let mut digits = rounded_decimal_digits(mantissa, binary_exponent, decimal_scale);
    let precision_power = decimal_power(precision as u32);
    if digits >= precision_power {
        digits /= 10u8;
        exponent += 1;
    }
    let digits = digits.to_str_radix(10);
    let mut padded_digits = String::with_capacity(precision);
    push_left_padded_digits(&mut padded_digits, &digits, precision);
    let mut scientific = String::with_capacity(precision + 8);
    if value.is_sign_negative() {
        scientific.push('-');
    }
    scientific.push(padded_digits.as_bytes()[0] as char);
    if precision > 1 {
        scientific.push('.');
        scientific.push_str(&padded_digits[1..]);
    }
    push_c_g_exponent(&mut scientific, exponent);
    scientific
}
fn c_g_fixed_from_scientific(scientific: &str, exponent: ::core::ffi::c_int) -> String {
    let mantissa = match scientific.find('e') {
        Some(exponent_index) => &scientific[..exponent_index],
        None => scientific,
    };
    let (sign, mantissa) = match mantissa.as_bytes().first().copied() {
        Some(b'-') => ("-", &mantissa[1..]),
        Some(b'+') => ("+", &mantissa[1..]),
        _ => ("", mantissa),
    };
    let mut digits = String::with_capacity(mantissa.len());
    for byte in mantissa.bytes() {
        if byte != b'.' {
            digits.push(byte as char);
        }
    }
    let decimal_position = exponent + 1;
    let mut fixed = String::with_capacity(sign.len() + digits.len() + 8);
    fixed.push_str(sign);
    if decimal_position <= 0 {
        fixed.push('0');
        fixed.push('.');
        for _ in 0..-decimal_position {
            fixed.push('0');
        }
        fixed.push_str(&digits);
    } else {
        let decimal_position = decimal_position as usize;
        if decimal_position >= digits.len() {
            fixed.push_str(&digits);
            for _ in digits.len()..decimal_position {
                fixed.push('0');
            }
        } else {
            fixed.push_str(&digits[..decimal_position]);
            fixed.push('.');
            fixed.push_str(&digits[decimal_position..]);
        }
    }
    trim_c_g_fraction(fixed)
}
fn parse_scientific_exponent(scientific: &str) -> ::core::ffi::c_int {
    let Some(exponent_index) = scientific.find('e') else {
        return 0;
    };
    scientific[exponent_index + 1..]
        .parse::<::core::ffi::c_int>()
        .unwrap_or(0)
}
fn push_unsigned_decimal(out: &mut String, mut value: u32, min_width: usize) {
    let mut digits = [0u8; 10];
    let mut len = 0usize;
    loop {
        digits[len] = b'0' + (value % 10) as u8;
        len += 1;
        value /= 10;
        if value == 0 {
            break;
        }
    }
    for _ in len..min_width {
        out.push('0');
    }
    for index in (0..len).rev() {
        out.push(digits[index] as char);
    }
}
fn push_c_g_exponent(out: &mut String, exponent: ::core::ffi::c_int) {
    out.push('e');
    let magnitude = if exponent < 0 {
        out.push('-');
        (0i64 - exponent as i64) as u32
    } else {
        out.push('+');
        exponent as u32
    };
    push_unsigned_decimal(out, magnitude, 2);
}
fn print_number(item: &cJSON, output_buffer: &mut printbuffer) -> cJSON_bool {
    let mut d: ::core::ffi::c_double = item.valuedouble;
    let mut rendered: String;
    if d != d || d - d != d - d && !(d != d) {
        rendered = "null".to_string();
    } else if d == item.valueint as ::core::ffi::c_double {
        rendered = item.valueint.to_string();
    } else {
        let mut precision_index = 0usize;
        let precisions = [15usize, 17usize];
        loop {
            let precision = precisions[precision_index];
            if d == 0.0 {
                rendered = "0".to_string();
            } else {
                let scientific = c_g_scientific(d, precision);
                let exponent = parse_scientific_exponent(&scientific);
                if exponent < -4 || exponent >= precision as ::core::ffi::c_int {
                    if let Some(exponent_index) = scientific.find('e') {
                        rendered = trim_c_g_fraction(scientific[..exponent_index].to_string());
                        push_c_g_exponent(&mut rendered, exponent);
                    } else {
                        rendered = scientific;
                    }
                } else {
                    rendered = c_g_fixed_from_scientific(&scientific, exponent);
                }
            }
            if precision_index + 1 == precisions.len()
                || rendered
                    .parse::<::core::ffi::c_double>()
                    .map_or(false, |test| compare_double(test, d) != 0)
            {
                break;
            }
            precision_index += 1;
        }
    }
    let rendered = rendered.as_bytes();
    if rendered.len()
        > (::core::mem::size_of::<[::core::ffi::c_uchar; 26]>() as usize).wrapping_sub(1 as usize)
    {
        return false_0;
    }
    let Some(output) = ensure(
        output_buffer,
        rendered
            .len()
            .wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 1]>() as size_t),
    ) else {
        return false_0;
    };
    copy_bytes_to_chars(&mut output[..rendered.len()], rendered);
    output[rendered.len()] = '\0' as i32 as ::core::ffi::c_char;
    output_buffer.offset = output_buffer.offset.wrapping_add(rendered.len() as size_t);
    return true_0;
}
fn parse_hex4(input: &[::core::ffi::c_uchar]) -> ::core::ffi::c_uint {
    let mut h: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    if input.len() < 4 {
        return 0 as ::core::ffi::c_uint;
    }
    for (i, byte) in input.iter().take(4).enumerate() {
        let digit = match *byte {
            b'0'..=b'9' => {
                (*byte as ::core::ffi::c_uint).wrapping_sub('0' as i32 as ::core::ffi::c_uint)
            }
            b'A'..=b'F' => (10 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(*byte as ::core::ffi::c_uint)
                .wrapping_sub('A' as i32 as ::core::ffi::c_uint),
            b'a'..=b'f' => (10 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(*byte as ::core::ffi::c_uint)
                .wrapping_sub('a' as i32 as ::core::ffi::c_uint),
            _ => return 0 as ::core::ffi::c_uint,
        };
        h = h.wrapping_add(digit);
        if i < 3 {
            h = h << 4 as ::core::ffi::c_int;
        }
    }
    return h;
}
fn utf16_literal_to_utf8(
    input: &[::core::ffi::c_uchar],
    output: &mut [::core::ffi::c_uchar],
) -> Option<(usize, usize)> {
    if input.len() < 6 {
        return None;
    }
    let first_code = parse_hex4(&input[2..]);
    if first_code >= 0xdc00 as ::core::ffi::c_uint && first_code <= 0xdfff as ::core::ffi::c_uint {
        return None;
    }
    let (mut codepoint, sequence_length) = if first_code >= 0xd800 as ::core::ffi::c_uint
        && first_code <= 0xdbff as ::core::ffi::c_uint
    {
        if input.len() < 12 || input[6] as ::core::ffi::c_int != '\\' as i32 {
            return None;
        }
        if input[7] as ::core::ffi::c_int != 'u' as i32 {
            return None;
        }
        let second_code = parse_hex4(&input[8..]);
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
            12usize,
        )
    } else {
        (first_code as ::core::ffi::c_ulong, 6usize)
    };
    let (utf8_length, first_byte_mark) = if codepoint < 0x80 as ::core::ffi::c_ulong {
        (1usize, 0 as ::core::ffi::c_ulong)
    } else if codepoint < 0x800 as ::core::ffi::c_ulong {
        (2usize, 0xc0 as ::core::ffi::c_ulong)
    } else if codepoint < 0x10000 as ::core::ffi::c_ulong {
        (3usize, 0xe0 as ::core::ffi::c_ulong)
    } else if codepoint <= 0x10ffff as ::core::ffi::c_ulong {
        (4usize, 0xf0 as ::core::ffi::c_ulong)
    } else {
        return None;
    };
    if output.len() < utf8_length {
        return None;
    }
    for utf8_position in (1..utf8_length).rev() {
        output[utf8_position] = ((codepoint | 0x80 as ::core::ffi::c_ulong)
            & 0xbf as ::core::ffi::c_ulong) as ::core::ffi::c_uchar;
        codepoint >>= 6 as ::core::ffi::c_int;
    }
    if utf8_length > 1 {
        output[0] =
            ((codepoint | first_byte_mark) & 0xff as ::core::ffi::c_ulong) as ::core::ffi::c_uchar;
    } else {
        output[0] = (codepoint & 0x7f as ::core::ffi::c_ulong) as ::core::ffi::c_uchar;
    }
    Some((sequence_length, utf8_length))
}

fn parse_buffer_byte(buffer: &parse_buffer<'_>, offset: size_t) -> Option<::core::ffi::c_uchar> {
    buffer
        .content
        .and_then(|content| content.get(offset).copied())
}

fn parse_buffer_starts_with(buffer: &parse_buffer<'_>, offset: size_t, prefix: &[u8]) -> bool {
    buffer
        .content
        .and_then(|content| content.get(offset..offset.wrapping_add(prefix.len())))
        == Some(prefix)
}

fn parse_string(item: &mut cJSON, buffer: &mut parse_buffer<'_>) -> cJSON_bool {
    let mut input_index = buffer.offset.wrapping_add(1);
    let initial_input_index = input_index;
    let content = match buffer.content {
        Some(content) => content,
        None => {
            buffer.offset = input_index;
            return false_0;
        }
    };
    if buffer.offset >= content.len() {
        buffer.offset = input_index;
        return false_0;
    }
    if content.get(buffer.offset).copied() != Some('"' as i32 as ::core::ffi::c_uchar) {
        buffer.offset = input_index;
        return false_0;
    }
    let mut input_end = input_index;
    let mut skipped_bytes: size_t = 0 as size_t;
    while input_end < buffer.length && content[input_end] as ::core::ffi::c_int != '"' as i32 {
        if content[input_end] as ::core::ffi::c_int == '\\' as i32 {
            if input_end.wrapping_add(1 as size_t) >= buffer.length {
                buffer.offset = input_index;
                return false_0;
            }
            skipped_bytes = skipped_bytes.wrapping_add(1);
            input_end = input_end.wrapping_add(1);
        }
        input_end = input_end.wrapping_add(1);
    }
    if input_end >= buffer.length || content[input_end] as ::core::ffi::c_int != '"' as i32 {
        buffer.offset = input_index;
        return false_0;
    }
    let allocation_length = input_end
        .wrapping_sub(buffer.offset)
        .wrapping_sub(skipped_bytes);
    let allocation_size = allocation_length
        .wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 1]>() as size_t);
    let mut decoded: Vec<::core::ffi::c_uchar> = Vec::new();
    if decoded.try_reserve_exact(allocation_size).is_err() {
        buffer.offset = initial_input_index;
        return false_0;
    }
    let mut output_index: size_t = 0 as size_t;
    while input_index < input_end {
        if content[input_index] as ::core::ffi::c_int != '\\' as i32 {
            decoded.push(content[input_index]);
            output_index = output_index.wrapping_add(1);
            input_index = input_index.wrapping_add(1);
        } else {
            let mut sequence_length: size_t = 2 as size_t;
            let escaped_index = input_index.wrapping_add(1 as size_t);
            if escaped_index >= input_end {
                buffer.offset = input_index;
                return false_0;
            }
            match content[escaped_index] as ::core::ffi::c_int {
                98 => {
                    decoded.push('\u{8}' as i32 as ::core::ffi::c_uchar);
                    output_index = output_index.wrapping_add(1);
                }
                102 => {
                    decoded.push('\u{c}' as i32 as ::core::ffi::c_uchar);
                    output_index = output_index.wrapping_add(1);
                }
                110 => {
                    decoded.push('\n' as i32 as ::core::ffi::c_uchar);
                    output_index = output_index.wrapping_add(1);
                }
                114 => {
                    decoded.push('\r' as i32 as ::core::ffi::c_uchar);
                    output_index = output_index.wrapping_add(1);
                }
                116 => {
                    decoded.push('\t' as i32 as ::core::ffi::c_uchar);
                    output_index = output_index.wrapping_add(1);
                }
                34 | 92 | 47 => {
                    decoded.push(content[escaped_index]);
                    output_index = output_index.wrapping_add(1);
                }
                117 => {
                    let mut utf8_buffer = [0 as ::core::ffi::c_uchar; 4];
                    if let Some((consumed, written)) =
                        utf16_literal_to_utf8(&content[input_index..input_end], &mut utf8_buffer)
                    {
                        sequence_length = consumed;
                        decoded.extend_from_slice(&utf8_buffer[..written]);
                        output_index = output_index.wrapping_add(written);
                    } else {
                        buffer.offset = input_index;
                        return false_0;
                    }
                }
                _ => {
                    buffer.offset = input_index;
                    return false_0;
                }
            }
            input_index = input_index.wrapping_add(sequence_length);
        }
    }
    if output_index >= allocation_size {
        buffer.offset = input_index;
        return false_0;
    }
    decoded.push('\0' as i32 as ::core::ffi::c_uchar);
    if decoded.len() > allocation_size {
        buffer.offset = input_index;
        return false_0;
    }
    item.type_0 = cJSON_String;
    install_valuestring_storage(item, decoded);
    item.valuestring_len = output_index;
    buffer.offset = input_end.wrapping_add(1);
    return true_0;
}
fn print_string_ptr(
    input: Option<&[::core::ffi::c_uchar]>,
    output_buffer: &mut printbuffer,
) -> cJSON_bool {
    let input = input.unwrap_or(&[]);
    let mut escape_characters: size_t = 0 as size_t;
    for byte in input {
        match *byte as ::core::ffi::c_int {
            34 | 92 | 8 | 12 | 10 | 13 | 9 => {
                escape_characters = escape_characters.wrapping_add(1);
            }
            _ => {
                if (*byte as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
                    escape_characters = escape_characters.wrapping_add(5 as size_t);
                }
            }
        }
    }
    let output_length = input.len().wrapping_add(escape_characters);
    let needed =
        output_length.wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t);
    let mut escaped = Vec::new();
    if escaped.try_reserve_exact(needed).is_err() {
        return false_0;
    }
    escaped.push('"' as i32 as ::core::ffi::c_char);
    for byte in input {
        if *byte as ::core::ffi::c_int > 31 as ::core::ffi::c_int
            && *byte as ::core::ffi::c_int != '"' as i32
            && *byte as ::core::ffi::c_int != '\\' as i32
        {
            escaped.push(*byte as ::core::ffi::c_char);
        } else {
            escaped.push('\\' as i32 as ::core::ffi::c_char);
            match *byte as ::core::ffi::c_int {
                92 => escaped.push('\\' as i32 as ::core::ffi::c_char),
                34 => escaped.push('"' as i32 as ::core::ffi::c_char),
                8 => escaped.push('b' as i32 as ::core::ffi::c_char),
                12 => escaped.push('f' as i32 as ::core::ffi::c_char),
                10 => escaped.push('n' as i32 as ::core::ffi::c_char),
                13 => escaped.push('r' as i32 as ::core::ffi::c_char),
                9 => escaped.push('t' as i32 as ::core::ffi::c_char),
                _ => {
                    const HEX: &[u8; 16] = b"0123456789abcdef";
                    escaped.push('u' as i32 as ::core::ffi::c_char);
                    escaped.push('0' as i32 as ::core::ffi::c_char);
                    escaped.push('0' as i32 as ::core::ffi::c_char);
                    escaped.push(HEX[((*byte >> 4) & 0xf) as usize] as ::core::ffi::c_char);
                    escaped.push(HEX[(*byte & 0xf) as usize] as ::core::ffi::c_char);
                }
            }
        }
    }
    escaped.push('"' as i32 as ::core::ffi::c_char);
    escaped.push('\0' as i32 as ::core::ffi::c_char);
    let Some(output) = ensure(output_buffer, needed) else {
        return false_0;
    };
    output[..escaped.len()].copy_from_slice(&escaped);
    output_buffer.offset = output_buffer
        .offset
        .wrapping_add(escaped.len().wrapping_sub(1) as size_t);
    return true_0;
}
fn print_string(item: &cJSON, p: &mut printbuffer) -> cJSON_bool {
    return print_string_ptr(valuestring_bytes(item), p);
}
fn buffer_skip_whitespace(buffer: &mut parse_buffer<'_>) {
    let Some(content) = buffer.content else {
        return;
    };
    if buffer.offset.wrapping_add(0 as size_t) >= buffer.length {
        return;
    }
    while buffer.offset.wrapping_add(0 as size_t) < buffer.length
        && content[buffer.offset] as ::core::ffi::c_int <= 32 as ::core::ffi::c_int
    {
        buffer.offset = buffer.offset.wrapping_add(1);
    }
    if buffer.offset == buffer.length {
        buffer.offset = buffer.offset.wrapping_sub(1);
    }
}
fn skip_utf8_bom(buffer: &mut parse_buffer<'_>) {
    let content = match buffer.content {
        Some(content) if buffer.offset == 0 as size_t => content,
        _ => return,
    };
    if buffer.offset.wrapping_add(4 as size_t) < buffer.length
        && content.get(buffer.offset..buffer.offset.wrapping_add(3 as size_t))
            == Some(&[0xef, 0xbb, 0xbf])
    {
        buffer.offset = buffer.offset.wrapping_add(3 as size_t);
    }
}
pub fn cJSON_ParseWithOpts(
    value: Option<&::core::ffi::CStr>,
    return_parse_end: Option<&mut size_t>,
    require_null_terminated: cJSON_bool,
) -> Option<::core::ptr::NonNull<cJSON>> {
    let value = value?;
    cJSON_ParseWithLengthOpts(
        Some(value.to_bytes_with_nul()),
        return_parse_end,
        require_null_terminated,
    )
}
#[export_name = "cJSON_ParseWithOpts"]
pub unsafe extern "C" fn cJSON_ParseWithOpts_ffi(
    value: *const ::core::ffi::c_char,
    return_parse_end: *mut *const ::core::ffi::c_char,
    require_null_terminated: cJSON_bool,
) -> *mut cJSON {
    let value_cstr = if value.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(value))
    };
    let mut return_parse_end_offset = 0 as size_t;
    let return_parse_end_ref = if return_parse_end.is_null() {
        None
    } else {
        Some(&mut return_parse_end_offset)
    };
    let item = cJSON_ParseWithOpts(value_cstr, return_parse_end_ref, require_null_terminated);
    if !value.is_null() && !return_parse_end.is_null() {
        *return_parse_end = value.wrapping_add(return_parse_end_offset);
    }
    match item {
        Some(item) => item.as_ptr(),
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_ParseWithLengthOpts(
    value: Option<&[::core::ffi::c_uchar]>,
    mut return_parse_end: Option<&mut size_t>,
    require_null_terminated: cJSON_bool,
) -> Option<::core::ptr::NonNull<cJSON>> {
    let mut c2rust_current_block: u64;
    let mut buffer: parse_buffer<'_> = parse_buffer {
        content: None,
        length: 0 as size_t,
        offset: 0 as size_t,
        depth: 0 as size_t,
        hooks: internal_hooks {
            allocate: None,
            deallocate: None,
        },
    };
    let mut item: Option<::core::ptr::NonNull<cJSON>> = None;
    global_error_json.store(0, Ordering::SeqCst);
    global_error_position.store(0 as size_t, Ordering::SeqCst);
    if let Some(content) = value {
        if !content.is_empty() {
            buffer.content = Some(content);
            buffer.length = content.len();
            buffer.offset = 0 as size_t;
            let hooks = current_global_hooks();
            buffer.hooks = hooks;
            let mut parsed = false;
            item = ::core::ptr::NonNull::new(cJSON_New_Item(
                &hooks,
                cJSON_Invalid,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_double,
                |item_ref| {
                    skip_utf8_bom(&mut buffer);
                    buffer_skip_whitespace(&mut buffer);
                    parsed = parse_value(item_ref, &mut buffer) != 0;
                },
            ));
            if let Some(item_ptr) = item {
                if parsed {
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
                            if let Some(return_parse_end) = return_parse_end.as_mut() {
                                **return_parse_end = buffer.offset;
                            }
                            return Some(item_ptr);
                        }
                    }
                }
            }
        }
    }
    if let Some(item) = item {
        cJSON_Delete(item.as_ptr());
    }
    if let Some(content) = value {
        let local_error_json = content.as_ptr() as *const ::core::ffi::c_char;
        let mut local_error_position = 0 as size_t;
        if buffer.offset < buffer.length {
            local_error_position = buffer.offset;
        } else if buffer.length > 0 as size_t {
            local_error_position = buffer.length.wrapping_sub(1 as size_t);
        }
        if let Some(return_parse_end) = return_parse_end.as_mut() {
            **return_parse_end = local_error_position;
        }
        global_error_position.store(local_error_position, Ordering::SeqCst);
        global_error_json.store(local_error_json as usize, Ordering::SeqCst);
    }
    None
}
#[export_name = "cJSON_ParseWithLengthOpts"]
pub unsafe extern "C" fn cJSON_ParseWithLengthOpts_ffi(
    value: *const ::core::ffi::c_char,
    buffer_length: size_t,
    return_parse_end: *mut *const ::core::ffi::c_char,
    require_null_terminated: cJSON_bool,
) -> *mut cJSON {
    let value_slice = if value.is_null() {
        None
    } else {
        Some(::core::slice::from_raw_parts(
            value as *const ::core::ffi::c_uchar,
            buffer_length,
        ))
    };
    let mut return_parse_end_offset = 0 as size_t;
    let return_parse_end_ref = if return_parse_end.is_null() {
        None
    } else {
        Some(&mut return_parse_end_offset)
    };
    let item =
        cJSON_ParseWithLengthOpts(value_slice, return_parse_end_ref, require_null_terminated);
    if !value.is_null() && !return_parse_end.is_null() {
        *return_parse_end = value.wrapping_add(return_parse_end_offset);
    }
    match item {
        Some(item) => item.as_ptr(),
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_Parse(value: Option<&::core::ffi::CStr>) -> Option<::core::ptr::NonNull<cJSON>> {
    cJSON_ParseWithOpts(value, None, 0 as cJSON_bool)
}
#[export_name = "cJSON_Parse"]
pub unsafe extern "C" fn cJSON_Parse_ffi(value: *const ::core::ffi::c_char) -> *mut cJSON {
    let value_cstr = if value.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(value))
    };
    match cJSON_Parse(value_cstr) {
        Some(item) => item.as_ptr(),
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_ParseWithLength(
    value: Option<&[::core::ffi::c_uchar]>,
) -> Option<::core::ptr::NonNull<cJSON>> {
    cJSON_ParseWithLengthOpts(value, None, 0 as cJSON_bool)
}
#[export_name = "cJSON_ParseWithLength"]
pub unsafe extern "C" fn cJSON_ParseWithLength_ffi(
    value: *const ::core::ffi::c_char,
    buffer_length: size_t,
) -> *mut cJSON {
    let value_slice = if value.is_null() {
        None
    } else {
        Some(::core::slice::from_raw_parts(
            value as *const ::core::ffi::c_uchar,
            buffer_length,
        ))
    };
    match cJSON_ParseWithLength(value_slice) {
        Some(item) => item.as_ptr(),
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
fn print(
    item: Option<&cJSON>,
    mut format: cJSON_bool,
    initial_capacity: size_t,
) -> Option<Vec<::core::ffi::c_char>> {
    let mut storage = Vec::new();
    if storage.try_reserve_exact(initial_capacity).is_err() {
        return None;
    }
    storage.resize(initial_capacity, 0);
    let mut buffer = printbuffer {
        storage: printbuffer_storage::Owned(storage),
        offset: 0,
        depth: 0,
        noalloc: 0,
        format,
        hooks: current_global_hooks(),
    };
    if print_value(item, &mut buffer) == 0 {
        return None;
    }
    let copy_length = buffer.offset.wrapping_add(1 as size_t);
    let print_slice = match &buffer.storage {
        printbuffer_storage::Owned(storage) => storage.get(..copy_length)?,
        printbuffer_storage::Borrowed(_) => return None,
    };
    let mut printed = print_slice.to_vec();
    printed[buffer.offset] = '\0' as i32 as ::core::ffi::c_char;
    return Some(printed);
}
pub fn cJSON_Print(item: Option<&cJSON>) -> Option<Vec<::core::ffi::c_char>> {
    print(item, true_0, 256 as size_t)
}
#[export_name = "cJSON_Print"]
pub unsafe extern "C" fn cJSON_Print_ffi(mut item: *const cJSON) -> *mut ::core::ffi::c_char {
    let Some(print_slice) = cJSON_Print(item.as_ref()) else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    let hooks = current_global_hooks();
    let printed = hooks.allocate.expect("non-null function pointer")(print_slice.len() as size_t)
        as *mut ::core::ffi::c_char;
    if printed.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    ::core::ptr::copy_nonoverlapping(print_slice.as_ptr(), printed, print_slice.len());
    printed
}
pub fn cJSON_PrintUnformatted(item: Option<&cJSON>) -> Option<Vec<::core::ffi::c_char>> {
    print(item, false_0, 256 as size_t)
}
#[export_name = "cJSON_PrintUnformatted"]
pub unsafe extern "C" fn cJSON_PrintUnformatted_ffi(
    mut item: *const cJSON,
) -> *mut ::core::ffi::c_char {
    let Some(print_slice) = cJSON_PrintUnformatted(item.as_ref()) else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    let hooks = current_global_hooks();
    let printed = hooks.allocate.expect("non-null function pointer")(print_slice.len() as size_t)
        as *mut ::core::ffi::c_char;
    if printed.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    ::core::ptr::copy_nonoverlapping(print_slice.as_ptr(), printed, print_slice.len());
    printed
}
pub fn cJSON_PrintBuffered(
    item: Option<&cJSON>,
    mut prebuffer: ::core::ffi::c_int,
    mut fmt: cJSON_bool,
) -> Option<Vec<::core::ffi::c_char>> {
    if prebuffer < 0 as ::core::ffi::c_int {
        return None;
    }
    return print(item, fmt, prebuffer as size_t);
}
#[export_name = "cJSON_PrintBuffered"]
pub unsafe extern "C" fn cJSON_PrintBuffered_ffi(
    mut item: *const cJSON,
    mut prebuffer: ::core::ffi::c_int,
    mut fmt: cJSON_bool,
) -> *mut ::core::ffi::c_char {
    let Some(print_slice) = cJSON_PrintBuffered(item.as_ref(), prebuffer, fmt) else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    let hooks = current_global_hooks();
    let printed = hooks.allocate.expect("non-null function pointer")(print_slice.len() as size_t)
        as *mut ::core::ffi::c_char;
    if printed.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    ::core::ptr::copy_nonoverlapping(print_slice.as_ptr(), printed, print_slice.len());
    printed
}
pub fn cJSON_PrintPreallocated(
    item: Option<&cJSON>,
    buffer: &mut [::core::ffi::c_char],
    format: cJSON_bool,
) -> cJSON_bool {
    let mut p: printbuffer = printbuffer {
        storage: printbuffer_storage::Borrowed(buffer),
        offset: 0 as size_t,
        depth: 0 as size_t,
        noalloc: true_0,
        format,
        hooks: internal_hooks {
            allocate: None,
            deallocate: None,
        },
    };
    p.hooks = current_global_hooks();
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
    let buffer = ::core::slice::from_raw_parts_mut(buffer, length as size_t);
    cJSON_PrintPreallocated(item.as_ref(), buffer, format)
}
fn parse_value(item: &mut cJSON, input_buffer: &mut parse_buffer<'_>) -> cJSON_bool {
    if input_buffer.content.is_none() {
        return false_0;
    }
    if input_buffer.offset.wrapping_add(4 as size_t) <= input_buffer.length
        && parse_buffer_starts_with(input_buffer, input_buffer.offset, b"null")
    {
        item.type_0 = cJSON_NULL;
        input_buffer.offset = input_buffer.offset.wrapping_add(4 as size_t);
        return true_0;
    }
    if input_buffer.offset.wrapping_add(5 as size_t) <= input_buffer.length
        && parse_buffer_starts_with(input_buffer, input_buffer.offset, b"false")
    {
        item.type_0 = cJSON_False;
        input_buffer.offset = input_buffer.offset.wrapping_add(5 as size_t);
        return true_0;
    }
    if input_buffer.offset.wrapping_add(4 as size_t) <= input_buffer.length
        && parse_buffer_starts_with(input_buffer, input_buffer.offset, b"true")
    {
        item.type_0 = cJSON_True;
        item.valueint = 1 as ::core::ffi::c_int;
        input_buffer.offset = input_buffer.offset.wrapping_add(4 as size_t);
        return true_0;
    }
    if input_buffer.offset.wrapping_add(0 as size_t) < input_buffer.length
        && parse_buffer_byte(input_buffer, input_buffer.offset)
            == Some('"' as i32 as ::core::ffi::c_uchar)
    {
        return parse_string(item, input_buffer);
    }
    if input_buffer.offset.wrapping_add(0 as size_t) < input_buffer.length
        && matches!(
            parse_buffer_byte(input_buffer, input_buffer.offset),
            Some(b'-' | b'0'..=b'9')
        )
    {
        return parse_number(item, input_buffer);
    }
    if input_buffer.offset.wrapping_add(0 as size_t) < input_buffer.length
        && parse_buffer_byte(input_buffer, input_buffer.offset)
            == Some('[' as i32 as ::core::ffi::c_uchar)
    {
        return parse_array(item, input_buffer);
    }
    if input_buffer.offset.wrapping_add(0 as size_t) < input_buffer.length
        && parse_buffer_byte(input_buffer, input_buffer.offset)
            == Some('{' as i32 as ::core::ffi::c_uchar)
    {
        return parse_object(item, input_buffer);
    }
    return false_0;
}
fn print_value(item: Option<&cJSON>, output_buffer: &mut printbuffer) -> cJSON_bool {
    let Some(item) = item else {
        return false_0;
    };
    let bytes: &[::core::ffi::c_uchar] = match item.type_0 & 0xff as ::core::ffi::c_int {
        cJSON_NULL => b"null\0",
        cJSON_False => b"false\0",
        cJSON_True => b"true\0",
        cJSON_Number => return print_number(item, output_buffer),
        cJSON_Raw => {
            let Some(bytes) = valuestring_bytes_with_nul(item) else {
                return false_0;
            };
            bytes
        }
        cJSON_String => return print_string(item, output_buffer),
        cJSON_Array => return print_array(item, output_buffer),
        cJSON_Object => return print_object(item, output_buffer),
        _ => return false_0,
    };
    let Some(output) = ensure(output_buffer, bytes.len() as size_t) else {
        return false_0;
    };
    copy_bytes_to_chars(&mut output[..bytes.len()], bytes);
    output_buffer.offset = output_buffer
        .offset
        .wrapping_add(bytes.len().wrapping_sub(1) as size_t);
    true_0
}
fn parse_array(item: &mut cJSON, input_buffer: &mut parse_buffer<'_>) -> cJSON_bool {
    let mut c2rust_current_block: u64;
    let mut head: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut current_item: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if input_buffer.depth >= CJSON_NESTING_LIMIT as size_t {
        return false_0;
    }
    input_buffer.depth = input_buffer.depth.wrapping_add(1);
    if !(parse_buffer_byte(input_buffer, input_buffer.offset)
        != Some('[' as i32 as ::core::ffi::c_uchar))
    {
        input_buffer.offset = input_buffer.offset.wrapping_add(1);
        buffer_skip_whitespace(input_buffer);
        if input_buffer.offset.wrapping_add(0 as size_t) < input_buffer.length
            && parse_buffer_byte(input_buffer, input_buffer.offset)
                == Some(']' as i32 as ::core::ffi::c_uchar)
        {
            c2rust_current_block = 6773356538935231690;
        } else if input_buffer.offset.wrapping_add(0 as size_t) >= input_buffer.length {
            input_buffer.offset = input_buffer.offset.wrapping_sub(1);
            c2rust_current_block = 1336238348363633231;
        } else {
            input_buffer.offset = input_buffer.offset.wrapping_sub(1);
            loop {
                let mut parsed = false_0;
                let hooks = input_buffer.hooks;
                let new_item: *mut cJSON = cJSON_New_Item(
                    &hooks,
                    cJSON_Invalid,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_double,
                    |new_item_ref| {
                        input_buffer.offset = input_buffer.offset.wrapping_add(1);
                        buffer_skip_whitespace(input_buffer);
                        parsed = parse_value(new_item_ref, input_buffer);
                    },
                );
                if new_item.is_null() {
                    c2rust_current_block = 1336238348363633231;
                    break;
                }
                if parsed == 0 {
                    cJSON_Delete(new_item);
                    c2rust_current_block = 1336238348363633231;
                    break;
                }
                if head.is_null() {
                    head = new_item;
                    current_item = head;
                } else {
                    suffix_object(
                        Some(::core::ptr::NonNull::new(current_item).expect("current array item")),
                        Some(::core::ptr::NonNull::new(new_item).expect("new array item")),
                        true,
                    );
                    current_item = new_item;
                }
                buffer_skip_whitespace(input_buffer);
                if !(input_buffer.offset.wrapping_add(0 as size_t) < input_buffer.length
                    && parse_buffer_byte(input_buffer, input_buffer.offset)
                        == Some(',' as i32 as ::core::ffi::c_uchar))
                {
                    c2rust_current_block = 15089075282327824602;
                    break;
                }
            }
            match c2rust_current_block {
                1336238348363633231 => {}
                _ => {
                    if !(input_buffer.offset.wrapping_add(0 as size_t) < input_buffer.length)
                        || parse_buffer_byte(input_buffer, input_buffer.offset)
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
                input_buffer.depth = input_buffer.depth.wrapping_sub(1);
                if !head.is_null() {
                    suffix_object(
                        Some(::core::ptr::NonNull::new(current_item).expect("current array item")),
                        Some(::core::ptr::NonNull::new(head).expect("array head")),
                        false,
                    );
                }
                item.type_0 = cJSON_Array;
                item.child.store(head as *mut cJSON, Ordering::Relaxed);
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
    let mut index = 0 as size_t;
    let Some(output) = ensure(output_buffer, 1 as size_t) else {
        return false_0;
    };
    output[0] = '[' as i32 as ::core::ffi::c_char;
    output_buffer.offset = output_buffer.offset.wrapping_add(1);
    output_buffer.depth = output_buffer.depth.wrapping_add(1);
    while let Some(current_item) = array_item_clone(Some(item), index) {
        if print_value(Some(&current_item), output_buffer) == 0 {
            return false_0;
        }
        if get_array_item_address(Some(item), index.wrapping_add(1)).is_some() {
            let format = output_buffer.format;
            let length = (if format != 0 {
                2 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as size_t;
            let Some(output) = ensure(output_buffer, length.wrapping_add(1 as size_t)) else {
                return false_0;
            };
            output[0] = ',' as i32 as ::core::ffi::c_char;
            if format != 0 {
                output[1] = ' ' as i32 as ::core::ffi::c_char;
            }
            output[length] = '\0' as i32 as ::core::ffi::c_char;
            output_buffer.offset = output_buffer.offset.wrapping_add(length);
        }
        index = index.wrapping_add(1);
    }
    let Some(output) = ensure(output_buffer, 2 as size_t) else {
        return false_0;
    };
    output[0] = ']' as i32 as ::core::ffi::c_char;
    output[1] = '\0' as i32 as ::core::ffi::c_char;
    output_buffer.offset = output_buffer.offset.wrapping_add(1 as size_t);
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
        || parse_buffer_byte(input_buffer, input_buffer.offset)
            != Some('{' as i32 as ::core::ffi::c_uchar))
    {
        input_buffer.offset = input_buffer.offset.wrapping_add(1);
        buffer_skip_whitespace(input_buffer);
        if input_buffer.offset.wrapping_add(0 as size_t) < input_buffer.length
            && parse_buffer_byte(input_buffer, input_buffer.offset)
                == Some('}' as i32 as ::core::ffi::c_uchar)
        {
            c2rust_current_block = 4359236900545362719;
        } else if input_buffer.offset.wrapping_add(0 as size_t) >= input_buffer.length {
            input_buffer.offset = input_buffer.offset.wrapping_sub(1);
            c2rust_current_block = 9990476168629568694;
        } else {
            input_buffer.offset = input_buffer.offset.wrapping_sub(1);
            loop {
                let mut parsed = false_0;
                let hooks = input_buffer.hooks;
                let new_item: *mut cJSON = cJSON_New_Item(
                    &hooks,
                    cJSON_Invalid,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_double,
                    |current_item_ref| {
                        if input_buffer.offset.wrapping_add(1 as size_t) >= input_buffer.length {
                            return;
                        }
                        input_buffer.offset = input_buffer.offset.wrapping_add(1);
                        buffer_skip_whitespace(input_buffer);
                        if parse_string(current_item_ref, input_buffer) == 0 {
                            return;
                        }
                        buffer_skip_whitespace(input_buffer);
                        current_item_ref.string_bytes =
                            take_owned_valuestring_storage(current_item_ref);
                        if !(input_buffer.offset.wrapping_add(0 as size_t) < input_buffer.length)
                            || parse_buffer_byte(input_buffer, input_buffer.offset)
                                != Some(':' as i32 as ::core::ffi::c_uchar)
                        {
                            return;
                        }
                        input_buffer.offset = input_buffer.offset.wrapping_add(1);
                        buffer_skip_whitespace(input_buffer);
                        if parse_value(current_item_ref, input_buffer) == 0 {
                            return;
                        }
                        buffer_skip_whitespace(input_buffer);
                        parsed = true_0;
                    },
                );
                if new_item.is_null() {
                    c2rust_current_block = 9990476168629568694;
                    break;
                }
                if parsed == 0 {
                    cJSON_Delete(new_item);
                    c2rust_current_block = 9990476168629568694;
                    break;
                }
                if head.is_null() {
                    head = new_item;
                    current_item = head;
                } else {
                    suffix_object(
                        Some(::core::ptr::NonNull::new(current_item).expect("current object item")),
                        Some(::core::ptr::NonNull::new(new_item).expect("new object item")),
                        true,
                    );
                    current_item = new_item;
                }
                if !(input_buffer.offset.wrapping_add(0 as size_t) < input_buffer.length
                    && parse_buffer_byte(input_buffer, input_buffer.offset)
                        == Some(',' as i32 as ::core::ffi::c_uchar))
                {
                    c2rust_current_block = 14359455889292382949;
                    break;
                }
            }
            match c2rust_current_block {
                9990476168629568694 => {}
                _ => {
                    if !(input_buffer.offset.wrapping_add(0 as size_t) < input_buffer.length)
                        || parse_buffer_byte(input_buffer, input_buffer.offset)
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
                input_buffer.depth = input_buffer.depth.wrapping_sub(1);
                if !head.is_null() {
                    suffix_object(
                        Some(::core::ptr::NonNull::new(current_item).expect("current object item")),
                        Some(::core::ptr::NonNull::new(head).expect("object head")),
                        false,
                    );
                }
                item.type_0 = cJSON_Object;
                item.child.store(head as *mut cJSON, Ordering::Relaxed);
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
    let mut index = 0 as size_t;
    let mut length = (if output_buffer.format != 0 {
        2 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    }) as size_t;
    let format = output_buffer.format;
    {
        let Some(output) = ensure(output_buffer, length.wrapping_add(1 as size_t)) else {
            return false_0;
        };
        output[0] = '{' as i32 as ::core::ffi::c_char;
        if format != 0 {
            output[1] = '\n' as i32 as ::core::ffi::c_char;
        }
    }
    output_buffer.depth = output_buffer.depth.wrapping_add(1);
    output_buffer.offset = output_buffer.offset.wrapping_add(length);
    while let Some(current_item) = array_item_clone(Some(item), index) {
        if output_buffer.format != 0 {
            let Some(output) = ensure(output_buffer, output_buffer.depth) else {
                return false_0;
            };
            output.fill('\t' as i32 as ::core::ffi::c_char);
            output_buffer.offset = output_buffer.offset.wrapping_add(output_buffer.depth);
        }
        if print_string_ptr(item_key_bytes(&current_item), output_buffer) == 0 {
            return false_0;
        }
        let format = output_buffer.format;
        length = (if format != 0 {
            2 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) as size_t;
        let Some(output) = ensure(output_buffer, length) else {
            return false_0;
        };
        output[0] = ':' as i32 as ::core::ffi::c_char;
        if format != 0 {
            output[1] = '\t' as i32 as ::core::ffi::c_char;
        }
        output_buffer.offset = output_buffer.offset.wrapping_add(length);
        if print_value(Some(&current_item), output_buffer) == 0 {
            return false_0;
        }
        let format = output_buffer.format;
        let has_next = get_array_item_address(Some(item), index.wrapping_add(1)).is_some();
        length = ((if format != 0 {
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
        let mut output_index = 0usize;
        if has_next {
            output[output_index] = ',' as i32 as ::core::ffi::c_char;
            output_index += 1;
        }
        if format != 0 {
            output[output_index] = '\n' as i32 as ::core::ffi::c_char;
            output_index += 1;
        }
        output[output_index] = '\0' as i32 as ::core::ffi::c_char;
        output_buffer.offset = output_buffer.offset.wrapping_add(length);
        index = index.wrapping_add(1);
    }
    let format = output_buffer.format;
    let depth = output_buffer.depth;
    let needed = if format != 0 {
        depth.wrapping_add(1 as size_t)
    } else {
        2 as size_t
    };
    let tab_count = if format != 0 {
        depth.wrapping_sub(1 as size_t)
    } else {
        0 as size_t
    };
    let Some(output) = ensure(output_buffer, needed) else {
        return false_0;
    };
    if format != 0 {
        output[..tab_count].fill('\t' as i32 as ::core::ffi::c_char);
    }
    output[tab_count] = '}' as i32 as ::core::ffi::c_char;
    output[tab_count + 1] = '\0' as i32 as ::core::ffi::c_char;
    output_buffer.offset = output_buffer.offset.wrapping_add(
        (if output_buffer.format != 0 {
            output_buffer.depth.wrapping_sub(1 as size_t)
        } else {
            0 as size_t
        })
        .wrapping_add(1 as size_t),
    );
    output_buffer.depth = output_buffer.depth.wrapping_sub(1);
    return true_0;
}
pub fn cJSON_GetArraySize(array: Option<&cJSON>) -> ::core::ffi::c_int {
    let mut size: size_t = 0 as size_t;
    while get_array_item_address(array, size).is_some() {
        size = size.wrapping_add(1);
    }
    return size as ::core::ffi::c_int;
}
#[export_name = "cJSON_GetArraySize"]
pub unsafe extern "C" fn cJSON_GetArraySize_ffi(mut array: *const cJSON) -> ::core::ffi::c_int {
    cJSON_GetArraySize(array.as_ref())
}
fn get_array_item_address(array: Option<&cJSON>, mut index: size_t) -> Option<usize> {
    let array = array?;
    let mut current_child = array.child.load(Ordering::Relaxed) as usize;
    while current_child != 0 {
        let (node_exists, next_child) = with_allocated_node(current_child, (false, 0usize), |current| {
            (true, current.next.load(Ordering::Relaxed) as usize)
        });
        if !node_exists {
            return None;
        }
        if index == 0 as size_t {
            return Some(current_child);
        }
        index = index.wrapping_sub(1);
        current_child = next_child;
    }
    None
}
fn array_first_and_tail_addresses(array_address: usize) -> (Option<usize>, Option<usize>) {
    let first_child = with_allocated_node(array_address, 0usize, |array| {
        array.child.load(Ordering::Relaxed) as usize
    });
    if first_child == 0 {
        return (None, None);
    }
    let mut tail = first_child;
    let mut current_child = first_child;
    while current_child != 0 {
        tail = current_child;
        current_child = with_allocated_node(current_child, 0usize, |current| {
            current.next.load(Ordering::Relaxed) as usize
        });
    }
    (Some(first_child), Some(tail))
}
fn array_item_clone(array: Option<&cJSON>, index: size_t) -> Option<cJSON> {
    let address = get_array_item_address(array, index)?;
    allocated_node_clone(address)
}
fn allocated_node_clone(address: usize) -> Option<cJSON> {
    with_allocated_node(address, None, |item| Some(item.clone()))
}
pub fn cJSON_GetArrayItem(array: Option<&cJSON>, index: ::core::ffi::c_int) -> Option<usize> {
    if index < 0 as ::core::ffi::c_int {
        return None;
    }
    return get_array_item_address(array, index as size_t);
}
#[export_name = "cJSON_GetArrayItem"]
pub unsafe extern "C" fn cJSON_GetArrayItem_ffi(
    mut array: *const cJSON,
    mut index: ::core::ffi::c_int,
) -> *mut cJSON {
    match cJSON_GetArrayItem(array.as_ref(), index) {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
fn get_object_item_address(
    object: Option<&cJSON>,
    name: Option<&::core::ffi::CStr>,
    case_sensitive: cJSON_bool,
) -> Option<usize> {
    let (Some(object), Some(name)) = (object, name) else {
        return None;
    };
    let mut index = 0 as size_t;
    if case_sensitive != 0 {
        while let Some(address) = get_array_item_address(Some(object), index) {
            let matches = with_allocated_node(address, false, |current| {
                match item_key_bytes(current) {
                    Some(current_name) => name.to_bytes() == current_name,
                    None => false,
                }
            });
            if matches {
                return Some(address);
            }
            index = index.wrapping_add(1);
        }
    } else {
        while let Some(address) = get_array_item_address(Some(object), index) {
            let matches = with_allocated_node(address, false, |current| {
                case_insensitive_bytes_cmp(Some(name.to_bytes()), item_key_bytes(current))
                    == 0 as ::core::ffi::c_int
            });
            if matches {
                return Some(address);
            }
            index = index.wrapping_add(1);
        }
    }
    None
}
fn get_object_item_by_key_bytes_address(
    object: Option<&cJSON>,
    name: Option<&[::core::ffi::c_uchar]>,
    case_sensitive: cJSON_bool,
) -> Option<usize> {
    let (Some(object), Some(name)) = (object, name) else {
        return None;
    };
    let mut index = 0 as size_t;
    if case_sensitive != 0 {
        while let Some(address) = get_array_item_address(Some(object), index) {
            let matches = with_allocated_node(address, false, |current| {
                match item_key_bytes(current) {
                    Some(current_name) => name == current_name,
                    None => false,
                }
            });
            if matches {
                return Some(address);
            }
            index = index.wrapping_add(1);
        }
    } else {
        while let Some(address) = get_array_item_address(Some(object), index) {
            let matches = with_allocated_node(address, false, |current| {
                case_insensitive_bytes_cmp(Some(name), item_key_bytes(current))
                    == 0 as ::core::ffi::c_int
            });
            if matches {
                return Some(address);
            }
            index = index.wrapping_add(1);
        }
    }
    None
}
pub fn cJSON_GetObjectItem<'a>(
    object: Option<&'a cJSON>,
    string: Option<&::core::ffi::CStr>,
) -> Option<usize> {
    return get_object_item_address(object, string, false_0);
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
    match cJSON_GetObjectItem(object.as_ref(), string) {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_GetObjectItemCaseSensitive<'a>(
    object: Option<&'a cJSON>,
    string: Option<&::core::ffi::CStr>,
) -> Option<usize> {
    return get_object_item_address(object, string, true_0);
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
    match cJSON_GetObjectItemCaseSensitive(object.as_ref(), string) {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_HasObjectItem(
    object: Option<&cJSON>,
    string: Option<&::core::ffi::CStr>,
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
        Some(::core::ffi::CStr::from_ptr(string))
    };
    cJSON_HasObjectItem(object.as_ref(), string)
}
fn suffix_object(
    prev: Option<::core::ptr::NonNull<cJSON>>,
    item: Option<::core::ptr::NonNull<cJSON>>,
    link_next: bool,
) {
    let prev_ptr = prev.map_or(
        ::core::ptr::null_mut::<cJSON>(),
        ::core::ptr::NonNull::as_ptr,
    );
    let item_ptr = item.map_or(
        ::core::ptr::null_mut::<cJSON>(),
        ::core::ptr::NonNull::as_ptr,
    );
    if link_next {
        if let Some(prev) = prev {
            with_allocated_node(prev.as_ptr() as usize, (), |prev| {
                prev.next.store(item_ptr, Ordering::Relaxed);
            });
        }
    }
    if let Some(item) = item {
        with_allocated_node(item.as_ptr() as usize, (), |item| {
            item.prev.store(prev_ptr, Ordering::Relaxed);
        });
    }
}
fn suffix_object_address(prev: Option<usize>, item: Option<usize>, link_next: bool) {
    match (prev, item) {
        (Some(prev_address), Some(item_address)) => {
            with_two_allocated_nodes_mut(prev_address, item_address, (), |prev, item| {
                if link_next {
                    prev.next.store(::core::ptr::from_mut(item), Ordering::Relaxed);
                }
                item.prev.store(::core::ptr::from_mut(prev), Ordering::Relaxed);
            });
        }
        (Some(prev_address), None) => {
            if link_next {
                with_allocated_node(prev_address, (), |prev| {
                    prev.next
                        .store(::core::ptr::null_mut::<cJSON>(), Ordering::Relaxed);
                });
            }
        }
        (None, Some(item_address)) => {
            with_allocated_node(item_address, (), |item| {
                item.prev
                    .store(::core::ptr::null_mut::<cJSON>(), Ordering::Relaxed);
            });
        }
        (None, None) => {}
    }
}
fn store_child_address(parent: &cJSON, child: Option<usize>) {
    match child {
        Some(child_address) => {
            with_allocated_node(child_address, (), |child| {
                parent
                    .child
                    .store(::core::ptr::from_ref(child).cast_mut(), Ordering::Relaxed);
            });
        }
        None => {
            parent
                .child
                .store(::core::ptr::null_mut::<cJSON>(), Ordering::Relaxed);
        }
    }
}
fn create_reference(item: Option<&cJSON>, hooks: &internal_hooks) -> *mut cJSON {
    let Some(item) = item else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    cJSON_New_Item(
        hooks,
        item.type_0 | cJSON_IsReference,
        item.valueint,
        item.valuedouble,
        |reference| {
            reference
                .child
                .store(item.child.load(Ordering::Relaxed), Ordering::Relaxed);
            reference.valuestring = item.valuestring.clone();
            reference.valuestring_len = item.valuestring_len;
        },
    )
}
fn keep_array_item(_: &mut cJSON) -> cJSON_bool {
    true_0
}
fn add_item_to_array_address(
    array_address: Option<usize>,
    item_address: Option<usize>,
    configure_item: impl FnOnce(&mut cJSON) -> cJSON_bool,
) -> cJSON_bool {
    let (Some(array_address), Some(item_address)) = (array_address, item_address) else {
        return false_0;
    };
    if array_address == item_address {
        return false_0;
    }
    if with_allocated_node_mut(item_address, false_0, configure_item) == 0 {
        return false_0;
    }
    if with_allocated_node(array_address, false, |array| {
        array.child.load(Ordering::Relaxed).is_null()
    }) {
        with_two_allocated_nodes_mut(array_address, item_address, (), |array, item_ref| {
            let item_ptr = ::core::ptr::from_mut(item_ref);
            array.child.store(item_ptr, Ordering::Relaxed);
            item_ref.prev.store(item_ptr, Ordering::Relaxed);
            item_ref
                .next
                .store(::core::ptr::null_mut::<cJSON>(), Ordering::Relaxed);
        });
    } else {
        let (child_address, tail_address) = array_first_and_tail_addresses(array_address);
        if let Some(tail_address) = tail_address {
            with_two_allocated_nodes_mut(tail_address, item_address, (), |tail, item_ref| {
                tail.next
                    .store(::core::ptr::from_mut(item_ref), Ordering::Relaxed);
                item_ref
                    .prev
                    .store(::core::ptr::from_mut(tail), Ordering::Relaxed);
            });
        }
        if let Some(child_address) = child_address {
            with_two_allocated_nodes_mut(child_address, item_address, (), |child, item_ref| {
                child
                    .prev
                    .store(::core::ptr::from_mut(item_ref), Ordering::Relaxed);
            });
        }
    }
    return true_0;
}
fn add_item_to_array(
    array: Option<::core::ptr::NonNull<cJSON>>,
    item: Option<::core::ptr::NonNull<cJSON>>,
    configure_item: impl FnOnce(&mut cJSON) -> cJSON_bool,
) -> cJSON_bool {
    let array_address = match array {
        Some(array) => Some(array.as_ptr() as usize),
        None => None,
    };
    let item_address = match item {
        Some(item) => Some(item.as_ptr() as usize),
        None => None,
    };
    add_item_to_array_address(
        array_address,
        item_address,
        configure_item,
    )
}
pub fn cJSON_AddItemToArray(
    array: Option<::core::ptr::NonNull<cJSON>>,
    item: Option<::core::ptr::NonNull<cJSON>>,
) -> cJSON_bool {
    return add_item_to_array(array, item, keep_array_item);
}
#[export_name = "cJSON_AddItemToArray"]
pub unsafe extern "C" fn cJSON_AddItemToArray_ffi(
    mut array: *mut cJSON,
    mut item: *mut cJSON,
) -> cJSON_bool {
    cJSON_AddItemToArray(
        ::core::ptr::NonNull::new(array),
        ::core::ptr::NonNull::new(item),
    )
}
fn add_item_to_object(
    object: Option<::core::ptr::NonNull<cJSON>>,
    string: Option<&::core::ffi::CStr>,
    item: Option<::core::ptr::NonNull<cJSON>>,
    _hooks: &internal_hooks,
    constant_key: cJSON_bool,
) -> cJSON_bool {
    let mut new_type: ::core::ffi::c_int = cJSON_Invalid;
    let (Some(object), Some(string), Some(item)) = (object, string, item) else {
        return false_0;
    };
    let object_ptr = object.as_ptr();
    let item_ptr = item.as_ptr();
    if object_ptr == item_ptr {
        return false_0;
    }
    let key_bytes = string.to_bytes_with_nul();
    let Some(new_key_storage) = make_c_string_storage(key_bytes) else {
        return false_0;
    };
    if new_key_storage.is_empty() {
        return false_0;
    }
    return add_item_to_array(Some(object), Some(item), |item_ref| {
        if constant_key != 0 {
            new_type = item_ref.type_0 | cJSON_StringIsConst;
        } else {
            new_type = item_ref.type_0 & !cJSON_StringIsConst;
        }
        install_item_key_storage(item_ref, new_key_storage);
        item_ref.type_0 = new_type;
        true_0
    });
}
pub fn cJSON_AddItemToObject(
    object: Option<::core::ptr::NonNull<cJSON>>,
    string: Option<&::core::ffi::CStr>,
    item: Option<::core::ptr::NonNull<cJSON>>,
) -> cJSON_bool {
    return add_item_to_object(object, string, item, &current_global_hooks(), false_0);
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
    cJSON_AddItemToObject(
        ::core::ptr::NonNull::new(object),
        string,
        ::core::ptr::NonNull::new(item),
    )
}
pub fn cJSON_AddItemToObjectCS(
    object: Option<::core::ptr::NonNull<cJSON>>,
    string: Option<&::core::ffi::CStr>,
    item: Option<::core::ptr::NonNull<cJSON>>,
) -> cJSON_bool {
    return add_item_to_object(object, string, item, &current_global_hooks(), true_0);
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
    cJSON_AddItemToObjectCS(
        ::core::ptr::NonNull::new(object),
        string,
        ::core::ptr::NonNull::new(item),
    )
}
pub fn cJSON_AddItemReferenceToArray(
    array: Option<::core::ptr::NonNull<cJSON>>,
    item: Option<&cJSON>,
) -> cJSON_bool {
    let Some(array) = array else {
        return false_0;
    };
    return add_item_to_array(
        Some(array),
        ::core::ptr::NonNull::new(create_reference(item, &current_global_hooks())),
        keep_array_item,
    );
}
#[export_name = "cJSON_AddItemReferenceToArray"]
pub unsafe extern "C" fn cJSON_AddItemReferenceToArray_ffi(
    mut array: *mut cJSON,
    mut item: *mut cJSON,
) -> cJSON_bool {
    cJSON_AddItemReferenceToArray(::core::ptr::NonNull::new(array), item.as_ref())
}
pub fn cJSON_AddItemReferenceToObject(
    object: Option<::core::ptr::NonNull<cJSON>>,
    string: Option<&::core::ffi::CStr>,
    item: Option<&cJSON>,
) -> cJSON_bool {
    let (Some(object), Some(string)) = (object, string) else {
        return false_0;
    };
    let hooks = current_global_hooks();
    return add_item_to_object(
        Some(object),
        Some(string),
        ::core::ptr::NonNull::new(create_reference(item, &hooks)),
        &hooks,
        false_0,
    );
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
        Some(::core::ffi::CStr::from_ptr(string))
    };
    cJSON_AddItemReferenceToObject(::core::ptr::NonNull::new(object), string, item.as_ref())
}
pub fn cJSON_AddNullToObject(
    object: Option<::core::ptr::NonNull<cJSON>>,
    name: Option<&::core::ffi::CStr>,
) -> Option<::core::ptr::NonNull<cJSON>> {
    let null: *mut cJSON = cJSON_CreateNull();
    if add_item_to_object(
        object,
        name,
        ::core::ptr::NonNull::new(null),
        &current_global_hooks(),
        false_0,
    ) != 0
    {
        return ::core::ptr::NonNull::new(null);
    }
    cJSON_Delete(null);
    None
}
#[export_name = "cJSON_AddNullToObject"]
pub unsafe extern "C" fn cJSON_AddNullToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let name = if name.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(name))
    };
    cJSON_AddNullToObject(::core::ptr::NonNull::new(object), name).map_or(
        ::core::ptr::null_mut::<cJSON>(),
        ::core::ptr::NonNull::as_ptr,
    )
}
pub fn cJSON_AddTrueToObject(
    object: Option<::core::ptr::NonNull<cJSON>>,
    name: Option<&::core::ffi::CStr>,
) -> Option<::core::ptr::NonNull<cJSON>> {
    let true_item: *mut cJSON = cJSON_CreateTrue();
    if add_item_to_object(
        object,
        name,
        ::core::ptr::NonNull::new(true_item),
        &current_global_hooks(),
        false_0,
    ) != 0
    {
        return ::core::ptr::NonNull::new(true_item);
    }
    cJSON_Delete(true_item);
    None
}
#[export_name = "cJSON_AddTrueToObject"]
pub unsafe extern "C" fn cJSON_AddTrueToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let name = if name.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(name))
    };
    cJSON_AddTrueToObject(::core::ptr::NonNull::new(object), name).map_or(
        ::core::ptr::null_mut::<cJSON>(),
        ::core::ptr::NonNull::as_ptr,
    )
}
pub fn cJSON_AddFalseToObject(
    object: Option<::core::ptr::NonNull<cJSON>>,
    name: Option<&::core::ffi::CStr>,
) -> Option<::core::ptr::NonNull<cJSON>> {
    let false_item: *mut cJSON = cJSON_CreateFalse();
    if add_item_to_object(
        object,
        name,
        ::core::ptr::NonNull::new(false_item),
        &current_global_hooks(),
        false_0,
    ) != 0
    {
        return ::core::ptr::NonNull::new(false_item);
    }
    cJSON_Delete(false_item);
    None
}
#[export_name = "cJSON_AddFalseToObject"]
pub unsafe extern "C" fn cJSON_AddFalseToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let name = if name.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(name))
    };
    cJSON_AddFalseToObject(::core::ptr::NonNull::new(object), name).map_or(
        ::core::ptr::null_mut::<cJSON>(),
        ::core::ptr::NonNull::as_ptr,
    )
}
pub fn cJSON_AddBoolToObject(
    object: Option<::core::ptr::NonNull<cJSON>>,
    name: Option<&::core::ffi::CStr>,
    boolean: cJSON_bool,
) -> Option<::core::ptr::NonNull<cJSON>> {
    let bool_item: *mut cJSON = cJSON_CreateBool(boolean);
    if add_item_to_object(
        object,
        name,
        ::core::ptr::NonNull::new(bool_item),
        &current_global_hooks(),
        false_0,
    ) != 0
    {
        return ::core::ptr::NonNull::new(bool_item);
    }
    cJSON_Delete(bool_item);
    None
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
        Some(::core::ffi::CStr::from_ptr(name))
    };
    cJSON_AddBoolToObject(::core::ptr::NonNull::new(object), name, boolean).map_or(
        ::core::ptr::null_mut::<cJSON>(),
        ::core::ptr::NonNull::as_ptr,
    )
}
pub fn cJSON_AddNumberToObject(
    object: Option<::core::ptr::NonNull<cJSON>>,
    name: Option<&::core::ffi::CStr>,
    number: ::core::ffi::c_double,
) -> Option<::core::ptr::NonNull<cJSON>> {
    let number_item: *mut cJSON = cJSON_CreateNumber(number);
    if add_item_to_object(
        object,
        name,
        ::core::ptr::NonNull::new(number_item),
        &current_global_hooks(),
        false_0,
    ) != 0
    {
        return ::core::ptr::NonNull::new(number_item);
    }
    cJSON_Delete(number_item);
    None
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
        Some(::core::ffi::CStr::from_ptr(name))
    };
    cJSON_AddNumberToObject(::core::ptr::NonNull::new(object), name, number).map_or(
        ::core::ptr::null_mut::<cJSON>(),
        ::core::ptr::NonNull::as_ptr,
    )
}
pub fn cJSON_AddStringToObject(
    object: Option<::core::ptr::NonNull<cJSON>>,
    name: Option<&::core::ffi::CStr>,
    string: Option<&::core::ffi::CStr>,
) -> Option<::core::ptr::NonNull<cJSON>> {
    let string_item: *mut cJSON = cJSON_CreateString(string);
    if add_item_to_object(
        object,
        name,
        ::core::ptr::NonNull::new(string_item),
        &current_global_hooks(),
        false_0,
    ) != 0
    {
        return ::core::ptr::NonNull::new(string_item);
    }
    cJSON_Delete(string_item);
    None
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
    cJSON_AddStringToObject(::core::ptr::NonNull::new(object), name, string).map_or(
        ::core::ptr::null_mut::<cJSON>(),
        ::core::ptr::NonNull::as_ptr,
    )
}
pub fn cJSON_AddRawToObject(
    object: Option<::core::ptr::NonNull<cJSON>>,
    name: Option<&::core::ffi::CStr>,
    raw: Option<&::core::ffi::CStr>,
) -> Option<::core::ptr::NonNull<cJSON>> {
    let raw_item: *mut cJSON = cJSON_CreateRaw(raw);
    if add_item_to_object(
        object,
        name,
        ::core::ptr::NonNull::new(raw_item),
        &current_global_hooks(),
        false_0,
    ) != 0
    {
        return ::core::ptr::NonNull::new(raw_item);
    }
    cJSON_Delete(raw_item);
    None
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
        Some(::core::ffi::CStr::from_ptr(name))
    };
    let raw = if raw.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(raw))
    };
    cJSON_AddRawToObject(::core::ptr::NonNull::new(object), name, raw).map_or(
        ::core::ptr::null_mut::<cJSON>(),
        ::core::ptr::NonNull::as_ptr,
    )
}
pub fn cJSON_AddObjectToObject(
    object: Option<::core::ptr::NonNull<cJSON>>,
    name: Option<&::core::ffi::CStr>,
) -> Option<::core::ptr::NonNull<cJSON>> {
    let object_item: *mut cJSON = cJSON_CreateObject();
    if add_item_to_object(
        object,
        name,
        ::core::ptr::NonNull::new(object_item),
        &current_global_hooks(),
        false_0,
    ) != 0
    {
        return ::core::ptr::NonNull::new(object_item);
    }
    cJSON_Delete(object_item);
    None
}
#[export_name = "cJSON_AddObjectToObject"]
pub unsafe extern "C" fn cJSON_AddObjectToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let name = if name.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(name))
    };
    cJSON_AddObjectToObject(::core::ptr::NonNull::new(object), name).map_or(
        ::core::ptr::null_mut::<cJSON>(),
        ::core::ptr::NonNull::as_ptr,
    )
}
pub fn cJSON_AddArrayToObject(
    object: Option<::core::ptr::NonNull<cJSON>>,
    name: Option<&::core::ffi::CStr>,
) -> Option<::core::ptr::NonNull<cJSON>> {
    let array: *mut cJSON = cJSON_CreateArray();
    if add_item_to_object(
        object,
        name,
        ::core::ptr::NonNull::new(array),
        &current_global_hooks(),
        false_0,
    ) != 0
    {
        return ::core::ptr::NonNull::new(array);
    }
    cJSON_Delete(array);
    None
}
#[export_name = "cJSON_AddArrayToObject"]
pub unsafe extern "C" fn cJSON_AddArrayToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let name = if name.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(name))
    };
    cJSON_AddArrayToObject(::core::ptr::NonNull::new(object), name).map_or(
        ::core::ptr::null_mut::<cJSON>(),
        ::core::ptr::NonNull::as_ptr,
    )
}
pub fn cJSON_DetachItemViaPointer(
    parent: Option<&mut cJSON>,
    item: Option<::core::ptr::NonNull<cJSON>>,
) -> Option<::core::ptr::NonNull<cJSON>> {
    let item_address = match item {
        Some(item) => Some(item.as_ptr() as usize),
        None => None,
    };
    detach_item_via_pointer_address(parent, item_address)?;
    item
}
fn detach_item_via_pointer_address(
    parent: Option<&mut cJSON>,
    item_address: Option<usize>,
) -> Option<usize> {
    let (Some(parent), Some(item_address)) = (parent, item_address) else {
        return None;
    };
    let mut index = 0 as size_t;
    let Some((previous_item, next_item, is_first_child)) = (loop {
        let Some(current_item_address) = get_array_item_address(Some(&*parent), index) else {
            break None;
        };
        if current_item_address == item_address {
            break Some(with_allocated_node(
                current_item_address,
                (None, None, false),
                |current_item| {
                    let previous_address = current_item.prev.load(Ordering::Relaxed) as usize;
                    let next_address = current_item.next.load(Ordering::Relaxed) as usize;
                    (
                        if previous_address == 0 {
                            None
                        } else {
                            Some(previous_address)
                        },
                        if next_address == 0 {
                            None
                        } else {
                            Some(next_address)
                        },
                        current_item_address == parent.child.load(Ordering::Relaxed) as usize,
                    )
                },
            ));
        }
        index = index.wrapping_add(1);
    }) else {
        return None;
    };
    if is_first_child {
        store_child_address(parent, next_item);
        if previous_item.is_some() && next_item.is_some() {
            suffix_object_address(previous_item, next_item, false);
        }
    } else {
        match (previous_item, next_item) {
            (Some(previous_item), Some(next_item)) => {
                suffix_object_address(Some(previous_item), Some(next_item), true);
            }
            (Some(previous_item), None) => {
                suffix_object_address(Some(previous_item), None, true);
                let first_child = parent.child.load(Ordering::Relaxed) as usize;
                if first_child != 0 {
                    suffix_object_address(Some(previous_item), Some(first_child), false);
                }
            }
            (None, _) => return None,
        }
    }
    suffix_object_address(Some(item_address), None, true);
    suffix_object_address(None, Some(item_address), false);
    return Some(item_address);
}
#[export_name = "cJSON_DetachItemViaPointer"]
pub unsafe extern "C" fn cJSON_DetachItemViaPointer_ffi(
    mut parent: *mut cJSON,
    item: *mut cJSON,
) -> *mut cJSON {
    cJSON_DetachItemViaPointer(parent.as_mut(), ::core::ptr::NonNull::new(item)).map_or(
        ::core::ptr::null_mut::<cJSON>(),
        ::core::ptr::NonNull::as_ptr,
    )
}
pub fn cJSON_DetachItemFromArray(
    array: Option<&mut cJSON>,
    mut which: ::core::ffi::c_int,
) -> Option<usize> {
    if which < 0 as ::core::ffi::c_int {
        return None;
    }
    let Some(array) = array else {
        return None;
    };
    let item = get_array_item_address(Some(&*array), which as size_t);
    return detach_item_via_pointer_address(Some(array), item);
}
#[export_name = "cJSON_DetachItemFromArray"]
pub unsafe extern "C" fn cJSON_DetachItemFromArray_ffi(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
) -> *mut cJSON {
    match cJSON_DetachItemFromArray(array.as_mut(), which) {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_DeleteItemFromArray(array: Option<&mut cJSON>, mut which: ::core::ffi::c_int) {
    if let Some(item) = cJSON_DetachItemFromArray(array, which) {
        cJSON_Delete_address(item);
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
    string: Option<&::core::ffi::CStr>,
) -> Option<usize> {
    let (Some(object), Some(string)) = (object, string) else {
        return None;
    };
    let to_detach = cJSON_GetObjectItem(Some(&*object), Some(string));
    return detach_item_via_pointer_address(Some(object), to_detach);
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
    match cJSON_DetachItemFromObject(object.as_mut(), string) {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_DetachItemFromObjectCaseSensitive(
    object: Option<&mut cJSON>,
    string: Option<&::core::ffi::CStr>,
) -> Option<usize> {
    let (Some(object), Some(string)) = (object, string) else {
        return None;
    };
    let to_detach = cJSON_GetObjectItemCaseSensitive(Some(&*object), Some(string));
    return detach_item_via_pointer_address(Some(object), to_detach);
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
    match cJSON_DetachItemFromObjectCaseSensitive(object.as_mut(), string) {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_DeleteItemFromObject(object: Option<&mut cJSON>, string: Option<&::core::ffi::CStr>) {
    if let Some(item) = cJSON_DetachItemFromObject(object, string) {
        cJSON_Delete_address(item);
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
        Some(::core::ffi::CStr::from_ptr(string))
    };
    cJSON_DeleteItemFromObject(object.as_mut(), string)
}
pub fn cJSON_DeleteItemFromObjectCaseSensitive(
    object: Option<&mut cJSON>,
    string: Option<&::core::ffi::CStr>,
) {
    if let Some(item) = cJSON_DetachItemFromObjectCaseSensitive(object, string) {
        cJSON_Delete_address(item);
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
        Some(::core::ffi::CStr::from_ptr(string))
    };
    cJSON_DeleteItemFromObjectCaseSensitive(object.as_mut(), string)
}
pub fn cJSON_InsertItemInArray(
    array: Option<&mut cJSON>,
    which: ::core::ffi::c_int,
    newitem: Option<::core::ptr::NonNull<cJSON>>,
) -> cJSON_bool {
    let newitem_address = match newitem {
        Some(newitem) => Some(newitem.as_ptr() as usize),
        None => None,
    };
    insert_item_in_array_address(
        array,
        which,
        newitem_address,
    )
}
fn insert_item_in_array_address(
    array: Option<&mut cJSON>,
    which: ::core::ffi::c_int,
    newitem_address: Option<usize>,
) -> cJSON_bool {
    let (Some(array), Some(newitem_address)) = (array, newitem_address) else {
        return false_0;
    };
    if which < 0 as ::core::ffi::c_int {
        return false_0;
    }
    let after_inserted = get_array_item_address(Some(&*array), which as size_t);
    let Some(after_inserted) = after_inserted else {
        return add_item_to_array_address(
            Some(::core::ptr::from_mut(array) as usize),
            Some(newitem_address),
            keep_array_item,
        );
    };
    let previous_item = with_allocated_node(after_inserted, None, |after_inserted_ref| {
        let previous_item = after_inserted_ref.prev.load(Ordering::Relaxed) as usize;
        if previous_item == 0 {
            None
        } else {
            Some(previous_item)
        }
    });
    let is_first_child = after_inserted == array.child.load(Ordering::Relaxed) as usize;
    if !is_first_child && previous_item.is_none() {
        return false_0;
    }
    if is_first_child {
        if let Some(previous_item) = previous_item {
            suffix_object_address(Some(previous_item), Some(newitem_address), false);
        }
        suffix_object_address(Some(newitem_address), Some(after_inserted), true);
        store_child_address(array, Some(newitem_address));
    } else {
        let Some(previous_item) = previous_item else {
            return false_0;
        };
        suffix_object_address(Some(newitem_address), Some(after_inserted), true);
        suffix_object_address(Some(previous_item), Some(newitem_address), true);
    }
    return true_0;
}
#[export_name = "cJSON_InsertItemInArray"]
pub unsafe extern "C" fn cJSON_InsertItemInArray_ffi(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
    mut newitem: *mut cJSON,
) -> cJSON_bool {
    cJSON_InsertItemInArray(array.as_mut(), which, ::core::ptr::NonNull::new(newitem))
}
pub fn cJSON_ReplaceItemViaPointer(
    parent: Option<&mut cJSON>,
    item: Option<::core::ptr::NonNull<cJSON>>,
    replacement: Option<&mut cJSON>,
) -> cJSON_bool {
    let item_address = match item {
        Some(item) => Some(item.as_ptr() as usize),
        None => None,
    };
    replace_item_via_pointer_address(
        parent,
        item_address,
        replacement,
    )
}
fn replace_item_via_pointer_address(
    parent: Option<&mut cJSON>,
    item_address: Option<usize>,
    replacement: Option<&mut cJSON>,
) -> cJSON_bool {
    let (Some(parent), Some(item_address), Some(replacement)) = (parent, item_address, replacement)
    else {
        return false_0;
    };
    let replacement_ptr = ::core::ptr::from_mut(replacement);
    let replacement_address = replacement_ptr as usize;
    if parent.child.load(Ordering::Relaxed).is_null() {
        return false_0;
    }
    if replacement_address == item_address {
        return true_0;
    }

    let mut index = 0 as size_t;
    let mut found = false;
    while let Some(current_address) = get_array_item_address(Some(&*parent), index) {
        if current_address == item_address {
            found = true;
            break;
        }
        index = index.wrapping_add(1);
    }
    if !found {
        return false_0;
    }

    let Ok(which) = ::core::ffi::c_int::try_from(index) else {
        return false_0;
    };
    replacement
        .next
        .store(::core::ptr::null_mut::<cJSON>(), Ordering::Relaxed);
    replacement
        .prev
        .store(::core::ptr::null_mut::<cJSON>(), Ordering::Relaxed);
    let Some(detached) = detach_item_via_pointer_address(Some(parent), Some(item_address)) else {
        return false_0;
    };
    if insert_item_in_array_address(Some(parent), which, Some(replacement_address)) == 0 {
        let _ = insert_item_in_array_address(Some(parent), which, Some(detached));
        return false_0;
    }
    cJSON_Delete_address(detached);
    return true_0;
}
#[export_name = "cJSON_ReplaceItemViaPointer"]
pub unsafe extern "C" fn cJSON_ReplaceItemViaPointer_ffi(
    mut parent: *mut cJSON,
    item: *mut cJSON,
    mut replacement: *mut cJSON,
) -> cJSON_bool {
    cJSON_ReplaceItemViaPointer(
        parent.as_mut(),
        ::core::ptr::NonNull::new(item),
        replacement.as_mut(),
    )
}
pub fn cJSON_ReplaceItemInArray(
    array: Option<&mut cJSON>,
    mut which: ::core::ffi::c_int,
    newitem: Option<&mut cJSON>,
) -> cJSON_bool {
    if which < 0 as ::core::ffi::c_int {
        return false_0;
    }
    let (Some(array), Some(newitem)) = (array, newitem) else {
        return false_0;
    };
    let item = get_array_item_address(Some(&*array), which as size_t);
    return replace_item_via_pointer_address(Some(array), item, Some(newitem));
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
    cJSON_ReplaceItemInArray(array.as_mut(), which, newitem.as_mut())
}
fn replace_item_in_object(
    object: Option<&mut cJSON>,
    string: Option<&::core::ffi::CStr>,
    replacement: Option<&mut cJSON>,
    mut case_sensitive: cJSON_bool,
) -> cJSON_bool {
    let (Some(string), Some(replacement)) = (string, replacement) else {
        return false_0;
    };
    let string_bytes = string.to_bytes_with_nul();
    let Some(new_key_storage) = make_c_string_storage(string_bytes) else {
        return false_0;
    };
    install_item_key_storage(replacement, new_key_storage);
    replacement.type_0 &= !cJSON_StringIsConst;
    let item = match object.as_ref() {
        Some(object) => get_object_item_address(Some(&**object), Some(string), case_sensitive),
        None => None,
    };
    return replace_item_via_pointer_address(object, item, Some(replacement));
}
pub fn cJSON_ReplaceItemInObject(
    object: Option<&mut cJSON>,
    string: Option<&::core::ffi::CStr>,
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
    if newitem.is_null() || string.is_null() {
        return false_0;
    }
    let string = Some(::core::ffi::CStr::from_ptr(string));
    cJSON_ReplaceItemInObject(object.as_mut(), string, newitem.as_mut())
}
pub fn cJSON_ReplaceItemInObjectCaseSensitive(
    object: Option<&mut cJSON>,
    string: Option<&::core::ffi::CStr>,
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
    if newitem.is_null() || string.is_null() {
        return false_0;
    }
    let string = Some(::core::ffi::CStr::from_ptr(string));
    cJSON_ReplaceItemInObjectCaseSensitive(object.as_mut(), string, newitem.as_mut())
}
pub extern "C" fn cJSON_CreateNull() -> *mut cJSON {
    return cJSON_New_Item(
        &current_global_hooks(),
        cJSON_NULL,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_double,
        default_item,
    );
}
#[export_name = "cJSON_CreateNull"]
pub unsafe extern "C" fn cJSON_CreateNull_ffi() -> *mut cJSON {
    cJSON_CreateNull()
}
pub extern "C" fn cJSON_CreateTrue() -> *mut cJSON {
    return cJSON_New_Item(
        &current_global_hooks(),
        cJSON_True,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_double,
        default_item,
    );
}
#[export_name = "cJSON_CreateTrue"]
pub unsafe extern "C" fn cJSON_CreateTrue_ffi() -> *mut cJSON {
    cJSON_CreateTrue()
}
pub extern "C" fn cJSON_CreateFalse() -> *mut cJSON {
    return cJSON_New_Item(
        &current_global_hooks(),
        cJSON_False,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_double,
        default_item,
    );
}
#[export_name = "cJSON_CreateFalse"]
pub unsafe extern "C" fn cJSON_CreateFalse_ffi() -> *mut cJSON {
    cJSON_CreateFalse()
}
pub extern "C" fn cJSON_CreateBool(mut boolean: cJSON_bool) -> *mut cJSON {
    return cJSON_New_Item(
        &current_global_hooks(),
        if boolean != 0 {
            cJSON_True
        } else {
            cJSON_False
        },
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_double,
        default_item,
    );
}
#[export_name = "cJSON_CreateBool"]
pub unsafe extern "C" fn cJSON_CreateBool_ffi(mut boolean: cJSON_bool) -> *mut cJSON {
    cJSON_CreateBool(boolean)
}
pub extern "C" fn cJSON_CreateNumber(mut num: ::core::ffi::c_double) -> *mut cJSON {
    return cJSON_New_Item(
        &current_global_hooks(),
        cJSON_Number,
        number_valueint(num),
        num,
        default_item,
    );
}
#[export_name = "cJSON_CreateNumber"]
pub unsafe extern "C" fn cJSON_CreateNumber_ffi(mut num: ::core::ffi::c_double) -> *mut cJSON {
    cJSON_CreateNumber(num)
}
pub fn cJSON_CreateString(string: Option<&::core::ffi::CStr>) -> *mut cJSON {
    let hooks = current_global_hooks();
    let mut created_valuestring = false;
    let item: *mut cJSON = cJSON_New_Item(
        &hooks,
        cJSON_String,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_double,
        |item| {
            if let Some(string) = string {
                created_valuestring =
                    set_valuestring_storage(item, string.to_bytes_with_nul()) != 0;
            }
        },
    );
    if !item.is_null() && !created_valuestring {
        cJSON_Delete(item);
        return ::core::ptr::null_mut::<cJSON>();
    }
    return item;
}
#[export_name = "cJSON_CreateString"]
pub unsafe extern "C" fn cJSON_CreateString_ffi(
    mut string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    cJSON_CreateString(if string.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(string))
    })
}
pub fn cJSON_CreateStringReference(string: Option<&'static ::core::ffi::CStr>) -> *mut cJSON {
    cJSON_New_Item(
        &current_global_hooks(),
        cJSON_String | cJSON_IsReference,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_double,
        |item| {
            if let Some(string) = string {
                if set_valuestring_storage(item, string.to_bytes_with_nul()) == 0 {
                    install_valuestring_reference(item, string);
                }
            }
        },
    )
}
#[export_name = "cJSON_CreateStringReference"]
pub unsafe extern "C" fn cJSON_CreateStringReference_ffi(
    mut string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    cJSON_CreateStringReference(if string.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(string))
    })
}
pub fn cJSON_CreateObjectReference(child: Option<&cJSON>) -> *mut cJSON {
    cJSON_New_Item(
        &current_global_hooks(),
        cJSON_Object | cJSON_IsReference,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_double,
        |item| {
            if let Some(child) = child {
                item.child
                    .store((child as *const cJSON).cast_mut(), Ordering::Relaxed);
            }
        },
    )
}
#[export_name = "cJSON_CreateObjectReference"]
pub unsafe extern "C" fn cJSON_CreateObjectReference_ffi(mut child: *const cJSON) -> *mut cJSON {
    cJSON_CreateObjectReference(child.as_ref())
}
pub fn cJSON_CreateArrayReference(child: Option<&cJSON>) -> *mut cJSON {
    cJSON_New_Item(
        &current_global_hooks(),
        cJSON_Array | cJSON_IsReference,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_double,
        |item| {
            if let Some(child) = child {
                item.child
                    .store((child as *const cJSON).cast_mut(), Ordering::Relaxed);
            }
        },
    )
}
#[export_name = "cJSON_CreateArrayReference"]
pub unsafe extern "C" fn cJSON_CreateArrayReference_ffi(mut child: *const cJSON) -> *mut cJSON {
    cJSON_CreateArrayReference(child.as_ref())
}
pub fn cJSON_CreateRaw(raw: Option<&::core::ffi::CStr>) -> *mut cJSON {
    let hooks = current_global_hooks();
    let mut created_valuestring = false;
    let item: *mut cJSON = cJSON_New_Item(
        &hooks,
        cJSON_Raw,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_double,
        |item| {
            if let Some(raw) = raw {
                created_valuestring = set_valuestring_storage(item, raw.to_bytes_with_nul()) != 0;
            }
        },
    );
    if !item.is_null() && !created_valuestring {
        cJSON_Delete(item);
        return ::core::ptr::null_mut::<cJSON>();
    }
    return item;
}
#[export_name = "cJSON_CreateRaw"]
pub unsafe extern "C" fn cJSON_CreateRaw_ffi(mut raw: *const ::core::ffi::c_char) -> *mut cJSON {
    cJSON_CreateRaw(if raw.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr(raw))
    })
}
pub extern "C" fn cJSON_CreateArray() -> *mut cJSON {
    return cJSON_New_Item(
        &current_global_hooks(),
        cJSON_Array,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_double,
        default_item,
    );
}
#[export_name = "cJSON_CreateArray"]
pub unsafe extern "C" fn cJSON_CreateArray_ffi() -> *mut cJSON {
    cJSON_CreateArray()
}
pub extern "C" fn cJSON_CreateObject() -> *mut cJSON {
    return cJSON_New_Item(
        &current_global_hooks(),
        cJSON_Object,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_double,
        default_item,
    );
}
#[export_name = "cJSON_CreateObject"]
pub unsafe extern "C" fn cJSON_CreateObject_ffi() -> *mut cJSON {
    cJSON_CreateObject()
}
pub fn cJSON_CreateIntArray(numbers: &[::core::ffi::c_int]) -> *mut cJSON {
    let a: *mut cJSON = cJSON_CreateArray();
    if a.is_null() {
        return a;
    }
    for &number in numbers {
        let n: *mut cJSON = cJSON_CreateNumber(number as ::core::ffi::c_double);
        if n.is_null()
            || add_item_to_array(
                ::core::ptr::NonNull::new(a),
                ::core::ptr::NonNull::new(n),
                keep_array_item,
            ) == 0
        {
            if !n.is_null() {
                cJSON_Delete(n);
            }
            cJSON_Delete(a);
            return ::core::ptr::null_mut::<cJSON>();
        }
    }
    return a;
}
#[export_name = "cJSON_CreateIntArray"]
pub unsafe extern "C" fn cJSON_CreateIntArray_ffi(
    mut numbers: *const ::core::ffi::c_int,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_CreateIntArray(::core::slice::from_raw_parts(numbers, count as size_t))
}
pub fn cJSON_CreateFloatArray(numbers: &[::core::ffi::c_float]) -> *mut cJSON {
    let a: *mut cJSON = cJSON_CreateArray();
    if a.is_null() {
        return a;
    }
    for &number in numbers {
        let n: *mut cJSON = cJSON_CreateNumber(number as ::core::ffi::c_double);
        if n.is_null()
            || add_item_to_array(
                ::core::ptr::NonNull::new(a),
                ::core::ptr::NonNull::new(n),
                keep_array_item,
            ) == 0
        {
            if !n.is_null() {
                cJSON_Delete(n);
            }
            cJSON_Delete(a);
            return ::core::ptr::null_mut::<cJSON>();
        }
    }
    return a;
}
#[export_name = "cJSON_CreateFloatArray"]
pub unsafe extern "C" fn cJSON_CreateFloatArray_ffi(
    mut numbers: *const ::core::ffi::c_float,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_CreateFloatArray(::core::slice::from_raw_parts(numbers, count as size_t))
}
pub fn cJSON_CreateDoubleArray(numbers: &[::core::ffi::c_double]) -> *mut cJSON {
    let a: *mut cJSON = cJSON_CreateArray();
    if a.is_null() {
        return a;
    }
    for &number in numbers {
        let n: *mut cJSON = cJSON_CreateNumber(number);
        if n.is_null()
            || add_item_to_array(
                ::core::ptr::NonNull::new(a),
                ::core::ptr::NonNull::new(n),
                keep_array_item,
            ) == 0
        {
            if !n.is_null() {
                cJSON_Delete(n);
            }
            cJSON_Delete(a);
            return ::core::ptr::null_mut::<cJSON>();
        }
    }
    return a;
}
#[export_name = "cJSON_CreateDoubleArray"]
pub unsafe extern "C" fn cJSON_CreateDoubleArray_ffi(
    mut numbers: *const ::core::ffi::c_double,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_CreateDoubleArray(::core::slice::from_raw_parts(numbers, count as size_t))
}
pub fn cJSON_CreateStringArray(strings: &[Option<&::core::ffi::CStr>]) -> *mut cJSON {
    let a: *mut cJSON = cJSON_CreateArray();
    if a.is_null() {
        return a;
    }
    for &string in strings {
        let n: *mut cJSON = cJSON_CreateString(string);
        if n.is_null()
            || add_item_to_array(
                ::core::ptr::NonNull::new(a),
                ::core::ptr::NonNull::new(n),
                keep_array_item,
            ) == 0
        {
            if !n.is_null() {
                cJSON_Delete(n);
            }
            cJSON_Delete(a);
            return ::core::ptr::null_mut::<cJSON>();
        }
    }
    return a;
}
#[export_name = "cJSON_CreateStringArray"]
pub unsafe extern "C" fn cJSON_CreateStringArray_ffi(
    mut strings: *const *const ::core::ffi::c_char,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    if count < 0 as ::core::ffi::c_int || strings.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let raw_strings = ::core::slice::from_raw_parts(strings, count as size_t);
    let mut safe_strings: Vec<Option<&::core::ffi::CStr>> = Vec::with_capacity(raw_strings.len());
    for &string in raw_strings {
        safe_strings.push(if string.is_null() {
            None
        } else {
            Some(::core::ffi::CStr::from_ptr(string))
        });
    }
    cJSON_CreateStringArray(&safe_strings)
}
pub fn cJSON_Duplicate(item: Option<&cJSON>, mut recurse: cJSON_bool) -> *mut cJSON {
    return cJSON_Duplicate_rec(item, 0 as size_t, recurse);
}
#[export_name = "cJSON_Duplicate"]
pub unsafe extern "C" fn cJSON_Duplicate_ffi(
    mut item: *const cJSON,
    mut recurse: cJSON_bool,
) -> *mut cJSON {
    cJSON_Duplicate(item.as_ref(), recurse)
}
pub fn cJSON_Duplicate_rec(
    item: Option<&cJSON>,
    mut depth: size_t,
    mut recurse: cJSON_bool,
) -> *mut cJSON {
    let mut newitem: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if let Some(item) = item {
        let mut copy_failed = false;
        newitem = cJSON_New_Item(
            &current_global_hooks(),
            item.type_0 & !cJSON_IsReference,
            item.valueint,
            item.valuedouble,
            |newitem_ref| {
                match &item.valuestring {
                    ValueStringStorage::None => {}
                    ValueStringStorage::Owned(valuestring_storage) => {
                        install_valuestring_storage(newitem_ref, valuestring_storage.clone());
                    }
                    ValueStringStorage::Borrowed(_) => {
                        copy_failed = true;
                        return;
                    }
                }
                if let Some(string_storage) = item.string_bytes.clone() {
                    install_item_key_storage(newitem_ref, string_storage);
                }
            },
        );
        if !newitem.is_null() {
            if !copy_failed {
                if recurse == 0 {
                    return newitem;
                }
                let mut index = 0 as size_t;
                loop {
                    let Some(child_item) = array_item_clone(Some(item), index) else {
                        return newitem;
                    };
                    if depth >= CJSON_CIRCULAR_LIMIT as size_t {
                        break;
                    }
                    let newchild =
                        cJSON_Duplicate_rec(Some(&child_item), depth.wrapping_add(1), true_0);
                    if newchild.is_null() {
                        break;
                    }
                    if add_item_to_array(
                        ::core::ptr::NonNull::new(newitem),
                        ::core::ptr::NonNull::new(newchild),
                        keep_array_item,
                    ) == 0
                    {
                        cJSON_Delete(newchild);
                        break;
                    }
                    index = index.wrapping_add(1);
                }
            }
        }
    }
    if !newitem.is_null() {
        cJSON_Delete(newitem);
    }
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_Duplicate_rec"]
pub unsafe extern "C" fn cJSON_Duplicate_rec_ffi(
    mut item: *const cJSON,
    mut depth: size_t,
    mut recurse: cJSON_bool,
) -> *mut cJSON {
    cJSON_Duplicate_rec(item.as_ref(), depth, recurse)
}
fn skip_oneline_comment(input: &[::core::ffi::c_char], mut read: usize) -> usize {
    read = read.wrapping_add(2);
    while read < input.len() && input[read] as ::core::ffi::c_int != '\0' as i32 {
        if input[read] as ::core::ffi::c_int == '\n' as i32 {
            return read.wrapping_add(1);
        }
        read = read.wrapping_add(1);
    }
    read
}
fn skip_multiline_comment(input: &[::core::ffi::c_char], mut read: usize) -> usize {
    read = read.wrapping_add(2);
    while read < input.len() && input[read] as ::core::ffi::c_int != '\0' as i32 {
        if input[read] as ::core::ffi::c_int == '*' as i32
            && input.get(read.wrapping_add(1)).copied().unwrap_or(0) as ::core::ffi::c_int
                == '/' as i32
        {
            return read.wrapping_add(2);
        }
        read = read.wrapping_add(1);
    }
    read
}
fn minify_string(
    input: &mut [::core::ffi::c_char],
    mut read: usize,
    mut output: usize,
) -> (usize, usize) {
    input[output] = input[read];
    read = read.wrapping_add(1);
    output = output.wrapping_add(1);
    while read < input.len() && input[read] as ::core::ffi::c_int != '\0' as i32 {
        input[output] = input[read];
        if input[read] as ::core::ffi::c_int == '"' as i32 {
            return (read.wrapping_add(1), output.wrapping_add(1));
        } else if input[read] as ::core::ffi::c_int == '\\' as i32
            && input.get(read.wrapping_add(1)).copied().unwrap_or(0) as ::core::ffi::c_int
                == '"' as i32
        {
            input[output.wrapping_add(1)] = input[read.wrapping_add(1)];
            read = read.wrapping_add(1);
            output = output.wrapping_add(1);
        }
        read = read.wrapping_add(1);
        output = output.wrapping_add(1);
    }
    (read, output)
}
pub fn cJSON_Minify(json: &mut [::core::ffi::c_char]) {
    let mut read: usize = 0;
    let mut output: usize = 0;
    while read < json.len() && json[read] as ::core::ffi::c_int != '\0' as i32 {
        match json[read] as ::core::ffi::c_int {
            32 | 9 | 13 | 10 => {
                read = read.wrapping_add(1);
            }
            47 => {
                if json.get(read.wrapping_add(1)).copied().unwrap_or(0) as ::core::ffi::c_int
                    == '/' as i32
                {
                    read = skip_oneline_comment(json, read);
                } else if json.get(read.wrapping_add(1)).copied().unwrap_or(0) as ::core::ffi::c_int
                    == '*' as i32
                {
                    read = skip_multiline_comment(json, read);
                } else {
                    read = read.wrapping_add(1);
                }
            }
            34 => {
                (read, output) = minify_string(json, read, output);
            }
            _ => {
                json[output] = json[read];
                read = read.wrapping_add(1);
                output = output.wrapping_add(1);
            }
        }
    }
    if output < json.len() {
        json[output] = '\0' as i32 as ::core::ffi::c_char;
    }
}
#[export_name = "cJSON_Minify"]
pub unsafe extern "C" fn cJSON_Minify_ffi(mut json: *mut ::core::ffi::c_char) {
    if json.is_null() {
        return;
    }
    let length = ::std::ffi::CStr::from_ptr(json).to_bytes_with_nul().len();
    cJSON_Minify(::core::slice::from_raw_parts_mut(json, length))
}
pub fn cJSON_IsInvalid(item: Option<&cJSON>) -> cJSON_bool {
    match item {
        Some(item) => {
            (item.type_0 & 0xff as ::core::ffi::c_int == cJSON_Invalid) as ::core::ffi::c_int
        }
        None => false_0,
    }
}
#[export_name = "cJSON_IsInvalid"]
pub unsafe extern "C" fn cJSON_IsInvalid_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsInvalid(item.as_ref())
}
pub fn cJSON_IsFalse(item: Option<&cJSON>) -> cJSON_bool {
    match item {
        Some(item) => {
            (item.type_0 & 0xff as ::core::ffi::c_int == cJSON_False) as ::core::ffi::c_int
        }
        None => false_0,
    }
}
#[export_name = "cJSON_IsFalse"]
pub unsafe extern "C" fn cJSON_IsFalse_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsFalse(item.as_ref())
}
pub fn cJSON_IsTrue(item: Option<&cJSON>) -> cJSON_bool {
    match item {
        Some(item) => {
            (item.type_0 & 0xff as ::core::ffi::c_int == cJSON_True) as ::core::ffi::c_int
        }
        None => false_0,
    }
}
#[export_name = "cJSON_IsTrue"]
pub unsafe extern "C" fn cJSON_IsTrue_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsTrue(item.as_ref())
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
    cJSON_IsBool(item.as_ref())
}
pub fn cJSON_IsNull(item: Option<&cJSON>) -> cJSON_bool {
    match item {
        Some(item) => {
            (item.type_0 & 0xff as ::core::ffi::c_int == cJSON_NULL) as ::core::ffi::c_int
        }
        None => false_0,
    }
}
#[export_name = "cJSON_IsNull"]
pub unsafe extern "C" fn cJSON_IsNull_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsNull(item.as_ref())
}
pub fn cJSON_IsNumber(item: Option<&cJSON>) -> cJSON_bool {
    return item
        .map(|item| {
            (item.type_0 & 0xff as ::core::ffi::c_int == cJSON_Number) as ::core::ffi::c_int
        })
        .unwrap_or(false_0);
}
#[export_name = "cJSON_IsNumber"]
pub unsafe extern "C" fn cJSON_IsNumber_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsNumber(item.as_ref())
}
pub fn cJSON_IsString(item: Option<&cJSON>) -> cJSON_bool {
    return item
        .map(|item| {
            (item.type_0 & 0xff as ::core::ffi::c_int == cJSON_String) as ::core::ffi::c_int
        })
        .unwrap_or(false_0);
}
#[export_name = "cJSON_IsString"]
pub unsafe extern "C" fn cJSON_IsString_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsString(item.as_ref())
}
pub fn cJSON_IsArray(item: Option<&cJSON>) -> cJSON_bool {
    match item {
        Some(item) => {
            (item.type_0 & 0xff as ::core::ffi::c_int == cJSON_Array) as ::core::ffi::c_int
        }
        None => false_0,
    }
}
#[export_name = "cJSON_IsArray"]
pub unsafe extern "C" fn cJSON_IsArray_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsArray(item.as_ref())
}
pub fn cJSON_IsObject(item: Option<&cJSON>) -> cJSON_bool {
    match item {
        Some(item) => {
            (item.type_0 & 0xff as ::core::ffi::c_int == cJSON_Object) as ::core::ffi::c_int
        }
        None => false_0,
    }
}
#[export_name = "cJSON_IsObject"]
pub unsafe extern "C" fn cJSON_IsObject_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsObject(item.as_ref())
}
pub fn cJSON_IsRaw(item: Option<&cJSON>) -> cJSON_bool {
    match item {
        Some(item) => (item.type_0 & 0xff as ::core::ffi::c_int == cJSON_Raw) as ::core::ffi::c_int,
        None => false_0,
    }
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
    if a.type_0 & 0xff as ::core::ffi::c_int != b.type_0 & 0xff as ::core::ffi::c_int {
        return false_0;
    }
    match a.type_0 & 0xff as ::core::ffi::c_int {
        cJSON_False | cJSON_True | cJSON_NULL | cJSON_Number | cJSON_String | cJSON_Raw
        | cJSON_Array | cJSON_Object => {}
        _ => return false_0,
    }
    if ::core::ptr::eq(a, b) {
        return true_0;
    }
    match a.type_0 & 0xff as ::core::ffi::c_int {
        cJSON_False | cJSON_True | cJSON_NULL => return true_0,
        cJSON_Number => {
            if compare_double(a.valuedouble, b.valuedouble) != 0 {
                return true_0;
            }
            return false_0;
        }
        cJSON_String | cJSON_Raw => {
            if valuestring_bytes(a).is_some() && valuestring_bytes(a) == valuestring_bytes(b) {
                return true_0;
            }
            return false_0;
        }
        cJSON_Array => {
            let mut index = 0 as size_t;
            loop {
                let a_element = array_item_clone(Some(a), index);
                let b_element = array_item_clone(Some(b), index);
                let (a_element, b_element) = match (a_element, b_element) {
                    (Some(a_element), Some(b_element)) => (a_element, b_element),
                    (None, None) => return true_0,
                    _ => return false_0,
                };
                if cJSON_Compare(Some(&a_element), Some(&b_element), case_sensitive) == 0 {
                    return false_0;
                }
                index = index.wrapping_add(1);
            }
        }
        cJSON_Object => {
            let mut index = 0 as size_t;
            while let Some(a_element) = array_item_clone(Some(a), index) {
                let Some(b_element_address) = get_object_item_by_key_bytes_address(
                    Some(b),
                    item_key_bytes(&a_element),
                    case_sensitive,
                ) else {
                    return false_0;
                };
                let Some(b_element) = allocated_node_clone(b_element_address) else {
                    return false_0;
                };
                if cJSON_Compare(Some(&a_element), Some(&b_element), case_sensitive) == 0 {
                    return false_0;
                }
                index = index.wrapping_add(1);
            }
            let mut index = 0 as size_t;
            while let Some(b_element) = array_item_clone(Some(b), index) {
                let Some(a_element_address) = get_object_item_by_key_bytes_address(
                    Some(a),
                    item_key_bytes(&b_element),
                    case_sensitive,
                ) else {
                    return false_0;
                };
                let Some(a_element) = allocated_node_clone(a_element_address) else {
                    return false_0;
                };
                if cJSON_Compare(Some(&b_element), Some(&a_element), case_sensitive) == 0 {
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
    current_global_hooks()
        .allocate
        .expect("non-null function pointer")(size)
}
pub fn cJSON_free(object: Option<::core::ptr::NonNull<::core::ffi::c_void>>) {
    let object = object.map_or(::core::ptr::null_mut(), ::core::ptr::NonNull::as_ptr);
    current_global_hooks()
        .deallocate
        .expect("non-null function pointer")(object);
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
