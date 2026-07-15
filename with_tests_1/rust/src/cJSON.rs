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
        Option<unsafe extern "C" fn(crate::__stddef_size_t_h::size_t) -> *mut ::core::ffi::c_void>,
    pub free_fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
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

#[derive(Copy, Clone)]
#[repr(C)]

pub struct internal_hooks {
    pub allocate:
        Option<unsafe extern "C" fn(crate::__stddef_size_t_h::size_t) -> *mut ::core::ffi::c_void>,
    pub deallocate: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub reallocate: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub can_allocate_items: bool,
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
pub enum printbuffer_storage<'a> {
    Owned(Vec<::core::ffi::c_uchar>),
    Borrowed(&'a mut [::core::ffi::c_uchar]),
}

pub struct printbuffer<'a> {
    pub storage: printbuffer_storage<'a>,
    pub length: crate::__stddef_size_t_h::size_t,
    pub offset: crate::__stddef_size_t_h::size_t,
    pub depth: crate::__stddef_size_t_h::size_t,
    pub noalloc: crate::src::cJSON::cJSON_bool,
    pub format: crate::src::cJSON::cJSON_bool,
}

struct printable_cjson<'a> {
    type_0: ::core::ffi::c_int,
    valuestring: Option<&'a ::std::ffi::CStr>,
    valueint: ::core::ffi::c_int,
    valuedouble: ::core::ffi::c_double,
    string: Option<&'a ::std::ffi::CStr>,
    children: Vec<printable_cjson<'a>>,
}

impl<'a> printbuffer<'a> {
    fn owned(
        length: crate::__stddef_size_t_h::size_t,
        format: crate::src::cJSON::cJSON_bool,
    ) -> Option<Self> {
        let length = usize::try_from(length).ok()?;
        let mut storage = Vec::new();
        storage.try_reserve_exact(length).ok()?;
        storage.resize(length, 0);
        Some(Self {
            storage: printbuffer_storage::Owned(storage),
            length: length as crate::__stddef_size_t_h::size_t,
            offset: 0,
            depth: 0,
            noalloc: false_0,
            format,
        })
    }

    fn borrowed(
        storage: &'a mut [::core::ffi::c_uchar],
        format: crate::src::cJSON::cJSON_bool,
    ) -> Self {
        Self {
            length: storage.len() as crate::__stddef_size_t_h::size_t,
            storage: printbuffer_storage::Borrowed(storage),
            offset: 0,
            depth: 0,
            noalloc: true_0,
            format,
        }
    }

    fn bytes(&self) -> &[::core::ffi::c_uchar] {
        match &self.storage {
            printbuffer_storage::Owned(storage) => storage.as_slice(),
            printbuffer_storage::Borrowed(storage) => storage,
        }
    }

    fn output_slice(
        &mut self,
        requested: crate::__stddef_size_t_h::size_t,
    ) -> Option<&mut [::core::ffi::c_uchar]> {
        let start = usize::try_from(self.offset).ok()?;
        let requested = usize::try_from(requested).ok()?;
        let end = start.checked_add(requested)?;
        match &mut self.storage {
            printbuffer_storage::Owned(storage) => storage.get_mut(start..end),
            printbuffer_storage::Borrowed(storage) => storage.get_mut(start..end),
        }
    }
}

fn printable_child_at_path_mut<'a, 'b>(
    mut item: &'a mut printable_cjson<'b>,
    path: &[usize],
) -> Option<&'a mut printable_cjson<'b>> {
    for &index in path {
        item = item.children.get_mut(index)?;
    }
    Some(item)
}

pub const true_0: crate::src::cJSON::cJSON_bool = 1 as ::core::ffi::c_int;

pub const false_0: crate::src::cJSON::cJSON_bool = 0 as ::core::ffi::c_int;

fn empty_cjson() -> crate::src::cJSON::cJSON {
    crate::src::cJSON::cJSON {
        next: ::core::ptr::null_mut::<crate::src::cJSON::cJSON>(),
        prev: ::core::ptr::null_mut::<crate::src::cJSON::cJSON>(),
        child: ::core::ptr::null_mut::<crate::src::cJSON::cJSON>(),
        type_0: crate::src::cJSON::cJSON_Invalid,
        valuestring: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        valueint: 0,
        valuedouble: 0.0,
        string: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    }
}

static GLOBAL_ERROR: ::std::sync::Mutex<error> = ::std::sync::Mutex::new(error {
    json: 0,
    position: 0 as crate::__stddef_size_t_h::size_t,
});

static RUST_OWNED_C_STRINGS: ::std::sync::LazyLock<
    ::std::sync::Mutex<::std::collections::HashMap<usize, Vec<::core::ffi::c_uchar>>>,
> = ::std::sync::LazyLock::new(|| ::std::sync::Mutex::new(::std::collections::HashMap::new()));

fn global_error() -> ::std::sync::MutexGuard<'static, error> {
    GLOBAL_ERROR
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn reset_global_error() {
    *global_error() = error {
        json: 0,
        position: 0 as crate::__stddef_size_t_h::size_t,
    };
}

fn set_global_error(error: error) {
    *global_error() = error;
}

fn rust_owned_c_strings(
) -> ::std::sync::MutexGuard<'static, ::std::collections::HashMap<usize, Vec<::core::ffi::c_uchar>>>
{
    RUST_OWNED_C_STRINGS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn unregister_owned_c_string(address: usize) -> bool {
    if address == 0 {
        return false;
    }
    rust_owned_c_strings().remove(&address).is_some()
}

pub fn cJSON_GetErrorPtr() -> Option<usize> {
    let error = global_error();
    if error.json == 0 {
        return None;
    }
    return Some(error.json.wrapping_add(error.position));
}
#[export_name = "cJSON_GetErrorPtr"]

pub unsafe extern "C" fn cJSON_GetErrorPtr_ffi() -> *const ::core::ffi::c_char {
    match cJSON_GetErrorPtr() {
        Some(error_ptr) => error_ptr as *const ::core::ffi::c_char,
        None => ::core::ptr::null(),
    }
}
#[export_name = "cJSON_GetStringValue"]

pub unsafe extern "C" fn cJSON_GetStringValue_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> *mut ::core::ffi::c_char {
    let item = unsafe { item.as_ref() };
    if cJSON_IsString(item) == 0 {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    item.expect("cJSON_IsString rejects null items").valuestring
}
pub fn cJSON_GetNumberValue(item: Option<&crate::src::cJSON::cJSON>) -> ::core::ffi::c_double {
    if cJSON_IsNumber(item) == 0 {
        return 0.0f64 / 0.0f64;
    }
    return item.expect("cJSON_IsNumber rejects null items").valuedouble;
}
#[export_name = "cJSON_GetNumberValue"]

pub unsafe extern "C" fn cJSON_GetNumberValue_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> ::core::ffi::c_double {
    cJSON_GetNumberValue(unsafe { item.as_ref() })
}
pub fn cJSON_Version() -> &'static [::core::ffi::c_char; 7] {
    static VERSION: [::core::ffi::c_char; 7] = [
        b'1' as ::core::ffi::c_char,
        b'.' as ::core::ffi::c_char,
        b'7' as ::core::ffi::c_char,
        b'.' as ::core::ffi::c_char,
        b'1' as ::core::ffi::c_char,
        b'9' as ::core::ffi::c_char,
        0,
    ];
    return &VERSION;
}
#[export_name = "cJSON_Version"]

pub unsafe extern "C" fn cJSON_Version_ffi() -> *const ::core::ffi::c_char {
    cJSON_Version().as_ptr()
}
fn case_insensitive_strcmp(
    string1: Option<&::std::ffi::CStr>,
    string2: Option<&::std::ffi::CStr>,
) -> ::core::ffi::c_int {
    let (Some(string1), Some(string2)) = (string1, string2) else {
        return 1 as ::core::ffi::c_int;
    };
    if string1.as_ptr() == string2.as_ptr() {
        return 0 as ::core::ffi::c_int;
    }
    for (&byte1, &byte2) in string1
        .to_bytes_with_nul()
        .iter()
        .zip(string2.to_bytes_with_nul().iter())
    {
        let lower1 = byte1.to_ascii_lowercase();
        let lower2 = byte2.to_ascii_lowercase();
        if lower1 != lower2 {
            return lower1 as ::core::ffi::c_int - lower2 as ::core::ffi::c_int;
        }
        if byte1 as ::core::ffi::c_int == '\0' as i32 {
            return 0 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}

static global_hooks: ::std::sync::Mutex<internal_hooks> = ::std::sync::Mutex::new(internal_hooks {
    allocate: Some(
        crate::stdlib::malloc
            as unsafe extern "C" fn(crate::__stddef_size_t_h::size_t) -> *mut ::core::ffi::c_void,
    ),
    deallocate: Some(crate::stdlib::free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    reallocate: Some(
        crate::stdlib::realloc
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                crate::__stddef_size_t_h::size_t,
            ) -> *mut ::core::ffi::c_void,
    ),
    can_allocate_items: true,
});

fn global_hooks_lock() -> ::std::sync::MutexGuard<'static, internal_hooks> {
    global_hooks
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn global_hooks_snapshot() -> internal_hooks {
    *global_hooks_lock()
}

fn set_global_hooks(hooks: internal_hooks) {
    *global_hooks_lock() = hooks;
}

fn cJSON_strdup(string: Option<&::std::ffi::CStr>) -> Option<::std::ffi::CString> {
    let string = string?;
    let bytes = string.to_bytes();
    let mut copy = Vec::new();
    copy.try_reserve_exact(bytes.len()).ok()?;
    copy.extend_from_slice(bytes);
    ::std::ffi::CString::new(copy).ok()
}
pub fn cJSON_InitHooks(hooks: Option<&crate::src::cJSON::cJSON_Hooks>) {
    let Some(hooks) = hooks else {
        set_global_hooks(internal_hooks {
            allocate: Some(
                crate::stdlib::malloc
                    as unsafe extern "C" fn(
                        crate::__stddef_size_t_h::size_t,
                    ) -> *mut ::core::ffi::c_void,
            ),
            deallocate: Some(
                crate::stdlib::free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
            ),
            reallocate: Some(
                crate::stdlib::realloc
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        crate::__stddef_size_t_h::size_t,
                    ) -> *mut ::core::ffi::c_void,
            ),
            can_allocate_items: true,
        });
        return;
    };

    let mut new_hooks = internal_hooks {
        allocate: Some(
            crate::stdlib::malloc
                as unsafe extern "C" fn(
                    crate::__stddef_size_t_h::size_t,
                ) -> *mut ::core::ffi::c_void,
        ),
        deallocate: Some(
            crate::stdlib::free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
        ),
        reallocate: None,
        can_allocate_items: true,
    };
    if hooks.malloc_fn.is_some() {
        new_hooks.allocate = hooks.malloc_fn;
        new_hooks.can_allocate_items = false;
    }
    if hooks.free_fn.is_some() {
        new_hooks.deallocate = hooks.free_fn;
    }
    if new_hooks.allocate
        == Some(
            crate::stdlib::malloc
                as unsafe extern "C" fn(
                    crate::__stddef_size_t_h::size_t,
                ) -> *mut ::core::ffi::c_void,
        )
        && new_hooks.deallocate
            == Some(crate::stdlib::free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ())
    {
        new_hooks.reallocate = Some(
            crate::stdlib::realloc
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    crate::__stddef_size_t_h::size_t,
                ) -> *mut ::core::ffi::c_void,
        );
    }
    set_global_hooks(new_hooks);
}
#[export_name = "cJSON_InitHooks"]

pub unsafe extern "C" fn cJSON_InitHooks_ffi(mut hooks: *mut crate::src::cJSON::cJSON_Hooks) {
    cJSON_InitHooks(unsafe { hooks.as_ref() })
}
fn cJSON_New_Item(hooks: &internal_hooks) -> Option<&'static mut crate::src::cJSON::cJSON> {
    if !hooks.can_allocate_items {
        return None;
    }
    let mut allocation = Vec::new();
    allocation.try_reserve_exact(1).ok()?;
    allocation.push(empty_cjson());
    Box::leak(allocation.into_boxed_slice()).first_mut()
}

#[export_name = "cJSON_Delete"]

pub unsafe extern "C" fn cJSON_Delete_ffi(mut item: *mut crate::src::cJSON::cJSON) {
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
            let valuestring_address = (*item).valuestring as usize;
            if !unregister_owned_c_string(valuestring_address) {
                global_hooks_snapshot()
                    .deallocate
                    .expect("non-null function pointer")(
                    (*item).valuestring as *mut ::core::ffi::c_void,
                );
            }
            (*item).valuestring = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if (*item).type_0 & crate::src::cJSON::cJSON_StringIsConst == 0 && !(*item).string.is_null()
        {
            let string_address = (*item).string as usize;
            if !unregister_owned_c_string(string_address) {
                global_hooks_snapshot()
                    .deallocate
                    .expect("non-null function pointer")(
                    (*item).string as *mut ::core::ffi::c_void,
                );
            }
            (*item).string = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        drop(unsafe { Box::from_raw(::core::ptr::slice_from_raw_parts_mut(item, 1)) });
        item = next;
    }
}
fn parse_number(
    item: &mut crate::src::cJSON::cJSON,
    input_buffer: &mut parse_buffer<'_>,
    content: &[::core::ffi::c_uchar],
) -> crate::src::cJSON::cJSON_bool {
    let Some(remaining) = content.get(input_buffer.offset as usize..) else {
        return false_0;
    };
    let mut number_string_length: usize = 0;
    for byte in remaining {
        match *byte as ::core::ffi::c_int {
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
    }

    let source_number = &remaining[..number_string_length];
    let mut parsed_number: Option<(::core::ffi::c_double, usize)> = None;
    for end in (1..=source_number.len()).rev() {
        let Ok(number_text) = ::core::str::from_utf8(&source_number[..end]) else {
            continue;
        };
        if let Ok(number) = number_text.parse::<::core::ffi::c_double>() {
            parsed_number = Some((number, end));
            break;
        }
    }
    let Some((number, parsed_length)) = parsed_number else {
        return false_0;
    };

    item.valuedouble = number;
    if number >= crate::limits_h::INT_MAX as ::core::ffi::c_double {
        item.valueint = crate::limits_h::INT_MAX;
    } else if number <= crate::limits_h::INT_MIN as ::core::ffi::c_double {
        item.valueint = crate::limits_h::INT_MIN;
    } else {
        item.valueint = number as ::core::ffi::c_int;
    }
    item.type_0 = crate::src::cJSON::cJSON_Number;
    input_buffer.offset = input_buffer
        .offset
        .wrapping_add(parsed_length as crate::__stddef_size_t_h::size_t);
    return true_0;
}
pub fn cJSON_SetNumberHelper(
    object: Option<&mut crate::src::cJSON::cJSON>,
    mut number: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let Some(object) = object else {
        return 0.0f64 / 0.0f64;
    };

    if number >= crate::limits_h::INT_MAX as ::core::ffi::c_double {
        object.valueint = crate::limits_h::INT_MAX;
    } else if number <= crate::limits_h::INT_MIN as ::core::ffi::c_double {
        object.valueint = crate::limits_h::INT_MIN;
    } else {
        object.valueint = number as ::core::ffi::c_int;
    }
    object.valuedouble = number;
    return object.valuedouble;
}
#[export_name = "cJSON_SetNumberHelper"]

pub unsafe extern "C" fn cJSON_SetNumberHelper_ffi(
    mut object: *mut crate::src::cJSON::cJSON,
    mut number: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    cJSON_SetNumberHelper(unsafe { object.as_mut() }, number)
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
    let new_value = ::std::ffi::CStr::from_ptr(valuestring);
    let old_value = ::std::ffi::CStr::from_ptr((*object).valuestring);
    v1_len = new_value.to_bytes().len();
    v2_len = old_value.to_bytes().len();
    if v1_len <= v2_len {
        if !(valuestring.offset(v1_len as isize)
            < (*object).valuestring as *const ::core::ffi::c_char
            || (*object).valuestring.offset(v2_len as isize)
                < valuestring as *mut ::core::ffi::c_char)
        {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        let source = new_value.to_bytes_with_nul();
        let destination = ::core::slice::from_raw_parts_mut(
            (*object).valuestring as *mut ::core::ffi::c_uchar,
            source.len(),
        );
        destination.copy_from_slice(source);
        return (*object).valuestring;
    }
    copy = match cJSON_strdup(Some(new_value)) {
        Some(copy) => copy.into_raw(),
        None => ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    if copy.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !(*object).valuestring.is_null() {
        let old_valuestring_address = (*object).valuestring as usize;
        if !unregister_owned_c_string(old_valuestring_address) {
            global_hooks_snapshot()
                .deallocate
                .expect("non-null function pointer")(
                (*object).valuestring as *mut ::core::ffi::c_void,
            );
        }
    }
    (*object).valuestring = copy;
    return copy;
}
fn ensure<'p>(
    p: Option<&'p mut printbuffer<'_>>,
    mut needed: crate::__stddef_size_t_h::size_t,
) -> Option<&'p mut [::core::ffi::c_uchar]> {
    let Some(p) = p else {
        return None;
    };
    let mut newsize: crate::__stddef_size_t_h::size_t = 0 as crate::__stddef_size_t_h::size_t;
    if p.length > 0 as crate::__stddef_size_t_h::size_t && p.offset >= p.length {
        return None;
    }
    if needed > crate::limits_h::INT_MAX as crate::__stddef_size_t_h::size_t {
        return None;
    }
    let requested = needed;
    needed = needed.checked_add(p.offset.wrapping_add(1 as crate::__stddef_size_t_h::size_t))?;
    if needed <= p.length {
        return p.output_slice(requested);
    }
    if p.noalloc != 0 {
        return None;
    }
    if needed
        > (crate::limits_h::INT_MAX / 2 as ::core::ffi::c_int) as crate::__stddef_size_t_h::size_t
    {
        if needed <= crate::limits_h::INT_MAX as crate::__stddef_size_t_h::size_t {
            newsize = crate::limits_h::INT_MAX as crate::__stddef_size_t_h::size_t;
        } else {
            return None;
        }
    } else {
        newsize = needed.wrapping_mul(2 as crate::__stddef_size_t_h::size_t);
    }
    let printbuffer_storage::Owned(storage) = &mut p.storage else {
        return None;
    };
    let newsize = usize::try_from(newsize).ok()?;
    if storage.len() < newsize {
        storage.try_reserve_exact(newsize - storage.len()).ok()?;
        storage.resize(newsize, 0);
    }
    p.length = newsize;
    return p.output_slice(requested);
}

fn compare_double(
    mut a: ::core::ffi::c_double,
    mut b: ::core::ffi::c_double,
) -> crate::src::cJSON::cJSON_bool {
    let mut maxVal: ::core::ffi::c_double = if a.abs() > b.abs() { a.abs() } else { b.abs() };
    return ((a - b).abs() <= maxVal * crate::float_h::DBL_EPSILON) as ::core::ffi::c_int;
}

fn trim_fraction(number: &mut String) {
    if !number.contains('.') {
        return;
    }
    while number.ends_with('0') {
        number.pop();
    }
    if number.ends_with('.') {
        number.pop();
    }
}

fn c_style_exponent(exponent: ::core::ffi::c_int) -> String {
    let mut output = String::new();
    output.push('e');
    let magnitude = if exponent < 0 {
        output.push('-');
        exponent.unsigned_abs()
    } else {
        output.push('+');
        exponent as ::core::ffi::c_uint
    };
    if magnitude < 10 {
        output.push('0');
    }

    let mut digits = [0u8; 10];
    let mut digit_count = 0usize;
    let mut remaining = magnitude;
    loop {
        digits[digit_count] = b'0'.wrapping_add((remaining % 10) as u8);
        digit_count = digit_count.wrapping_add(1);
        remaining /= 10;
        if remaining == 0 {
            break;
        }
    }
    while digit_count > 0 {
        digit_count = digit_count.wrapping_sub(1);
        output.push(char::from(digits[digit_count]));
    }

    output
}

enum DoubleStyle {
    Normal,
    Scientific,
}

fn format_double(
    value: ::core::ffi::c_double,
    precision: ::core::ffi::c_int,
    style: DoubleStyle,
) -> Option<String> {
    let precision = ::core::ffi::c_uint::try_from(precision).ok()?;
    let mut format = String::from("%.");
    let mut digits = [0u8; 10];
    let mut digit_count = 0usize;
    let mut remaining = precision;
    loop {
        digits[digit_count] = b'0'.wrapping_add((remaining % 10) as u8);
        digit_count = digit_count.wrapping_add(1);
        remaining /= 10;
        if remaining == 0 {
            break;
        }
    }
    while digit_count > 0 {
        digit_count = digit_count.wrapping_sub(1);
        format.push(char::from(digits[digit_count]));
    }
    match style {
        DoubleStyle::Normal => format.push('f'),
        DoubleStyle::Scientific => format.push('e'),
    }
    let args: [&dyn sprintf::Printf; 1] = [&value];
    sprintf::vsprintf(&format, &args).ok()
}

fn format_g_scientific(scientific: &str, negative: bool, exponent: ::core::ffi::c_int) -> String {
    let mantissa_end = scientific.find('e').unwrap_or(scientific.len());
    let mut mantissa = scientific[..mantissa_end].to_string();
    trim_fraction(&mut mantissa);

    let mut output = String::new();
    if negative {
        output.push('-');
    }
    output.push_str(&mantissa);
    output.push_str(&c_style_exponent(exponent));
    output
}

fn format_g(value: ::core::ffi::c_double, precision: usize) -> Option<String> {
    if precision == 0 || !value.is_finite() {
        return None;
    }

    let negative = value.is_sign_negative();
    let value = value.abs();
    if value == 0.0 {
        Some("0".to_string())
    } else {
        let scientific = format_double(
            value,
            precision.wrapping_sub(1) as ::core::ffi::c_int,
            DoubleStyle::Scientific,
        )?;
        match scientific
            .split_once('e')
            .and_then(|(_, exponent)| exponent.parse::<::core::ffi::c_int>().ok())
        {
            Some(exponent) => {
                if exponent < -4 || exponent >= precision as ::core::ffi::c_int {
                    Some(format_g_scientific(&scientific, negative, exponent))
                } else {
                    let fraction_digits = (precision as ::core::ffi::c_int)
                        .saturating_sub(exponent.saturating_add(1))
                        .max(0);
                    let mut output = format_double(value, fraction_digits, DoubleStyle::Normal)?;
                    trim_fraction(&mut output);
                    if negative {
                        output.insert(0, '-');
                    }
                    Some(output)
                }
            }
            None => None,
        }
    }
}

fn print_number(
    valueint: ::core::ffi::c_int,
    valuedouble: ::core::ffi::c_double,
    output_buffer: Option<&mut printbuffer<'_>>,
) -> crate::src::cJSON::cJSON_bool {
    let Some(output_buffer) = output_buffer else {
        return false_0;
    };
    let d: ::core::ffi::c_double = valuedouble;
    let number = if !d.is_finite() {
        "null".to_string()
    } else if d == valueint as ::core::ffi::c_double {
        valueint.to_string()
    } else {
        let Some(candidate) = format_g(d, 15) else {
            return false_0;
        };
        if candidate
            .parse::<::core::ffi::c_double>()
            .map_or(true, |test| compare_double(test, d) == 0)
        {
            let Some(fallback) = format_g(d, 17) else {
                return false_0;
            };
            fallback
        } else {
            candidate
        }
    };

    let length = number.len();
    if length > 25 {
        return false_0;
    }
    let needed = (length as crate::__stddef_size_t_h::size_t).wrapping_add(1);
    let Some(output) = ensure(Some(&mut *output_buffer), needed) else {
        return false_0;
    };
    output[..length].copy_from_slice(number.as_bytes());
    output[length] = b'\0';
    output_buffer.offset = output_buffer
        .offset
        .wrapping_add(length as crate::__stddef_size_t_h::size_t);
    return true_0;
}

fn parse_hex4(input: &[::core::ffi::c_uchar]) -> ::core::ffi::c_uint {
    let mut h: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut i: usize = 0;
    while i < 4 {
        let byte = input[i];
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
        if i < 3 {
            h = h << 4 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return h;
}

fn utf16_literal_to_utf8(
    input: &[::core::ffi::c_uchar],
    output: &mut Vec<::core::ffi::c_uchar>,
) -> ::core::ffi::c_uchar {
    if input.len() < 6 {
        return 0 as ::core::ffi::c_uchar;
    }

    let first_code = parse_hex4(&input[2..6]);
    if (0xdc00 as ::core::ffi::c_uint..=0xdfff as ::core::ffi::c_uint).contains(&first_code) {
        return 0 as ::core::ffi::c_uchar;
    }

    let (sequence_length, mut codepoint) = if (0xd800 as ::core::ffi::c_uint
        ..=0xdbff as ::core::ffi::c_uint)
        .contains(&first_code)
    {
        if input.len() < 12
            || input[6] as ::core::ffi::c_int != '\\' as i32
            || input[7] as ::core::ffi::c_int != 'u' as i32
        {
            return 0 as ::core::ffi::c_uchar;
        }

        let second_code = parse_hex4(&input[8..12]);
        if !(0xdc00 as ::core::ffi::c_uint..=0xdfff as ::core::ffi::c_uint).contains(&second_code) {
            return 0 as ::core::ffi::c_uchar;
        }

        (
            12 as ::core::ffi::c_uchar,
            (0x10000 as ::core::ffi::c_int as ::core::ffi::c_uint).wrapping_add(
                (first_code & 0x3ff as ::core::ffi::c_uint) << 10 as ::core::ffi::c_int
                    | second_code & 0x3ff as ::core::ffi::c_uint,
            ) as ::core::ffi::c_ulong,
        )
    } else {
        (
            6 as ::core::ffi::c_uchar,
            first_code as ::core::ffi::c_ulong,
        )
    };

    let (utf8_length, first_byte_mark) = if codepoint < 0x80 as ::core::ffi::c_ulong {
        (1usize, 0 as ::core::ffi::c_uchar)
    } else if codepoint < 0x800 as ::core::ffi::c_ulong {
        (2usize, 0xc0 as ::core::ffi::c_uchar)
    } else if codepoint < 0x10000 as ::core::ffi::c_ulong {
        (3usize, 0xe0 as ::core::ffi::c_uchar)
    } else if codepoint <= 0x10ffff as ::core::ffi::c_ulong {
        (4usize, 0xf0 as ::core::ffi::c_uchar)
    } else {
        return 0 as ::core::ffi::c_uchar;
    };

    if output.capacity().saturating_sub(output.len()) < utf8_length {
        return 0 as ::core::ffi::c_uchar;
    }

    let mut encoded = [0 as ::core::ffi::c_uchar; 4];
    let mut utf8_position = utf8_length - 1;
    while utf8_position > 0 {
        encoded[utf8_position] = ((codepoint | 0x80 as ::core::ffi::c_ulong)
            & 0xbf as ::core::ffi::c_ulong)
            as ::core::ffi::c_uchar;
        codepoint >>= 6 as ::core::ffi::c_int;
        utf8_position -= 1;
    }
    if utf8_length > 1 {
        encoded[0] = ((codepoint | first_byte_mark as ::core::ffi::c_ulong)
            & 0xff as ::core::ffi::c_ulong) as ::core::ffi::c_uchar;
    } else {
        encoded[0] = (codepoint & 0x7f as ::core::ffi::c_ulong) as ::core::ffi::c_uchar;
    }
    output.extend_from_slice(&encoded[..utf8_length]);
    return sequence_length;
}

fn parse_string(
    item: &mut crate::src::cJSON::cJSON,
    input_buffer: &mut parse_buffer<'_>,
    content: &[::core::ffi::c_uchar],
) -> crate::src::cJSON::cJSON_bool {
    let start = input_buffer.offset;
    let mut input_index = start.wrapping_add(1);
    if content.get(start) != Some(&b'"') {
        input_buffer.offset = input_index;
        return false_0;
    }

    let mut input_end = input_index;
    let mut skipped_bytes: crate::__stddef_size_t_h::size_t = 0 as crate::__stddef_size_t_h::size_t;
    let mut scan_failed = false;
    while input_end < input_buffer.length && content.get(input_end) != Some(&b'"') {
        let Some(&byte) = content.get(input_end) else {
            scan_failed = true;
            break;
        };
        if byte as ::core::ffi::c_int == '\\' as i32 {
            if input_end.wrapping_add(1) >= input_buffer.length {
                scan_failed = true;
                break;
            }
            skipped_bytes = skipped_bytes.wrapping_add(1);
            input_end = input_end.wrapping_add(1);
        }
        input_end = input_end.wrapping_add(1);
    }

    if scan_failed || input_end >= input_buffer.length || content.get(input_end) != Some(&b'"') {
        input_buffer.offset = input_index;
        return false_0;
    }

    let allocation_length = input_end.wrapping_sub(start).wrapping_sub(skipped_bytes);
    let allocation_size = allocation_length
        .wrapping_add(
            ::core::mem::size_of::<[::core::ffi::c_char; 1]>() as crate::__stddef_size_t_h::size_t
        );
    let mut decoded_bytes = Vec::new();
    if decoded_bytes
        .try_reserve_exact(allocation_size as usize)
        .is_err()
    {
        input_buffer.offset = input_index;
        return false_0;
    }
    let allocation_failure_offset = input_index;

    let decoded = {
        let mut decode_failed = false;
        while input_index < input_end {
            let Some(&byte) = content.get(input_index) else {
                decode_failed = true;
                break;
            };
            if byte as ::core::ffi::c_int != '\\' as i32 {
                if decoded_bytes.capacity() > decoded_bytes.len() {
                    decoded_bytes.push(byte);
                    input_index = input_index.wrapping_add(1);
                } else {
                    decode_failed = true;
                    break;
                }
            } else {
                let mut sequence_length: ::core::ffi::c_uchar = 2 as ::core::ffi::c_uchar;
                if input_end.wrapping_sub(input_index) < 1 as crate::__stddef_size_t_h::size_t {
                    decode_failed = true;
                    break;
                }
                let Some(&escaped) = content.get(input_index.wrapping_add(1)) else {
                    decode_failed = true;
                    break;
                };
                match escaped as ::core::ffi::c_int {
                    98 => {
                        if decoded_bytes.capacity() > decoded_bytes.len() {
                            decoded_bytes.push('\u{8}' as i32 as ::core::ffi::c_uchar);
                        } else {
                            decode_failed = true;
                            break;
                        }
                    }
                    102 => {
                        if decoded_bytes.capacity() > decoded_bytes.len() {
                            decoded_bytes.push('\u{c}' as i32 as ::core::ffi::c_uchar);
                        } else {
                            decode_failed = true;
                            break;
                        }
                    }
                    110 => {
                        if decoded_bytes.capacity() > decoded_bytes.len() {
                            decoded_bytes.push('\n' as i32 as ::core::ffi::c_uchar);
                        } else {
                            decode_failed = true;
                            break;
                        }
                    }
                    114 => {
                        if decoded_bytes.capacity() > decoded_bytes.len() {
                            decoded_bytes.push('\r' as i32 as ::core::ffi::c_uchar);
                        } else {
                            decode_failed = true;
                            break;
                        }
                    }
                    116 => {
                        if decoded_bytes.capacity() > decoded_bytes.len() {
                            decoded_bytes.push('\t' as i32 as ::core::ffi::c_uchar);
                        } else {
                            decode_failed = true;
                            break;
                        }
                    }
                    34 | 92 | 47 => {
                        if decoded_bytes.capacity() > decoded_bytes.len() {
                            decoded_bytes.push(escaped);
                        } else {
                            decode_failed = true;
                            break;
                        }
                    }
                    117 => {
                        let Some(remaining_input) = content.get(input_index..input_end) else {
                            decode_failed = true;
                            break;
                        };
                        sequence_length =
                            utf16_literal_to_utf8(remaining_input, &mut decoded_bytes);
                        if sequence_length as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            decode_failed = true;
                            break;
                        }
                    }
                    _ => {
                        decode_failed = true;
                        break;
                    }
                }
                input_index = input_index.wrapping_add(sequence_length as usize);
            }
        }
        if !decode_failed {
            if decoded_bytes.capacity() > decoded_bytes.len() {
                decoded_bytes.push('\0' as i32 as ::core::ffi::c_uchar);
            } else {
                decode_failed = true;
            }
        }
        !decode_failed
    };

    if decoded {
        let Some(output_byte) = decoded_bytes.first_mut() else {
            input_buffer.offset = allocation_failure_offset;
            return false_0;
        };
        let output = output_byte as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char;
        {
            let mut owned_strings = rust_owned_c_strings();
            if owned_strings.try_reserve(1).is_err() {
                input_buffer.offset = allocation_failure_offset;
                return false_0;
            }
            owned_strings.insert(output as usize, decoded_bytes);
        }
        item.type_0 = crate::src::cJSON::cJSON_String;
        item.valuestring = output;
        input_buffer.offset = input_end.wrapping_add(1);
        return true_0;
    }

    input_buffer.offset = input_index;
    return false_0;
}

fn hex_digit(n: ::core::ffi::c_uchar) -> ::core::ffi::c_uchar {
    match n {
        0..=9 => b'0'.wrapping_add(n),
        _ => b'a'.wrapping_add(n.wrapping_sub(10)),
    }
}

fn print_string_ptr(
    input: Option<&::std::ffi::CStr>,
    output_buffer: Option<&mut printbuffer<'_>>,
) -> crate::src::cJSON::cJSON_bool {
    let Some(output_buffer) = output_buffer else {
        return false_0;
    };
    let input = match input {
        Some(input) => input.to_bytes(),
        None => b"",
    };
    let mut escape_characters: crate::__stddef_size_t_h::size_t =
        0 as crate::__stddef_size_t_h::size_t;
    for &byte in input {
        match byte as ::core::ffi::c_int {
            34 | 92 | 8 | 12 | 10 | 13 | 9 => {
                escape_characters = escape_characters.wrapping_add(1);
            }
            _ => {
                if (byte as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
                    escape_characters =
                        escape_characters.wrapping_add(5 as crate::__stddef_size_t_h::size_t);
                }
            }
        }
    }
    let output_length =
        (input.len() as crate::__stddef_size_t_h::size_t).wrapping_add(escape_characters);
    let needed =
        output_length
            .wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 3]>()
                as crate::__stddef_size_t_h::size_t);
    let Some(output) = ensure(Some(&mut *output_buffer), needed) else {
        return false_0;
    };
    if escape_characters == 0 as crate::__stddef_size_t_h::size_t {
        output[0] = b'"';
        output[1..1 + input.len()].copy_from_slice(input);
        output[output_length.wrapping_add(1)] = b'"';
        output[output_length.wrapping_add(2)] = b'\0';
        output_buffer.offset = output_buffer
            .offset
            .wrapping_add(output_length.wrapping_add(2));
        return true_0;
    }
    output[0] = b'"';
    let mut output_offset = 1usize;
    for &byte in input {
        if byte as ::core::ffi::c_int > 31 as ::core::ffi::c_int
            && byte as ::core::ffi::c_int != '"' as i32
            && byte as ::core::ffi::c_int != '\\' as i32
        {
            output[output_offset] = byte;
            output_offset = output_offset.wrapping_add(1);
        } else {
            output[output_offset] = b'\\';
            output_offset = output_offset.wrapping_add(1);
            match byte as ::core::ffi::c_int {
                92 => {
                    output[output_offset] = b'\\';
                    output_offset = output_offset.wrapping_add(1);
                }
                34 => {
                    output[output_offset] = b'"';
                    output_offset = output_offset.wrapping_add(1);
                }
                8 => {
                    output[output_offset] = b'b';
                    output_offset = output_offset.wrapping_add(1);
                }
                12 => {
                    output[output_offset] = b'f';
                    output_offset = output_offset.wrapping_add(1);
                }
                10 => {
                    output[output_offset] = b'n';
                    output_offset = output_offset.wrapping_add(1);
                }
                13 => {
                    output[output_offset] = b'r';
                    output_offset = output_offset.wrapping_add(1);
                }
                9 => {
                    output[output_offset] = b't';
                    output_offset = output_offset.wrapping_add(1);
                }
                _ => {
                    output[output_offset] = b'u';
                    output[output_offset.wrapping_add(1)] = b'0';
                    output[output_offset.wrapping_add(2)] = b'0';
                    output[output_offset.wrapping_add(3)] = hex_digit(byte >> 4);
                    output[output_offset.wrapping_add(4)] = hex_digit(byte & 0xf);
                    output_offset = output_offset.wrapping_add(5);
                }
            }
        }
    }
    output[output_length.wrapping_add(1)] = b'"';
    output[output_length.wrapping_add(2)] = b'\0';
    output_buffer.offset = output_buffer
        .offset
        .wrapping_add(output_length.wrapping_add(2));
    return true_0;
}

fn buffer_skip_whitespace(buffer: Option<&mut parse_buffer<'_>>) -> bool {
    let Some(buffer) = buffer else {
        return false;
    };
    if buffer
        .offset
        .wrapping_add(0 as crate::__stddef_size_t_h::size_t)
        >= buffer.length
    {
        return true;
    }
    let content = buffer.content;
    while let Some(&byte) = content.get(buffer.offset as usize) {
        if byte as ::core::ffi::c_int > 32 as ::core::ffi::c_int {
            break;
        }
        buffer.offset = buffer.offset.wrapping_add(1);
    }
    if buffer.offset == buffer.length {
        buffer.offset = buffer.offset.wrapping_sub(1);
    }
    true
}

fn skip_utf8_bom(buffer: Option<&mut parse_buffer<'_>>) -> bool {
    let Some(buffer) = buffer else {
        return false;
    };
    if buffer.offset != 0 as crate::__stddef_size_t_h::size_t {
        return false;
    }
    let content = buffer.content;
    if buffer
        .offset
        .wrapping_add(4 as crate::__stddef_size_t_h::size_t)
        < buffer.length
        && content.get(buffer.offset as usize..buffer.offset as usize + 3)
            == Some(&b"\xEF\xBB\xBF"[..])
    {
        buffer.offset = buffer
            .offset
            .wrapping_add(3 as crate::__stddef_size_t_h::size_t);
    }
    true
}

macro_rules! printable_tree_from_raw {
    ($item:expr) => {{
        'build: {
            let Some(raw_item) = (unsafe { ($item).as_ref() }) else {
                break 'build None;
            };
            let raw_type = raw_item.type_0 & 0xff as ::core::ffi::c_int;
            let mut root = printable_cjson {
                type_0: raw_item.type_0,
                valuestring: if raw_type == crate::src::cJSON::cJSON_String
                    || raw_type == crate::src::cJSON::cJSON_Raw
                {
                    if raw_item.valuestring.is_null() {
                        None
                    } else {
                        Some(unsafe { ::std::ffi::CStr::from_ptr(raw_item.valuestring) })
                    }
                } else {
                    None
                },
                valueint: raw_item.valueint,
                valuedouble: raw_item.valuedouble,
                string: None,
                children: Vec::new(),
            };
            let mut stack: Vec<(Vec<usize>, *const crate::src::cJSON::cJSON, bool)> = Vec::new();
            if (raw_type == crate::src::cJSON::cJSON_Array
                || raw_type == crate::src::cJSON::cJSON_Object)
                && !raw_item.child.is_null()
            {
                if stack.try_reserve_exact(1).is_err() {
                    break 'build None;
                }
                stack.push((
                    Vec::new(),
                    raw_item.child,
                    raw_type == crate::src::cJSON::cJSON_Object,
                ));
            }
            while let Some((parent_path, mut child, names_are_used)) = stack.pop() {
                while let Some(child_ref) = unsafe { child.as_ref() } {
                    let child_type = child_ref.type_0 & 0xff as ::core::ffi::c_int;
                    let printable_child = printable_cjson {
                        type_0: child_ref.type_0,
                        valuestring: if child_type == crate::src::cJSON::cJSON_String
                            || child_type == crate::src::cJSON::cJSON_Raw
                        {
                            if child_ref.valuestring.is_null() {
                                None
                            } else {
                                Some(unsafe { ::std::ffi::CStr::from_ptr(child_ref.valuestring) })
                            }
                        } else {
                            None
                        },
                        valueint: child_ref.valueint,
                        valuedouble: child_ref.valuedouble,
                        string: if names_are_used && !child_ref.string.is_null() {
                            Some(unsafe { ::std::ffi::CStr::from_ptr(child_ref.string) })
                        } else {
                            None
                        },
                        children: Vec::new(),
                    };
                    let child_index = {
                        let Some(parent) = printable_child_at_path_mut(&mut root, &parent_path)
                        else {
                            break 'build None;
                        };
                        if parent.children.try_reserve_exact(1).is_err() {
                            break 'build None;
                        }
                        parent.children.push(printable_child);
                        parent.children.len().wrapping_sub(1)
                    };
                    if (child_type == crate::src::cJSON::cJSON_Array
                        || child_type == crate::src::cJSON::cJSON_Object)
                        && !child_ref.child.is_null()
                    {
                        let mut child_path = parent_path.clone();
                        if child_path.try_reserve_exact(1).is_err() {
                            break 'build None;
                        }
                        child_path.push(child_index);
                        if stack.try_reserve_exact(1).is_err() {
                            break 'build None;
                        }
                        stack.push((
                            child_path,
                            child_ref.child,
                            child_type == crate::src::cJSON::cJSON_Object,
                        ));
                    }
                    child = child_ref.next;
                }
            }
            Some(root)
        }
    }};
}

macro_rules! duplicate_tree_from_raw {
    ($item:expr, $recurse:expr, $depth:expr) => {{
        'build: {
            let Some(raw_item) = (unsafe { ($item).as_ref() }) else {
                break 'build None;
            };
            let mut root = printable_cjson {
                type_0: raw_item.type_0,
                valuestring: if raw_item.valuestring.is_null() {
                    None
                } else {
                    Some(unsafe { ::std::ffi::CStr::from_ptr(raw_item.valuestring) })
                },
                valueint: raw_item.valueint,
                valuedouble: raw_item.valuedouble,
                string: if raw_item.string.is_null() {
                    None
                } else {
                    Some(unsafe { ::std::ffi::CStr::from_ptr(raw_item.string) })
                },
                children: Vec::new(),
            };
            let raw_type = raw_item.type_0 & 0xff as ::core::ffi::c_int;
            let mut stack: Vec<(
                Vec<usize>,
                *const crate::src::cJSON::cJSON,
                crate::__stddef_size_t_h::size_t,
            )> = Vec::new();
            if $recurse != 0
                && (raw_type == crate::src::cJSON::cJSON_Array
                    || raw_type == crate::src::cJSON::cJSON_Object)
                && !raw_item.child.is_null()
            {
                if stack.try_reserve_exact(1).is_err() {
                    break 'build None;
                }
                stack.push((Vec::new(), raw_item.child, $depth));
            }
            while let Some((parent_path, mut child, parent_depth)) = stack.pop() {
                if parent_depth
                    >= crate::src::cJSON::CJSON_CIRCULAR_LIMIT as crate::__stddef_size_t_h::size_t
                {
                    break 'build None;
                }
                while let Some(child_ref) = unsafe { child.as_ref() } {
                    let child_type = child_ref.type_0 & 0xff as ::core::ffi::c_int;
                    let duplicate_child = printable_cjson {
                        type_0: child_ref.type_0,
                        valuestring: if child_ref.valuestring.is_null() {
                            None
                        } else {
                            Some(unsafe { ::std::ffi::CStr::from_ptr(child_ref.valuestring) })
                        },
                        valueint: child_ref.valueint,
                        valuedouble: child_ref.valuedouble,
                        string: if child_ref.string.is_null() {
                            None
                        } else {
                            Some(unsafe { ::std::ffi::CStr::from_ptr(child_ref.string) })
                        },
                        children: Vec::new(),
                    };
                    let child_index = {
                        let Some(parent) = printable_child_at_path_mut(&mut root, &parent_path)
                        else {
                            break 'build None;
                        };
                        if parent.children.try_reserve_exact(1).is_err() {
                            break 'build None;
                        }
                        parent.children.push(duplicate_child);
                        parent.children.len().wrapping_sub(1)
                    };
                    if (child_type == crate::src::cJSON::cJSON_Array
                        || child_type == crate::src::cJSON::cJSON_Object)
                        && !child_ref.child.is_null()
                    {
                        let mut child_path = parent_path.clone();
                        if child_path.try_reserve_exact(1).is_err() {
                            break 'build None;
                        }
                        child_path.push(child_index);
                        if stack.try_reserve_exact(1).is_err() {
                            break 'build None;
                        }
                        stack.push((
                            child_path,
                            child_ref.child,
                            parent_depth.wrapping_add(1 as crate::__stddef_size_t_h::size_t),
                        ));
                    }
                    child = child_ref.next;
                }
            }
            Some(root)
        }
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
    buffer_length = ::std::ffi::CStr::from_ptr(value).to_bytes_with_nul().len();
    return cJSON_ParseWithLengthOpts_ffi(
        value,
        buffer_length,
        return_parse_end,
        require_null_terminated,
    );
}
fn cJSON_ParseWithLengthOpts(
    value: &[::core::ffi::c_uchar],
    item: &mut crate::src::cJSON::cJSON,
    hooks: internal_hooks,
    mut require_null_terminated: crate::src::cJSON::cJSON_bool,
) -> Result<crate::__stddef_size_t_h::size_t, crate::__stddef_size_t_h::size_t> {
    let mut c2rust_current_block: u64;
    let mut buffer: parse_buffer<'_> = parse_buffer {
        content: value,
        length: value.len() as crate::__stddef_size_t_h::size_t,
        offset: 0 as crate::__stddef_size_t_h::size_t,
        depth: 0 as crate::__stddef_size_t_h::size_t,
        hooks,
    };
    if !value.is_empty() {
        let valid_buffer =
            skip_utf8_bom(Some(&mut buffer)) && buffer_skip_whitespace(Some(&mut buffer));
        if valid_buffer && !(parse_value(item, &mut buffer) == 0) {
            if require_null_terminated != 0 {
                buffer_skip_whitespace(Some(&mut buffer));
                if buffer.offset >= buffer.length
                    || buffer.content.get(buffer.offset as usize) != Some(&b'\0')
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
                    return Ok(buffer.offset);
                }
            }
        }
    }
    let mut local_error_position = 0 as crate::__stddef_size_t_h::size_t;
    if buffer.offset < buffer.length {
        local_error_position = buffer.offset;
    } else if buffer.length > 0 as crate::__stddef_size_t_h::size_t {
        local_error_position = buffer
            .length
            .wrapping_sub(1 as crate::__stddef_size_t_h::size_t);
    }
    Err(local_error_position)
}
#[export_name = "cJSON_ParseWithLengthOpts"]

pub unsafe extern "C" fn cJSON_ParseWithLengthOpts_ffi(
    mut value: *const ::core::ffi::c_char,
    mut buffer_length: crate::__stddef_size_t_h::size_t,
    mut return_parse_end: *mut *const ::core::ffi::c_char,
    mut require_null_terminated: crate::src::cJSON::cJSON_bool,
) -> *mut crate::src::cJSON::cJSON {
    let input = if value.is_null() {
        None
    } else {
        Some(unsafe {
            ::core::slice::from_raw_parts(value as *const ::core::ffi::c_uchar, buffer_length)
        })
    };
    reset_global_error();
    let Some(input) = input else {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    };
    let hooks = global_hooks_snapshot();
    let mut item: *mut crate::src::cJSON::cJSON =
        ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    if !input.is_empty() {
        if let Some(item_ref) = cJSON_New_Item(&hooks) {
            item = item_ref as *mut crate::src::cJSON::cJSON;
            match cJSON_ParseWithLengthOpts(input, item_ref, hooks, require_null_terminated) {
                Ok(parse_end_offset) => {
                    if !return_parse_end.is_null() {
                        unsafe {
                            *return_parse_end = value.wrapping_add(parse_end_offset);
                        }
                    }
                    return item;
                }
                Err(parse_end_offset) => {
                    if !return_parse_end.is_null() {
                        unsafe {
                            *return_parse_end = value.wrapping_add(parse_end_offset);
                        }
                    }
                    set_global_error(error {
                        json: value as usize,
                        position: parse_end_offset,
                    });
                    unsafe {
                        cJSON_Delete_ffi(item);
                    }
                    return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
                }
            }
        }
    }
    if !return_parse_end.is_null() {
        unsafe {
            *return_parse_end = value;
        }
    }
    set_global_error(error {
        json: value as usize,
        position: 0 as crate::__stddef_size_t_h::size_t,
    });
    ::core::ptr::null_mut::<crate::src::cJSON::cJSON>()
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
fn print(
    item: Option<&printable_cjson<'_>>,
    format: crate::src::cJSON::cJSON_bool,
) -> Option<Vec<::core::ffi::c_uchar>> {
    let default_buffer_size: crate::__stddef_size_t_h::size_t =
        256 as crate::__stddef_size_t_h::size_t;
    let Some(mut buffer) = printbuffer::owned(default_buffer_size, format) else {
        return None;
    };
    let printed_value = match item {
        Some(item) => print_value(item, &mut buffer),
        None => false_0,
    };
    if printed_value != 0 {
        let output_length = buffer
            .offset
            .wrapping_add(1 as crate::__stddef_size_t_h::size_t);
        let Some(bytes) = buffer.bytes().get(..output_length as usize) else {
            return None;
        };
        let mut printed = Vec::new();
        if printed.try_reserve_exact(bytes.len()).is_err() {
            return None;
        }
        printed.extend_from_slice(bytes);
        return Some(printed);
    }
    return None;
}
#[export_name = "cJSON_Print"]

pub unsafe extern "C" fn cJSON_Print_ffi(
    mut item: *const crate::src::cJSON::cJSON,
) -> *mut ::core::ffi::c_char {
    let hooks = global_hooks_snapshot();
    let Some(printable_item) = printable_tree_from_raw!(item) else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    let Some(bytes) = print(Some(&printable_item), true_0) else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    let printed = unsafe {
        hooks.allocate.expect("non-null function pointer")(
            bytes.len() as crate::__stddef_size_t_h::size_t
        )
    } as *mut ::core::ffi::c_uchar;
    if printed.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    unsafe {
        ::core::ptr::copy_nonoverlapping(bytes.as_ptr(), printed, bytes.len());
    }
    return printed as *mut ::core::ffi::c_char;
}
#[export_name = "cJSON_PrintUnformatted"]

pub unsafe extern "C" fn cJSON_PrintUnformatted_ffi(
    mut item: *const crate::src::cJSON::cJSON,
) -> *mut ::core::ffi::c_char {
    let hooks = global_hooks_snapshot();
    let Some(printable_item) = printable_tree_from_raw!(item) else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    let Some(bytes) = print(Some(&printable_item), false_0) else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    let printed = unsafe {
        hooks.allocate.expect("non-null function pointer")(
            bytes.len() as crate::__stddef_size_t_h::size_t
        )
    } as *mut ::core::ffi::c_uchar;
    if printed.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    unsafe {
        ::core::ptr::copy_nonoverlapping(bytes.as_ptr(), printed, bytes.len());
    }
    return printed as *mut ::core::ffi::c_char;
}
#[export_name = "cJSON_PrintBuffered"]

pub unsafe extern "C" fn cJSON_PrintBuffered_ffi(
    mut item: *const crate::src::cJSON::cJSON,
    mut prebuffer: ::core::ffi::c_int,
    mut fmt: crate::src::cJSON::cJSON_bool,
) -> *mut ::core::ffi::c_char {
    if prebuffer < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let hooks = global_hooks_snapshot();
    let Some(mut p) = printbuffer::owned(prebuffer as crate::__stddef_size_t_h::size_t, fmt) else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    let Some(printable_item) = printable_tree_from_raw!(item) else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    let printed_value = print_value(&printable_item, &mut p);
    if printed_value == 0 {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let output_length = p.offset.wrapping_add(1 as crate::__stddef_size_t_h::size_t);
    let Some(bytes) = p.bytes().get(..output_length as usize) else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    let printed = hooks.allocate.expect("non-null function pointer")(
        bytes.len() as crate::__stddef_size_t_h::size_t
    ) as *mut ::core::ffi::c_uchar;
    if printed.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    ::core::ptr::copy_nonoverlapping(bytes.as_ptr(), printed, bytes.len());
    return printed as *mut ::core::ffi::c_char;
}
#[export_name = "cJSON_PrintPreallocated"]

pub unsafe extern "C" fn cJSON_PrintPreallocated_ffi(
    mut item: *mut crate::src::cJSON::cJSON,
    mut buffer: *mut ::core::ffi::c_char,
    length: ::core::ffi::c_int,
    format: crate::src::cJSON::cJSON_bool,
) -> crate::src::cJSON::cJSON_bool {
    if length < 0 as ::core::ffi::c_int || buffer.is_null() {
        return false_0;
    }
    let output = unsafe {
        ::core::slice::from_raw_parts_mut(buffer as *mut ::core::ffi::c_uchar, length as usize)
    };
    let mut p = printbuffer::borrowed(output, format);
    match printable_tree_from_raw!(item) {
        Some(printable_item) => print_value(&printable_item, &mut p),
        None => false_0,
    }
}
fn parse_value(
    item: &mut crate::src::cJSON::cJSON,
    input_buffer: &mut parse_buffer<'_>,
) -> crate::src::cJSON::cJSON_bool {
    let content = input_buffer.content;
    let Some(remaining) = content.get(input_buffer.offset as usize..) else {
        return false_0;
    };
    if remaining.starts_with(b"null") {
        item.type_0 = crate::src::cJSON::cJSON_NULL;
        input_buffer.offset = input_buffer
            .offset
            .wrapping_add(4 as crate::__stddef_size_t_h::size_t);
        return true_0;
    }
    if remaining.starts_with(b"false") {
        item.type_0 = crate::src::cJSON::cJSON_False;
        input_buffer.offset = input_buffer
            .offset
            .wrapping_add(5 as crate::__stddef_size_t_h::size_t);
        return true_0;
    }
    if remaining.starts_with(b"true") {
        item.type_0 = crate::src::cJSON::cJSON_True;
        item.valueint = 1 as ::core::ffi::c_int;
        input_buffer.offset = input_buffer
            .offset
            .wrapping_add(4 as crate::__stddef_size_t_h::size_t);
        return true_0;
    }

    let Some(first_byte) = remaining.first().copied() else {
        return false_0;
    };
    if first_byte as ::core::ffi::c_int == '"' as i32 {
        return parse_string(item, input_buffer, content);
    }
    if first_byte as ::core::ffi::c_int == '-' as i32
        || first_byte as ::core::ffi::c_int >= '0' as i32
            && first_byte as ::core::ffi::c_int <= '9' as i32
    {
        return parse_number(item, input_buffer, content);
    }
    if first_byte as ::core::ffi::c_int == '[' as i32 {
        return parse_array(item, input_buffer, content);
    }
    if first_byte as ::core::ffi::c_int == '{' as i32 {
        return parse_object(item, input_buffer, content);
    }
    return false_0;
}

fn print_value(
    item: &printable_cjson<'_>,
    output_buffer: &mut printbuffer<'_>,
) -> crate::src::cJSON::cJSON_bool {
    let bytes = match item.type_0 & 0xff as ::core::ffi::c_int {
        crate::src::cJSON::cJSON_NULL => b"null\0".as_slice(),
        crate::src::cJSON::cJSON_False => b"false\0".as_slice(),
        crate::src::cJSON::cJSON_True => b"true\0".as_slice(),
        crate::src::cJSON::cJSON_Number => {
            return print_number(item.valueint, item.valuedouble, Some(output_buffer));
        }
        crate::src::cJSON::cJSON_Raw | crate::src::cJSON::cJSON_String => {
            if item.type_0 & 0xff as ::core::ffi::c_int == crate::src::cJSON::cJSON_String {
                return print_string_ptr(item.valuestring, Some(output_buffer));
            }
            let Some(value_string) = item.valuestring else {
                return false_0;
            };
            value_string.to_bytes_with_nul()
        }
        crate::src::cJSON::cJSON_Array => return print_array(item, output_buffer),
        crate::src::cJSON::cJSON_Object => return print_object(item, output_buffer),
        _ => return false_0,
    };
    let Some(output) = ensure(
        Some(&mut *output_buffer),
        bytes.len() as crate::__stddef_size_t_h::size_t,
    ) else {
        return false_0;
    };
    output.copy_from_slice(bytes);
    output_buffer.offset = output_buffer
        .offset
        .wrapping_add(bytes.len().wrapping_sub(1) as crate::__stddef_size_t_h::size_t);
    return true_0;
}

fn parse_array(
    item: &mut crate::src::cJSON::cJSON,
    input_buffer: &mut parse_buffer<'_>,
    content: &[::core::ffi::c_uchar],
) -> crate::src::cJSON::cJSON_bool {
    let mut c2rust_current_block: u64;
    let mut parsed_children: Vec<&mut crate::src::cJSON::cJSON> = Vec::new();
    if input_buffer.depth
        >= crate::src::cJSON::CJSON_NESTING_LIMIT as crate::__stddef_size_t_h::size_t
    {
        return false_0;
    }
    input_buffer.depth = input_buffer.depth.wrapping_add(1);
    if content.get(input_buffer.offset as usize) == Some(&b'[') {
        input_buffer.offset = input_buffer.offset.wrapping_add(1);
        buffer_skip_whitespace(Some(&mut *input_buffer));
        if content.get(input_buffer.offset as usize) == Some(&b']') {
            c2rust_current_block = 6773356538935231690;
        } else if !(input_buffer
            .offset
            .wrapping_add(0 as crate::__stddef_size_t_h::size_t)
            < input_buffer.length)
        {
            input_buffer.offset = input_buffer.offset.wrapping_sub(1);
            c2rust_current_block = 1336238348363633231;
        } else {
            input_buffer.offset = input_buffer.offset.wrapping_sub(1);
            loop {
                let Some(new_item_ref) = cJSON_New_Item(&input_buffer.hooks) else {
                    c2rust_current_block = 1336238348363633231;
                    break;
                };
                input_buffer.offset = input_buffer.offset.wrapping_add(1);
                buffer_skip_whitespace(Some(&mut *input_buffer));
                let parsed_value = parse_value(new_item_ref, input_buffer);
                parsed_children.push(new_item_ref);
                if parsed_value == 0 {
                    c2rust_current_block = 1336238348363633231;
                    break;
                }
                buffer_skip_whitespace(Some(&mut *input_buffer));
                if content.get(input_buffer.offset as usize) != Some(&b',') {
                    c2rust_current_block = 15089075282327824602;
                    break;
                }
            }
            match c2rust_current_block {
                1336238348363633231 => {}
                _ => {
                    if content.get(input_buffer.offset as usize) != Some(&b']') {
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
                item.type_0 = crate::src::cJSON::cJSON_Array;
                assign_linked_children(item, &mut parsed_children);
                input_buffer.offset = input_buffer.offset.wrapping_add(1);
                return true_0;
            }
        }
    }
    assign_linked_children(item, &mut parsed_children);
    return false_0;
}

fn print_array(
    item: &printable_cjson<'_>,
    output_buffer: &mut printbuffer<'_>,
) -> crate::src::cJSON::cJSON_bool {
    let mut length: crate::__stddef_size_t_h::size_t = 0 as crate::__stddef_size_t_h::size_t;
    let Some(output) = ensure(
        Some(&mut *output_buffer),
        1 as crate::__stddef_size_t_h::size_t,
    ) else {
        return false_0;
    };
    output[0] = b'[';
    output_buffer.offset = output_buffer.offset.wrapping_add(1);
    output_buffer.depth = output_buffer.depth.wrapping_add(1);
    for (index, current_element) in item.children.iter().enumerate() {
        if print_value(current_element, output_buffer) == 0 {
            return false_0;
        }
        if index + 1 < item.children.len() {
            let formatted = output_buffer.format != 0;
            length = (if formatted {
                2 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as crate::__stddef_size_t_h::size_t;
            let Some(output) = ensure(
                Some(&mut *output_buffer),
                length.wrapping_add(1 as crate::__stddef_size_t_h::size_t),
            ) else {
                return false_0;
            };
            output[0] = b',';
            if formatted {
                output[1] = b' ';
            }
            output[length as usize] = b'\0';
            output_buffer.offset = output_buffer.offset.wrapping_add(length);
        }
    }
    let Some(output) = ensure(
        Some(&mut *output_buffer),
        2 as crate::__stddef_size_t_h::size_t,
    ) else {
        return false_0;
    };
    output[0] = b']';
    output[1] = b'\0';
    output_buffer.offset = output_buffer.offset.wrapping_add(1);
    output_buffer.depth = output_buffer.depth.wrapping_sub(1);
    return true_0;
}

fn parse_object(
    item: &mut crate::src::cJSON::cJSON,
    input_buffer: &mut parse_buffer<'_>,
    content: &[::core::ffi::c_uchar],
) -> crate::src::cJSON::cJSON_bool {
    let mut c2rust_current_block: u64;
    let mut parsed_children: Vec<&mut crate::src::cJSON::cJSON> = Vec::new();
    if input_buffer.depth
        >= crate::src::cJSON::CJSON_NESTING_LIMIT as crate::__stddef_size_t_h::size_t
    {
        return false_0;
    }
    input_buffer.depth = input_buffer.depth.wrapping_add(1);
    if content.get(input_buffer.offset as usize) == Some(&b'{') {
        input_buffer.offset = input_buffer.offset.wrapping_add(1);
        buffer_skip_whitespace(Some(&mut *input_buffer));
        if content.get(input_buffer.offset as usize) == Some(&b'}') {
            c2rust_current_block = 4359236900545362719;
        } else if !(input_buffer
            .offset
            .wrapping_add(0 as crate::__stddef_size_t_h::size_t)
            < input_buffer.length)
        {
            input_buffer.offset = input_buffer.offset.wrapping_sub(1);
            c2rust_current_block = 9990476168629568694;
        } else {
            input_buffer.offset = input_buffer.offset.wrapping_sub(1);
            loop {
                if let Some(current_item_ref) = cJSON_New_Item(&input_buffer.hooks) {
                    if !(input_buffer
                        .offset
                        .wrapping_add(1 as crate::__stddef_size_t_h::size_t)
                        < input_buffer.length)
                    {
                        parsed_children.push(current_item_ref);
                        c2rust_current_block = 9990476168629568694;
                        break;
                    } else {
                        input_buffer.offset = input_buffer.offset.wrapping_add(1);
                        buffer_skip_whitespace(Some(&mut *input_buffer));
                        if parse_string(current_item_ref, input_buffer, content) == 0 {
                            parsed_children.push(current_item_ref);
                            c2rust_current_block = 9990476168629568694;
                            break;
                        } else {
                            buffer_skip_whitespace(Some(&mut *input_buffer));
                            current_item_ref.string = current_item_ref.valuestring;
                            current_item_ref.valuestring =
                                ::core::ptr::null_mut::<::core::ffi::c_char>();
                            if content.get(input_buffer.offset as usize) != Some(&b':') {
                                parsed_children.push(current_item_ref);
                                c2rust_current_block = 9990476168629568694;
                                break;
                            } else {
                                input_buffer.offset = input_buffer.offset.wrapping_add(1);
                                buffer_skip_whitespace(Some(&mut *input_buffer));
                                if parse_value(current_item_ref, input_buffer) == 0 {
                                    parsed_children.push(current_item_ref);
                                    c2rust_current_block = 9990476168629568694;
                                    break;
                                } else {
                                    buffer_skip_whitespace(Some(&mut *input_buffer));
                                    parsed_children.push(current_item_ref);
                                    if content.get(input_buffer.offset as usize) != Some(&b',') {
                                        c2rust_current_block = 14359455889292382949;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    c2rust_current_block = 9990476168629568694;
                    break;
                }
            }
            match c2rust_current_block {
                9990476168629568694 => {}
                _ => {
                    if content.get(input_buffer.offset as usize) != Some(&b'}') {
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
                item.type_0 = crate::src::cJSON::cJSON_Object;
                assign_linked_children(item, &mut parsed_children);
                input_buffer.offset = input_buffer.offset.wrapping_add(1);
                return true_0;
            }
        }
    }
    assign_linked_children(item, &mut parsed_children);
    return false_0;
}

fn print_object(
    item: &printable_cjson<'_>,
    output_buffer: &mut printbuffer<'_>,
) -> crate::src::cJSON::cJSON_bool {
    let mut length: crate::__stddef_size_t_h::size_t = 0 as crate::__stddef_size_t_h::size_t;
    length = (if output_buffer.format != 0 {
        2 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    }) as crate::__stddef_size_t_h::size_t;
    {
        let formatted = output_buffer.format != 0;
        let Some(output) = ensure(
            Some(&mut *output_buffer),
            length.wrapping_add(1 as crate::__stddef_size_t_h::size_t),
        ) else {
            return false_0;
        };
        output[0] = b'{';
        if formatted {
            output[1] = b'\n';
        }
    }
    output_buffer.depth = output_buffer.depth.wrapping_add(1);
    output_buffer.offset = output_buffer.offset.wrapping_add(length);
    for (index, current_item) in item.children.iter().enumerate() {
        if output_buffer.format != 0 {
            let mut i: crate::__stddef_size_t_h::size_t = 0;
            let depth = output_buffer.depth;
            let Some(output) = ensure(Some(&mut *output_buffer), depth) else {
                return false_0;
            };
            i = 0 as crate::__stddef_size_t_h::size_t;
            while i < depth {
                output[i as usize] = b'\t';
                i = i.wrapping_add(1);
            }
            output_buffer.offset = output_buffer.offset.wrapping_add(depth);
        }
        if print_string_ptr(current_item.string, Some(&mut *output_buffer)) == 0 {
            return false_0;
        }
        let formatted = output_buffer.format != 0;
        length = (if formatted {
            2 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) as crate::__stddef_size_t_h::size_t;
        let Some(output) = ensure(Some(&mut *output_buffer), length) else {
            return false_0;
        };
        output[0] = b':';
        if formatted {
            output[1] = b'\t';
        }
        output_buffer.offset = output_buffer.offset.wrapping_add(length);
        if print_value(current_item, output_buffer) == 0 {
            return false_0;
        }
        let formatted = output_buffer.format != 0;
        let has_next = index + 1 < item.children.len();
        length = ((if formatted {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as crate::__stddef_size_t_h::size_t)
            .wrapping_add(
                (if has_next {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as crate::__stddef_size_t_h::size_t,
            );
        let Some(output) = ensure(
            Some(&mut *output_buffer),
            length.wrapping_add(1 as crate::__stddef_size_t_h::size_t),
        ) else {
            return false_0;
        };
        let mut output_index = 0usize;
        if has_next {
            output[output_index] = b',';
            output_index = output_index.wrapping_add(1);
        }
        if formatted {
            output[output_index] = b'\n';
            output_index = output_index.wrapping_add(1);
        }
        output[output_index] = b'\0';
        output_buffer.offset = output_buffer.offset.wrapping_add(length);
    }
    let formatted = output_buffer.format != 0;
    let closing_length = if formatted {
        output_buffer
            .depth
            .wrapping_add(1 as crate::__stddef_size_t_h::size_t)
    } else {
        2 as crate::__stddef_size_t_h::size_t
    };
    let closing_depth = output_buffer
        .depth
        .wrapping_sub(1 as crate::__stddef_size_t_h::size_t);
    let Some(output) = ensure(Some(&mut *output_buffer), closing_length) else {
        return false_0;
    };
    let mut output_index = 0usize;
    if formatted {
        let mut i_0: crate::__stddef_size_t_h::size_t = 0;
        i_0 = 0 as crate::__stddef_size_t_h::size_t;
        while i_0 < closing_depth {
            output[output_index] = b'\t';
            output_index = output_index.wrapping_add(1);
            i_0 = i_0.wrapping_add(1);
        }
    }
    output[output_index] = b'}';
    output_index = output_index.wrapping_add(1);
    output[output_index] = b'\0';
    output_buffer.offset = output_buffer
        .offset
        .wrapping_add(closing_length.wrapping_sub(1));
    output_buffer.depth = output_buffer.depth.wrapping_sub(1);
    return true_0;
}
#[export_name = "cJSON_GetArraySize"]

pub unsafe extern "C" fn cJSON_GetArraySize_ffi(
    mut array: *const crate::src::cJSON::cJSON,
) -> ::core::ffi::c_int {
    let mut child: *mut crate::src::cJSON::cJSON =
        ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    let mut size: crate::__stddef_size_t_h::size_t = 0 as crate::__stddef_size_t_h::size_t;
    if array.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    child = (*array).child as *mut crate::src::cJSON::cJSON;
    while !child.is_null() {
        size = size.wrapping_add(1);
        child = (*child).next as *mut crate::src::cJSON::cJSON;
    }
    return size as ::core::ffi::c_int;
}
#[export_name = "cJSON_GetArrayItem"]

pub unsafe extern "C" fn cJSON_GetArrayItem_ffi(
    mut array: *const crate::src::cJSON::cJSON,
    mut index: ::core::ffi::c_int,
) -> *mut crate::src::cJSON::cJSON {
    let mut current_child: *mut crate::src::cJSON::cJSON =
        ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    if index < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    }
    if array.is_null() {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    }
    current_child = (*array).child as *mut crate::src::cJSON::cJSON;
    let mut index = index as crate::__stddef_size_t_h::size_t;
    while !current_child.is_null() && index > 0 as crate::__stddef_size_t_h::size_t {
        index = index.wrapping_sub(1);
        current_child = (*current_child).next as *mut crate::src::cJSON::cJSON;
    }
    return current_child;
}
fn object_item_name_match(
    name: &::std::ffi::CStr,
    current_name: Option<&::std::ffi::CStr>,
    case_sensitive: crate::src::cJSON::cJSON_bool,
) -> Option<bool> {
    let Some(current_name) = current_name else {
        if case_sensitive != 0 {
            return None;
        }
        return Some(false);
    };
    if case_sensitive != 0 {
        Some(name == current_name)
    } else {
        Some(case_insensitive_strcmp(Some(name), Some(current_name)) == 0 as ::core::ffi::c_int)
    }
}

macro_rules! get_object_item_from_raw {
    ($object:expr, $name:expr, $case_sensitive:expr) => {{
        'lookup: {
            let Some(name) = $name else {
                break 'lookup ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
            };
            let Some(object) = (unsafe { ($object).as_ref() }) else {
                break 'lookup ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
            };
            let mut current_element = object.child;
            while let Some(element_ref) = unsafe { current_element.as_ref() } {
                let current_name = if element_ref.string.is_null() {
                    None
                } else {
                    Some(unsafe { ::std::ffi::CStr::from_ptr(element_ref.string) })
                };
                match object_item_name_match(name, current_name, $case_sensitive) {
                    Some(true) => {
                        break 'lookup element_ref as *const crate::src::cJSON::cJSON
                            as *mut crate::src::cJSON::cJSON;
                    }
                    Some(false) => {}
                    None => break,
                }
                current_element = element_ref.next;
            }
            ::core::ptr::null_mut::<crate::src::cJSON::cJSON>()
        }
    }};
}
#[export_name = "cJSON_GetObjectItem"]

pub unsafe extern "C" fn cJSON_GetObjectItem_ffi(
    object: *const crate::src::cJSON::cJSON,
    string: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    let name = if string.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(string) })
    };
    get_object_item_from_raw!(object, name, false_0)
}
#[export_name = "cJSON_GetObjectItemCaseSensitive"]

pub unsafe extern "C" fn cJSON_GetObjectItemCaseSensitive_ffi(
    object: *const crate::src::cJSON::cJSON,
    string: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    let name = if string.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(string) })
    };
    get_object_item_from_raw!(object, name, true_0)
}
#[export_name = "cJSON_HasObjectItem"]

pub unsafe extern "C" fn cJSON_HasObjectItem_ffi(
    mut object: *const crate::src::cJSON::cJSON,
    mut string: *const ::core::ffi::c_char,
) -> crate::src::cJSON::cJSON_bool {
    let name = if string.is_null() {
        None
    } else {
        Some(unsafe { ::std::ffi::CStr::from_ptr(string) })
    };
    return if !get_object_item_from_raw!(object, name, false_0).is_null() {
        1 as crate::src::cJSON::cJSON_bool
    } else {
        0 as crate::src::cJSON::cJSON_bool
    };
}
fn suffix_object(prev: &mut crate::src::cJSON::cJSON, item: &mut crate::src::cJSON::cJSON) {
    prev.next = item as *mut crate::src::cJSON::cJSON;
    item.prev = prev as *mut crate::src::cJSON::cJSON;
}

fn assign_linked_children(
    parent: &mut crate::src::cJSON::cJSON,
    children: &mut [&mut crate::src::cJSON::cJSON],
) {
    if children.is_empty() {
        parent.child = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
        return;
    }

    let mut child_ptrs: Vec<*mut crate::src::cJSON::cJSON> = Vec::with_capacity(children.len());
    for child in children.iter_mut() {
        child_ptrs.push(*child as *mut crate::src::cJSON::cJSON);
    }
    let last_index = child_ptrs.len() - 1;
    for (index, child) in children.iter_mut().enumerate() {
        child.prev = if index == 0 {
            child_ptrs[last_index]
        } else {
            child_ptrs[index - 1]
        };
        child.next = if index == last_index {
            ::core::ptr::null_mut::<crate::src::cJSON::cJSON>()
        } else {
            child_ptrs[index + 1]
        };
    }
    parent.child = child_ptrs[0];
}

macro_rules! add_item_to_array_raw {
    ($array:expr, $item:expr) => {{
        let array = $array;
        let item = $item;
        if array.is_null() || item.is_null() || array == item {
            false_0
        } else {
            let item_ptr = item as *mut crate::src::cJSON::cJSON;
            let child = (*array).child;
            if child.is_null() {
                (*array).child = item_ptr;
                (*item).prev = item_ptr;
                (*item).next = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
            } else if !(*child).prev.is_null() {
                (*(*child).prev).next = item_ptr;
                (*item).prev = (*child).prev;
                (*child).prev = item_ptr;
            }
            true_0
        }
    }};
}

macro_rules! add_item_to_object_raw {
    ($object:expr, $string:expr, $item:expr, $hooks:expr, $constant_key:expr) => {{
        let object = $object;
        let string = $string;
        let item = $item;
        if object.is_null() || string.is_null() || item.is_null() || object == item {
            false_0
        } else {
            'add: {
                let mut new_key: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                let mut new_type: ::core::ffi::c_int = crate::src::cJSON::cJSON_Invalid;
                if $constant_key != 0 {
                    new_key = string as *mut ::core::ffi::c_char;
                    new_type = (*item).type_0 | crate::src::cJSON::cJSON_StringIsConst;
                } else {
                    new_key = match cJSON_strdup(Some(::std::ffi::CStr::from_ptr(string))) {
                        Some(copy) => copy.into_raw(),
                        None => ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    };
                    if new_key.is_null() {
                        break 'add false_0;
                    }
                    new_type = (*item).type_0 & !crate::src::cJSON::cJSON_StringIsConst;
                }
                let old_key_to_free =
                    if (*item).type_0 & crate::src::cJSON::cJSON_StringIsConst == 0 {
                        (*item).string
                    } else {
                        ::core::ptr::null_mut::<::core::ffi::c_char>()
                    };
                (*item).string = new_key;
                (*item).type_0 = new_type;
                if !old_key_to_free.is_null() {
                    let old_key_address = old_key_to_free as usize;
                    if !unregister_owned_c_string(old_key_address) {
                        $hooks.deallocate.expect("non-null function pointer")(
                            old_key_to_free as *mut ::core::ffi::c_void,
                        );
                    }
                }
                break 'add add_item_to_array_raw!(object, item);
            }
        }
    }};
}
#[export_name = "cJSON_AddItemToArray"]

pub unsafe extern "C" fn cJSON_AddItemToArray_ffi(
    mut array: *mut crate::src::cJSON::cJSON,
    mut item: *mut crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    if item.is_null() || array.is_null() || array == item {
        return false_0;
    }
    return add_item_to_array_raw!(array, item);
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
    let hooks = global_hooks_snapshot();
    return add_item_to_object_raw!(object, string, item, &hooks, false_0);
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
    let hooks = global_hooks_snapshot();
    return add_item_to_object_raw!(object, string, item, &hooks, true_0);
}
#[export_name = "cJSON_AddItemReferenceToArray"]

pub unsafe extern "C" fn cJSON_AddItemReferenceToArray_ffi(
    mut array: *mut crate::src::cJSON::cJSON,
    mut item: *mut crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    if array.is_null() {
        return false_0;
    }
    let hooks = global_hooks_snapshot();
    if item.is_null() {
        return false_0;
    }
    let Some(reference) = cJSON_New_Item(&hooks) else {
        return false_0;
    };
    *reference = *item;
    reference.string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    reference.type_0 |= crate::src::cJSON::cJSON_IsReference;
    reference.prev = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    reference.next = reference.prev;
    let reference_ptr = reference as *mut crate::src::cJSON::cJSON;
    if array == reference_ptr {
        return false_0;
    }
    return add_item_to_array_raw!(array, reference_ptr);
}
#[export_name = "cJSON_AddItemReferenceToObject"]

pub unsafe extern "C" fn cJSON_AddItemReferenceToObject_ffi(
    mut object: *mut crate::src::cJSON::cJSON,
    mut string: *const ::core::ffi::c_char,
    mut item: *mut crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    if object.is_null() || string.is_null() {
        return false_0;
    }
    let hooks = global_hooks_snapshot();
    if item.is_null() {
        return false_0;
    }
    let Some(reference) = cJSON_New_Item(&hooks) else {
        return false_0;
    };
    *reference = *item;
    reference.string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    reference.type_0 |= crate::src::cJSON::cJSON_IsReference;
    reference.prev = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    reference.next = reference.prev;
    let reference_ptr = reference as *mut crate::src::cJSON::cJSON;
    if object == reference_ptr {
        return false_0;
    }
    return add_item_to_object_raw!(object, string, reference_ptr, &hooks, false_0);
}
#[export_name = "cJSON_AddNullToObject"]

pub unsafe extern "C" fn cJSON_AddNullToObject_ffi(
    object: *mut crate::src::cJSON::cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    let mut null: *mut crate::src::cJSON::cJSON = cJSON_CreateNull_ffi();
    let hooks = global_hooks_snapshot();
    if !object.is_null()
        && !name.is_null()
        && !null.is_null()
        && object != null
        && add_item_to_object_raw!(object, name, null, &hooks, false_0) != 0
    {
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
    let mut true_item: *mut crate::src::cJSON::cJSON = cJSON_CreateTrue_ffi();
    let hooks = global_hooks_snapshot();
    if !object.is_null()
        && !name.is_null()
        && !true_item.is_null()
        && object != true_item
        && add_item_to_object_raw!(object, name, true_item, &hooks, false_0) != 0
    {
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
    let mut false_item: *mut crate::src::cJSON::cJSON = cJSON_CreateFalse_ffi();
    let hooks = global_hooks_snapshot();
    if !object.is_null()
        && !name.is_null()
        && !false_item.is_null()
        && object != false_item
        && add_item_to_object_raw!(object, name, false_item, &hooks, false_0) != 0
    {
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
    let mut bool_item: *mut crate::src::cJSON::cJSON = cJSON_CreateBool_ffi(boolean);
    let hooks = global_hooks_snapshot();
    if !object.is_null()
        && !name.is_null()
        && !bool_item.is_null()
        && object != bool_item
        && add_item_to_object_raw!(object, name, bool_item, &hooks, false_0) != 0
    {
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
    let mut number_item: *mut crate::src::cJSON::cJSON = cJSON_CreateNumber_ffi(number);
    let hooks = global_hooks_snapshot();
    if !object.is_null()
        && !name.is_null()
        && !number_item.is_null()
        && object != number_item
        && add_item_to_object_raw!(object, name, number_item, &hooks, false_0) != 0
    {
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
    let mut string_item: *mut crate::src::cJSON::cJSON = cJSON_CreateString_ffi(string);
    let hooks = global_hooks_snapshot();
    if !object.is_null()
        && !name.is_null()
        && !string_item.is_null()
        && object != string_item
        && add_item_to_object_raw!(object, name, string_item, &hooks, false_0) != 0
    {
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
    let mut raw_item: *mut crate::src::cJSON::cJSON = cJSON_CreateRaw_ffi(raw);
    let hooks = global_hooks_snapshot();
    if !object.is_null()
        && !name.is_null()
        && !raw_item.is_null()
        && object != raw_item
        && add_item_to_object_raw!(object, name, raw_item, &hooks, false_0) != 0
    {
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
    let mut object_item: *mut crate::src::cJSON::cJSON = cJSON_CreateObject_ffi();
    let hooks = global_hooks_snapshot();
    if !object.is_null()
        && !name.is_null()
        && !object_item.is_null()
        && object != object_item
        && add_item_to_object_raw!(object, name, object_item, &hooks, false_0) != 0
    {
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
    let mut array: *mut crate::src::cJSON::cJSON = cJSON_CreateArray_ffi();
    let hooks = global_hooks_snapshot();
    if !object.is_null()
        && !name.is_null()
        && !array.is_null()
        && object != array
        && add_item_to_object_raw!(object, name, array, &hooks, false_0) != 0
    {
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
    let mut item: *mut crate::src::cJSON::cJSON =
        ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    if which < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    }
    if !array.is_null() {
        item = (*array).child as *mut crate::src::cJSON::cJSON;
        let mut index = which as crate::__stddef_size_t_h::size_t;
        while !item.is_null() && index > 0 as crate::__stddef_size_t_h::size_t {
            index = index.wrapping_sub(1);
            item = (*item).next as *mut crate::src::cJSON::cJSON;
        }
    }
    return cJSON_DetachItemViaPointer_ffi(array, item);
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
    let name = if string.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(string))
    };
    let to_detach: *mut crate::src::cJSON::cJSON = get_object_item_from_raw!(object, name, false_0);
    return cJSON_DetachItemViaPointer_ffi(object, to_detach);
}
#[export_name = "cJSON_DetachItemFromObjectCaseSensitive"]

pub unsafe extern "C" fn cJSON_DetachItemFromObjectCaseSensitive_ffi(
    mut object: *mut crate::src::cJSON::cJSON,
    mut string: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    let name = if string.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(string))
    };
    let to_detach: *mut crate::src::cJSON::cJSON = get_object_item_from_raw!(object, name, true_0);
    return cJSON_DetachItemViaPointer_ffi(object, to_detach);
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
    if !array.is_null() {
        after_inserted = (*array).child as *mut crate::src::cJSON::cJSON;
        let mut index = which as crate::__stddef_size_t_h::size_t;
        while !after_inserted.is_null() && index > 0 as crate::__stddef_size_t_h::size_t {
            index = index.wrapping_sub(1);
            after_inserted = (*after_inserted).next as *mut crate::src::cJSON::cJSON;
        }
    }
    if after_inserted.is_null() {
        if array.is_null() || array == newitem {
            return false_0;
        }
        return add_item_to_array_raw!(array, newitem);
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
    return true_0;
}
#[export_name = "cJSON_ReplaceItemInArray"]

pub unsafe extern "C" fn cJSON_ReplaceItemInArray_ffi(
    mut array: *mut crate::src::cJSON::cJSON,
    mut which: ::core::ffi::c_int,
    mut newitem: *mut crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    let mut item: *mut crate::src::cJSON::cJSON =
        ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    if which < 0 as ::core::ffi::c_int {
        return false_0;
    }
    if !array.is_null() {
        item = (*array).child as *mut crate::src::cJSON::cJSON;
        let mut index = which as crate::__stddef_size_t_h::size_t;
        while !item.is_null() && index > 0 as crate::__stddef_size_t_h::size_t {
            index = index.wrapping_sub(1);
            item = (*item).next as *mut crate::src::cJSON::cJSON;
        }
    }
    return cJSON_ReplaceItemViaPointer_ffi(array, item, newitem);
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
        let old_string_address = (*newitem).string as usize;
        if !unregister_owned_c_string(old_string_address) {
            global_hooks_snapshot()
                .deallocate
                .expect("non-null function pointer")(
                (*newitem).string as *mut ::core::ffi::c_void
            );
        }
    }
    (*newitem).string = match cJSON_strdup(Some(::std::ffi::CStr::from_ptr(string))) {
        Some(copy) => copy.into_raw(),
        None => ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    if (*newitem).string.is_null() {
        return false_0;
    }
    (*newitem).type_0 &= !crate::src::cJSON::cJSON_StringIsConst;
    return cJSON_ReplaceItemViaPointer_ffi(
        object,
        {
            let name = if string.is_null() {
                None
            } else {
                Some(::std::ffi::CStr::from_ptr(string))
            };
            get_object_item_from_raw!(object, name, false_0)
        },
        newitem,
    );
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
        let old_string_address = (*newitem).string as usize;
        if !unregister_owned_c_string(old_string_address) {
            global_hooks_snapshot()
                .deallocate
                .expect("non-null function pointer")(
                (*newitem).string as *mut ::core::ffi::c_void
            );
        }
    }
    (*newitem).string = match cJSON_strdup(Some(::std::ffi::CStr::from_ptr(string))) {
        Some(copy) => copy.into_raw(),
        None => ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    if (*newitem).string.is_null() {
        return false_0;
    }
    (*newitem).type_0 &= !crate::src::cJSON::cJSON_StringIsConst;
    return cJSON_ReplaceItemViaPointer_ffi(
        object,
        {
            let name = if string.is_null() {
                None
            } else {
                Some(::std::ffi::CStr::from_ptr(string))
            };
            get_object_item_from_raw!(object, name, true_0)
        },
        newitem,
    );
}
#[export_name = "cJSON_CreateNull"]

pub unsafe extern "C" fn cJSON_CreateNull_ffi() -> *mut crate::src::cJSON::cJSON {
    let hooks = global_hooks_snapshot();
    let Some(item) = cJSON_New_Item(&hooks) else {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    };
    item.type_0 = crate::src::cJSON::cJSON_NULL;
    return item as *mut crate::src::cJSON::cJSON;
}
#[export_name = "cJSON_CreateTrue"]

pub unsafe extern "C" fn cJSON_CreateTrue_ffi() -> *mut crate::src::cJSON::cJSON {
    let hooks = global_hooks_snapshot();
    let Some(item) = cJSON_New_Item(&hooks) else {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    };
    item.type_0 = crate::src::cJSON::cJSON_True;
    return item as *mut crate::src::cJSON::cJSON;
}
#[export_name = "cJSON_CreateFalse"]

pub unsafe extern "C" fn cJSON_CreateFalse_ffi() -> *mut crate::src::cJSON::cJSON {
    let hooks = global_hooks_snapshot();
    let Some(item) = cJSON_New_Item(&hooks) else {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    };
    item.type_0 = crate::src::cJSON::cJSON_False;
    return item as *mut crate::src::cJSON::cJSON;
}
#[export_name = "cJSON_CreateBool"]

pub unsafe extern "C" fn cJSON_CreateBool_ffi(
    mut boolean: crate::src::cJSON::cJSON_bool,
) -> *mut crate::src::cJSON::cJSON {
    let hooks = global_hooks_snapshot();
    let Some(item) = cJSON_New_Item(&hooks) else {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    };
    item.type_0 = if boolean != 0 {
        crate::src::cJSON::cJSON_True
    } else {
        crate::src::cJSON::cJSON_False
    };
    return item as *mut crate::src::cJSON::cJSON;
}
#[export_name = "cJSON_CreateNumber"]

pub unsafe extern "C" fn cJSON_CreateNumber_ffi(
    mut num: ::core::ffi::c_double,
) -> *mut crate::src::cJSON::cJSON {
    let hooks = global_hooks_snapshot();
    let Some(item) = cJSON_New_Item(&hooks) else {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    };
    item.type_0 = crate::src::cJSON::cJSON_Number;
    item.valuedouble = num;
    if num >= crate::limits_h::INT_MAX as ::core::ffi::c_double {
        item.valueint = crate::limits_h::INT_MAX;
    } else if num <= crate::limits_h::INT_MIN as ::core::ffi::c_double {
        item.valueint = crate::limits_h::INT_MIN;
    } else {
        item.valueint = num as ::core::ffi::c_int;
    }
    return item as *mut crate::src::cJSON::cJSON;
}
#[export_name = "cJSON_CreateString"]

pub unsafe extern "C" fn cJSON_CreateString_ffi(
    mut string: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    let hooks = global_hooks_snapshot();
    let Some(item) = cJSON_New_Item(&hooks) else {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    };
    item.type_0 = crate::src::cJSON::cJSON_String;
    item.valuestring = match cJSON_strdup(if string.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(string))
    }) {
        Some(copy) => copy.into_raw(),
        None => ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    if item.valuestring.is_null() {
        cJSON_Delete_ffi(item as *mut crate::src::cJSON::cJSON);
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    }
    return item as *mut crate::src::cJSON::cJSON;
}
#[export_name = "cJSON_CreateStringReference"]

pub unsafe extern "C" fn cJSON_CreateStringReference_ffi(
    mut string: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    let hooks = global_hooks_snapshot();
    let Some(item) = cJSON_New_Item(&hooks) else {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    };
    item.type_0 = crate::src::cJSON::cJSON_String | crate::src::cJSON::cJSON_IsReference;
    item.valuestring = string as *mut ::core::ffi::c_char;
    item as *mut crate::src::cJSON::cJSON
}
#[export_name = "cJSON_CreateObjectReference"]

pub unsafe extern "C" fn cJSON_CreateObjectReference_ffi(
    mut child: *const crate::src::cJSON::cJSON,
) -> *mut crate::src::cJSON::cJSON {
    let hooks = global_hooks_snapshot();
    let Some(item) = cJSON_New_Item(&hooks) else {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    };
    item.type_0 = crate::src::cJSON::cJSON_Object | crate::src::cJSON::cJSON_IsReference;
    item.child = child as *mut crate::src::cJSON::cJSON;
    item as *mut crate::src::cJSON::cJSON
}
#[export_name = "cJSON_CreateArrayReference"]

pub unsafe extern "C" fn cJSON_CreateArrayReference_ffi(
    mut child: *const crate::src::cJSON::cJSON,
) -> *mut crate::src::cJSON::cJSON {
    let hooks = global_hooks_snapshot();
    let Some(item) = cJSON_New_Item(&hooks) else {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    };
    item.type_0 = crate::src::cJSON::cJSON_Array | crate::src::cJSON::cJSON_IsReference;
    item.child = child as *mut crate::src::cJSON::cJSON;
    item as *mut crate::src::cJSON::cJSON
}
#[export_name = "cJSON_CreateRaw"]

pub unsafe extern "C" fn cJSON_CreateRaw_ffi(
    mut raw: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    let hooks = global_hooks_snapshot();
    let Some(item) = cJSON_New_Item(&hooks) else {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    };
    item.type_0 = crate::src::cJSON::cJSON_Raw;
    item.valuestring = match cJSON_strdup(if raw.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(raw))
    }) {
        Some(copy) => copy.into_raw(),
        None => ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    if item.valuestring.is_null() {
        cJSON_Delete_ffi(item as *mut crate::src::cJSON::cJSON);
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    }
    return item as *mut crate::src::cJSON::cJSON;
}
#[export_name = "cJSON_CreateArray"]

pub unsafe extern "C" fn cJSON_CreateArray_ffi() -> *mut crate::src::cJSON::cJSON {
    let hooks = global_hooks_snapshot();
    let Some(item) = cJSON_New_Item(&hooks) else {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    };
    item.type_0 = crate::src::cJSON::cJSON_Array;
    return item as *mut crate::src::cJSON::cJSON;
}
#[export_name = "cJSON_CreateObject"]

pub unsafe extern "C" fn cJSON_CreateObject_ffi() -> *mut crate::src::cJSON::cJSON {
    let hooks = global_hooks_snapshot();
    let Some(item) = cJSON_New_Item(&hooks) else {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    };
    item.type_0 = crate::src::cJSON::cJSON_Object;
    return item as *mut crate::src::cJSON::cJSON;
}
#[export_name = "cJSON_CreateIntArray"]

pub unsafe extern "C" fn cJSON_CreateIntArray_ffi(
    mut numbers: *const ::core::ffi::c_int,
    mut count: ::core::ffi::c_int,
) -> *mut crate::src::cJSON::cJSON {
    let mut i: crate::__stddef_size_t_h::size_t = 0 as crate::__stddef_size_t_h::size_t;
    let mut n: *mut crate::src::cJSON::cJSON = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    let mut p: *mut crate::src::cJSON::cJSON = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    let mut a: *mut crate::src::cJSON::cJSON = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    }
    a = cJSON_CreateArray_ffi();
    if a.is_null() {
        return a;
    }
    let array = &mut *a;
    i = 0 as crate::__stddef_size_t_h::size_t;
    while i < count as crate::__stddef_size_t_h::size_t {
        n = cJSON_CreateNumber_ffi(*numbers.offset(i as isize) as ::core::ffi::c_double);
        if n.is_null() {
            cJSON_Delete_ffi(a);
            return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
        }
        if i == 0 {
            array.child = n as *mut crate::src::cJSON::cJSON;
        } else {
            suffix_object(&mut *p, &mut *n);
        }
        p = n;
        i = i.wrapping_add(1);
    }
    if !array.child.is_null() {
        (*array.child).prev = n as *mut crate::src::cJSON::cJSON;
    }
    return a;
}
#[export_name = "cJSON_CreateFloatArray"]

pub unsafe extern "C" fn cJSON_CreateFloatArray_ffi(
    mut numbers: *const ::core::ffi::c_float,
    mut count: ::core::ffi::c_int,
) -> *mut crate::src::cJSON::cJSON {
    let mut i: crate::__stddef_size_t_h::size_t = 0 as crate::__stddef_size_t_h::size_t;
    let mut n: *mut crate::src::cJSON::cJSON = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    let mut p: *mut crate::src::cJSON::cJSON = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    let mut a: *mut crate::src::cJSON::cJSON = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    }
    a = cJSON_CreateArray_ffi();
    if a.is_null() {
        return a;
    }
    let array = &mut *a;
    i = 0 as crate::__stddef_size_t_h::size_t;
    while i < count as crate::__stddef_size_t_h::size_t {
        n = cJSON_CreateNumber_ffi(*numbers.offset(i as isize) as ::core::ffi::c_double);
        if n.is_null() {
            cJSON_Delete_ffi(a);
            return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
        }
        if i == 0 {
            array.child = n as *mut crate::src::cJSON::cJSON;
        } else {
            suffix_object(&mut *p, &mut *n);
        }
        p = n;
        i = i.wrapping_add(1);
    }
    if !array.child.is_null() {
        (*array.child).prev = n as *mut crate::src::cJSON::cJSON;
    }
    return a;
}
#[export_name = "cJSON_CreateDoubleArray"]

pub unsafe extern "C" fn cJSON_CreateDoubleArray_ffi(
    mut numbers: *const ::core::ffi::c_double,
    mut count: ::core::ffi::c_int,
) -> *mut crate::src::cJSON::cJSON {
    let mut i: crate::__stddef_size_t_h::size_t = 0 as crate::__stddef_size_t_h::size_t;
    let mut n: *mut crate::src::cJSON::cJSON = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    let mut p: *mut crate::src::cJSON::cJSON = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    let mut a: *mut crate::src::cJSON::cJSON = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    }
    a = cJSON_CreateArray_ffi();
    if a.is_null() {
        return a;
    }
    let array = &mut *a;
    i = 0 as crate::__stddef_size_t_h::size_t;
    while i < count as crate::__stddef_size_t_h::size_t {
        n = cJSON_CreateNumber_ffi(*numbers.offset(i as isize));
        if n.is_null() {
            cJSON_Delete_ffi(a);
            return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
        }
        if i == 0 {
            array.child = n as *mut crate::src::cJSON::cJSON;
        } else {
            suffix_object(&mut *p, &mut *n);
        }
        p = n;
        i = i.wrapping_add(1);
    }
    if !array.child.is_null() {
        (*array.child).prev = n as *mut crate::src::cJSON::cJSON;
    }
    return a;
}
#[export_name = "cJSON_CreateStringArray"]

pub unsafe extern "C" fn cJSON_CreateStringArray_ffi(
    mut strings: *const *const ::core::ffi::c_char,
    mut count: ::core::ffi::c_int,
) -> *mut crate::src::cJSON::cJSON {
    let mut i: crate::__stddef_size_t_h::size_t = 0 as crate::__stddef_size_t_h::size_t;
    let mut n: *mut crate::src::cJSON::cJSON = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    let mut p: *mut crate::src::cJSON::cJSON = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    let mut a: *mut crate::src::cJSON::cJSON = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    if count < 0 as ::core::ffi::c_int || strings.is_null() {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    }
    a = cJSON_CreateArray_ffi();
    if a.is_null() {
        return a;
    }
    let array = &mut *a;
    i = 0 as crate::__stddef_size_t_h::size_t;
    while i < count as crate::__stddef_size_t_h::size_t {
        n = cJSON_CreateString_ffi(*strings.offset(i as isize));
        if n.is_null() {
            cJSON_Delete_ffi(a);
            return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
        }
        if i == 0 {
            array.child = n as *mut crate::src::cJSON::cJSON;
        } else {
            suffix_object(&mut *p, &mut *n);
        }
        p = n;
        i = i.wrapping_add(1);
    }
    if !array.child.is_null() {
        (*array.child).prev = n as *mut crate::src::cJSON::cJSON;
    }
    return a;
}
#[export_name = "cJSON_Duplicate"]

pub unsafe extern "C" fn cJSON_Duplicate_ffi(
    mut item: *const crate::src::cJSON::cJSON,
    mut recurse: crate::src::cJSON::cJSON_bool,
) -> *mut crate::src::cJSON::cJSON {
    let Some(item) = duplicate_tree_from_raw!(item, recurse, 0 as crate::__stddef_size_t_h::size_t)
    else {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    };
    match cJSON_Duplicate_rec(&item, 0 as crate::__stddef_size_t_h::size_t, recurse) {
        DuplicateResult::Success(duplicate) => duplicate as *mut crate::src::cJSON::cJSON,
        DuplicateResult::Failure(partial) => {
            if let Some(partial) = partial {
                cJSON_Delete_ffi(partial as *mut crate::src::cJSON::cJSON);
            }
            ::core::ptr::null_mut::<crate::src::cJSON::cJSON>()
        }
    }
}

enum DuplicateResult {
    Success(&'static mut crate::src::cJSON::cJSON),
    Failure(Option<&'static mut crate::src::cJSON::cJSON>),
}

fn cJSON_Duplicate_rec(
    item: &printable_cjson<'_>,
    mut depth: crate::__stddef_size_t_h::size_t,
    mut recurse: crate::src::cJSON::cJSON_bool,
) -> DuplicateResult {
    let hooks = global_hooks_snapshot();

    let Some(newitem_ref) = cJSON_New_Item(&hooks) else {
        return DuplicateResult::Failure(None);
    };

    newitem_ref.type_0 = item.type_0 & !crate::src::cJSON::cJSON_IsReference;
    newitem_ref.valueint = item.valueint;
    newitem_ref.valuedouble = item.valuedouble;

    let mut duplicated_children: Vec<&mut crate::src::cJSON::cJSON> = Vec::new();
    let mut failed_child: Option<&mut crate::src::cJSON::cJSON> = None;
    let duplicate_ok = 'duplicate: {
        if let Some(value_string) = item.valuestring {
            newitem_ref.valuestring = match cJSON_strdup(Some(value_string)) {
                Some(copy) => copy.into_raw(),
                None => ::core::ptr::null_mut::<::core::ffi::c_char>(),
            };
            if newitem_ref.valuestring.is_null() {
                break 'duplicate false;
            }
        }

        if let Some(item_string) = item.string {
            newitem_ref.string = if item.type_0 & crate::src::cJSON::cJSON_StringIsConst != 0 {
                item_string.as_ptr() as *mut ::core::ffi::c_char
            } else {
                match cJSON_strdup(Some(item_string)) {
                    Some(copy) => copy.into_raw(),
                    None => ::core::ptr::null_mut::<::core::ffi::c_char>(),
                }
            };
            if newitem_ref.string.is_null() {
                break 'duplicate false;
            }
        }

        if recurse == 0 {
            break 'duplicate true;
        }

        for child_ref in &item.children {
            if depth >= crate::src::cJSON::CJSON_CIRCULAR_LIMIT as crate::__stddef_size_t_h::size_t
            {
                break 'duplicate false;
            }

            match cJSON_Duplicate_rec(
                child_ref,
                depth.wrapping_add(1 as crate::__stddef_size_t_h::size_t),
                true_0,
            ) {
                DuplicateResult::Success(newchild_ref) => duplicated_children.push(newchild_ref),
                DuplicateResult::Failure(partial_child) => {
                    failed_child = partial_child;
                    break 'duplicate false;
                }
            }
        }

        true
    };

    if let Some(failed_child) = failed_child {
        duplicated_children.push(failed_child);
    }
    assign_linked_children(newitem_ref, &mut duplicated_children);
    if duplicate_ok {
        return DuplicateResult::Success(newitem_ref);
    }
    return DuplicateResult::Failure(Some(newitem_ref));
}
#[export_name = "cJSON_Duplicate_rec"]

pub unsafe extern "C" fn cJSON_Duplicate_rec_ffi(
    mut item: *const crate::src::cJSON::cJSON,
    mut depth: crate::__stddef_size_t_h::size_t,
    mut recurse: crate::src::cJSON::cJSON_bool,
) -> *mut crate::src::cJSON::cJSON {
    let Some(item) = duplicate_tree_from_raw!(item, recurse, depth) else {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    };
    match cJSON_Duplicate_rec(&item, depth, recurse) {
        DuplicateResult::Success(duplicate) => duplicate as *mut crate::src::cJSON::cJSON,
        DuplicateResult::Failure(partial) => {
            if let Some(partial) = partial {
                cJSON_Delete_ffi(partial as *mut crate::src::cJSON::cJSON);
            }
            ::core::ptr::null_mut::<crate::src::cJSON::cJSON>()
        }
    }
}
fn skip_oneline_comment(input: &mut usize, buffer: &[::core::ffi::c_char]) {
    *input += 2;
    while *input < buffer.len() && buffer[*input] != b'\0' as ::core::ffi::c_char {
        if buffer[*input] == b'\n' as ::core::ffi::c_char {
            *input += 1;
            return;
        }
        *input += 1;
    }
}

fn skip_multiline_comment(input: &mut usize, buffer: &[::core::ffi::c_char]) {
    *input += 2;
    while *input < buffer.len() && buffer[*input] != b'\0' as ::core::ffi::c_char {
        if buffer[*input] == b'*' as ::core::ffi::c_char
            && buffer.get(*input + 1).copied() == Some(b'/' as ::core::ffi::c_char)
        {
            *input += 2;
            return;
        }
        *input += 1;
    }
}

fn minify_string(input: &mut usize, output: &mut usize, buffer: &mut [::core::ffi::c_char]) {
    buffer[*output] = buffer[*input];
    *input += 1;
    *output += 1;
    while *input < buffer.len() && buffer[*input] != b'\0' as ::core::ffi::c_char {
        let current = buffer[*input];
        buffer[*output] = current;
        if current == b'"' as ::core::ffi::c_char {
            *input += 1;
            *output += 1;
            return;
        } else if current == b'\\' as ::core::ffi::c_char
            && buffer.get(*input + 1).copied() == Some(b'"' as ::core::ffi::c_char)
        {
            buffer[*output + 1] = buffer[*input + 1];
            *input += 1;
            *output += 1;
        }
        *input += 1;
        *output += 1;
    }
}

pub fn cJSON_Minify(json: &mut [::core::ffi::c_char]) {
    let mut input: usize = 0;
    let mut output: usize = 0;

    while input < json.len() && json[input] != b'\0' as ::core::ffi::c_char {
        let current = json[input];
        if current == b' ' as ::core::ffi::c_char
            || current == b'\t' as ::core::ffi::c_char
            || current == b'\r' as ::core::ffi::c_char
            || current == b'\n' as ::core::ffi::c_char
        {
            input += 1;
        } else if current == b'/' as ::core::ffi::c_char {
            if json.get(input + 1).copied() == Some(b'/' as ::core::ffi::c_char) {
                skip_oneline_comment(&mut input, json);
            } else if json.get(input + 1).copied() == Some(b'*' as ::core::ffi::c_char) {
                skip_multiline_comment(&mut input, json);
            } else {
                input += 1;
            }
        } else if current == b'"' as ::core::ffi::c_char {
            minify_string(&mut input, &mut output, json);
        } else {
            json[output] = current;
            input += 1;
            output += 1;
        }
    }
    if output < json.len() {
        json[output] = b'\0' as ::core::ffi::c_char;
    }
}
#[export_name = "cJSON_Minify"]

pub unsafe extern "C" fn cJSON_Minify_ffi(mut json: *mut ::core::ffi::c_char) {
    if json.is_null() {
        return;
    }
    let length = unsafe { ::std::ffi::CStr::from_ptr(json).to_bytes_with_nul().len() };
    let buffer = unsafe { ::core::slice::from_raw_parts_mut(json, length) };
    cJSON_Minify(buffer)
}
fn cjson_has_type(
    item: Option<&crate::src::cJSON::cJSON>,
    type_0: ::core::ffi::c_int,
) -> crate::src::cJSON::cJSON_bool {
    return item
        .map(|item| (item.type_0 & 0xff as ::core::ffi::c_int == type_0) as ::core::ffi::c_int)
        .unwrap_or(false_0);
}

pub fn cJSON_IsInvalid(item: Option<&crate::src::cJSON::cJSON>) -> crate::src::cJSON::cJSON_bool {
    return cjson_has_type(item, crate::src::cJSON::cJSON_Invalid);
}
#[export_name = "cJSON_IsInvalid"]

pub unsafe extern "C" fn cJSON_IsInvalid_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    cJSON_IsInvalid(unsafe { item.as_ref() })
}
pub fn cJSON_IsFalse(item: Option<&crate::src::cJSON::cJSON>) -> crate::src::cJSON::cJSON_bool {
    return cjson_has_type(item, crate::src::cJSON::cJSON_False);
}
#[export_name = "cJSON_IsFalse"]

pub unsafe extern "C" fn cJSON_IsFalse_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    cJSON_IsFalse(unsafe { item.as_ref() })
}
pub fn cJSON_IsTrue(item: Option<&crate::src::cJSON::cJSON>) -> crate::src::cJSON::cJSON_bool {
    return cjson_has_type(item, crate::src::cJSON::cJSON_True);
}
#[export_name = "cJSON_IsTrue"]

pub unsafe extern "C" fn cJSON_IsTrue_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    cJSON_IsTrue(unsafe { item.as_ref() })
}
pub fn cJSON_IsBool(item: Option<&crate::src::cJSON::cJSON>) -> crate::src::cJSON::cJSON_bool {
    return item
        .map(|item| {
            (item.type_0 & (crate::src::cJSON::cJSON_True | crate::src::cJSON::cJSON_False)
                != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        })
        .unwrap_or(false_0);
}
#[export_name = "cJSON_IsBool"]

pub unsafe extern "C" fn cJSON_IsBool_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    cJSON_IsBool(unsafe { item.as_ref() })
}
pub fn cJSON_IsNull(item: Option<&crate::src::cJSON::cJSON>) -> crate::src::cJSON::cJSON_bool {
    return cjson_has_type(item, crate::src::cJSON::cJSON_NULL);
}
#[export_name = "cJSON_IsNull"]

pub unsafe extern "C" fn cJSON_IsNull_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    cJSON_IsNull(unsafe { item.as_ref() })
}
pub fn cJSON_IsNumber(item: Option<&crate::src::cJSON::cJSON>) -> crate::src::cJSON::cJSON_bool {
    return cjson_has_type(item, crate::src::cJSON::cJSON_Number);
}
#[export_name = "cJSON_IsNumber"]

pub unsafe extern "C" fn cJSON_IsNumber_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    cJSON_IsNumber(unsafe { item.as_ref() })
}
pub fn cJSON_IsString(item: Option<&crate::src::cJSON::cJSON>) -> crate::src::cJSON::cJSON_bool {
    return cjson_has_type(item, crate::src::cJSON::cJSON_String);
}
#[export_name = "cJSON_IsString"]

pub unsafe extern "C" fn cJSON_IsString_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    cJSON_IsString(unsafe { item.as_ref() })
}
pub fn cJSON_IsArray(item: Option<&crate::src::cJSON::cJSON>) -> crate::src::cJSON::cJSON_bool {
    return cjson_has_type(item, crate::src::cJSON::cJSON_Array);
}
#[export_name = "cJSON_IsArray"]

pub unsafe extern "C" fn cJSON_IsArray_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    cJSON_IsArray(unsafe { item.as_ref() })
}
pub fn cJSON_IsObject(item: Option<&crate::src::cJSON::cJSON>) -> crate::src::cJSON::cJSON_bool {
    return cjson_has_type(item, crate::src::cJSON::cJSON_Object);
}
#[export_name = "cJSON_IsObject"]

pub unsafe extern "C" fn cJSON_IsObject_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    cJSON_IsObject(unsafe { item.as_ref() })
}
pub fn cJSON_IsRaw(item: Option<&crate::src::cJSON::cJSON>) -> crate::src::cJSON::cJSON_bool {
    return cjson_has_type(item, crate::src::cJSON::cJSON_Raw);
}
#[export_name = "cJSON_IsRaw"]

pub unsafe extern "C" fn cJSON_IsRaw_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    cJSON_IsRaw(unsafe { item.as_ref() })
}
fn get_printable_object_item<'a, 'c>(
    object: &'a printable_cjson<'c>,
    name: Option<&::std::ffi::CStr>,
    case_sensitive: crate::src::cJSON::cJSON_bool,
) -> Option<&'a printable_cjson<'c>> {
    let name = name?;
    for element in &object.children {
        match object_item_name_match(name, element.string, case_sensitive) {
            Some(true) => return Some(element),
            Some(false) => {}
            None => break,
        }
    }
    None
}

fn cJSON_Compare(
    a: Option<&printable_cjson<'_>>,
    b: Option<&printable_cjson<'_>>,
    case_sensitive: crate::src::cJSON::cJSON_bool,
) -> crate::src::cJSON::cJSON_bool {
    let (Some(a), Some(b)) = (a, b) else {
        return false_0;
    };
    if a.type_0 & 0xff as ::core::ffi::c_int != b.type_0 & 0xff as ::core::ffi::c_int {
        return false_0;
    }
    match a.type_0 & 0xff as ::core::ffi::c_int {
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
    if ::core::ptr::eq(a, b) {
        return true_0;
    }
    match a.type_0 & 0xff as ::core::ffi::c_int {
        crate::src::cJSON::cJSON_False
        | crate::src::cJSON::cJSON_True
        | crate::src::cJSON::cJSON_NULL => return true_0,
        crate::src::cJSON::cJSON_Number => {
            if compare_double(a.valuedouble, b.valuedouble) != 0 {
                return true_0;
            }
            return false_0;
        }
        crate::src::cJSON::cJSON_String | crate::src::cJSON::cJSON_Raw => {
            let (Some(a_string), Some(b_string)) = (a.valuestring, b.valuestring) else {
                return false_0;
            };
            if a_string == b_string {
                return true_0;
            }
            return false_0;
        }
        crate::src::cJSON::cJSON_Array => {
            let mut a_element = a.children.iter();
            let mut b_element = b.children.iter();
            loop {
                match (a_element.next(), b_element.next()) {
                    (Some(a_ref), Some(b_ref)) => {
                        if cJSON_Compare(Some(a_ref), Some(b_ref), case_sensitive) == 0 {
                            return false_0;
                        }
                    }
                    (None, None) => return true_0,
                    _ => return false_0,
                }
            }
        }
        crate::src::cJSON::cJSON_Object => {
            for a_ref in &a.children {
                let b_element_0 = get_printable_object_item(b, a_ref.string, case_sensitive);
                let Some(b_ptr) = b_element_0 else {
                    return false_0;
                };
                let b_ref = b_ptr;
                if cJSON_Compare(Some(a_ref), Some(b_ref), case_sensitive) == 0 {
                    return false_0;
                }
            }
            for b_ref in &b.children {
                let a_element_1 = get_printable_object_item(a, b_ref.string, case_sensitive);
                let Some(a_ref) = a_element_1 else {
                    return false_0;
                };
                if cJSON_Compare(Some(b_ref), Some(a_ref), case_sensitive) == 0 {
                    return false_0;
                }
            }
            return true_0;
        }
        _ => return false_0,
    };
}
#[export_name = "cJSON_Compare"]

pub unsafe extern "C" fn cJSON_Compare_ffi(
    a: *const crate::src::cJSON::cJSON,
    b: *const crate::src::cJSON::cJSON,
    case_sensitive: crate::src::cJSON::cJSON_bool,
) -> crate::src::cJSON::cJSON_bool {
    let (Some(a_ref), Some(b_ref)) = (unsafe { a.as_ref() }, unsafe { b.as_ref() }) else {
        return false_0;
    };
    if a_ref.type_0 & 0xff as ::core::ffi::c_int != b_ref.type_0 & 0xff as ::core::ffi::c_int {
        return false_0;
    }
    match a_ref.type_0 & 0xff as ::core::ffi::c_int {
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
    let Some(a_item) = printable_tree_from_raw!(a) else {
        return false_0;
    };
    let Some(b_item) = printable_tree_from_raw!(b) else {
        return false_0;
    };
    cJSON_Compare(Some(&a_item), Some(&b_item), case_sensitive)
}
#[export_name = "cJSON_malloc"]

pub unsafe extern "C" fn cJSON_malloc_ffi(
    mut size: crate::__stddef_size_t_h::size_t,
) -> *mut ::core::ffi::c_void {
    return global_hooks_snapshot()
        .allocate
        .expect("non-null function pointer")(size);
}
#[export_name = "cJSON_free"]

pub unsafe extern "C" fn cJSON_free_ffi(mut object: *mut ::core::ffi::c_void) {
    global_hooks_snapshot()
        .deallocate
        .expect("non-null function pointer")(object);
}
