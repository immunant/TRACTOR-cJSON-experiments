extern "C" {
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strtod(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
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
    valuestring_storage: Option<Vec<u8>>,
    string_storage: Option<Vec<u8>>,
    children: Vec<Box<cJSON>>,
}
impl Clone for cJSON {
    fn clone(&self) -> Self {
        let mut cloned = Self {
            next: self.next,
            prev: self.prev,
            child: self.child,
            type_0: self.type_0,
            valuestring: self.valuestring,
            valueint: self.valueint,
            valuedouble: self.valuedouble,
            string: self.string,
            valuestring_storage: self.valuestring_storage.clone(),
            string_storage: self.string_storage.clone(),
            children: Vec::new(),
        };
        if let Some(storage) = cloned.valuestring_storage.as_mut() {
            cloned.valuestring = storage.as_mut_ptr() as *mut ::core::ffi::c_char;
        }
        if let Some(storage) = cloned.string_storage.as_mut() {
            cloned.string = storage.as_mut_ptr() as *mut ::core::ffi::c_char;
        }
        cloned
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
    pub reallocate:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct error {
    pub json: *const ::core::ffi::c_uchar,
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
impl parse_buffer<'_> {
    fn can_access_at_offset(&self, offset: size_t) -> bool {
        self.offset
            .checked_add(offset)
            .is_some_and(|index| index < self.length && index < self.content.len())
    }

    fn byte_at_offset(&self, offset: size_t) -> Option<::core::ffi::c_uchar> {
        if !self.can_access_at_offset(offset) {
            return None;
        }
        self.content.get(self.offset.wrapping_add(offset)).copied()
    }

    fn starts_with(&self, bytes: &[::core::ffi::c_uchar]) -> bool {
        let Some(end) = self.offset.checked_add(bytes.len()) else {
            return false;
        };
        end <= self.length && self.content.get(self.offset..end) == Some(bytes)
    }
}
#[derive(Clone)]
pub struct printbuffer {
    pub buffer: Vec<::core::ffi::c_uchar>,
    pub length: size_t,
    pub offset: size_t,
    pub depth: size_t,
    pub noalloc: cJSON_bool,
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
static global_error: ::std::sync::atomic::AtomicUsize =
    ::std::sync::atomic::AtomicUsize::new(0 as size_t);
static global_error_position: ::std::sync::atomic::AtomicUsize =
    ::std::sync::atomic::AtomicUsize::new(0 as size_t);
pub fn cJSON_GetErrorPtr() -> *const ::core::ffi::c_char {
    let json_address = global_error.load(::std::sync::atomic::Ordering::Relaxed);
    let position = global_error_position.load(::std::sync::atomic::Ordering::Relaxed);
    let json = ::core::ptr::with_exposed_provenance::<::core::ffi::c_uchar>(json_address);
    return json.wrapping_add(position) as *const ::core::ffi::c_char;
}
#[export_name = "cJSON_GetErrorPtr"]
pub unsafe extern "C" fn cJSON_GetErrorPtr_ffi() -> *const ::core::ffi::c_char {
    cJSON_GetErrorPtr()
}
pub fn cJSON_GetStringValue(item: Option<&cJSON>) -> *mut ::core::ffi::c_char {
    let item = match item {
        Some(item) => item,
        None => return ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    if cJSON_IsString(Some(item)) == 0 {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return item.valuestring;
}
#[export_name = "cJSON_GetStringValue"]
pub unsafe extern "C" fn cJSON_GetStringValue_ffi(item: *const cJSON) -> *mut ::core::ffi::c_char {
    cJSON_GetStringValue(item.as_ref())
}
pub fn cJSON_GetNumberValue(item: Option<&cJSON>) -> ::core::ffi::c_double {
    let item = match item {
        Some(item) => item,
        None => return 0.0f64 / 0.0f64,
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
fn cstr_cmp(
    string1: Option<&::std::ffi::CStr>,
    string2: Option<&::std::ffi::CStr>,
) -> ::core::ffi::c_int {
    let (string1, string2) = match (string1, string2) {
        (Some(string1), Some(string2)) => (string1, string2),
        _ => {
            return 1 as ::core::ffi::c_int;
        }
    };
    if string1.as_ptr() == string2.as_ptr() {
        return 0 as ::core::ffi::c_int;
    }
    let string1 = string1.to_bytes_with_nul();
    let string2 = string2.to_bytes_with_nul();
    for (&byte1, &byte2) in string1.iter().zip(string2.iter()) {
        if byte1 != byte2 {
            return byte1 as ::core::ffi::c_int - byte2 as ::core::ffi::c_int;
        }
    }
    if string1.len() == string2.len() {
        return 0 as ::core::ffi::c_int;
    }
    if string1.len() < string2.len() {
        return 0 as ::core::ffi::c_int - string2[string1.len()] as ::core::ffi::c_int;
    }
    return string1[string2.len()] as ::core::ffi::c_int;
}
fn case_insensitive_cstr_cmp(
    string1: Option<&::std::ffi::CStr>,
    string2: Option<&::std::ffi::CStr>,
) -> ::core::ffi::c_int {
    let (string1, string2) = match (string1, string2) {
        (Some(string1), Some(string2)) => (string1, string2),
        _ => {
            return 1 as ::core::ffi::c_int;
        }
    };
    if string1.as_ptr() == string2.as_ptr() {
        return 0 as ::core::ffi::c_int;
    }
    for (&byte1, &byte2) in string1
        .to_bytes_with_nul()
        .iter()
        .zip(string2.to_bytes_with_nul().iter())
    {
        let byte1 = byte1.to_ascii_lowercase();
        let byte2 = byte2.to_ascii_lowercase();
        if byte1 != byte2 {
            return byte1 as ::core::ffi::c_int - byte2 as ::core::ffi::c_int;
        }
    }
    if string1.to_bytes_with_nul().len() == string2.to_bytes_with_nul().len() {
        return 0 as ::core::ffi::c_int;
    }
    if string1.to_bytes_with_nul().len() < string2.to_bytes_with_nul().len() {
        return 0 as ::core::ffi::c_int
            - string2.to_bytes_with_nul()[string1.to_bytes_with_nul().len()].to_ascii_lowercase()
                as ::core::ffi::c_int;
    }
    return string1.to_bytes_with_nul()[string2.to_bytes_with_nul().len()].to_ascii_lowercase()
        as ::core::ffi::c_int;
}

static global_hooks: ::std::sync::RwLock<internal_hooks> =
    ::std::sync::RwLock::new(internal_hooks {
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
fn current_global_hooks() -> internal_hooks {
    *global_hooks.read().expect("global hooks lock poisoned")
}
fn cstr_storage_from_cstr(string: &::std::ffi::CStr) -> Option<Vec<u8>> {
    let bytes = string.to_bytes_with_nul();
    let mut storage = Vec::new();
    if storage.try_reserve_exact(bytes.len()).is_err() {
        return None;
    }
    storage.extend_from_slice(bytes);
    Some(storage)
}
fn valuestring_cstr(item: &cJSON) -> Option<&::std::ffi::CStr> {
    item.valuestring_storage
        .as_deref()
        .map(::std::ffi::CStr::from_bytes_with_nul)
        .and_then(Result::ok)
}
fn string_cstr(item: &cJSON) -> Option<&::std::ffi::CStr> {
    item.string_storage
        .as_deref()
        .map(::std::ffi::CStr::from_bytes_with_nul)
        .and_then(Result::ok)
}
fn set_owned_valuestring(item: &mut cJSON, string: &::std::ffi::CStr) -> bool {
    let Some(storage) = cstr_storage_from_cstr(string) else {
        return false;
    };
    item.valuestring_storage = Some(storage);
    item.valuestring = item
        .valuestring_storage
        .as_mut()
        .expect("valuestring storage was just assigned")
        .as_mut_ptr() as *mut ::core::ffi::c_char;
    true
}
fn set_owned_string(item: &mut cJSON, string: &::std::ffi::CStr) -> bool {
    let Some(storage) = cstr_storage_from_cstr(string) else {
        return false;
    };
    item.string_storage = Some(storage);
    item.string = item
        .string_storage
        .as_mut()
        .expect("string storage was just assigned")
        .as_mut_ptr() as *mut ::core::ffi::c_char;
    true
}
fn try_resize_vec(
    buffer: &mut Vec<::core::ffi::c_uchar>,
    new_len: usize,
    value: ::core::ffi::c_uchar,
) -> bool {
    if new_len > buffer.len() {
        let additional = new_len.wrapping_sub(buffer.len());
        if buffer.try_reserve_exact(additional).is_err() {
            return false;
        }
    }
    buffer.resize(new_len, value);
    true
}
pub fn cJSON_InitHooks(hooks: Option<&cJSON_Hooks>) {
    let mut new_hooks = internal_hooks {
        allocate: Some(malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void),
        deallocate: Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        reallocate: Some(
            realloc
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                ) -> *mut ::core::ffi::c_void,
        ),
    };
    if let Some(hooks) = hooks {
        if hooks.malloc_fn.is_some() {
            new_hooks.allocate = hooks.malloc_fn;
        }
        if hooks.free_fn.is_some() {
            new_hooks.deallocate = hooks.free_fn;
        }
        new_hooks.reallocate = None;
        if new_hooks.allocate
            == Some(malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void)
            && new_hooks.deallocate
                == Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ())
        {
            new_hooks.reallocate = Some(
                realloc
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        size_t,
                    ) -> *mut ::core::ffi::c_void,
            );
        }
    }
    *global_hooks.write().expect("global hooks lock poisoned") = new_hooks;
}
#[export_name = "cJSON_InitHooks"]
pub unsafe extern "C" fn cJSON_InitHooks_ffi(mut hooks: *mut cJSON_Hooks) {
    cJSON_InitHooks(hooks.as_ref())
}
fn cJSON_New_Item(hooks: &internal_hooks) -> Option<Box<cJSON>> {
    if hooks.allocate.is_none() {
        return None;
    }
    Some(Box::new(cJSON {
        next: ::core::ptr::null_mut::<cJSON>(),
        prev: ::core::ptr::null_mut::<cJSON>(),
        child: ::core::ptr::null_mut::<cJSON>(),
        type_0: 0 as ::core::ffi::c_int,
        valuestring: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        valueint: 0 as ::core::ffi::c_int,
        valuedouble: 0 as ::core::ffi::c_double,
        string: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        valuestring_storage: None,
        string_storage: None,
        children: Vec::new(),
    }))
}
pub fn cJSON_Delete(item: Option<Box<cJSON>>) {
    drop(item);
}
#[export_name = "cJSON_Delete"]
pub unsafe extern "C" fn cJSON_Delete_ffi(mut item: *mut cJSON) {
    let mut next: *mut cJSON;
    while !item.is_null() {
        let item_ref = &mut *item;
        next = item_ref.next as *mut cJSON;
        if item_ref.type_0 & cJSON_IsReference == 0
            && item_ref.children.is_empty()
            && !item_ref.child.is_null()
        {
            cJSON_Delete_ffi(item_ref.child as *mut cJSON);
        }
        let item_to_delete = item;
        item = next;
        drop(Box::from_raw(item_to_delete));
    }
}
fn consume_ascii_digits(input: &[::core::ffi::c_uchar], index: &mut usize) -> bool {
    let start = *index;
    while input
        .get(*index)
        .copied()
        .is_some_and(|byte| byte.is_ascii_digit())
    {
        *index = index.wrapping_add(1);
    }
    *index != start
}
fn number_prefix_len(input: &[::core::ffi::c_uchar]) -> Option<usize> {
    let mut index = 0usize;
    if input
        .get(index)
        .copied()
        .is_some_and(|byte| byte == b'+' || byte == b'-')
    {
        index = index.wrapping_add(1);
    }
    let has_integer_digits = consume_ascii_digits(input, &mut index);
    let mut has_fraction_digits = false;
    if input.get(index).copied() == Some(b'.') {
        index = index.wrapping_add(1);
        has_fraction_digits = consume_ascii_digits(input, &mut index);
    }
    if !has_integer_digits && !has_fraction_digits {
        return None;
    }
    let mantissa_end = index;
    if input
        .get(index)
        .copied()
        .is_some_and(|byte| byte == b'e' || byte == b'E')
    {
        let mut exponent_index = index.wrapping_add(1);
        if input
            .get(exponent_index)
            .copied()
            .is_some_and(|byte| byte == b'+' || byte == b'-')
        {
            exponent_index = exponent_index.wrapping_add(1);
        }
        if consume_ascii_digits(input, &mut exponent_index) {
            index = exponent_index;
        } else {
            index = mantissa_end;
        }
    }
    Some(index)
}
fn parse_number(item: &mut cJSON, input_buffer: &mut parse_buffer) -> cJSON_bool {
    let Some(remaining_input) = input_buffer
        .content
        .get(input_buffer.offset..input_buffer.length)
    else {
        return false_0;
    };
    let Some(number_string_length) = number_prefix_len(remaining_input) else {
        return false_0;
    };
    let Some(number_bytes) = remaining_input.get(..number_string_length) else {
        return false_0;
    };
    let Ok(number_text) = ::core::str::from_utf8(number_bytes) else {
        return false_0;
    };
    let Ok(number) = number_text.parse::<::core::ffi::c_double>() else {
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
    input_buffer.offset = input_buffer
        .offset
        .wrapping_add(number_string_length as size_t);
    return true_0;
}
pub fn cJSON_SetNumberHelper(
    object: Option<&mut cJSON>,
    mut number: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let object = match object {
        Some(object) => object,
        None => return number,
    };
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
    cJSON_SetNumberHelper(object.as_mut(), number)
}
enum SetValuestringAction {
    Fail,
    CopyIntoExisting,
    AllocateNew,
}
fn cJSON_SetValuestring(
    object: Option<&cJSON>,
    valuestring: Option<&::std::ffi::CStr>,
    current_valuestring_len: Option<size_t>,
    storage_is_disjoint: bool,
) -> SetValuestringAction {
    let object = match object {
        Some(object) => object,
        None => return SetValuestringAction::Fail,
    };
    if object.type_0 & cJSON_String == 0 || object.type_0 & cJSON_IsReference != 0 {
        return SetValuestringAction::Fail;
    }
    let valuestring = match valuestring {
        Some(valuestring) => valuestring,
        None => return SetValuestringAction::Fail,
    };
    let current_valuestring_len = match current_valuestring_len {
        Some(current_valuestring_len) => current_valuestring_len,
        None => return SetValuestringAction::Fail,
    };
    if valuestring.to_bytes().len() <= current_valuestring_len {
        if !storage_is_disjoint {
            return SetValuestringAction::Fail;
        }
        return SetValuestringAction::CopyIntoExisting;
    }
    SetValuestringAction::AllocateNew
}
#[export_name = "cJSON_SetValuestring"]
pub unsafe extern "C" fn cJSON_SetValuestring_ffi(
    mut object: *mut cJSON,
    mut valuestring: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let object_ref = object.as_ref();
    let object_ref = match object_ref {
        Some(object)
            if object.type_0 & cJSON_String != 0 && object.type_0 & cJSON_IsReference == 0 =>
        {
            object
        }
        _ => return ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    if object_ref.valuestring.is_null() || valuestring.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let valuestring_cstr = Some(::std::ffi::CStr::from_ptr(valuestring));
    let current_valuestring_len = Some(
        ::std::ffi::CStr::from_ptr(object_ref.valuestring)
            .to_bytes()
            .len(),
    );
    let valuestring_len = valuestring_cstr
        .expect("validated value string")
        .to_bytes()
        .len();
    let current_valuestring_len_value = current_valuestring_len.expect("validated value string");
    let storage_is_disjoint = valuestring.wrapping_add(valuestring_len)
        < object_ref.valuestring as *const ::core::ffi::c_char
        || object_ref
            .valuestring
            .wrapping_add(current_valuestring_len_value)
            < valuestring as *mut ::core::ffi::c_char;
    match cJSON_SetValuestring(
        Some(object_ref),
        valuestring_cstr,
        current_valuestring_len,
        storage_is_disjoint,
    ) {
        SetValuestringAction::Fail => ::core::ptr::null_mut::<::core::ffi::c_char>(),
        SetValuestringAction::CopyIntoExisting => {
            let object = object.as_ref().expect("validated object");
            let valuestring_cstr = valuestring_cstr.expect("validated value string");
            let current_valuestring_len = current_valuestring_len.expect("validated value string");
            let source = valuestring_cstr.to_bytes_with_nul();
            let destination = ::std::slice::from_raw_parts_mut(
                object.valuestring as *mut u8,
                current_valuestring_len.wrapping_add(1 as size_t),
            );
            destination[..source.len()].copy_from_slice(source);
            object.valuestring
        }
        SetValuestringAction::AllocateNew => {
            let Some(mut new_valuestring_storage) =
                cstr_storage_from_cstr(valuestring_cstr.expect("validated value string"))
            else {
                return ::core::ptr::null_mut::<::core::ffi::c_char>();
            };
            let new_valuestring = new_valuestring_storage.as_mut_ptr() as *mut ::core::ffi::c_char;
            if !(*object).valuestring.is_null() {
                if (*object).valuestring_storage.take().is_none() {
                    let hooks = current_global_hooks();
                    hooks.deallocate.expect("non-null function pointer")(
                        (*object).valuestring as *mut ::core::ffi::c_void,
                    );
                }
            }
            let object = object.as_mut().expect("validated object");
            object.valuestring_storage = Some(new_valuestring_storage);
            object.valuestring = new_valuestring;
            object.valuestring
        }
    }
}
fn ensure(p: &mut printbuffer, needed: size_t) -> Option<&mut [::core::ffi::c_uchar]> {
    let mut newsize: size_t = 0 as size_t;
    if p.length > 0 as size_t && p.offset >= p.length {
        return None;
    }
    if needed > INT_MAX as size_t {
        return None;
    }
    let needed_with_offset = needed.wrapping_add(p.offset.wrapping_add(1 as size_t));
    if needed_with_offset > p.length {
        if p.noalloc != 0 {
            return None;
        }
        if needed_with_offset > (INT_MAX / 2 as ::core::ffi::c_int) as size_t {
            if needed_with_offset <= INT_MAX as size_t {
                newsize = INT_MAX as size_t;
            } else {
                return None;
            }
        } else {
            newsize = needed_with_offset.wrapping_mul(2 as size_t);
        }
        if !try_resize_vec(&mut p.buffer, newsize as usize, 0) {
            p.length = 0 as size_t;
            p.buffer.clear();
            return None;
        }
        p.length = newsize;
    }
    let start = p.offset as usize;
    let end = start.checked_add(needed as usize)?;
    return p.buffer.get_mut(start..end);
}
fn update_offset(buffer: &mut printbuffer) {
    if buffer.offset >= buffer.length {
        return;
    }
    let remaining = buffer.length.wrapping_sub(buffer.offset);
    let start = buffer.offset as usize;
    let end = start
        .saturating_add(remaining as usize)
        .min(buffer.buffer.len());
    let bytes = &buffer.buffer[start..end];
    let string_len = bytes
        .iter()
        .position(|byte| *byte == 0 as ::core::ffi::c_uchar)
        .unwrap_or(bytes.len());
    buffer.offset = buffer.offset.wrapping_add(string_len);
}
fn compare_double(mut a: ::core::ffi::c_double, mut b: ::core::ffi::c_double) -> cJSON_bool {
    let mut maxVal: ::core::ffi::c_double = if a.abs() > b.abs() { a.abs() } else { b.abs() };
    return ((a - b).abs() <= maxVal * DBL_EPSILON) as ::core::ffi::c_int;
}
fn trim_trailing_zero_fraction(mut number: String) -> String {
    if let Some(decimal_index) = number.find('.') {
        while number.ends_with('0') {
            number.pop();
        }
        if number.len() == decimal_index.wrapping_add(1) {
            number.pop();
        }
    }
    number
}
fn format_c_exponent(exponent: ::core::ffi::c_int) -> String {
    let sign = if exponent < 0 { '-' } else { '+' };
    let mut magnitude = exponent.abs() as usize;
    let mut digits = Vec::new();
    if magnitude == 0 {
        digits.push('0');
    }
    while magnitude > 0 {
        digits.push((b'0' + (magnitude % 10) as u8) as char);
        magnitude /= 10;
    }
    let mut output = String::new();
    output.push('e');
    output.push(sign);
    if digits.len() < 2 {
        output.push('0');
    }
    for digit in digits.iter().rev() {
        output.push(*digit);
    }
    output
}
fn c_g_from_scientific(scientific: &str, precision: usize) -> Option<String> {
    let (mantissa, exponent_text) = scientific.split_once('e')?;
    let exponent = exponent_text.parse::<::core::ffi::c_int>().ok()?;
    let mut sign = "";
    let mut mantissa_digits = mantissa;
    if let Some(stripped) = mantissa_digits.strip_prefix('-') {
        sign = "-";
        mantissa_digits = stripped;
    }
    let mut digits = String::with_capacity(precision);
    for byte in mantissa_digits.bytes() {
        if byte != b'.' {
            digits.push(byte as char);
        }
    }
    if exponent < -4 || exponent >= precision as ::core::ffi::c_int {
        let mut mantissa_output = String::new();
        mantissa_output.push_str(sign);
        if let Some(first_digit) = digits.chars().next() {
            mantissa_output.push(first_digit);
        } else {
            return None;
        }
        let fraction = digits.get(1..).unwrap_or("");
        if !fraction.is_empty() {
            mantissa_output.push('.');
            mantissa_output.push_str(fraction);
        }
        let mut output = trim_trailing_zero_fraction(mantissa_output);
        output.push_str(&format_c_exponent(exponent));
        return Some(output);
    }

    let mut output = String::new();
    output.push_str(sign);
    if exponent >= 0 {
        let integer_digits = exponent as usize + 1;
        if integer_digits >= digits.len() {
            output.push_str(&digits);
            for _ in digits.len()..integer_digits {
                output.push('0');
            }
            return Some(output);
        }
        output.push_str(&digits[..integer_digits]);
        output.push('.');
        output.push_str(&digits[integer_digits..]);
        return Some(trim_trailing_zero_fraction(output));
    }

    output.push_str("0.");
    for _ in 0..((-exponent - 1) as usize) {
        output.push('0');
    }
    output.push_str(&digits);
    Some(trim_trailing_zero_fraction(output))
}
fn print_number(item: &cJSON, output_buffer: &mut printbuffer) -> cJSON_bool {
    use ::std::fmt::Write;

    let mut d: ::core::ffi::c_double = item.valuedouble;
    let number = if !d.is_finite() {
        "null".to_owned()
    } else if d == item.valueint as ::core::ffi::c_double {
        item.valueint.to_string()
    } else {
        let mut scientific_15 = String::new();
        if write!(&mut scientific_15, "{:.14e}", d).is_err() {
            return false_0;
        }
        let Some(number_15) = c_g_from_scientific(&scientific_15, 15) else {
            return false_0;
        };
        let use_17 = match number_15.parse::<::core::ffi::c_double>() {
            Ok(test) => compare_double(test, d) == 0,
            Err(_) => true,
        };
        if use_17 {
            let mut scientific_17 = String::new();
            if write!(&mut scientific_17, "{:.16e}", d).is_err() {
                return false_0;
            }
            let Some(number_17) = c_g_from_scientific(&scientific_17, 17) else {
                return false_0;
            };
            number_17
        } else {
            number_15
        }
    };
    let number_bytes = number.as_bytes();
    if number_bytes.len()
        > (::core::mem::size_of::<[::core::ffi::c_uchar; 26]>() as usize).wrapping_sub(1)
    {
        return false_0;
    }
    let Some(output) = ensure(
        output_buffer,
        (number_bytes.len() as size_t)
            .wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 1]>() as size_t),
    ) else {
        return false_0;
    };
    output[..number_bytes.len()].copy_from_slice(number_bytes);
    output[number_bytes.len()] = '\0' as i32 as ::core::ffi::c_uchar;
    output_buffer.offset = output_buffer
        .offset
        .wrapping_add(number_bytes.len() as size_t);
    return true_0;
}
fn parse_hex4(input: &[::core::ffi::c_uchar]) -> ::core::ffi::c_uint {
    if input.len() < 4 {
        return 0 as ::core::ffi::c_uint;
    }
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
) -> Option<(
    ::core::ffi::c_uchar,
    [::core::ffi::c_uchar; 4],
    ::core::ffi::c_uchar,
)> {
    let mut c2rust_current_block: u64;
    let mut codepoint: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    let mut first_code: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut utf8_length: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
    let mut utf8_position: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
    let mut sequence_length: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
    let mut first_byte_mark: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
    let mut output = [0 as ::core::ffi::c_uchar; 4];
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
                } else if input[6 as usize] as ::core::ffi::c_int != '\\' as i32
                    || input[7 as usize] as ::core::ffi::c_int != 'u' as i32
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
                                output[utf8_position as usize] = ((codepoint
                                    | 0x80 as ::core::ffi::c_ulong)
                                    & 0xbf as ::core::ffi::c_ulong)
                                    as ::core::ffi::c_uchar;
                                codepoint >>= 6 as ::core::ffi::c_int;
                                utf8_position = utf8_position.wrapping_sub(1);
                            }
                            if utf8_length as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                output[0 as usize] = ((codepoint
                                    | first_byte_mark as ::core::ffi::c_ulong)
                                    & 0xff as ::core::ffi::c_ulong)
                                    as ::core::ffi::c_uchar;
                            } else {
                                output[0 as usize] = (codepoint & 0x7f as ::core::ffi::c_ulong)
                                    as ::core::ffi::c_uchar;
                            }
                            return Some((sequence_length, output, utf8_length));
                        }
                    }
                }
            }
        }
    }
    return None;
}
fn find_string_end(
    input: &[::core::ffi::c_uchar],
    input_offset: size_t,
) -> Option<(usize, size_t)> {
    let mut input_end = (input_offset as usize).wrapping_add(1);
    let mut skipped_bytes = 0 as size_t;
    while input_end < input.len() && input[input_end] as ::core::ffi::c_int != '"' as i32 {
        if input[input_end] as ::core::ffi::c_int == '\\' as i32 {
            if input_end.wrapping_add(1) >= input.len() {
                return None;
            }
            skipped_bytes = skipped_bytes.wrapping_add(1);
            input_end = input_end.wrapping_add(1);
        }
        input_end = input_end.wrapping_add(1);
    }
    if input_end >= input.len() || input[input_end] as ::core::ffi::c_int != '"' as i32 {
        return None;
    }
    Some((input_end, skipped_bytes))
}
fn parse_string(
    item: &mut cJSON,
    input_buffer: &mut parse_buffer,
    input: &[::core::ffi::c_uchar],
) -> cJSON_bool {
    let input_offset = input_buffer.offset;
    let mut input_pointer_offset = input_offset.wrapping_add(1 as size_t);
    if input.get(input_offset as usize) == Some(&('"' as i32 as ::core::ffi::c_uchar)) {
        if let Some((input_end, skipped_bytes)) = find_string_end(input, input_offset) {
            let allocation_length = (input_end as size_t)
                .wrapping_sub(input_offset)
                .wrapping_sub(skipped_bytes);
            let allocation_size = allocation_length
                .wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 1]>() as size_t);
            let mut output = Vec::new();
            if output.try_reserve_exact(allocation_size as usize).is_ok() {
                let input_start = (input_offset as usize).wrapping_add(1);
                let string_input = &input[input_start..input_end];
                let mut input_index = 0usize;
                let mut parse_failed = false;
                while input_index < string_input.len() {
                    input_pointer_offset = input_start.wrapping_add(input_index) as size_t;
                    if string_input[input_index] as ::core::ffi::c_int != '\\' as i32 {
                        output.push(string_input[input_index]);
                        input_index = input_index.wrapping_add(1);
                        continue;
                    }

                    let remaining = &string_input[input_index..];
                    if remaining.len() < 2 {
                        parse_failed = true;
                        break;
                    }
                    let mut sequence_length = 2 as usize;
                    match remaining[1] as ::core::ffi::c_int {
                        98 => output.push(b'\x08'),
                        102 => output.push(b'\x0c'),
                        110 => output.push(b'\n'),
                        114 => output.push(b'\r'),
                        116 => output.push(b'\t'),
                        34 | 92 | 47 => output.push(remaining[1]),
                        117 => {
                            if let Some((parsed_sequence_length, utf8_bytes, utf8_length)) =
                                utf16_literal_to_utf8(remaining)
                            {
                                sequence_length = parsed_sequence_length as usize;
                                for utf8_index in 0..utf8_length as usize {
                                    output.push(utf8_bytes[utf8_index]);
                                }
                                input_index = input_index.wrapping_add(sequence_length);
                                continue;
                            }
                            parse_failed = true;
                            break;
                        }
                        _ => {
                            parse_failed = true;
                            break;
                        }
                    }
                    input_index = input_index.wrapping_add(sequence_length);
                }
                if !parse_failed {
                    output.push(b'\0');
                    item.type_0 = cJSON_String;
                    item.valuestring_storage = Some(output);
                    item.valuestring =
                        item.valuestring_storage
                            .as_mut()
                            .expect("valuestring storage was just set")
                            .as_mut_ptr() as *mut ::core::ffi::c_char;
                    input_buffer.offset = (input_end as size_t).wrapping_add(1 as size_t);
                    return true_0;
                }
            }
        }
    }
    input_buffer.offset = input_pointer_offset;
    return false_0;
}
fn print_string_ptr(
    input: Option<&::std::ffi::CStr>,
    output_buffer: &mut printbuffer,
) -> cJSON_bool {
    let mut escape_characters: size_t = 0 as size_t;
    let Some(input) = input else {
        let bytes = b"\"\"\0";
        let Some(output) = ensure(output_buffer, bytes.len() as size_t) else {
            return false_0;
        };
        output.copy_from_slice(bytes);
        return true_0;
    };
    let input_bytes = input.to_bytes();
    for byte in input_bytes {
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
    let output_length = (input_bytes.len() as size_t).wrapping_add(escape_characters);
    let Some(output) = ensure(
        output_buffer,
        output_length.wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t),
    ) else {
        return false_0;
    };
    if escape_characters == 0 as size_t {
        output[0] = '"' as i32 as ::core::ffi::c_uchar;
        output[1..input_bytes.len().wrapping_add(1)].copy_from_slice(input_bytes);
        output[output_length.wrapping_add(1) as usize] = '"' as i32 as ::core::ffi::c_uchar;
        output[output_length.wrapping_add(2) as usize] = '\0' as i32 as ::core::ffi::c_uchar;
        return true_0;
    }
    output[0] = '"' as i32 as ::core::ffi::c_uchar;
    let mut output_index = 1usize;
    for byte in input_bytes {
        if *byte as ::core::ffi::c_int > 31 as ::core::ffi::c_int
            && *byte as ::core::ffi::c_int != '"' as i32
            && *byte as ::core::ffi::c_int != '\\' as i32
        {
            output[output_index] = *byte;
            output_index = output_index.wrapping_add(1);
        } else {
            output[output_index] = '\\' as i32 as ::core::ffi::c_uchar;
            output_index = output_index.wrapping_add(1);
            match *byte as ::core::ffi::c_int {
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
                    let hex = b"0123456789abcdef";
                    output[output_index] = 'u' as i32 as ::core::ffi::c_uchar;
                    output[output_index.wrapping_add(1)] = '0' as i32 as ::core::ffi::c_uchar;
                    output[output_index.wrapping_add(2)] = '0' as i32 as ::core::ffi::c_uchar;
                    output[output_index.wrapping_add(3)] = hex[(*byte >> 4) as usize];
                    output[output_index.wrapping_add(4)] = hex[(*byte & 0xf) as usize];
                    output_index = output_index.wrapping_add(5);
                }
            }
        }
    }
    output[output_length.wrapping_add(1) as usize] = '"' as i32 as ::core::ffi::c_uchar;
    output[output_length.wrapping_add(2) as usize] = '\0' as i32 as ::core::ffi::c_uchar;
    return true_0;
}
fn print_string(item: &cJSON, p: &mut printbuffer) -> cJSON_bool {
    return print_string_ptr(valuestring_cstr(item), p);
}
fn buffer_skip_whitespace<'buffer, 'content>(
    buffer: &'buffer mut parse_buffer<'content>,
) -> &'buffer mut parse_buffer<'content> {
    if !buffer.can_access_at_offset(0 as size_t) {
        return buffer;
    }
    while buffer.can_access_at_offset(0 as size_t)
        && buffer.byte_at_offset(0 as size_t).unwrap() as ::core::ffi::c_int
            <= 32 as ::core::ffi::c_int
    {
        buffer.offset = buffer.offset.wrapping_add(1);
    }
    if buffer.offset == buffer.length {
        buffer.offset = buffer.offset.wrapping_sub(1);
    }
    return buffer;
}
fn skip_utf8_bom<'buffer, 'content>(
    buffer: &'buffer mut parse_buffer<'content>,
) -> Option<&'buffer mut parse_buffer<'content>> {
    if buffer.offset != 0 as size_t {
        return None;
    }
    if buffer.offset.wrapping_add(4 as size_t) < buffer.length
        && buffer.starts_with(b"\xEF\xBB\xBF")
    {
        buffer.offset = buffer.offset.wrapping_add(3 as size_t);
    }
    Some(buffer)
}
struct ParseWithLengthResult {
    item: Option<Box<cJSON>>,
    offset: size_t,
}

fn parse_with_length_opts(
    value: &[::core::ffi::c_uchar],
    hooks: internal_hooks,
    require_null_terminated: cJSON_bool,
) -> ParseWithLengthResult {
    let mut buffer: parse_buffer = parse_buffer {
        content: value,
        length: value.len(),
        offset: 0 as size_t,
        depth: 0 as size_t,
        hooks,
    };
    if !value.is_empty() {
        if let Some(mut item_ref) = cJSON_New_Item(&buffer.hooks) {
            let parse_started = match skip_utf8_bom(&mut buffer) {
                Some(input_buffer) => {
                    parse_value(item_ref.as_mut(), buffer_skip_whitespace(input_buffer)) != 0
                }
                None => false,
            };
            if parse_started {
                let parse_succeeded = if require_null_terminated != 0 {
                    buffer_skip_whitespace(&mut buffer);
                    buffer.offset < buffer.length
                        && buffer.content.get(buffer.offset)
                            == Some(&('\0' as i32 as ::core::ffi::c_uchar))
                } else {
                    true
                };
                if parse_succeeded {
                    return ParseWithLengthResult {
                        item: Some(item_ref),
                        offset: buffer.offset,
                    };
                }
            }
        }
    }

    let offset = if buffer.offset < buffer.length {
        buffer.offset
    } else if buffer.length > 0 as size_t {
        buffer.length.wrapping_sub(1 as size_t)
    } else {
        0 as size_t
    };
    ParseWithLengthResult { item: None, offset }
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
    global_error.store(0 as size_t, ::std::sync::atomic::Ordering::Relaxed);
    global_error_position.store(0 as size_t, ::std::sync::atomic::Ordering::Relaxed);

    let input = ::std::ffi::CStr::from_ptr(value);
    let result = parse_with_length_opts(
        input.to_bytes_with_nul(),
        current_global_hooks(),
        require_null_terminated,
    );
    if let Some(item) = result.item {
        if !return_parse_end.is_null() {
            *return_parse_end = value.wrapping_add(result.offset);
        }
        return Box::into_raw(item);
    }
    if !return_parse_end.is_null() {
        *return_parse_end = value.wrapping_add(result.offset);
    }
    global_error.store(
        value as *const ::core::ffi::c_uchar as size_t,
        ::std::sync::atomic::Ordering::Relaxed,
    );
    global_error_position.store(result.offset, ::std::sync::atomic::Ordering::Relaxed);
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_ParseWithLengthOpts"]
pub unsafe extern "C" fn cJSON_ParseWithLengthOpts_ffi(
    mut value: *const ::core::ffi::c_char,
    mut buffer_length: size_t,
    mut return_parse_end: *mut *const ::core::ffi::c_char,
    mut require_null_terminated: cJSON_bool,
) -> *mut cJSON {
    global_error.store(0 as size_t, ::std::sync::atomic::Ordering::Relaxed);
    global_error_position.store(0 as size_t, ::std::sync::atomic::Ordering::Relaxed);
    if value.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }

    let input = ::std::slice::from_raw_parts(value as *const ::core::ffi::c_uchar, buffer_length);
    let result = parse_with_length_opts(input, current_global_hooks(), require_null_terminated);
    if let Some(item) = result.item {
        if !return_parse_end.is_null() {
            *return_parse_end = value.wrapping_add(result.offset);
        }
        return Box::into_raw(item);
    }
    if !return_parse_end.is_null() {
        *return_parse_end = value.wrapping_add(result.offset);
    }
    global_error.store(
        value as *const ::core::ffi::c_uchar as size_t,
        ::std::sync::atomic::Ordering::Relaxed,
    );
    global_error_position.store(result.offset, ::std::sync::atomic::Ordering::Relaxed);
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_Parse"]
pub unsafe extern "C" fn cJSON_Parse_ffi(mut value: *const ::core::ffi::c_char) -> *mut cJSON {
    cJSON_ParseWithOpts_ffi(
        value,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        0 as cJSON_bool,
    )
}
#[export_name = "cJSON_ParseWithLength"]
pub unsafe extern "C" fn cJSON_ParseWithLength_ffi(
    mut value: *const ::core::ffi::c_char,
    mut buffer_length: size_t,
) -> *mut cJSON {
    cJSON_ParseWithLengthOpts_ffi(
        value,
        buffer_length,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        0 as cJSON_bool,
    )
}
fn print(
    item: Option<&cJSON>,
    format: cJSON_bool,
    _hooks: internal_hooks,
) -> Option<Vec<::core::ffi::c_uchar>> {
    let default_buffer_size: size_t = 256 as size_t;
    let mut buffer = printbuffer {
        buffer: Vec::new(),
        length: 0,
        offset: 0,
        depth: 0,
        noalloc: 0,
        format: 0,
    };
    if !try_resize_vec(&mut buffer.buffer, default_buffer_size as usize, 0) {
        return None;
    }
    buffer.length = default_buffer_size;
    buffer.format = format;
    if print_value(item, &mut buffer) != 0 {
        update_offset(&mut buffer);
        let printed_len = buffer.offset.wrapping_add(1 as size_t);
        if printed_len > INT_MAX as size_t || printed_len > buffer.buffer.len() {
            return None;
        }
        buffer.buffer.truncate(printed_len);
        if let Some(last) = buffer.buffer.last_mut() {
            *last = '\0' as i32 as ::core::ffi::c_uchar;
        }
        return Some(buffer.buffer);
    }
    return None;
}
pub fn cJSON_Print(
    item: Option<&cJSON>,
    hooks: internal_hooks,
) -> Option<Vec<::core::ffi::c_uchar>> {
    return print(item, true_0, hooks);
}
#[export_name = "cJSON_Print"]
pub unsafe extern "C" fn cJSON_Print_ffi(mut item: *const cJSON) -> *mut ::core::ffi::c_char {
    let hooks = current_global_hooks();
    let Some(printed) = cJSON_Print(item.as_ref(), hooks) else {
        return ::core::ptr::null_mut();
    };
    let output = hooks.allocate.expect("non-null function pointer")(printed.len() as size_t)
        as *mut ::core::ffi::c_char;
    if output.is_null() {
        return ::core::ptr::null_mut();
    }
    let destination = ::std::slice::from_raw_parts_mut(output as *mut u8, printed.len());
    destination.copy_from_slice(&printed);
    return output;
}
pub fn cJSON_PrintUnformatted(
    item: Option<&cJSON>,
    hooks: internal_hooks,
) -> Option<Vec<::core::ffi::c_uchar>> {
    return print(item, false_0, hooks);
}
#[export_name = "cJSON_PrintUnformatted"]
pub unsafe extern "C" fn cJSON_PrintUnformatted_ffi(
    mut item: *const cJSON,
) -> *mut ::core::ffi::c_char {
    let hooks = current_global_hooks();
    let Some(printed) = cJSON_PrintUnformatted(item.as_ref(), hooks) else {
        return ::core::ptr::null_mut();
    };
    let output = hooks.allocate.expect("non-null function pointer")(printed.len() as size_t)
        as *mut ::core::ffi::c_char;
    if output.is_null() {
        return ::core::ptr::null_mut();
    }
    let destination = ::std::slice::from_raw_parts_mut(output as *mut u8, printed.len());
    destination.copy_from_slice(&printed);
    return output;
}
pub fn cJSON_PrintBuffered(
    item: Option<&cJSON>,
    prebuffer: ::core::ffi::c_int,
    fmt: cJSON_bool,
    _hooks: internal_hooks,
) -> Option<Vec<::core::ffi::c_uchar>> {
    let mut p: printbuffer = printbuffer {
        buffer: Vec::new(),
        length: 0 as size_t,
        offset: 0 as size_t,
        depth: 0 as size_t,
        noalloc: 0 as cJSON_bool,
        format: 0 as cJSON_bool,
    };
    if prebuffer < 0 as ::core::ffi::c_int {
        return None;
    }
    if !try_resize_vec(&mut p.buffer, prebuffer as usize, 0) {
        return None;
    }
    p.length = prebuffer as size_t;
    p.offset = 0 as size_t;
    p.noalloc = false_0;
    p.format = fmt;
    if print_value(item, &mut p) == 0 {
        return None;
    }
    return Some(p.buffer);
}
#[export_name = "cJSON_PrintBuffered"]
pub unsafe extern "C" fn cJSON_PrintBuffered_ffi(
    mut item: *const cJSON,
    mut prebuffer: ::core::ffi::c_int,
    mut fmt: cJSON_bool,
) -> *mut ::core::ffi::c_char {
    let hooks = current_global_hooks();
    let Some(printed) = cJSON_PrintBuffered(item.as_ref(), prebuffer, fmt, hooks) else {
        return ::core::ptr::null_mut();
    };
    let output = hooks.allocate.expect("non-null function pointer")(printed.len() as size_t)
        as *mut ::core::ffi::c_char;
    if output.is_null() {
        return ::core::ptr::null_mut();
    }
    let destination = ::std::slice::from_raw_parts_mut(output as *mut u8, printed.len());
    destination.copy_from_slice(&printed);
    return output;
}
pub fn cJSON_PrintPreallocated(
    item: Option<&cJSON>,
    buffer: &mut [::core::ffi::c_char],
    format: cJSON_bool,
    _hooks: internal_hooks,
) -> cJSON_bool {
    let mut p: printbuffer = printbuffer {
        buffer: Vec::new(),
        length: 0 as size_t,
        offset: 0 as size_t,
        depth: 0 as size_t,
        noalloc: 0 as cJSON_bool,
        format: 0 as cJSON_bool,
    };
    if !try_resize_vec(&mut p.buffer, buffer.len(), 0) {
        return false_0;
    }
    p.length = buffer.len() as size_t;
    p.offset = 0 as size_t;
    p.noalloc = true_0;
    p.format = format;
    let printed = print_value(item, &mut p);
    for (destination, source) in buffer.iter_mut().zip(p.buffer.iter()) {
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
    let buffer = ::std::slice::from_raw_parts_mut(buffer, length as usize);
    cJSON_PrintPreallocated(item.as_ref(), buffer, format, current_global_hooks())
}
fn parse_value(item: &mut cJSON, input_buffer: &mut parse_buffer) -> cJSON_bool {
    if input_buffer.starts_with(b"null") {
        item.type_0 = cJSON_NULL;
        input_buffer.offset = input_buffer.offset.wrapping_add(4 as size_t);
        return true_0;
    }
    if input_buffer.starts_with(b"false") {
        item.type_0 = cJSON_False;
        input_buffer.offset = input_buffer.offset.wrapping_add(5 as size_t);
        return true_0;
    }
    if input_buffer.starts_with(b"true") {
        item.type_0 = cJSON_True;
        item.valueint = 1 as ::core::ffi::c_int;
        input_buffer.offset = input_buffer.offset.wrapping_add(4 as size_t);
        return true_0;
    }
    if input_buffer.byte_at_offset(0 as size_t) == Some('"' as i32 as ::core::ffi::c_uchar) {
        return parse_string(item, input_buffer, input_buffer.content);
    }
    if input_buffer
        .byte_at_offset(0 as size_t)
        .is_some_and(|byte| {
            byte as ::core::ffi::c_int == '-' as i32
                || byte as ::core::ffi::c_int >= '0' as i32
                    && byte as ::core::ffi::c_int <= '9' as i32
        })
    {
        return parse_number(item, input_buffer);
    }
    if input_buffer.byte_at_offset(0 as size_t) == Some('[' as i32 as ::core::ffi::c_uchar) {
        return parse_array(item, input_buffer);
    }
    if input_buffer.byte_at_offset(0 as size_t) == Some('{' as i32 as ::core::ffi::c_uchar) {
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
            let Some(output) = ensure(output_buffer, bytes.len() as size_t) else {
                return false_0;
            };
            output.copy_from_slice(bytes);
            return true_0;
        }
        cJSON_False => {
            let bytes = b"false\0";
            let Some(output) = ensure(output_buffer, bytes.len() as size_t) else {
                return false_0;
            };
            output.copy_from_slice(bytes);
            return true_0;
        }
        cJSON_True => {
            let bytes = b"true\0";
            let Some(output) = ensure(output_buffer, bytes.len() as size_t) else {
                return false_0;
            };
            output.copy_from_slice(bytes);
            return true_0;
        }
        cJSON_Number => return print_number(item, output_buffer),
        cJSON_Raw => {
            if item.valuestring.is_null() {
                return false_0;
            }
            let Some(raw) = valuestring_cstr(item) else {
                return false_0;
            };
            let raw = raw.to_bytes_with_nul();
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
fn parse_array(item: &mut cJSON, input_buffer: &mut parse_buffer) -> cJSON_bool {
    if input_buffer.depth >= CJSON_NESTING_LIMIT as size_t {
        return false_0;
    }
    input_buffer.depth = input_buffer.depth.wrapping_add(1);
    if input_buffer.byte_at_offset(0 as size_t) != Some('[' as i32 as ::core::ffi::c_uchar) {
        return false_0;
    }

    item.children.clear();
    item.child = ::core::ptr::null_mut::<cJSON>();
    input_buffer.offset = input_buffer.offset.wrapping_add(1);
    buffer_skip_whitespace(input_buffer);
    if input_buffer.byte_at_offset(0 as size_t) == Some(']' as i32 as ::core::ffi::c_uchar) {
        input_buffer.depth = input_buffer.depth.wrapping_sub(1);
        item.type_0 = cJSON_Array;
        input_buffer.offset = input_buffer.offset.wrapping_add(1);
        return true_0;
    }
    if !input_buffer.can_access_at_offset(0 as size_t) {
        input_buffer.offset = input_buffer.offset.wrapping_sub(1);
        return false_0;
    }
    input_buffer.offset = input_buffer.offset.wrapping_sub(1);
    loop {
        let Some(mut new_item_ref) = cJSON_New_Item(&input_buffer.hooks) else {
            item.children.clear();
            item.child = ::core::ptr::null_mut::<cJSON>();
            return false_0;
        };
        input_buffer.offset = input_buffer.offset.wrapping_add(1);
        buffer_skip_whitespace(input_buffer);
        if parse_value(new_item_ref.as_mut(), input_buffer) == 0 {
            item.children.clear();
            item.child = ::core::ptr::null_mut::<cJSON>();
            return false_0;
        }
        if add_item_to_array(item, new_item_ref).is_err() {
            item.children.clear();
            item.child = ::core::ptr::null_mut::<cJSON>();
            return false_0;
        }
        buffer_skip_whitespace(input_buffer);
        if input_buffer.byte_at_offset(0 as size_t) != Some(',' as i32 as ::core::ffi::c_uchar) {
            break;
        }
    }
    if input_buffer.byte_at_offset(0 as size_t) != Some(']' as i32 as ::core::ffi::c_uchar) {
        item.children.clear();
        item.child = ::core::ptr::null_mut::<cJSON>();
        return false_0;
    }
    input_buffer.depth = input_buffer.depth.wrapping_sub(1);
    item.type_0 = cJSON_Array;
    input_buffer.offset = input_buffer.offset.wrapping_add(1);
    return true_0;
}
fn print_array(item: &cJSON, output_buffer: &mut printbuffer) -> cJSON_bool {
    let mut length: size_t = 0 as size_t;
    let Some(output) = ensure(output_buffer, 1 as size_t) else {
        return false_0;
    };
    output[0] = '[' as i32 as ::core::ffi::c_uchar;
    output_buffer.offset = output_buffer.offset.wrapping_add(1);
    output_buffer.depth = output_buffer.depth.wrapping_add(1);
    let mut index: size_t = 0;
    while let Some(element) = get_array_item(Some(item), index) {
        if print_value(Some(element), output_buffer) == 0 {
            return false_0;
        }
        update_offset(output_buffer);
        if get_array_item(Some(item), index.wrapping_add(1)).is_some() {
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
        index = index.wrapping_add(1);
    }
    let Some(output) = ensure(output_buffer, 2 as size_t) else {
        return false_0;
    };
    output[0] = ']' as i32 as ::core::ffi::c_uchar;
    output[1] = '\0' as i32 as ::core::ffi::c_uchar;
    output_buffer.depth = output_buffer.depth.wrapping_sub(1);
    return true_0;
}
fn parse_object(item: &mut cJSON, input_buffer: &mut parse_buffer) -> cJSON_bool {
    if input_buffer.depth >= CJSON_NESTING_LIMIT as size_t {
        return false_0;
    }
    input_buffer.depth = input_buffer.depth.wrapping_add(1);
    if !input_buffer.can_access_at_offset(0 as size_t)
        || input_buffer.byte_at_offset(0 as size_t) != Some('{' as i32 as ::core::ffi::c_uchar)
    {
        return false_0;
    }

    item.children.clear();
    item.child = ::core::ptr::null_mut::<cJSON>();
    input_buffer.offset = input_buffer.offset.wrapping_add(1);
    buffer_skip_whitespace(input_buffer);
    if input_buffer.byte_at_offset(0 as size_t) == Some('}' as i32 as ::core::ffi::c_uchar) {
        input_buffer.depth = input_buffer.depth.wrapping_sub(1);
        item.type_0 = cJSON_Object;
        input_buffer.offset = input_buffer.offset.wrapping_add(1);
        return true_0;
    }
    if !input_buffer.can_access_at_offset(0 as size_t) {
        input_buffer.offset = input_buffer.offset.wrapping_sub(1);
        return false_0;
    }
    input_buffer.offset = input_buffer.offset.wrapping_sub(1);
    loop {
        let Some(mut new_item_ref) = cJSON_New_Item(&input_buffer.hooks) else {
            item.children.clear();
            item.child = ::core::ptr::null_mut::<cJSON>();
            return false_0;
        };
        if input_buffer.offset.wrapping_add(1 as size_t) >= input_buffer.length {
            item.children.clear();
            item.child = ::core::ptr::null_mut::<cJSON>();
            return false_0;
        }
        input_buffer.offset = input_buffer.offset.wrapping_add(1);
        buffer_skip_whitespace(input_buffer);
        if parse_string(new_item_ref.as_mut(), input_buffer, input_buffer.content) == 0 {
            item.children.clear();
            item.child = ::core::ptr::null_mut::<cJSON>();
            return false_0;
        }
        buffer_skip_whitespace(input_buffer);
        let new_item = new_item_ref.as_mut();
        new_item.string = new_item.valuestring;
        new_item.string_storage = new_item.valuestring_storage.take();
        if let Some(storage) = new_item.string_storage.as_mut() {
            new_item.string = storage.as_mut_ptr() as *mut ::core::ffi::c_char;
        }
        new_item.valuestring = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if input_buffer.byte_at_offset(0 as size_t) != Some(':' as i32 as ::core::ffi::c_uchar) {
            item.children.clear();
            item.child = ::core::ptr::null_mut::<cJSON>();
            return false_0;
        }
        input_buffer.offset = input_buffer.offset.wrapping_add(1);
        buffer_skip_whitespace(input_buffer);
        if parse_value(new_item_ref.as_mut(), input_buffer) == 0 {
            item.children.clear();
            item.child = ::core::ptr::null_mut::<cJSON>();
            return false_0;
        }
        if add_item_to_array(item, new_item_ref).is_err() {
            item.children.clear();
            item.child = ::core::ptr::null_mut::<cJSON>();
            return false_0;
        }
        buffer_skip_whitespace(input_buffer);
        if input_buffer.byte_at_offset(0 as size_t) != Some(',' as i32 as ::core::ffi::c_uchar) {
            break;
        }
    }
    if input_buffer.byte_at_offset(0 as size_t) != Some('}' as i32 as ::core::ffi::c_uchar) {
        item.children.clear();
        item.child = ::core::ptr::null_mut::<cJSON>();
        return false_0;
    }
    input_buffer.depth = input_buffer.depth.wrapping_sub(1);
    item.type_0 = cJSON_Object;
    input_buffer.offset = input_buffer.offset.wrapping_add(1);
    return true_0;
}
fn print_object(item: &cJSON, output_buffer: &mut printbuffer) -> cJSON_bool {
    let mut length: size_t = 0 as size_t;
    let object = item;
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
    let mut index: size_t = 0;
    while let Some(item) = get_array_item(Some(object), index) {
        if formatted {
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
        if print_string_ptr(string_cstr(item), output_buffer) == 0 {
            return false_0;
        }
        update_offset(output_buffer);
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
        if print_value(Some(item), output_buffer) == 0 {
            return false_0;
        }
        update_offset(output_buffer);
        let has_next = get_array_item(Some(object), index.wrapping_add(1)).is_some();
        length = ((if formatted {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as size_t)
            .wrapping_add((if has_next { 1 } else { 0 }) as size_t);
        let Some(output) = ensure(output_buffer, length.wrapping_add(1 as size_t)) else {
            return false_0;
        };
        let mut output_index = 0usize;
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
        index = index.wrapping_add(1);
    }
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
    if let Some(array_ref) = array.as_ref() {
        if array_ref.children.is_empty() && !array_ref.child.is_null() {
            let mut size: size_t = 0;
            let mut current_child = array_ref.child;
            while !current_child.is_null() {
                let child = &*current_child;
                size = size.wrapping_add(1);
                current_child = child.next;
            }
            return size as ::core::ffi::c_int;
        }
    }
    cJSON_GetArraySize(array.as_ref())
}
fn get_array_item(array: Option<&cJSON>, mut index: size_t) -> Option<&cJSON> {
    let Some(array) = array else {
        return None;
    };
    array.children.get(index).map(Box::as_ref)
}
pub fn cJSON_GetArrayItem(array: Option<&cJSON>, mut index: ::core::ffi::c_int) -> Option<&cJSON> {
    if index < 0 as ::core::ffi::c_int {
        return None;
    }
    get_array_item(array, index as size_t)
}
#[export_name = "cJSON_GetArrayItem"]
pub unsafe extern "C" fn cJSON_GetArrayItem_ffi(
    mut array: *const cJSON,
    mut index: ::core::ffi::c_int,
) -> *mut cJSON {
    if index < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<cJSON>();
    }
    if let Some(array_ref) = array.as_ref() {
        if array_ref.children.is_empty() && !array_ref.child.is_null() {
            let mut current_child = array_ref.child;
            let mut remaining = index as size_t;
            while !current_child.is_null() {
                let child = &*current_child;
                if remaining == 0 as size_t {
                    return current_child;
                }
                remaining = remaining.wrapping_sub(1);
                current_child = child.next;
            }
            return ::core::ptr::null_mut::<cJSON>();
        }
    }
    match cJSON_GetArrayItem(array.as_ref(), index) {
        Some(item) => item as *const cJSON as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
fn get_object_item<'a>(
    object: Option<&'a cJSON>,
    name: Option<&::std::ffi::CStr>,
    case_sensitive: cJSON_bool,
) -> Option<&'a cJSON> {
    let Some(object) = object else {
        return None;
    };
    let Some(name) = name else {
        return None;
    };

    let mut index: size_t = 0;
    while let Some(current) = get_array_item(Some(object), index) {
        if current.string.is_null() {
            if case_sensitive != 0 {
                break;
            }
            index = index.wrapping_add(1);
            continue;
        }

        let Some(current_name) = string_cstr(current) else {
            if case_sensitive != 0 {
                break;
            }
            index = index.wrapping_add(1);
            continue;
        };
        let comparison = if case_sensitive != 0 {
            cstr_cmp(Some(name), Some(current_name))
        } else {
            case_insensitive_cstr_cmp(Some(name), Some(current_name))
        };
        if comparison == 0 as ::core::ffi::c_int {
            return Some(current);
        }
        index = index.wrapping_add(1);
    }
    None
}
pub fn cJSON_GetObjectItem<'a>(
    object: Option<&'a cJSON>,
    string: Option<&::std::ffi::CStr>,
) -> Option<&'a cJSON> {
    get_object_item(object, string, false_0)
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
    if let (Some(object_ref), Some(name)) = (object.as_ref(), string) {
        if object_ref.children.is_empty() && !object_ref.child.is_null() {
            let mut current_child = object_ref.child;
            while !current_child.is_null() {
                let current = &*current_child;
                if let Some(current_name) = string_cstr(current) {
                    if case_insensitive_cstr_cmp(Some(name), Some(current_name))
                        == 0 as ::core::ffi::c_int
                    {
                        return current_child;
                    }
                }
                current_child = current.next;
            }
            return ::core::ptr::null_mut::<cJSON>();
        }
    }
    match cJSON_GetObjectItem(object.as_ref(), string) {
        Some(item) => item as *const cJSON as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_GetObjectItemCaseSensitive<'a>(
    object: Option<&'a cJSON>,
    string: Option<&::std::ffi::CStr>,
) -> Option<&'a cJSON> {
    get_object_item(object, string, true_0)
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
    if let (Some(object_ref), Some(name)) = (object.as_ref(), string) {
        if object_ref.children.is_empty() && !object_ref.child.is_null() {
            let mut current_child = object_ref.child;
            while !current_child.is_null() {
                let current = &*current_child;
                if let Some(current_name) = string_cstr(current) {
                    if cstr_cmp(Some(name), Some(current_name)) == 0 as ::core::ffi::c_int {
                        return current_child;
                    }
                }
                current_child = current.next;
            }
            return ::core::ptr::null_mut::<cJSON>();
        }
    }
    match cJSON_GetObjectItemCaseSensitive(object.as_ref(), string) {
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
    object: *const cJSON,
    string: *const ::core::ffi::c_char,
) -> cJSON_bool {
    if cJSON_GetObjectItem_ffi(object, string).is_null() {
        false_0
    } else {
        true_0
    }
}
fn create_reference(item: Option<&cJSON>, hooks: &internal_hooks) -> Option<Box<cJSON>> {
    let Some(item) = item else {
        return None;
    };
    let Some(mut reference_ref) = cJSON_New_Item(hooks) else {
        return None;
    };
    let reference = reference_ref.as_mut();
    *reference = item.clone();
    reference.string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    reference.type_0 |= cJSON_IsReference;
    reference.prev = ::core::ptr::null_mut::<cJSON>();
    reference.next = reference.prev;
    Some(reference_ref)
}
fn suffix_object(prev: &mut cJSON, item: &mut cJSON) {
    prev.next = item as *mut cJSON;
    item.prev = prev as *mut cJSON;
}
fn refresh_child_links(parent: &mut cJSON) {
    if parent.children.is_empty() {
        parent.child = ::core::ptr::null_mut::<cJSON>();
        return;
    }

    parent.child = parent.children[0].as_mut() as *mut cJSON;
    let last_index = parent.children.len().wrapping_sub(1);
    let last_ptr = parent.children[last_index].as_ref() as *const cJSON as *mut cJSON;
    for index in 0..parent.children.len() {
        let prev = if index == 0 {
            last_ptr
        } else {
            parent.children[index.wrapping_sub(1)].as_ref() as *const cJSON as *mut cJSON
        };
        let next = if index == last_index {
            ::core::ptr::null_mut::<cJSON>()
        } else {
            parent.children[index.wrapping_add(1)].as_ref() as *const cJSON as *mut cJSON
        };
        let child = parent.children[index].as_mut();
        child.prev = prev;
        child.next = next;
    }
}
enum AddItemError {
    Invalid(Box<cJSON>),
    RawList(Box<cJSON>),
}

fn add_item_to_array(array_ref: &mut cJSON, item: Box<cJSON>) -> Result<(), AddItemError> {
    array_ref.children.push(item);
    refresh_child_links(array_ref);
    Ok(())
}
pub fn cJSON_AddItemToArray(array: Option<&mut cJSON>, item: Option<Box<cJSON>>) -> cJSON_bool {
    let Some(array) = array else {
        return false_0;
    };
    let Some(item) = item else {
        return false_0;
    };
    match add_item_to_array(array, item) {
        Ok(()) => true_0,
        Err(_) => false_0,
    }
}
#[export_name = "cJSON_AddItemToArray"]
pub unsafe extern "C" fn cJSON_AddItemToArray_ffi(
    mut array: *mut cJSON,
    mut item: *mut cJSON,
) -> cJSON_bool {
    if array.is_null() || item.is_null() || array == item {
        return false_0;
    }
    let Some(array_ref) = array.as_mut() else {
        return false_0;
    };
    if array_ref.children.is_empty() && !array_ref.child.is_null() {
        let item_ref = &mut *item;
        let child = array_ref.child;
        if child.is_null() {
            array_ref.child = item;
            item_ref.prev = item;
            item_ref.next = ::core::ptr::null_mut::<cJSON>();
        } else {
            let child_ref = &mut *child;
            if !child_ref.prev.is_null() {
                let prev_ref = &mut *child_ref.prev;
                suffix_object(prev_ref, item_ref);
                child_ref.prev = item;
            }
        }
        return true_0;
    }
    let item_box = Box::from_raw(item);
    match add_item_to_array(array_ref, item_box) {
        Ok(()) => true_0,
        Err(AddItemError::Invalid(item_box)) => {
            let _ = Box::into_raw(item_box);
            false_0
        }
        Err(AddItemError::RawList(item_box)) => {
            let item = Box::into_raw(item_box);
            let item_ref = &mut *item;
            let child = array_ref.child;
            if child.is_null() {
                array_ref.child = item;
                item_ref.prev = item;
                item_ref.next = ::core::ptr::null_mut::<cJSON>();
            } else {
                let child_ref = &mut *child;
                if !child_ref.prev.is_null() {
                    let prev_ref = &mut *child_ref.prev;
                    suffix_object(prev_ref, item_ref);
                    child_ref.prev = item;
                }
            }
            true_0
        }
    }
}
fn add_item_to_object(
    object: Option<&mut cJSON>,
    string: Option<&::std::ffi::CStr>,
    mut item: Box<cJSON>,
    constant_key: cJSON_bool,
) -> Result<(), AddItemError> {
    let Some(object_ref) = object else {
        return Err(AddItemError::Invalid(item));
    };
    let Some(string) = string else {
        return Err(AddItemError::Invalid(item));
    };
    let item_ref = item.as_mut();
    let item_ptr = item_ref as *mut cJSON;
    if object_ref as *mut cJSON == item_ptr {
        return Err(AddItemError::Invalid(item));
    }

    let new_key: *mut ::core::ffi::c_char;
    let new_type: ::core::ffi::c_int;
    let mut new_key_storage: Option<Vec<u8>> = None;

    if constant_key != 0 {
        let Some(storage) = cstr_storage_from_cstr(string) else {
            return Err(AddItemError::Invalid(item));
        };
        new_key_storage = Some(storage);
        new_key = new_key_storage
            .as_mut()
            .expect("string storage was just assigned")
            .as_mut_ptr() as *mut ::core::ffi::c_char;
        new_type = item.as_ref().type_0 | cJSON_StringIsConst;
    } else {
        let Some(storage) = cstr_storage_from_cstr(string) else {
            return Err(AddItemError::Invalid(item));
        };
        new_key_storage = Some(storage);
        new_key = new_key_storage
            .as_mut()
            .expect("string storage was just assigned")
            .as_mut_ptr() as *mut ::core::ffi::c_char;
        new_type = item.as_ref().type_0 & !cJSON_StringIsConst;
    }

    let item_ref = item.as_mut();
    item_ref.string_storage.take();
    item_ref.string = new_key;
    item_ref.string_storage = new_key_storage;
    item_ref.type_0 = new_type;

    add_item_to_array(object_ref, item)
}
pub fn cJSON_AddItemToObject(
    object: Option<&mut cJSON>,
    string: Option<&::std::ffi::CStr>,
    item: Option<Box<cJSON>>,
) -> cJSON_bool {
    let Some(item) = item else {
        return false_0;
    };
    match add_item_to_object(object, string, item, false_0) {
        Ok(()) => true_0,
        Err(_) => false_0,
    }
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
    let old_key = if (*item).type_0 & cJSON_StringIsConst == 0
        && !(*item).string.is_null()
        && (*item).string_storage.is_none()
    {
        Some((*item).string as *mut ::core::ffi::c_void)
    } else {
        None
    };
    let object_ref = &mut *object;
    if object_ref.children.is_empty() && !object_ref.child.is_null() {
        let item_ref = &mut *item;
        let Some(mut new_key_storage) = cstr_storage_from_cstr(::std::ffi::CStr::from_ptr(string))
        else {
            return false_0;
        };
        item_ref.string_storage.take();
        item_ref.string = new_key_storage.as_mut_ptr() as *mut ::core::ffi::c_char;
        item_ref.string_storage = Some(new_key_storage);
        item_ref.type_0 &= !cJSON_StringIsConst;

        let child = object_ref.child;
        if child.is_null() {
            object_ref.child = item;
            item_ref.prev = item;
            item_ref.next = ::core::ptr::null_mut::<cJSON>();
        } else {
            let child_ref = &mut *child;
            if !child_ref.prev.is_null() {
                let prev_ref = &mut *child_ref.prev;
                suffix_object(prev_ref, item_ref);
                child_ref.prev = item;
            }
        }
        if let Some(old_key) = old_key {
            let hooks = current_global_hooks();
            hooks.deallocate.expect("non-null function pointer")(old_key);
        }
        return true_0;
    }
    let item_box = Box::from_raw(item);
    let result = match add_item_to_object(
        object.as_mut(),
        Some(::std::ffi::CStr::from_ptr(string)),
        item_box,
        false_0,
    ) {
        Ok(()) => true_0,
        Err(AddItemError::Invalid(item_box)) => {
            let _ = Box::into_raw(item_box);
            false_0
        }
        Err(AddItemError::RawList(item_box)) => {
            let item = Box::into_raw(item_box);
            let item_ref = &mut *item;
            let object_ref = &mut *object;
            let child = object_ref.child;
            if child.is_null() {
                object_ref.child = item;
                item_ref.prev = item;
                item_ref.next = ::core::ptr::null_mut::<cJSON>();
            } else {
                let child_ref = &mut *child;
                if !child_ref.prev.is_null() {
                    let prev_ref = &mut *child_ref.prev;
                    suffix_object(prev_ref, item_ref);
                    child_ref.prev = item;
                }
            }
            true_0
        }
    };
    if result != 0 {
        if let Some(old_key) = old_key {
            let hooks = current_global_hooks();
            hooks.deallocate.expect("non-null function pointer")(old_key);
        }
    }
    result
}
pub fn cJSON_AddItemToObjectCS(
    object: Option<&mut cJSON>,
    string: Option<&::std::ffi::CStr>,
    item: Option<Box<cJSON>>,
) -> cJSON_bool {
    let Some(item) = item else {
        return false_0;
    };
    match add_item_to_object(object, string, item, true_0) {
        Ok(()) => true_0,
        Err(_) => false_0,
    }
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
    let old_key = if (*item).type_0 & cJSON_StringIsConst == 0
        && !(*item).string.is_null()
        && (*item).string_storage.is_none()
    {
        Some((*item).string as *mut ::core::ffi::c_void)
    } else {
        None
    };
    let object_ref = &mut *object;
    if object_ref.children.is_empty() && !object_ref.child.is_null() {
        let item_ref = &mut *item;
        let Some(mut new_key_storage) = cstr_storage_from_cstr(::std::ffi::CStr::from_ptr(string))
        else {
            return false_0;
        };
        item_ref.string_storage.take();
        item_ref.string = new_key_storage.as_mut_ptr() as *mut ::core::ffi::c_char;
        item_ref.string_storage = Some(new_key_storage);
        item_ref.type_0 |= cJSON_StringIsConst;

        let child = object_ref.child;
        if child.is_null() {
            object_ref.child = item;
            item_ref.prev = item;
            item_ref.next = ::core::ptr::null_mut::<cJSON>();
        } else {
            let child_ref = &mut *child;
            if !child_ref.prev.is_null() {
                let prev_ref = &mut *child_ref.prev;
                suffix_object(prev_ref, item_ref);
                child_ref.prev = item;
            }
        }
        if let Some(old_key) = old_key {
            let hooks = current_global_hooks();
            hooks.deallocate.expect("non-null function pointer")(old_key);
        }
        return true_0;
    }
    let item_box = Box::from_raw(item);
    let result = match add_item_to_object(
        object.as_mut(),
        Some(::std::ffi::CStr::from_ptr(string)),
        item_box,
        true_0,
    ) {
        Ok(()) => true_0,
        Err(AddItemError::Invalid(item_box)) => {
            let _ = Box::into_raw(item_box);
            false_0
        }
        Err(AddItemError::RawList(item_box)) => {
            let item = Box::into_raw(item_box);
            let item_ref = &mut *item;
            let object_ref = &mut *object;
            let child = object_ref.child;
            if child.is_null() {
                object_ref.child = item;
                item_ref.prev = item;
                item_ref.next = ::core::ptr::null_mut::<cJSON>();
            } else {
                let child_ref = &mut *child;
                if !child_ref.prev.is_null() {
                    let prev_ref = &mut *child_ref.prev;
                    suffix_object(prev_ref, item_ref);
                    child_ref.prev = item;
                }
            }
            true_0
        }
    };
    if result != 0 {
        if let Some(old_key) = old_key {
            let hooks = current_global_hooks();
            hooks.deallocate.expect("non-null function pointer")(old_key);
        }
    }
    result
}
pub fn cJSON_AddItemReferenceToArray(
    array: Option<&mut cJSON>,
    item: Option<&cJSON>,
) -> cJSON_bool {
    let Some(array) = array else {
        return false_0;
    };
    let hooks = current_global_hooks();
    let Some(reference) = create_reference(item, &hooks) else {
        return false_0;
    };
    return match add_item_to_array(array, reference) {
        Ok(()) => true_0,
        Err(_) => false_0,
    };
}
#[export_name = "cJSON_AddItemReferenceToArray"]
pub unsafe extern "C" fn cJSON_AddItemReferenceToArray_ffi(
    mut array: *mut cJSON,
    mut item: *mut cJSON,
) -> cJSON_bool {
    cJSON_AddItemReferenceToArray(array.as_mut(), item.as_ref())
}
pub fn cJSON_AddItemReferenceToObject(
    object: Option<&mut cJSON>,
    string: Option<&::std::ffi::CStr>,
    item: Option<&cJSON>,
) -> cJSON_bool {
    if object.is_none() || string.is_none() {
        return false_0;
    }
    let hooks = current_global_hooks();
    let Some(reference) = create_reference(item, &hooks) else {
        return false_0;
    };
    return match add_item_to_object(object, string, reference, false_0) {
        Ok(()) => true_0,
        Err(_) => false_0,
    };
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
        Some(::std::ffi::CStr::from_ptr(string))
    };
    cJSON_AddItemReferenceToObject(object.as_mut(), string, item.as_ref())
}
pub fn cJSON_AddNullToObject(
    object: Option<&mut cJSON>,
    name: Option<&::std::ffi::CStr>,
) -> *mut cJSON {
    let Some(null_ref) = cJSON_CreateNull() else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let mut null_ref = null_ref;
    let null: *mut cJSON = null_ref.as_mut();
    if add_item_to_object(object, name, null_ref, false_0).is_ok() {
        return null;
    }
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddNullToObject"]
pub unsafe extern "C" fn cJSON_AddNullToObject_ffi(
    mut object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let name = if name.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(name))
    };
    cJSON_AddNullToObject(object.as_mut(), name)
}
pub fn cJSON_AddTrueToObject(
    object: Option<&mut cJSON>,
    name: Option<&::std::ffi::CStr>,
) -> *mut cJSON {
    let Some(true_item_ref) = cJSON_CreateTrue() else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let mut true_item_ref = true_item_ref;
    let true_item: *mut cJSON = true_item_ref.as_mut();
    if add_item_to_object(object, name, true_item_ref, false_0).is_ok() {
        return true_item;
    }
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddTrueToObject"]
pub unsafe extern "C" fn cJSON_AddTrueToObject_ffi(
    mut object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let name = if name.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(name))
    };
    cJSON_AddTrueToObject(object.as_mut(), name)
}
pub fn cJSON_AddFalseToObject(
    object: Option<&mut cJSON>,
    name: Option<&::std::ffi::CStr>,
) -> *mut cJSON {
    let Some(false_item_ref) = cJSON_CreateFalse() else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let mut false_item_ref = false_item_ref;
    let false_item: *mut cJSON = false_item_ref.as_mut();
    if add_item_to_object(object, name, false_item_ref, false_0).is_ok() {
        return false_item;
    }
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddFalseToObject"]
pub unsafe extern "C" fn cJSON_AddFalseToObject_ffi(
    mut object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let name = if name.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(name))
    };
    cJSON_AddFalseToObject(object.as_mut(), name)
}
pub fn cJSON_AddBoolToObject(
    object: Option<&mut cJSON>,
    name: Option<&::std::ffi::CStr>,
    boolean: cJSON_bool,
) -> *mut cJSON {
    let Some(bool_item_ref) = cJSON_CreateBool(boolean) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let mut bool_item_ref = bool_item_ref;
    let bool_item: *mut cJSON = bool_item_ref.as_mut();
    if add_item_to_object(object, name, bool_item_ref, false_0).is_ok() {
        return bool_item;
    }
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddBoolToObject"]
pub unsafe extern "C" fn cJSON_AddBoolToObject_ffi(
    mut object: *mut cJSON,
    name: *const ::core::ffi::c_char,
    boolean: cJSON_bool,
) -> *mut cJSON {
    let name = if name.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(name))
    };
    cJSON_AddBoolToObject(object.as_mut(), name, boolean)
}
pub fn cJSON_AddNumberToObject(
    object: Option<&mut cJSON>,
    name: Option<&::std::ffi::CStr>,
    number: ::core::ffi::c_double,
) -> *mut cJSON {
    let Some(number_item_ref) = cJSON_CreateNumber(number) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let mut number_item_ref = number_item_ref;
    let number_item: *mut cJSON = number_item_ref.as_mut();
    if add_item_to_object(object, name, number_item_ref, false_0).is_ok() {
        return number_item;
    }
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddNumberToObject"]
pub unsafe extern "C" fn cJSON_AddNumberToObject_ffi(
    mut object: *mut cJSON,
    name: *const ::core::ffi::c_char,
    number: ::core::ffi::c_double,
) -> *mut cJSON {
    let name = if name.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(name))
    };
    cJSON_AddNumberToObject(object.as_mut(), name, number)
}
pub fn cJSON_AddStringToObject(
    object: Option<&mut cJSON>,
    name: Option<&::std::ffi::CStr>,
    string: Option<&::std::ffi::CStr>,
) -> *mut cJSON {
    let Some(string_item_ref) = cJSON_CreateString(string) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let mut string_item_ref = string_item_ref;
    let string_item: *mut cJSON = string_item_ref.as_mut();
    if add_item_to_object(object, name, string_item_ref, false_0).is_ok() {
        return string_item;
    }
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddStringToObject"]
pub unsafe extern "C" fn cJSON_AddStringToObject_ffi(
    mut object: *mut cJSON,
    name: *const ::core::ffi::c_char,
    string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let name = if name.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(name))
    };
    let string = if string.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(string))
    };
    cJSON_AddStringToObject(object.as_mut(), name, string)
}
pub fn cJSON_AddRawToObject(
    object: Option<&mut cJSON>,
    name: Option<&::std::ffi::CStr>,
    raw: Option<&::std::ffi::CStr>,
) -> *mut cJSON {
    let Some(raw_item_ref) = cJSON_CreateRaw(raw) else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let mut raw_item_ref = raw_item_ref;
    let raw_item: *mut cJSON = raw_item_ref.as_mut();
    if add_item_to_object(object, name, raw_item_ref, false_0).is_ok() {
        return raw_item;
    }
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
        Some(::std::ffi::CStr::from_ptr(name))
    };
    let raw = if raw.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(raw))
    };
    cJSON_AddRawToObject(object.as_mut(), name, raw)
}
pub fn cJSON_AddObjectToObject(
    object: Option<&mut cJSON>,
    name: Option<&::std::ffi::CStr>,
) -> *mut cJSON {
    let Some(object_item_ref) = cJSON_CreateObject() else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let mut object_item_ref = object_item_ref;
    let object_item: *mut cJSON = object_item_ref.as_mut();
    if add_item_to_object(object, name, object_item_ref, false_0).is_ok() {
        return object_item;
    }
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddObjectToObject"]
pub unsafe extern "C" fn cJSON_AddObjectToObject_ffi(
    mut object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let name = if name.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(name))
    };
    cJSON_AddObjectToObject(object.as_mut(), name)
}
pub fn cJSON_AddArrayToObject(
    object: Option<&mut cJSON>,
    name: Option<&::std::ffi::CStr>,
) -> *mut cJSON {
    let Some(array_ref) = cJSON_CreateArray() else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let mut array_ref = array_ref;
    let array: *mut cJSON = array_ref.as_mut();
    if add_item_to_object(object, name, array_ref, false_0).is_ok() {
        return array;
    }
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddArrayToObject"]
pub unsafe extern "C" fn cJSON_AddArrayToObject_ffi(
    mut object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let name = if name.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(name))
    };
    cJSON_AddArrayToObject(object.as_mut(), name)
}
fn detach_item_via_pointer(
    parent: Option<&mut cJSON>,
    item_address: Option<usize>,
) -> Option<Box<cJSON>> {
    let item_address = item_address?;

    let Some(parent_ref) = parent else {
        return None;
    };
    let item_index = parent_ref
        .children
        .iter()
        .position(|child| ::core::ptr::from_ref::<cJSON>(child.as_ref()).addr() == item_address)?;
    let mut detached = parent_ref.children.remove(item_index);
    refresh_child_links(parent_ref);
    let detached_ref = detached.as_mut();
    detached_ref.prev = ::core::ptr::null_mut::<cJSON>();
    detached_ref.next = ::core::ptr::null_mut::<cJSON>();
    Some(detached)
}
#[export_name = "cJSON_DetachItemViaPointer"]
pub unsafe extern "C" fn cJSON_DetachItemViaPointer_ffi(
    mut parent: *mut cJSON,
    item: *mut cJSON,
) -> *mut cJSON {
    if item.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    if let Some(parent_ref) = parent.as_mut() {
        if let Some(item) = detach_item_via_pointer(Some(parent_ref), Some(item.addr())) {
            return Box::into_raw(item);
        }
    } else {
        return ::core::ptr::null_mut::<cJSON>();
    }

    let Some(parent_ref) = parent.as_mut() else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let Some(item_ref) = item.as_mut() else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    let item_next = item_ref.next;
    let item_prev = item_ref.prev;

    if item != parent_ref.child && item_prev.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    if item != parent_ref.child {
        let Some(prev_ref) = item_prev.as_mut() else {
            return ::core::ptr::null_mut::<cJSON>();
        };
        prev_ref.next = item_next;
    }
    if !item_next.is_null() {
        let Some(next_ref) = item_next.as_mut() else {
            return ::core::ptr::null_mut::<cJSON>();
        };
        next_ref.prev = item_prev;
    }
    if item == parent_ref.child {
        parent_ref.child = item_next;
    } else if item_next.is_null() {
        let Some(first_child_ref) = parent_ref.child.as_mut() else {
            return ::core::ptr::null_mut::<cJSON>();
        };
        first_child_ref.prev = item_prev;
    }
    item_ref.prev = ::core::ptr::null_mut::<cJSON>();
    item_ref.next = ::core::ptr::null_mut::<cJSON>();
    match item.as_mut() {
        Some(item) => item as *mut cJSON,
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_DetachItemFromArray(
    array: Option<&mut cJSON>,
    mut which: ::core::ffi::c_int,
) -> Option<Box<cJSON>> {
    if which < 0 as ::core::ffi::c_int {
        return None;
    }
    let Some(array) = array else {
        return None;
    };
    let item_address = get_array_item(Some(&*array), which as size_t)
        .map(|item| ::core::ptr::from_ref::<cJSON>(item).addr());
    return detach_item_via_pointer(Some(array), item_address);
}
#[export_name = "cJSON_DetachItemFromArray"]
pub unsafe extern "C" fn cJSON_DetachItemFromArray_ffi(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
) -> *mut cJSON {
    let item = cJSON_GetArrayItem_ffi(array, which);
    cJSON_DetachItemViaPointer_ffi(array, item)
}
pub fn cJSON_DeleteItemFromArray(array: Option<&mut cJSON>, mut which: ::core::ffi::c_int) {
    cJSON_Delete(cJSON_DetachItemFromArray(array, which));
}
#[export_name = "cJSON_DeleteItemFromArray"]
pub unsafe extern "C" fn cJSON_DeleteItemFromArray_ffi(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
) {
    let item = cJSON_DetachItemFromArray_ffi(array, which);
    cJSON_Delete_ffi(item)
}
pub fn cJSON_DetachItemFromObject(
    object: Option<&mut cJSON>,
    string: Option<&::std::ffi::CStr>,
) -> Option<Box<cJSON>> {
    let Some(object) = object else {
        return None;
    };
    let item_address = cJSON_GetObjectItem(Some(&*object), string)
        .map(|item| ::core::ptr::from_ref::<cJSON>(item).addr());
    return detach_item_via_pointer(Some(object), item_address);
}
#[export_name = "cJSON_DetachItemFromObject"]
pub unsafe extern "C" fn cJSON_DetachItemFromObject_ffi(
    mut object: *mut cJSON,
    string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let item = cJSON_GetObjectItem_ffi(object, string);
    cJSON_DetachItemViaPointer_ffi(object, item)
}
pub fn cJSON_DetachItemFromObjectCaseSensitive(
    object: Option<&mut cJSON>,
    string: Option<&::std::ffi::CStr>,
) -> Option<Box<cJSON>> {
    let Some(object) = object else {
        return None;
    };
    let item_address = cJSON_GetObjectItemCaseSensitive(Some(&*object), string)
        .map(|item| ::core::ptr::from_ref::<cJSON>(item).addr());
    return detach_item_via_pointer(Some(object), item_address);
}
#[export_name = "cJSON_DetachItemFromObjectCaseSensitive"]
pub unsafe extern "C" fn cJSON_DetachItemFromObjectCaseSensitive_ffi(
    mut object: *mut cJSON,
    string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let item = cJSON_GetObjectItemCaseSensitive_ffi(object, string);
    cJSON_DetachItemViaPointer_ffi(object, item)
}
pub fn cJSON_DeleteItemFromObject(object: Option<&mut cJSON>, string: Option<&::std::ffi::CStr>) {
    cJSON_Delete(cJSON_DetachItemFromObject(object, string));
}
#[export_name = "cJSON_DeleteItemFromObject"]
pub unsafe extern "C" fn cJSON_DeleteItemFromObject_ffi(
    mut object: *mut cJSON,
    string: *const ::core::ffi::c_char,
) {
    let item = cJSON_DetachItemFromObject_ffi(object, string);
    cJSON_Delete_ffi(item)
}
pub fn cJSON_DeleteItemFromObjectCaseSensitive(
    object: Option<&mut cJSON>,
    string: Option<&::std::ffi::CStr>,
) {
    cJSON_Delete(cJSON_DetachItemFromObjectCaseSensitive(object, string));
}
#[export_name = "cJSON_DeleteItemFromObjectCaseSensitive"]
pub unsafe extern "C" fn cJSON_DeleteItemFromObjectCaseSensitive_ffi(
    mut object: *mut cJSON,
    string: *const ::core::ffi::c_char,
) {
    let item = cJSON_DetachItemFromObjectCaseSensitive_ffi(object, string);
    cJSON_Delete_ffi(item)
}
pub fn cJSON_InsertItemInArray(
    array: Option<&mut cJSON>,
    mut which: ::core::ffi::c_int,
    newitem: Option<Box<cJSON>>,
) -> cJSON_bool {
    if which < 0 as ::core::ffi::c_int {
        return false_0;
    }
    let Some(array) = array else {
        return false_0;
    };
    let Some(newitem_ref) = newitem else {
        return false_0;
    };
    if array.children.is_empty() && !array.child.is_null() {
        return false_0;
    }
    if which as usize >= array.children.len() {
        return match add_item_to_array(array, newitem_ref) {
            Ok(()) => true_0,
            Err(_) => false_0,
        };
    }
    if array.children.try_reserve_exact(1).is_err() {
        return false_0;
    }
    array.children.insert(which as usize, newitem_ref);
    refresh_child_links(array);
    return true_0;
}
#[export_name = "cJSON_InsertItemInArray"]
pub unsafe extern "C" fn cJSON_InsertItemInArray_ffi(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
    mut newitem: *mut cJSON,
) -> cJSON_bool {
    if array.is_null() || newitem.is_null() || array == newitem || which < 0 {
        return false_0;
    }
    let Some(array_ref) = array.as_mut() else {
        return false_0;
    };
    if array_ref.children.is_empty() && !array_ref.child.is_null() {
        let after_inserted = cJSON_GetArrayItem_ffi(array, which);
        if after_inserted.is_null() {
            return cJSON_AddItemToArray_ffi(array, newitem);
        }
        let newitem_ref = &mut *newitem;
        let after_inserted_ref = &mut *after_inserted;
        let after_inserted_prev = after_inserted_ref.prev;
        if after_inserted != array_ref.child && after_inserted_prev.is_null() {
            return false_0;
        }
        newitem_ref.next = after_inserted;
        newitem_ref.prev = after_inserted_prev;
        after_inserted_ref.prev = newitem;
        if after_inserted == array_ref.child {
            array_ref.child = newitem;
        } else {
            let prev_ref = &mut *newitem_ref.prev;
            prev_ref.next = newitem;
        }
        return true_0;
    }
    let newitem = Box::from_raw(newitem);
    cJSON_InsertItemInArray(Some(array_ref), which, Some(newitem))
}
pub fn cJSON_ReplaceItemViaPointer(
    parent: Option<&mut cJSON>,
    item_address: Option<usize>,
    replacement: Option<Box<cJSON>>,
) -> cJSON_bool {
    let Some(mut replacement) = replacement else {
        return false_0;
    };
    let Some(parent_ref) = parent else {
        return false_0;
    };
    let Some(item_address) = item_address else {
        return false_0;
    };
    let Some(item_index) = parent_ref
        .children
        .iter()
        .position(|child| ::core::ptr::from_ref::<cJSON>(child.as_ref()).addr() == item_address)
    else {
        return false_0;
    };
    parent_ref.children[item_index] = replacement;
    refresh_child_links(parent_ref);
    true_0
}
#[export_name = "cJSON_ReplaceItemViaPointer"]
pub unsafe extern "C" fn cJSON_ReplaceItemViaPointer_ffi(
    mut parent: *mut cJSON,
    item: *mut cJSON,
    mut replacement: *mut cJSON,
) -> cJSON_bool {
    if parent.is_null() || item.is_null() || replacement.is_null() {
        return false_0;
    }
    if item == replacement {
        return true_0;
    }
    let Some(parent_ref) = parent.as_mut() else {
        return false_0;
    };
    if parent_ref.children.is_empty() && !parent_ref.child.is_null() {
        let Some(item_ref) = item.as_mut() else {
            return false_0;
        };
        let Some(replacement_ref) = replacement.as_mut() else {
            return false_0;
        };
        if parent_ref.child.is_null() {
            return false_0;
        }
        let replacement_ptr = replacement_ref as *mut cJSON;
        let item_ptr = item_ref as *mut cJSON;
        if replacement_ptr == item_ptr {
            return true_0;
        }
        let item_next = item_ref.next;
        let item_prev = item_ref.prev;

        replacement_ref.next = item_next;
        replacement_ref.prev = item_prev;
        if !item_next.is_null() {
            let next_ref = &mut *item_next;
            next_ref.prev = replacement_ptr;
        }
        if parent_ref.child == item_ptr {
            if item_ref.prev == item {
                replacement_ref.prev = replacement_ptr;
            }
            parent_ref.child = replacement_ptr;
        } else {
            if !replacement_ref.prev.is_null() {
                let prev_ref = &mut *replacement_ref.prev;
                prev_ref.next = replacement_ptr;
            }
            if replacement_ref.next.is_null() {
                let first_child_ref = &mut *parent_ref.child;
                first_child_ref.prev = replacement_ptr;
            }
        }
        item_ref.next = ::core::ptr::null_mut::<cJSON>();
        item_ref.prev = ::core::ptr::null_mut::<cJSON>();
        cJSON_Delete_ffi(item);
        return true_0;
    }
    if parent_ref.children.is_empty() {
        return false_0;
    }
    let mut replacement_box = Box::from_raw(replacement);
    let Some(item_index) = parent_ref
        .children
        .iter()
        .position(|child| ::core::ptr::from_ref::<cJSON>(child.as_ref()).addr() == item.addr())
    else {
        let _ = Box::into_raw(replacement_box);
        return false_0;
    };
    replacement_box.next = ::core::ptr::null_mut::<cJSON>();
    replacement_box.prev = ::core::ptr::null_mut::<cJSON>();
    parent_ref.children[item_index] = replacement_box;
    refresh_child_links(parent_ref);
    true_0
}
pub fn cJSON_ReplaceItemInArray(
    array: Option<&mut cJSON>,
    mut which: ::core::ffi::c_int,
    newitem: Option<Box<cJSON>>,
) -> cJSON_bool {
    if which < 0 as ::core::ffi::c_int {
        return false_0;
    }
    let Some(array) = array else {
        return false_0;
    };
    let item_address = get_array_item(Some(&*array), which as size_t)
        .map(|item| ::core::ptr::from_ref(item).addr());
    return cJSON_ReplaceItemViaPointer(Some(array), item_address, newitem);
}
#[export_name = "cJSON_ReplaceItemInArray"]
pub unsafe extern "C" fn cJSON_ReplaceItemInArray_ffi(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
    mut newitem: *mut cJSON,
) -> cJSON_bool {
    let item = cJSON_GetArrayItem_ffi(array, which);
    cJSON_ReplaceItemViaPointer_ffi(array, item, newitem)
}
fn replace_item_in_object(
    mut object: Option<&mut cJSON>,
    string: Option<&::std::ffi::CStr>,
    mut replacement: Box<cJSON>,
    mut case_sensitive: cJSON_bool,
) -> cJSON_bool {
    let Some(string) = string else {
        return false_0;
    };
    let replacement_ref = replacement.as_mut();
    if replacement_ref.type_0 & cJSON_StringIsConst == 0 && !replacement_ref.string.is_null() {
        replacement_ref.string_storage.take();
    }
    if !set_owned_string(replacement_ref, string) {
        return false_0;
    }
    let replacement_ref = replacement.as_mut();
    replacement_ref.type_0 &= !cJSON_StringIsConst;
    let item_address = get_object_item(object.as_deref(), Some(string), case_sensitive)
        .map(|item| ::core::ptr::from_ref(item).addr());
    return cJSON_ReplaceItemViaPointer(object, item_address, Some(replacement));
}
pub fn cJSON_ReplaceItemInObject(
    object: Option<&mut cJSON>,
    string: Option<&::std::ffi::CStr>,
    newitem: Option<Box<cJSON>>,
) -> cJSON_bool {
    let Some(newitem) = newitem else {
        return false_0;
    };
    return replace_item_in_object(object, string, newitem, false_0);
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
    if !string.is_null() && !newitem.is_null() {
        if (*newitem).type_0 & cJSON_StringIsConst == 0
            && !(*newitem).string.is_null()
            && (*newitem).string_storage.is_none()
        {
            let hooks = current_global_hooks();
            hooks.deallocate.expect("non-null function pointer")(
                (*newitem).string as *mut ::core::ffi::c_void,
            );
        }
    }
    let string = if string.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(string))
    };
    let Some(string_ref) = string else {
        return false_0;
    };
    let Some(object_ref) = object.as_mut() else {
        return false_0;
    };
    if object_ref.children.is_empty() && !object_ref.child.is_null() {
        let Some(newitem_ref) = newitem.as_mut() else {
            return false_0;
        };
        newitem_ref.string_storage.take();
        if !set_owned_string(newitem_ref, string_ref) {
            return false_0;
        }
        newitem_ref.type_0 &= !cJSON_StringIsConst;
        let item = cJSON_GetObjectItem_ffi(object, string_ref.as_ptr());
        return cJSON_ReplaceItemViaPointer_ffi(object, item, newitem);
    }
    let mut newitem_box = Box::from_raw(newitem);
    if newitem_box.type_0 & cJSON_StringIsConst == 0 && !newitem_box.string.is_null() {
        newitem_box.string_storage.take();
    }
    if !set_owned_string(newitem_box.as_mut(), string_ref) {
        let _ = Box::into_raw(newitem_box);
        return false_0;
    }
    newitem_box.type_0 &= !cJSON_StringIsConst;
    let item_address = get_object_item(Some(&*object_ref), Some(string_ref), false_0)
        .map(|item| ::core::ptr::from_ref(item).addr());
    let Some(item_address) = item_address else {
        let _ = Box::into_raw(newitem_box);
        return false_0;
    };
    let Some(item_index) = object_ref
        .children
        .iter()
        .position(|child| ::core::ptr::from_ref::<cJSON>(child.as_ref()).addr() == item_address)
    else {
        let _ = Box::into_raw(newitem_box);
        return false_0;
    };
    newitem_box.next = ::core::ptr::null_mut::<cJSON>();
    newitem_box.prev = ::core::ptr::null_mut::<cJSON>();
    object_ref.children[item_index] = newitem_box;
    refresh_child_links(object_ref);
    true_0
}
pub fn cJSON_ReplaceItemInObjectCaseSensitive(
    object: Option<&mut cJSON>,
    string: Option<&::std::ffi::CStr>,
    newitem: Option<Box<cJSON>>,
) -> cJSON_bool {
    let Some(newitem) = newitem else {
        return false_0;
    };
    return replace_item_in_object(object, string, newitem, true_0);
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
    if !string.is_null() && !newitem.is_null() {
        if (*newitem).type_0 & cJSON_StringIsConst == 0
            && !(*newitem).string.is_null()
            && (*newitem).string_storage.is_none()
        {
            let hooks = current_global_hooks();
            hooks.deallocate.expect("non-null function pointer")(
                (*newitem).string as *mut ::core::ffi::c_void,
            );
        }
    }
    let string = if string.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(string))
    };
    let Some(string_ref) = string else {
        return false_0;
    };
    let Some(object_ref) = object.as_mut() else {
        return false_0;
    };
    if object_ref.children.is_empty() && !object_ref.child.is_null() {
        let Some(newitem_ref) = newitem.as_mut() else {
            return false_0;
        };
        newitem_ref.string_storage.take();
        if !set_owned_string(newitem_ref, string_ref) {
            return false_0;
        }
        newitem_ref.type_0 &= !cJSON_StringIsConst;
        let item = cJSON_GetObjectItemCaseSensitive_ffi(object, string_ref.as_ptr());
        return cJSON_ReplaceItemViaPointer_ffi(object, item, newitem);
    }
    let mut newitem_box = Box::from_raw(newitem);
    if newitem_box.type_0 & cJSON_StringIsConst == 0 && !newitem_box.string.is_null() {
        newitem_box.string_storage.take();
    }
    if !set_owned_string(newitem_box.as_mut(), string_ref) {
        let _ = Box::into_raw(newitem_box);
        return false_0;
    }
    newitem_box.type_0 &= !cJSON_StringIsConst;
    let item_address = get_object_item(Some(&*object_ref), Some(string_ref), true_0)
        .map(|item| ::core::ptr::from_ref(item).addr());
    let Some(item_address) = item_address else {
        let _ = Box::into_raw(newitem_box);
        return false_0;
    };
    let Some(item_index) = object_ref
        .children
        .iter()
        .position(|child| ::core::ptr::from_ref::<cJSON>(child.as_ref()).addr() == item_address)
    else {
        let _ = Box::into_raw(newitem_box);
        return false_0;
    };
    newitem_box.next = ::core::ptr::null_mut::<cJSON>();
    newitem_box.prev = ::core::ptr::null_mut::<cJSON>();
    object_ref.children[item_index] = newitem_box;
    refresh_child_links(object_ref);
    true_0
}
pub fn cJSON_CreateNull() -> Option<Box<cJSON>> {
    let hooks = current_global_hooks();
    let Some(mut item) = cJSON_New_Item(&hooks) else {
        return None;
    };
    item.as_mut().type_0 = cJSON_NULL;
    return Some(item);
}
#[export_name = "cJSON_CreateNull"]
pub unsafe extern "C" fn cJSON_CreateNull_ffi() -> *mut cJSON {
    match cJSON_CreateNull() {
        Some(item) => Box::into_raw(item),
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_CreateTrue() -> Option<Box<cJSON>> {
    let hooks = current_global_hooks();
    let Some(mut item) = cJSON_New_Item(&hooks) else {
        return None;
    };
    item.as_mut().type_0 = cJSON_True;
    return Some(item);
}
#[export_name = "cJSON_CreateTrue"]
pub unsafe extern "C" fn cJSON_CreateTrue_ffi() -> *mut cJSON {
    match cJSON_CreateTrue() {
        Some(item) => Box::into_raw(item),
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_CreateFalse() -> Option<Box<cJSON>> {
    let hooks = current_global_hooks();
    let Some(mut item) = cJSON_New_Item(&hooks) else {
        return None;
    };
    item.as_mut().type_0 = cJSON_False;
    return Some(item);
}
#[export_name = "cJSON_CreateFalse"]
pub unsafe extern "C" fn cJSON_CreateFalse_ffi() -> *mut cJSON {
    match cJSON_CreateFalse() {
        Some(item) => Box::into_raw(item),
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_CreateBool(mut boolean: cJSON_bool) -> Option<Box<cJSON>> {
    let hooks = current_global_hooks();
    let Some(mut item) = cJSON_New_Item(&hooks) else {
        return None;
    };
    item.as_mut().type_0 = if boolean != 0 {
        cJSON_True
    } else {
        cJSON_False
    };
    return Some(item);
}
#[export_name = "cJSON_CreateBool"]
pub unsafe extern "C" fn cJSON_CreateBool_ffi(mut boolean: cJSON_bool) -> *mut cJSON {
    match cJSON_CreateBool(boolean) {
        Some(item) => Box::into_raw(item),
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_CreateNumber(mut num: ::core::ffi::c_double) -> Option<Box<cJSON>> {
    let hooks = current_global_hooks();
    let Some(mut item) = cJSON_New_Item(&hooks) else {
        return None;
    };
    let item_ref = item.as_mut();
    item_ref.type_0 = cJSON_Number;
    item_ref.valuedouble = num;
    if num >= INT_MAX as ::core::ffi::c_double {
        item_ref.valueint = INT_MAX;
    } else if num <= INT_MIN as ::core::ffi::c_double {
        item_ref.valueint = INT_MIN;
    } else {
        item_ref.valueint = num as ::core::ffi::c_int;
    }
    return Some(item);
}
#[export_name = "cJSON_CreateNumber"]
pub unsafe extern "C" fn cJSON_CreateNumber_ffi(mut num: ::core::ffi::c_double) -> *mut cJSON {
    match cJSON_CreateNumber(num) {
        Some(item) => Box::into_raw(item),
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_CreateString(string: Option<&::std::ffi::CStr>) -> Option<Box<cJSON>> {
    let hooks = current_global_hooks();
    let Some(mut item) = cJSON_New_Item(&hooks) else {
        return None;
    };
    item.as_mut().type_0 = cJSON_String;
    if !string.is_some_and(|string| set_owned_valuestring(item.as_mut(), string)) {
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
        Some(::std::ffi::CStr::from_ptr(string))
    };
    match cJSON_CreateString(string) {
        Some(item) => Box::into_raw(item),
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_CreateStringReference(string: Option<&::std::ffi::CStr>) -> Option<Box<cJSON>> {
    let hooks = current_global_hooks();
    let Some(mut item) = cJSON_New_Item(&hooks) else {
        return None;
    };
    item.as_mut().type_0 = cJSON_String | cJSON_IsReference;
    if let Some(string) = string {
        if !set_owned_valuestring(item.as_mut(), string) {
            return None;
        }
    }
    return Some(item);
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
    match cJSON_CreateStringReference(string) {
        Some(item) => Box::into_raw(item),
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_CreateObjectReference(child: Option<&cJSON>) -> Option<Box<cJSON>> {
    let hooks = current_global_hooks();
    let Some(mut item) = cJSON_New_Item(&hooks) else {
        return None;
    };
    let item_ref = item.as_mut();
    item_ref.type_0 = cJSON_Object | cJSON_IsReference;
    item_ref.child = match child {
        Some(child) => child as *const cJSON as *mut cJSON,
        None => ::core::ptr::null_mut(),
    };
    return Some(item);
}
#[export_name = "cJSON_CreateObjectReference"]
pub unsafe extern "C" fn cJSON_CreateObjectReference_ffi(mut child: *const cJSON) -> *mut cJSON {
    let child = child.as_ref();
    match cJSON_CreateObjectReference(child) {
        Some(item) => Box::into_raw(item),
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_CreateArrayReference(child: Option<&cJSON>) -> Option<Box<cJSON>> {
    let hooks = current_global_hooks();
    let Some(mut item) = cJSON_New_Item(&hooks) else {
        return None;
    };
    let item_ref = item.as_mut();
    item_ref.type_0 = cJSON_Array | cJSON_IsReference;
    item_ref.child = match child {
        Some(child) => child as *const cJSON as *mut cJSON,
        None => ::core::ptr::null_mut(),
    };
    return Some(item);
}
#[export_name = "cJSON_CreateArrayReference"]
pub unsafe extern "C" fn cJSON_CreateArrayReference_ffi(mut child: *const cJSON) -> *mut cJSON {
    let child = child.as_ref();
    match cJSON_CreateArrayReference(child) {
        Some(item) => Box::into_raw(item),
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_CreateRaw(raw: Option<&::std::ffi::CStr>) -> Option<Box<cJSON>> {
    let hooks = current_global_hooks();
    let Some(mut item) = cJSON_New_Item(&hooks) else {
        return None;
    };
    item.as_mut().type_0 = cJSON_Raw;
    if !raw.is_some_and(|raw| set_owned_valuestring(item.as_mut(), raw)) {
        return None;
    }
    return Some(item);
}
#[export_name = "cJSON_CreateRaw"]
pub unsafe extern "C" fn cJSON_CreateRaw_ffi(mut raw: *const ::core::ffi::c_char) -> *mut cJSON {
    let raw = if raw.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(raw))
    };
    match cJSON_CreateRaw(raw) {
        Some(item) => Box::into_raw(item),
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_CreateArray() -> Option<Box<cJSON>> {
    let hooks = current_global_hooks();
    let Some(mut item) = cJSON_New_Item(&hooks) else {
        return None;
    };
    item.as_mut().type_0 = cJSON_Array;
    return Some(item);
}
#[export_name = "cJSON_CreateArray"]
pub unsafe extern "C" fn cJSON_CreateArray_ffi() -> *mut cJSON {
    match cJSON_CreateArray() {
        Some(item) => Box::into_raw(item),
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_CreateObject() -> Option<Box<cJSON>> {
    let hooks = current_global_hooks();
    let Some(mut item) = cJSON_New_Item(&hooks) else {
        return None;
    };
    item.as_mut().type_0 = cJSON_Object;
    return Some(item);
}
#[export_name = "cJSON_CreateObject"]
pub unsafe extern "C" fn cJSON_CreateObject_ffi() -> *mut cJSON {
    match cJSON_CreateObject() {
        Some(item) => Box::into_raw(item),
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_CreateIntArray(numbers: &[::core::ffi::c_int]) -> *mut cJSON {
    let Some(mut array) = cJSON_CreateArray() else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    for &number in numbers {
        let Some(new_item_ref) = cJSON_CreateNumber(number as ::core::ffi::c_double) else {
            return ::core::ptr::null_mut::<cJSON>();
        };
        if add_item_to_array(array.as_mut(), new_item_ref).is_err() {
            return ::core::ptr::null_mut::<cJSON>();
        }
    }
    return Box::into_raw(array);
}
#[export_name = "cJSON_CreateIntArray"]
pub unsafe extern "C" fn cJSON_CreateIntArray_ffi(
    mut numbers: *const ::core::ffi::c_int,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let numbers = ::std::slice::from_raw_parts(numbers, count as usize);
    cJSON_CreateIntArray(numbers)
}
pub fn cJSON_CreateFloatArray(numbers: &[::core::ffi::c_float]) -> *mut cJSON {
    let Some(mut array) = cJSON_CreateArray() else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    for &number in numbers {
        let Some(new_item_ref) = cJSON_CreateNumber(number as ::core::ffi::c_double) else {
            return ::core::ptr::null_mut::<cJSON>();
        };
        if add_item_to_array(array.as_mut(), new_item_ref).is_err() {
            return ::core::ptr::null_mut::<cJSON>();
        }
    }
    return Box::into_raw(array);
}
#[export_name = "cJSON_CreateFloatArray"]
pub unsafe extern "C" fn cJSON_CreateFloatArray_ffi(
    mut numbers: *const ::core::ffi::c_float,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let numbers = ::std::slice::from_raw_parts(numbers, count as usize);
    cJSON_CreateFloatArray(numbers)
}
pub fn cJSON_CreateDoubleArray(numbers: &[::core::ffi::c_double]) -> *mut cJSON {
    let Some(mut array) = cJSON_CreateArray() else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    for &number in numbers {
        let Some(new_item_ref) = cJSON_CreateNumber(number) else {
            return ::core::ptr::null_mut::<cJSON>();
        };
        if add_item_to_array(array.as_mut(), new_item_ref).is_err() {
            return ::core::ptr::null_mut::<cJSON>();
        }
    }
    return Box::into_raw(array);
}
#[export_name = "cJSON_CreateDoubleArray"]
pub unsafe extern "C" fn cJSON_CreateDoubleArray_ffi(
    mut numbers: *const ::core::ffi::c_double,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let numbers = ::std::slice::from_raw_parts(numbers, count as usize);
    cJSON_CreateDoubleArray(numbers)
}
pub fn cJSON_CreateStringArray(strings: &[Option<&::std::ffi::CStr>]) -> *mut cJSON {
    let Some(mut array) = cJSON_CreateArray() else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    for &string in strings {
        let Some(new_item_ref) = cJSON_CreateString(string) else {
            return ::core::ptr::null_mut::<cJSON>();
        };
        if add_item_to_array(array.as_mut(), new_item_ref).is_err() {
            return ::core::ptr::null_mut::<cJSON>();
        }
    }
    return Box::into_raw(array);
}
#[export_name = "cJSON_CreateStringArray"]
pub unsafe extern "C" fn cJSON_CreateStringArray_ffi(
    mut strings: *const *const ::core::ffi::c_char,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    if count < 0 as ::core::ffi::c_int || strings.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let raw_strings = ::std::slice::from_raw_parts(strings, count as usize);
    let mut string_refs: Vec<Option<&::std::ffi::CStr>> = Vec::new();
    if string_refs.try_reserve_exact(raw_strings.len()).is_err() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    for &string in raw_strings {
        let string_ref = if string.is_null() {
            None
        } else {
            Some(::std::ffi::CStr::from_ptr(string))
        };
        string_refs.push(string_ref);
    }
    cJSON_CreateStringArray(&string_refs)
}
pub fn cJSON_Duplicate(item: Option<&cJSON>, recurse: cJSON_bool) -> Option<Box<cJSON>> {
    return cJSON_Duplicate_rec(item, 0 as size_t, recurse);
}
#[export_name = "cJSON_Duplicate"]
pub unsafe extern "C" fn cJSON_Duplicate_ffi(
    mut item: *const cJSON,
    mut recurse: cJSON_bool,
) -> *mut cJSON {
    match cJSON_Duplicate(item.as_ref(), recurse) {
        Some(item) => Box::into_raw(item),
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
pub fn cJSON_Duplicate_rec(
    item: Option<&cJSON>,
    depth: size_t,
    recurse: cJSON_bool,
) -> Option<Box<cJSON>> {
    let Some(item) = item else {
        return None;
    };
    let hooks = current_global_hooks();
    let Some(mut newitem_ref) = cJSON_New_Item(&hooks) else {
        return None;
    };
    let newitem = newitem_ref.as_mut();
    newitem.type_0 = item.type_0 & !cJSON_IsReference;
    newitem.valueint = item.valueint;
    newitem.valuedouble = item.valuedouble;

    if !item.valuestring.is_null() {
        let Some(valuestring) = valuestring_cstr(item) else {
            return None;
        };
        if !set_owned_valuestring(newitem_ref.as_mut(), valuestring) {
            return None;
        }
    }

    if !item.string.is_null() {
        if item.type_0 & cJSON_StringIsConst != 0 {
            let Some(string) = string_cstr(item) else {
                return None;
            };
            if !set_owned_string(newitem_ref.as_mut(), string) {
                return None;
            }
            let newitem = newitem_ref.as_mut();
            newitem.type_0 |= cJSON_StringIsConst;
        } else {
            let Some(string) = string_cstr(item) else {
                return None;
            };
            if !set_owned_string(newitem_ref.as_mut(), string) {
                return None;
            }
        }
        if newitem_ref.as_ref().string.is_null() {
            return None;
        }
    }

    if recurse == 0 {
        return Some(newitem_ref);
    }

    let mut index: size_t = 0;
    while let Some(child_ref) = get_array_item(Some(item), index) {
        if depth >= CJSON_CIRCULAR_LIMIT as size_t {
            return None;
        }
        let Some(newchild_ref) =
            cJSON_Duplicate_rec(Some(child_ref), depth.wrapping_add(1 as size_t), true_0)
        else {
            return None;
        };
        if add_item_to_array(newitem_ref.as_mut(), newchild_ref).is_err() {
            return None;
        }
        index = index.wrapping_add(1);
    }
    return Some(newitem_ref);
}
#[export_name = "cJSON_Duplicate_rec"]
pub unsafe extern "C" fn cJSON_Duplicate_rec_ffi(
    mut item: *const cJSON,
    mut depth: size_t,
    mut recurse: cJSON_bool,
) -> *mut cJSON {
    match cJSON_Duplicate_rec(item.as_ref(), depth, recurse) {
        Some(item) => Box::into_raw(item),
        None => ::core::ptr::null_mut::<cJSON>(),
    }
}
fn skip_oneline_comment(input: &mut usize, json: &[::core::ffi::c_char]) {
    *input = input.wrapping_add(2);
    while json.get(*input).is_some_and(|&byte| byte != 0) {
        if json[*input] as ::core::ffi::c_int == '\n' as i32 {
            *input = input.wrapping_add(1);
            return;
        }
        *input = input.wrapping_add(1);
    }
}
fn skip_multiline_comment(input: &mut usize, json: &[::core::ffi::c_char]) {
    *input = input.wrapping_add(2);
    while json.get(*input).is_some_and(|&byte| byte != 0) {
        if json[*input] as ::core::ffi::c_int == '*' as i32
            && json
                .get(input.wrapping_add(1))
                .is_some_and(|&byte| byte as ::core::ffi::c_int == '/' as i32)
        {
            *input = input.wrapping_add(2);
            return;
        }
        *input = input.wrapping_add(1);
    }
}
fn minify_string(input: &mut usize, output: &mut usize, json: &mut [::core::ffi::c_char]) {
    json[*output] = json[*input];
    *input = input.wrapping_add(1);
    *output = output.wrapping_add(1);
    while json.get(*input).is_some_and(|&byte| byte != 0) {
        json[*output] = json[*input];
        if json[*input] as ::core::ffi::c_int == '"' as i32 {
            json[*output] = '"' as i32 as ::core::ffi::c_char;
            *input = input.wrapping_add(1);
            *output = output.wrapping_add(1);
            return;
        } else if json[*input] as ::core::ffi::c_int == '\\' as i32
            && json
                .get(input.wrapping_add(1))
                .is_some_and(|&byte| byte as ::core::ffi::c_int == '"' as i32)
        {
            json[output.wrapping_add(1)] = json[input.wrapping_add(1)];
            *input = input.wrapping_add(1);
            *output = output.wrapping_add(1);
        }
        *input = input.wrapping_add(1);
        *output = output.wrapping_add(1);
    }
}
pub fn cJSON_Minify(json: Option<&mut [::core::ffi::c_char]>) {
    let Some(json) = json else {
        return;
    };
    let mut input = 0usize;
    let mut output = 0usize;
    while json.get(input).is_some_and(|&byte| byte != 0) {
        match json[input] as ::core::ffi::c_int {
            32 | 9 | 13 | 10 => {
                input = input.wrapping_add(1);
            }
            47 => {
                if json
                    .get(input.wrapping_add(1))
                    .is_some_and(|&byte| byte as ::core::ffi::c_int == '/' as i32)
                {
                    skip_oneline_comment(&mut input, json);
                } else if json
                    .get(input.wrapping_add(1))
                    .is_some_and(|&byte| byte as ::core::ffi::c_int == '*' as i32)
                {
                    skip_multiline_comment(&mut input, json);
                } else {
                    input = input.wrapping_add(1);
                }
            }
            34 => {
                minify_string(&mut input, &mut output, json);
            }
            _ => {
                json[output] = json[input];
                input = input.wrapping_add(1);
                output = output.wrapping_add(1);
            }
        }
    }
    if let Some(byte) = json.get_mut(output) {
        *byte = '\0' as i32 as ::core::ffi::c_char;
    }
}
#[export_name = "cJSON_Minify"]
pub unsafe extern "C" fn cJSON_Minify_ffi(mut json: *mut ::core::ffi::c_char) {
    if json.is_null() {
        return;
    }
    let length = ::std::ffi::CStr::from_ptr(json).to_bytes_with_nul().len();
    let json = ::std::slice::from_raw_parts_mut(json, length);
    cJSON_Minify(Some(json))
}
fn cjson_bool(value: bool) -> cJSON_bool {
    value as ::core::ffi::c_int
}
fn cjson_type_matches(item: Option<&cJSON>, expected_type: ::core::ffi::c_int) -> cJSON_bool {
    cjson_bool(item.is_some_and(|item| item.type_0 & 0xff as ::core::ffi::c_int == expected_type))
}
pub fn cJSON_IsInvalid(item: Option<&cJSON>) -> cJSON_bool {
    cjson_type_matches(item, cJSON_Invalid)
}
#[export_name = "cJSON_IsInvalid"]
pub unsafe extern "C" fn cJSON_IsInvalid_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsInvalid(item.as_ref())
}
pub fn cJSON_IsFalse(item: Option<&cJSON>) -> cJSON_bool {
    cjson_type_matches(item, cJSON_False)
}
#[export_name = "cJSON_IsFalse"]
pub unsafe extern "C" fn cJSON_IsFalse_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsFalse(item.as_ref())
}
pub fn cJSON_IsTrue(item: Option<&cJSON>) -> cJSON_bool {
    cjson_type_matches(item, cJSON_True)
}
#[export_name = "cJSON_IsTrue"]
pub unsafe extern "C" fn cJSON_IsTrue_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsTrue(item.as_ref())
}
pub fn cJSON_IsBool(item: Option<&cJSON>) -> cJSON_bool {
    cjson_bool(
        item.is_some_and(|item| {
            item.type_0 & (cJSON_True | cJSON_False) != 0 as ::core::ffi::c_int
        }),
    )
}
#[export_name = "cJSON_IsBool"]
pub unsafe extern "C" fn cJSON_IsBool_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsBool(item.as_ref())
}
pub fn cJSON_IsNull(item: Option<&cJSON>) -> cJSON_bool {
    cjson_type_matches(item, cJSON_NULL)
}
#[export_name = "cJSON_IsNull"]
pub unsafe extern "C" fn cJSON_IsNull_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsNull(item.as_ref())
}
pub fn cJSON_IsNumber(item: Option<&cJSON>) -> cJSON_bool {
    cjson_type_matches(item, cJSON_Number)
}
#[export_name = "cJSON_IsNumber"]
pub unsafe extern "C" fn cJSON_IsNumber_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsNumber(item.as_ref())
}
pub fn cJSON_IsString(item: Option<&cJSON>) -> cJSON_bool {
    cjson_type_matches(item, cJSON_String)
}
#[export_name = "cJSON_IsString"]
pub unsafe extern "C" fn cJSON_IsString_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsString(item.as_ref())
}
pub fn cJSON_IsArray(item: Option<&cJSON>) -> cJSON_bool {
    cjson_type_matches(item, cJSON_Array)
}
#[export_name = "cJSON_IsArray"]
pub unsafe extern "C" fn cJSON_IsArray_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsArray(item.as_ref())
}
pub fn cJSON_IsObject(item: Option<&cJSON>) -> cJSON_bool {
    cjson_type_matches(item, cJSON_Object)
}
#[export_name = "cJSON_IsObject"]
pub unsafe extern "C" fn cJSON_IsObject_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsObject(item.as_ref())
}
pub fn cJSON_IsRaw(item: Option<&cJSON>) -> cJSON_bool {
    cjson_type_matches(item, cJSON_Raw)
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
    let (a, b) = match (a, b) {
        (Some(a), Some(b)) => (a, b),
        _ => return false_0,
    };
    let a_type = a.type_0 & 0xff as ::core::ffi::c_int;
    let b_type = b.type_0 & 0xff as ::core::ffi::c_int;
    if a_type != b_type {
        return false_0;
    }
    match a_type {
        cJSON_False | cJSON_True | cJSON_NULL | cJSON_Number | cJSON_String | cJSON_Raw
        | cJSON_Array | cJSON_Object => {}
        _ => return false_0,
    }
    if ::core::ptr::eq(a, b) {
        return true_0;
    }
    match a_type {
        cJSON_False | cJSON_True | cJSON_NULL => return true_0,
        cJSON_Number => {
            if compare_double(a.valuedouble, b.valuedouble) != 0 {
                return true_0;
            }
            return false_0;
        }
        cJSON_String | cJSON_Raw => {
            if a.valuestring.is_null() || b.valuestring.is_null() {
                return false_0;
            }
            let (Some(a_valuestring), Some(b_valuestring)) =
                (valuestring_cstr(a), valuestring_cstr(b))
            else {
                return false_0;
            };
            if cstr_cmp(Some(a_valuestring), Some(b_valuestring)) == 0 as ::core::ffi::c_int {
                return true_0;
            }
            return false_0;
        }
        cJSON_Array => {
            let mut index: size_t = 0;
            loop {
                match (
                    get_array_item(Some(a), index),
                    get_array_item(Some(b), index),
                ) {
                    (Some(a_child), Some(b_child)) => {
                        if cJSON_Compare(Some(a_child), Some(b_child), case_sensitive) == 0 {
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
            let mut a_index: size_t = 0;
            while let Some(a_child) = get_array_item(Some(a), a_index) {
                let a_child_name = string_cstr(a_child);
                let Some(b_child) = get_object_item(Some(b), a_child_name, case_sensitive) else {
                    return false_0;
                };
                if cJSON_Compare(Some(a_child), Some(b_child), case_sensitive) == 0 {
                    return false_0;
                }
                a_index = a_index.wrapping_add(1);
            }
            let mut b_index: size_t = 0;
            while let Some(b_child) = get_array_item(Some(b), b_index) {
                let b_child_name = string_cstr(b_child);
                let Some(a_child) = get_object_item(Some(a), b_child_name, case_sensitive) else {
                    return false_0;
                };
                if cJSON_Compare(Some(b_child), Some(a_child), case_sensitive) == 0 {
                    return false_0;
                }
                b_index = b_index.wrapping_add(1);
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
    let hooks = current_global_hooks();
    return hooks.allocate.expect("non-null function pointer")(size);
}
#[export_name = "cJSON_free"]
pub unsafe extern "C" fn cJSON_free_ffi(mut object: *mut ::core::ffi::c_void) {
    let hooks = current_global_hooks();
    hooks.deallocate.expect("non-null function pointer")(object);
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __DBL_EPSILON__: ::core::ffi::c_double = 2.2204460492503131e-16f64;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const INT_MIN: ::core::ffi::c_int = -2147483647 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
pub const DBL_EPSILON: ::core::ffi::c_double = __DBL_EPSILON__;
