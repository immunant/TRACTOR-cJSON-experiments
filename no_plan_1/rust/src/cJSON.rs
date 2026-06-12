extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sscanf(
        __s: *const ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn strtod(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn localeconv() -> *mut lconv;
}
pub type size_t = usize;
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
pub struct parse_buffer {
    pub content: *const ::core::ffi::c_uchar,
    pub length: size_t,
    pub offset: size_t,
    pub depth: size_t,
    pub hooks: internal_hooks,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct printbuffer {
    pub buffer: *mut ::core::ffi::c_uchar,
    pub length: size_t,
    pub offset: size_t,
    pub depth: size_t,
    pub noalloc: cJSON_bool,
    pub format: cJSON_bool,
    pub hooks: internal_hooks,
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
static GLOBAL_ERROR_JSON_ADDRESS: ::core::sync::atomic::AtomicUsize =
    ::core::sync::atomic::AtomicUsize::new(0 as size_t);
static GLOBAL_ERROR_POSITION: ::core::sync::atomic::AtomicUsize =
    ::core::sync::atomic::AtomicUsize::new(0 as size_t);

fn clear_global_error() {
    GLOBAL_ERROR_JSON_ADDRESS.store(0 as size_t, ::core::sync::atomic::Ordering::Relaxed);
    GLOBAL_ERROR_POSITION.store(0 as size_t, ::core::sync::atomic::Ordering::Relaxed);
}

fn set_global_error(json_address: size_t, position: size_t) {
    GLOBAL_ERROR_POSITION.store(position, ::core::sync::atomic::Ordering::Relaxed);
    GLOBAL_ERROR_JSON_ADDRESS.store(json_address, ::core::sync::atomic::Ordering::Relaxed);
}

fn cJSON_GetErrorPtr() -> Option<(size_t, size_t)> {
    let json_address = GLOBAL_ERROR_JSON_ADDRESS.load(::core::sync::atomic::Ordering::Relaxed);
    if json_address == 0 as size_t {
        return None;
    }
    let position = GLOBAL_ERROR_POSITION.load(::core::sync::atomic::Ordering::Relaxed);
    Some((json_address, position))
}
#[export_name = "cJSON_GetErrorPtr"]
pub unsafe extern "C" fn cJSON_GetErrorPtr_ffi() -> *const ::core::ffi::c_char {
    match cJSON_GetErrorPtr() {
        Some((json_address, position)) => (json_address as *const ::core::ffi::c_uchar)
            .wrapping_add(position)
            as *const ::core::ffi::c_char,
        None => ::core::ptr::null::<::core::ffi::c_char>(),
    }
}
pub fn cJSON_GetStringValue(
    item: Option<&cJSON>,
) -> Option<::core::ptr::NonNull<::core::ffi::c_char>> {
    if cJSON_IsString(item) == 0 {
        return None;
    }
    ::core::ptr::NonNull::new(item.unwrap().valuestring)
}
#[export_name = "cJSON_GetStringValue"]
pub unsafe extern "C" fn cJSON_GetStringValue_ffi(item: *const cJSON) -> *mut ::core::ffi::c_char {
    match cJSON_GetStringValue(unsafe { item.as_ref() }) {
        Some(value) => value.as_ptr(),
        None => ::core::ptr::null_mut(),
    }
}
pub fn cJSON_GetNumberValue(item: Option<&cJSON>) -> ::core::ffi::c_double {
    if cJSON_IsNumber(item) == 0 {
        return 0.0f64 / 0.0f64;
    }
    return item.unwrap().valuedouble;
}
#[export_name = "cJSON_GetNumberValue"]
pub unsafe extern "C" fn cJSON_GetNumberValue_ffi(item: *const cJSON) -> ::core::ffi::c_double {
    cJSON_GetNumberValue(unsafe { item.as_ref() })
}
pub fn cJSON_Version() -> &'static ::core::ffi::CStr {
    c"1.7.19"
}
#[export_name = "cJSON_Version"]
pub unsafe extern "C" fn cJSON_Version_ffi() -> *const ::core::ffi::c_char {
    cJSON_Version().as_ptr()
}
fn compare_case_insensitive_bytes(
    string1: &[::core::ffi::c_uchar],
    string2: &[::core::ffi::c_uchar],
) -> ::core::ffi::c_int {
    let mut index = 0usize;
    loop {
        let char1 = string1
            .get(index)
            .copied()
            .unwrap_or(0)
            .to_ascii_lowercase();
        let char2 = string2
            .get(index)
            .copied()
            .unwrap_or(0)
            .to_ascii_lowercase();
        if char1 != char2 {
            return char1 as ::core::ffi::c_int - char2 as ::core::ffi::c_int;
        }
        if char1 == 0 {
            return 0 as ::core::ffi::c_int;
        }
        index += 1;
    }
}
static global_hooks: ::std::sync::Mutex<internal_hooks> = ::std::sync::Mutex::new(internal_hooks {
    allocate: Some(malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void),
    deallocate: Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    reallocate: Some(
        realloc
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void,
    ),
});

fn global_hooks_guard() -> ::std::sync::MutexGuard<'static, internal_hooks> {
    match global_hooks.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

fn current_global_hooks() -> internal_hooks {
    *global_hooks_guard()
}

fn set_current_global_hooks(hooks: internal_hooks) {
    *global_hooks_guard() = hooks;
}
unsafe extern "C" fn cJSON_strdup(
    mut string: *const ::core::ffi::c_uchar,
    hooks: *const internal_hooks,
) -> *mut ::core::ffi::c_uchar {
    let mut length: size_t = 0 as size_t;
    let mut copy: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if string.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    length = strlen(string as *const ::core::ffi::c_char)
        .wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 1]>() as size_t);
    copy =
        (*hooks).allocate.expect("non-null function pointer")(length) as *mut ::core::ffi::c_uchar;
    if copy.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    memcpy(
        copy as *mut ::core::ffi::c_void,
        string as *const ::core::ffi::c_void,
        length,
    );
    return copy;
}
pub fn cJSON_InitHooks(hooks: Option<&cJSON_Hooks>) {
    let Some(hooks) = hooks else {
        set_current_global_hooks(internal_hooks {
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
    if hooks.malloc_fn.is_some() {
        new_hooks.allocate = hooks.malloc_fn;
    }
    if hooks.free_fn.is_some() {
        new_hooks.deallocate = hooks.free_fn;
    }
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
        )
            as Option<
                unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void,
            >;
    }
    set_current_global_hooks(new_hooks);
}
#[export_name = "cJSON_InitHooks"]
pub unsafe extern "C" fn cJSON_InitHooks_ffi(mut hooks: *mut cJSON_Hooks) {
    cJSON_InitHooks(unsafe { hooks.as_ref() })
}
fn cJSON_New_Item(
    hooks: &internal_hooks,
    item_type: ::core::ffi::c_int,
    valueint: ::core::ffi::c_int,
    valuedouble: ::core::ffi::c_double,
    valuestring: Option<&::core::ffi::CStr>,
    child: Option<&cJSON>,
) -> *mut cJSON {
    let mut node: *mut cJSON = unsafe {
        hooks.allocate.expect("non-null function pointer")(::core::mem::size_of::<cJSON>() as size_t)
    } as *mut cJSON;
    if !node.is_null() {
        let child = match child {
            Some(child) => child as *const cJSON as *mut cJSON,
            None => ::core::ptr::null_mut::<cJSON>(),
        };
        let valuestring = match valuestring {
            Some(valuestring) => valuestring.as_ptr() as *mut ::core::ffi::c_char,
            None => ::core::ptr::null_mut::<::core::ffi::c_char>(),
        };
        let initialized = cJSON {
            next: ::core::ptr::null_mut::<cJSON>(),
            prev: ::core::ptr::null_mut::<cJSON>(),
            child,
            type_0: item_type,
            valuestring,
            valueint,
            valuedouble,
            string: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        };
        unsafe {
            ::core::ptr::copy_nonoverlapping(&initialized as *const cJSON, node, 1);
        }
    }
    return node;
}
pub unsafe extern "C" fn cJSON_Delete(mut item: *mut cJSON) {
    let mut next: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let hooks = current_global_hooks();
    while !item.is_null() {
        next = (*item).next as *mut cJSON;
        if (*item).type_0 & cJSON_IsReference == 0 && !(*item).child.is_null() {
            cJSON_Delete((*item).child as *mut cJSON);
        }
        if (*item).type_0 & cJSON_IsReference == 0 && !(*item).valuestring.is_null() {
            hooks.deallocate.expect("non-null function pointer")(
                (*item).valuestring as *mut ::core::ffi::c_void,
            );
            (*item).valuestring = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if (*item).type_0 & cJSON_StringIsConst == 0 && !(*item).string.is_null() {
            hooks.deallocate.expect("non-null function pointer")(
                (*item).string as *mut ::core::ffi::c_void,
            );
            (*item).string = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        hooks.deallocate.expect("non-null function pointer")(item as *mut ::core::ffi::c_void);
        item = next;
    }
}
#[export_name = "cJSON_Delete"]
pub unsafe extern "C" fn cJSON_Delete_ffi(mut item: *mut cJSON) {
    cJSON_Delete(item)
}
unsafe extern "C" fn get_decimal_point() -> ::core::ffi::c_uchar {
    let mut lconv: *mut lconv = localeconv();
    return *(*lconv)
        .decimal_point
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar;
}
unsafe extern "C" fn parse_number(item: *mut cJSON, input_buffer: *mut parse_buffer) -> cJSON_bool {
    let mut number: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut after_end: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut number_c_string: *mut ::core::ffi::c_uchar =
        ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut decimal_point: ::core::ffi::c_uchar = get_decimal_point();
    let mut i: size_t = 0 as size_t;
    let mut number_string_length: size_t = 0 as size_t;
    let mut has_decimal_point: cJSON_bool = false_0;
    if input_buffer.is_null() || (*input_buffer).content.is_null() {
        return false_0;
    }
    i = 0 as size_t;
    while !input_buffer.is_null() && (*input_buffer).offset.wrapping_add(i) < (*input_buffer).length
    {
        match *(*input_buffer)
            .content
            .offset((*input_buffer).offset as isize)
            .offset(i as isize) as ::core::ffi::c_int
        {
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 43 | 45 | 101 | 69 => {
                number_string_length = number_string_length.wrapping_add(1);
            }
            46 => {
                number_string_length = number_string_length.wrapping_add(1);
                has_decimal_point = true_0;
            }
            _ => {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    number_c_string = (*input_buffer)
        .hooks
        .allocate
        .expect("non-null function pointer")(
        number_string_length.wrapping_add(1 as size_t)
    ) as *mut ::core::ffi::c_uchar;
    if number_c_string.is_null() {
        return false_0;
    }
    memcpy(
        number_c_string as *mut ::core::ffi::c_void,
        (*input_buffer)
            .content
            .offset((*input_buffer).offset as isize) as *const ::core::ffi::c_void,
        number_string_length,
    );
    *number_c_string.offset(number_string_length as isize) = '\0' as i32 as ::core::ffi::c_uchar;
    if has_decimal_point != 0 {
        i = 0 as size_t;
        while i < number_string_length {
            if *number_c_string.offset(i as isize) as ::core::ffi::c_int == '.' as i32 {
                *number_c_string.offset(i as isize) = decimal_point;
            }
            i = i.wrapping_add(1);
        }
    }
    number = strtod(
        number_c_string as *const ::core::ffi::c_char,
        &raw mut after_end as *mut *mut ::core::ffi::c_char,
    );
    if number_c_string == after_end {
        (*input_buffer)
            .hooks
            .deallocate
            .expect("non-null function pointer")(
            number_c_string as *mut ::core::ffi::c_void
        );
        return false_0;
    }
    (*item).valuedouble = number;
    if number >= INT_MAX as ::core::ffi::c_double {
        (*item).valueint = INT_MAX;
    } else if number <= INT_MIN as ::core::ffi::c_double {
        (*item).valueint = INT_MIN;
    } else {
        (*item).valueint = number as ::core::ffi::c_int;
    }
    (*item).type_0 = cJSON_Number;
    (*input_buffer).offset = (*input_buffer)
        .offset
        .wrapping_add(after_end.offset_from(number_c_string) as ::core::ffi::c_long as size_t);
    (*input_buffer)
        .hooks
        .deallocate
        .expect("non-null function pointer")(number_c_string as *mut ::core::ffi::c_void);
    return true_0;
}
pub fn cJSON_SetNumberHelper(
    object: Option<&mut cJSON>,
    mut number: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if let Some(object) = object {
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
    return number;
}
#[export_name = "cJSON_SetNumberHelper"]
pub unsafe extern "C" fn cJSON_SetNumberHelper_ffi(
    mut object: *mut cJSON,
    mut number: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    cJSON_SetNumberHelper(unsafe { object.as_mut() }, number)
}
pub unsafe extern "C" fn cJSON_SetValuestring(
    mut object: *mut cJSON,
    mut valuestring: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut copy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut v1_len: size_t = 0;
    let mut v2_len: size_t = 0;
    if object.is_null()
        || (*object).type_0 & cJSON_String == 0
        || (*object).type_0 & cJSON_IsReference != 0
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if (*object).valuestring.is_null() || valuestring.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    v1_len = strlen(valuestring);
    v2_len = strlen((*object).valuestring);
    if v1_len <= v2_len {
        if !(valuestring.offset(v1_len as isize)
            < (*object).valuestring as *const ::core::ffi::c_char
            || (*object).valuestring.offset(v2_len as isize)
                < valuestring as *mut ::core::ffi::c_char)
        {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        strcpy((*object).valuestring, valuestring);
        return (*object).valuestring;
    }
    let hooks = current_global_hooks();
    copy = cJSON_strdup(valuestring as *const ::core::ffi::c_uchar, &hooks)
        as *mut ::core::ffi::c_char;
    if copy.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !(*object).valuestring.is_null() {
        hooks.deallocate.expect("non-null function pointer")(
            (*object).valuestring as *mut ::core::ffi::c_void,
        );
    }
    (*object).valuestring = copy;
    return copy;
}
#[export_name = "cJSON_SetValuestring"]
pub unsafe extern "C" fn cJSON_SetValuestring_ffi(
    mut object: *mut cJSON,
    mut valuestring: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    cJSON_SetValuestring(object, valuestring)
}
unsafe extern "C" fn ensure(p: *mut printbuffer, mut needed: size_t) -> *mut ::core::ffi::c_uchar {
    let mut newbuffer: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut newsize: size_t = 0 as size_t;
    if p.is_null() || (*p).buffer.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    if (*p).length > 0 as size_t && (*p).offset >= (*p).length {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    if needed > INT_MAX as size_t {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    needed = needed.wrapping_add((*p).offset.wrapping_add(1 as size_t));
    if needed <= (*p).length {
        return (*p).buffer.offset((*p).offset as isize);
    }
    if (*p).noalloc != 0 {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    if needed > (INT_MAX / 2 as ::core::ffi::c_int) as size_t {
        if needed <= INT_MAX as size_t {
            newsize = INT_MAX as size_t;
        } else {
            return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        }
    } else {
        newsize = needed.wrapping_mul(2 as size_t);
    }
    if (*p).hooks.reallocate.is_some() {
        newbuffer = (*p).hooks.reallocate.expect("non-null function pointer")(
            (*p).buffer as *mut ::core::ffi::c_void,
            newsize,
        ) as *mut ::core::ffi::c_uchar;
        if newbuffer.is_null() {
            (*p).hooks.deallocate.expect("non-null function pointer")(
                (*p).buffer as *mut ::core::ffi::c_void,
            );
            (*p).length = 0 as size_t;
            (*p).buffer = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
            return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        }
    } else {
        newbuffer = (*p).hooks.allocate.expect("non-null function pointer")(newsize)
            as *mut ::core::ffi::c_uchar;
        if newbuffer.is_null() {
            (*p).hooks.deallocate.expect("non-null function pointer")(
                (*p).buffer as *mut ::core::ffi::c_void,
            );
            (*p).length = 0 as size_t;
            (*p).buffer = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
            return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        }
        memcpy(
            newbuffer as *mut ::core::ffi::c_void,
            (*p).buffer as *const ::core::ffi::c_void,
            (*p).offset.wrapping_add(1 as size_t),
        );
        (*p).hooks.deallocate.expect("non-null function pointer")(
            (*p).buffer as *mut ::core::ffi::c_void,
        );
    }
    (*p).length = newsize;
    (*p).buffer = newbuffer;
    return newbuffer.offset((*p).offset as isize);
}
unsafe extern "C" fn update_offset(buffer: *mut printbuffer) {
    let mut buffer_pointer: *const ::core::ffi::c_uchar =
        ::core::ptr::null::<::core::ffi::c_uchar>();
    if buffer.is_null() || (*buffer).buffer.is_null() {
        return;
    }
    buffer_pointer = (*buffer).buffer.offset((*buffer).offset as isize);
    (*buffer).offset = (*buffer)
        .offset
        .wrapping_add(strlen(buffer_pointer as *const ::core::ffi::c_char));
}
fn compare_double(a: ::core::ffi::c_double, b: ::core::ffi::c_double) -> cJSON_bool {
    let maxVal: ::core::ffi::c_double = if a.abs() > b.abs() { a.abs() } else { b.abs() };
    return ((a - b).abs() <= maxVal * DBL_EPSILON) as ::core::ffi::c_int;
}
unsafe extern "C" fn print_number(
    item: *const cJSON,
    output_buffer: *mut printbuffer,
) -> cJSON_bool {
    let mut output_pointer: *mut ::core::ffi::c_uchar =
        ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut d: ::core::ffi::c_double = (*item).valuedouble;
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
    let mut decimal_point: ::core::ffi::c_uchar = get_decimal_point();
    let mut test: ::core::ffi::c_double = 0.0f64;
    if output_buffer.is_null() {
        return false_0;
    }
    if d != d || d - d != d - d && !(d != d) {
        length = sprintf(
            &raw mut number_buffer as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
            b"null\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else if d == (*item).valueint as ::core::ffi::c_double {
        length = sprintf(
            &raw mut number_buffer as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
            b"%d\0".as_ptr() as *const ::core::ffi::c_char,
            (*item).valueint,
        );
    } else {
        length = sprintf(
            &raw mut number_buffer as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
            b"%1.15g\0".as_ptr() as *const ::core::ffi::c_char,
            d,
        );
        if sscanf(
            &raw mut number_buffer as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
            b"%lg\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut test,
        ) != 1 as ::core::ffi::c_int
            || compare_double(test, d) == 0
        {
            length = sprintf(
                &raw mut number_buffer as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
                b"%1.17g\0".as_ptr() as *const ::core::ffi::c_char,
                d,
            );
        }
    }
    if length < 0 as ::core::ffi::c_int
        || length
            > (::core::mem::size_of::<[::core::ffi::c_uchar; 26]>() as usize)
                .wrapping_sub(1 as usize) as ::core::ffi::c_int
    {
        return false_0;
    }
    output_pointer = ensure(
        output_buffer,
        (length as size_t)
            .wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 1]>() as size_t),
    );
    if output_pointer.is_null() {
        return false_0;
    }
    i = 0 as size_t;
    while i < length as size_t {
        if number_buffer[i as usize] as ::core::ffi::c_int == decimal_point as ::core::ffi::c_int {
            *output_pointer.offset(i as isize) = '.' as i32 as ::core::ffi::c_uchar;
        } else {
            *output_pointer.offset(i as isize) = number_buffer[i as usize];
        }
        i = i.wrapping_add(1);
    }
    *output_pointer.offset(i as isize) = '\0' as i32 as ::core::ffi::c_uchar;
    (*output_buffer).offset = (*output_buffer).offset.wrapping_add(length as size_t);
    return true_0;
}
fn parse_hex4(input: &[::core::ffi::c_uchar]) -> ::core::ffi::c_uint {
    let mut h: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let Some(input) = input.get(..4) else {
        return 0 as ::core::ffi::c_uint;
    };
    for (i, byte) in input.iter().copied().enumerate() {
        let digit = match byte {
            b'0'..=b'9' => {
                (byte as ::core::ffi::c_uint).wrapping_sub('0' as i32 as ::core::ffi::c_uint)
            }
            b'A'..=b'F' => (10 as ::core::ffi::c_uint)
                .wrapping_add(byte as ::core::ffi::c_uint)
                .wrapping_sub('A' as i32 as ::core::ffi::c_uint),
            b'a'..=b'f' => (10 as ::core::ffi::c_uint)
                .wrapping_add(byte as ::core::ffi::c_uint)
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
fn decode_utf16_literal_to_utf8(
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
    if input.len() >= 6 {
        first_code = parse_hex4(&input[2..]);
        if !(first_code >= 0xdc00 as ::core::ffi::c_uint
            && first_code <= 0xdfff as ::core::ffi::c_uint)
        {
            if first_code >= 0xd800 as ::core::ffi::c_uint
                && first_code <= 0xdbff as ::core::ffi::c_uint
            {
                let second_sequence = &input[6..];
                let mut second_code: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                sequence_length = 12 as ::core::ffi::c_uchar;
                if second_sequence.len() < 6 {
                    c2rust_current_block = 2136517548508416331;
                } else if second_sequence[0] as ::core::ffi::c_int != '\\' as i32
                    || second_sequence[1] as ::core::ffi::c_int != 'u' as i32
                {
                    c2rust_current_block = 2136517548508416331;
                } else {
                    second_code = parse_hex4(&second_sequence[2..]);
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
                                output[0] = ((codepoint | first_byte_mark as ::core::ffi::c_ulong)
                                    & 0xff as ::core::ffi::c_ulong)
                                    as ::core::ffi::c_uchar;
                            } else {
                                output[0] = (codepoint & 0x7f as ::core::ffi::c_ulong)
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
unsafe extern "C" fn parse_string(item: *mut cJSON, input_buffer: *mut parse_buffer) -> cJSON_bool {
    let input_buffer = &mut *input_buffer;
    let mut c2rust_current_block: u64;
    let input_start = input_buffer.content.offset(input_buffer.offset as isize);
    let mut input_pointer: *const ::core::ffi::c_uchar =
        input_start.offset(1 as ::core::ffi::c_int as isize);
    let mut input_end: *const ::core::ffi::c_uchar =
        input_start.offset(1 as ::core::ffi::c_int as isize);
    let mut output_pointer: *mut ::core::ffi::c_uchar =
        ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut output: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if !(*input_start as ::core::ffi::c_int != '"' as i32) {
        let mut allocation_length: size_t = 0 as size_t;
        let mut skipped_bytes: size_t = 0 as size_t;
        loop {
            if !((input_end.offset_from(input_buffer.content) as ::core::ffi::c_long as size_t)
                < input_buffer.length
                && *input_end as ::core::ffi::c_int != '"' as i32)
            {
                c2rust_current_block = 11812396948646013369;
                break;
            }
            if *input_end as ::core::ffi::c_int == '\\' as i32 {
                if input_end
                    .offset(1 as ::core::ffi::c_int as isize)
                    .offset_from(input_buffer.content) as ::core::ffi::c_long
                    as size_t
                    >= input_buffer.length
                {
                    c2rust_current_block = 4600858903266057594;
                    break;
                }
                skipped_bytes = skipped_bytes.wrapping_add(1);
                input_end = input_end.offset(1);
            }
            input_end = input_end.offset(1);
        }
        match c2rust_current_block {
            4600858903266057594 => {}
            _ => {
                if !(input_end.offset_from(input_buffer.content) as ::core::ffi::c_long as size_t
                    >= input_buffer.length
                    || *input_end as ::core::ffi::c_int != '"' as i32)
                {
                    allocation_length = (input_end.offset_from(input_start) as ::core::ffi::c_long
                        as size_t)
                        .wrapping_sub(skipped_bytes);
                    output = input_buffer
                        .hooks
                        .allocate
                        .expect("non-null function pointer")(
                        allocation_length.wrapping_add(::core::mem::size_of::<
                            [::core::ffi::c_char; 1],
                        >() as size_t),
                    ) as *mut ::core::ffi::c_uchar;
                    if !output.is_null() {
                        output_pointer = output;
                        loop {
                            if !(input_pointer < input_end) {
                                c2rust_current_block = 7828949454673616476;
                                break;
                            }
                            if *input_pointer as ::core::ffi::c_int != '\\' as i32 {
                                let c2rust_fresh0 = input_pointer;
                                input_pointer = input_pointer.offset(1);
                                let c2rust_fresh1 = output_pointer;
                                output_pointer = output_pointer.offset(1);
                                *c2rust_fresh1 = *c2rust_fresh0;
                            } else {
                                let mut sequence_length: ::core::ffi::c_uchar =
                                    2 as ::core::ffi::c_uchar;
                                if (input_end.offset_from(input_pointer) as ::core::ffi::c_long)
                                    < 1 as ::core::ffi::c_long
                                {
                                    c2rust_current_block = 4600858903266057594;
                                    break;
                                }
                                match *input_pointer.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                {
                                    98 => {
                                        let c2rust_fresh2 = output_pointer;
                                        output_pointer = output_pointer.offset(1);
                                        *c2rust_fresh2 = '\u{8}' as i32 as ::core::ffi::c_uchar;
                                    }
                                    102 => {
                                        let c2rust_fresh3 = output_pointer;
                                        output_pointer = output_pointer.offset(1);
                                        *c2rust_fresh3 = '\u{c}' as i32 as ::core::ffi::c_uchar;
                                    }
                                    110 => {
                                        let c2rust_fresh4 = output_pointer;
                                        output_pointer = output_pointer.offset(1);
                                        *c2rust_fresh4 = '\n' as i32 as ::core::ffi::c_uchar;
                                    }
                                    114 => {
                                        let c2rust_fresh5 = output_pointer;
                                        output_pointer = output_pointer.offset(1);
                                        *c2rust_fresh5 = '\r' as i32 as ::core::ffi::c_uchar;
                                    }
                                    116 => {
                                        let c2rust_fresh6 = output_pointer;
                                        output_pointer = output_pointer.offset(1);
                                        *c2rust_fresh6 = '\t' as i32 as ::core::ffi::c_uchar;
                                    }
                                    34 | 92 | 47 => {
                                        let c2rust_fresh7 = output_pointer;
                                        output_pointer = output_pointer.offset(1);
                                        *c2rust_fresh7 =
                                            *input_pointer.offset(1 as ::core::ffi::c_int as isize);
                                    }
                                    117 => {
                                        let unicode_escape = ::core::slice::from_raw_parts(
                                            input_pointer,
                                            input_end.offset_from(input_pointer) as usize,
                                        );
                                        match decode_utf16_literal_to_utf8(unicode_escape) {
                                            Some((
                                                decoded_sequence_length,
                                                utf8_bytes,
                                                utf8_length,
                                            )) => {
                                                sequence_length = decoded_sequence_length;
                                                for byte in utf8_bytes[..utf8_length as usize]
                                                    .iter()
                                                    .copied()
                                                {
                                                    let output = output_pointer;
                                                    output_pointer = output_pointer.offset(1);
                                                    *output = byte;
                                                }
                                            }
                                            None => {
                                                c2rust_current_block = 4600858903266057594;
                                                break;
                                            }
                                        }
                                    }
                                    _ => {
                                        c2rust_current_block = 4600858903266057594;
                                        break;
                                    }
                                }
                                input_pointer = input_pointer
                                    .offset(sequence_length as ::core::ffi::c_int as isize);
                            }
                        }
                        match c2rust_current_block {
                            4600858903266057594 => {}
                            _ => {
                                *output_pointer = '\0' as i32 as ::core::ffi::c_uchar;
                                (*item).type_0 = cJSON_String;
                                (*item).valuestring = output as *mut ::core::ffi::c_char;
                                input_buffer.offset = input_end.offset_from(input_buffer.content)
                                    as ::core::ffi::c_long
                                    as size_t;
                                input_buffer.offset = input_buffer.offset.wrapping_add(1);
                                return true_0;
                            }
                        }
                    }
                }
            }
        }
    }
    if !output.is_null() {
        input_buffer
            .hooks
            .deallocate
            .expect("non-null function pointer")(output as *mut ::core::ffi::c_void);
        output = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    if !input_pointer.is_null() {
        input_buffer.offset =
            input_pointer.offset_from(input_buffer.content) as ::core::ffi::c_long as size_t;
    }
    return false_0;
}
unsafe extern "C" fn print_string_ptr(
    input: *const ::core::ffi::c_uchar,
    output_buffer: *mut printbuffer,
) -> cJSON_bool {
    let mut input_pointer: *const ::core::ffi::c_uchar =
        ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut output: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut output_pointer: *mut ::core::ffi::c_uchar =
        ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut output_length: size_t = 0 as size_t;
    let mut escape_characters: size_t = 0 as size_t;
    if output_buffer.is_null() {
        return false_0;
    }
    if input.is_null() {
        output = ensure(
            output_buffer,
            ::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t,
        );
        if output.is_null() {
            return false_0;
        }
        strcpy(
            output as *mut ::core::ffi::c_char,
            b"\"\"\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return true_0;
    }
    input_pointer = input;
    while *input_pointer != 0 {
        match *input_pointer as ::core::ffi::c_int {
            34 | 92 | 8 | 12 | 10 | 13 | 9 => {
                escape_characters = escape_characters.wrapping_add(1);
            }
            _ => {
                if (*input_pointer as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
                    escape_characters = escape_characters.wrapping_add(5 as size_t);
                }
            }
        }
        input_pointer = input_pointer.offset(1);
    }
    output_length = (input_pointer.offset_from(input) as ::core::ffi::c_long as size_t)
        .wrapping_add(escape_characters);
    output = ensure(
        output_buffer,
        output_length.wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t),
    );
    if output.is_null() {
        return false_0;
    }
    if escape_characters == 0 as size_t {
        *output.offset(0 as ::core::ffi::c_int as isize) = '"' as i32 as ::core::ffi::c_uchar;
        memcpy(
            output.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            input as *const ::core::ffi::c_void,
            output_length,
        );
        *output.offset(output_length.wrapping_add(1 as size_t) as isize) =
            '"' as i32 as ::core::ffi::c_uchar;
        *output.offset(output_length.wrapping_add(2 as size_t) as isize) =
            '\0' as i32 as ::core::ffi::c_uchar;
        return true_0;
    }
    *output.offset(0 as ::core::ffi::c_int as isize) = '"' as i32 as ::core::ffi::c_uchar;
    output_pointer = output.offset(1 as ::core::ffi::c_int as isize);
    input_pointer = input;
    while *input_pointer as ::core::ffi::c_int != '\0' as i32 {
        if *input_pointer as ::core::ffi::c_int > 31 as ::core::ffi::c_int
            && *input_pointer as ::core::ffi::c_int != '"' as i32
            && *input_pointer as ::core::ffi::c_int != '\\' as i32
        {
            *output_pointer = *input_pointer;
        } else {
            let c2rust_fresh21 = output_pointer;
            output_pointer = output_pointer.offset(1);
            *c2rust_fresh21 = '\\' as i32 as ::core::ffi::c_uchar;
            match *input_pointer as ::core::ffi::c_int {
                92 => {
                    *output_pointer = '\\' as i32 as ::core::ffi::c_uchar;
                }
                34 => {
                    *output_pointer = '"' as i32 as ::core::ffi::c_uchar;
                }
                8 => {
                    *output_pointer = 'b' as i32 as ::core::ffi::c_uchar;
                }
                12 => {
                    *output_pointer = 'f' as i32 as ::core::ffi::c_uchar;
                }
                10 => {
                    *output_pointer = 'n' as i32 as ::core::ffi::c_uchar;
                }
                13 => {
                    *output_pointer = 'r' as i32 as ::core::ffi::c_uchar;
                }
                9 => {
                    *output_pointer = 't' as i32 as ::core::ffi::c_uchar;
                }
                _ => {
                    sprintf(
                        output_pointer as *mut ::core::ffi::c_char,
                        b"u%04x\0".as_ptr() as *const ::core::ffi::c_char,
                        *input_pointer as ::core::ffi::c_int,
                    );
                    output_pointer = output_pointer.offset(4 as ::core::ffi::c_int as isize);
                }
            }
        }
        input_pointer = input_pointer.offset(1);
        output_pointer = output_pointer.offset(1);
    }
    *output.offset(output_length.wrapping_add(1 as size_t) as isize) =
        '"' as i32 as ::core::ffi::c_uchar;
    *output.offset(output_length.wrapping_add(2 as size_t) as isize) =
        '\0' as i32 as ::core::ffi::c_uchar;
    return true_0;
}
unsafe extern "C" fn print_string(item: *const cJSON, p: *mut printbuffer) -> cJSON_bool {
    return print_string_ptr((*item).valuestring as *mut ::core::ffi::c_uchar, p);
}
unsafe extern "C" fn buffer_skip_whitespace(buffer: *mut parse_buffer) -> *mut parse_buffer {
    if buffer.is_null() || (*buffer).content.is_null() {
        return ::core::ptr::null_mut::<parse_buffer>();
    }
    if !(!buffer.is_null() && (*buffer).offset.wrapping_add(0 as size_t) < (*buffer).length) {
        return buffer;
    }
    while !buffer.is_null()
        && (*buffer).offset.wrapping_add(0 as size_t) < (*buffer).length
        && *(*buffer)
            .content
            .offset((*buffer).offset as isize)
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            <= 32 as ::core::ffi::c_int
    {
        (*buffer).offset = (*buffer).offset.wrapping_add(1);
    }
    if (*buffer).offset == (*buffer).length {
        (*buffer).offset = (*buffer).offset.wrapping_sub(1);
    }
    return buffer;
}
unsafe extern "C" fn skip_utf8_bom(buffer: *mut parse_buffer) -> *mut parse_buffer {
    if buffer.is_null() || (*buffer).content.is_null() || (*buffer).offset != 0 as size_t {
        return ::core::ptr::null_mut::<parse_buffer>();
    }
    if !buffer.is_null()
        && (*buffer).offset.wrapping_add(4 as size_t) < (*buffer).length
        && strncmp(
            (*buffer).content.offset((*buffer).offset as isize) as *const ::core::ffi::c_char,
            b"\xEF\xBB\xBF\0".as_ptr() as *const ::core::ffi::c_char,
            3 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        (*buffer).offset = (*buffer).offset.wrapping_add(3 as size_t);
    }
    return buffer;
}
pub unsafe extern "C" fn cJSON_ParseWithOpts(
    mut value: *const ::core::ffi::c_char,
    mut return_parse_end: *mut *const ::core::ffi::c_char,
    mut require_null_terminated: cJSON_bool,
) -> *mut cJSON {
    let mut buffer_length: size_t = 0;
    if value.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    buffer_length =
        strlen(value).wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 1]>() as size_t);
    return cJSON_ParseWithLengthOpts(
        value,
        buffer_length,
        return_parse_end,
        require_null_terminated,
    );
}
#[export_name = "cJSON_ParseWithOpts"]
pub unsafe extern "C" fn cJSON_ParseWithOpts_ffi(
    mut value: *const ::core::ffi::c_char,
    mut return_parse_end: *mut *const ::core::ffi::c_char,
    mut require_null_terminated: cJSON_bool,
) -> *mut cJSON {
    cJSON_ParseWithOpts(value, return_parse_end, require_null_terminated)
}
pub unsafe extern "C" fn cJSON_ParseWithLengthOpts(
    mut value: *const ::core::ffi::c_char,
    mut buffer_length: size_t,
    mut return_parse_end: *mut *const ::core::ffi::c_char,
    mut require_null_terminated: cJSON_bool,
) -> *mut cJSON {
    let mut c2rust_current_block: u64;
    let mut buffer: parse_buffer = parse_buffer {
        content: ::core::ptr::null::<::core::ffi::c_uchar>(),
        length: 0 as size_t,
        offset: 0 as size_t,
        depth: 0 as size_t,
        hooks: internal_hooks {
            allocate: None,
            deallocate: None,
            reallocate: None,
        },
    };
    let mut item: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    clear_global_error();
    if !(value.is_null() || 0 as size_t == buffer_length) {
        let hooks = current_global_hooks();
        buffer.content = value as *const ::core::ffi::c_uchar;
        buffer.length = buffer_length;
        buffer.offset = 0 as size_t;
        buffer.hooks = hooks;
        item = cJSON_New_Item(
            &hooks,
            cJSON_Invalid,
            0 as ::core::ffi::c_int,
            0.0f64,
            None,
            None,
        );
        if !item.is_null() {
            if !(parse_value(item, buffer_skip_whitespace(skip_utf8_bom(&raw mut buffer))) == 0) {
                if require_null_terminated != 0 {
                    buffer_skip_whitespace(&raw mut buffer);
                    if buffer.offset >= buffer.length
                        || *buffer
                            .content
                            .offset(buffer.offset as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
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
                        if !return_parse_end.is_null() {
                            *return_parse_end = buffer.content.offset(buffer.offset as isize)
                                as *const ::core::ffi::c_char;
                        }
                        return item;
                    }
                }
            }
        }
    }
    if !item.is_null() {
        cJSON_Delete(item);
    }
    if !value.is_null() {
        let local_error_json = value as *mut ::core::ffi::c_uchar;
        let mut local_error_position = 0 as size_t;
        if buffer.offset < buffer.length {
            local_error_position = buffer.offset;
        } else if buffer.length > 0 as size_t {
            local_error_position = buffer.length.wrapping_sub(1 as size_t);
        }
        if !return_parse_end.is_null() {
            *return_parse_end = (local_error_json as *const ::core::ffi::c_char)
                .offset(local_error_position as isize);
        }
        set_global_error(local_error_json as size_t, local_error_position);
    }
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_ParseWithLengthOpts"]
pub unsafe extern "C" fn cJSON_ParseWithLengthOpts_ffi(
    mut value: *const ::core::ffi::c_char,
    mut buffer_length: size_t,
    mut return_parse_end: *mut *const ::core::ffi::c_char,
    mut require_null_terminated: cJSON_bool,
) -> *mut cJSON {
    cJSON_ParseWithLengthOpts(
        value,
        buffer_length,
        return_parse_end,
        require_null_terminated,
    )
}
#[export_name = "cJSON_Parse"]
pub unsafe extern "C" fn cJSON_Parse_ffi(mut value: *const ::core::ffi::c_char) -> *mut cJSON {
    cJSON_ParseWithOpts(
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
    cJSON_ParseWithLengthOpts(
        value,
        buffer_length,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        0 as cJSON_bool,
    )
}
unsafe extern "C" fn print(
    item: *const cJSON,
    mut format: cJSON_bool,
    hooks: *const internal_hooks,
) -> *mut ::core::ffi::c_uchar {
    let mut c2rust_current_block: u64;
    let default_buffer_size: size_t = 256 as size_t;
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
    memset(
        &raw mut buffer as *mut printbuffer as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[printbuffer; 1]>() as size_t,
    );
    let ref mut c2rust_fresh8 = (*(&raw mut buffer as *mut printbuffer)).buffer;
    *c2rust_fresh8 = (*hooks).allocate.expect("non-null function pointer")(default_buffer_size)
        as *mut ::core::ffi::c_uchar;
    (*(&raw mut buffer as *mut printbuffer)).length = default_buffer_size;
    (*(&raw mut buffer as *mut printbuffer)).format = format;
    (*(&raw mut buffer as *mut printbuffer)).hooks = *hooks;
    if !(*(&raw mut buffer as *mut printbuffer)).buffer.is_null() {
        if !(print_value(item, &raw mut buffer as *mut printbuffer) == 0) {
            update_offset(&raw mut buffer as *mut printbuffer);
            if (*hooks).reallocate.is_some() {
                printed = (*hooks).reallocate.expect("non-null function pointer")(
                    (*(&raw mut buffer as *mut printbuffer)).buffer as *mut ::core::ffi::c_void,
                    (*(&raw mut buffer as *mut printbuffer))
                        .offset
                        .wrapping_add(1 as size_t),
                ) as *mut ::core::ffi::c_uchar;
                if printed.is_null() {
                    c2rust_current_block = 15492938347856135346;
                } else {
                    let ref mut c2rust_fresh9 = (*(&raw mut buffer as *mut printbuffer)).buffer;
                    *c2rust_fresh9 = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                    c2rust_current_block = 7149356873433890176;
                }
            } else {
                printed = (*hooks).allocate.expect("non-null function pointer")(
                    (*(&raw mut buffer as *mut printbuffer))
                        .offset
                        .wrapping_add(1 as size_t),
                ) as *mut ::core::ffi::c_uchar;
                if printed.is_null() {
                    c2rust_current_block = 15492938347856135346;
                } else {
                    memcpy(
                        printed as *mut ::core::ffi::c_void,
                        (*(&raw mut buffer as *mut printbuffer)).buffer
                            as *const ::core::ffi::c_void,
                        if (*(&raw mut buffer as *mut printbuffer)).length
                            < (*(&raw mut buffer as *mut printbuffer))
                                .offset
                                .wrapping_add(1 as size_t)
                        {
                            (*(&raw mut buffer as *mut printbuffer)).length
                        } else {
                            (*(&raw mut buffer as *mut printbuffer))
                                .offset
                                .wrapping_add(1 as size_t)
                        },
                    );
                    *printed.offset((*(&raw mut buffer as *mut printbuffer)).offset as isize) =
                        '\0' as i32 as ::core::ffi::c_uchar;
                    (*hooks).deallocate.expect("non-null function pointer")(
                        (*(&raw mut buffer as *mut printbuffer)).buffer as *mut ::core::ffi::c_void,
                    );
                    let ref mut c2rust_fresh10 = (*(&raw mut buffer as *mut printbuffer)).buffer;
                    *c2rust_fresh10 = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                    c2rust_current_block = 7149356873433890176;
                }
            }
            match c2rust_current_block {
                15492938347856135346 => {}
                _ => return printed,
            }
        }
    }
    if !(*(&raw mut buffer as *mut printbuffer)).buffer.is_null() {
        (*hooks).deallocate.expect("non-null function pointer")(
            (*(&raw mut buffer as *mut printbuffer)).buffer as *mut ::core::ffi::c_void,
        );
        let ref mut c2rust_fresh11 = (*(&raw mut buffer as *mut printbuffer)).buffer;
        *c2rust_fresh11 = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    if !printed.is_null() {
        (*hooks).deallocate.expect("non-null function pointer")(
            printed as *mut ::core::ffi::c_void,
        );
        printed = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
}
#[export_name = "cJSON_Print"]
pub unsafe extern "C" fn cJSON_Print_ffi(mut item: *const cJSON) -> *mut ::core::ffi::c_char {
    let hooks = current_global_hooks();
    print(item, true_0, &hooks) as *mut ::core::ffi::c_char
}
#[export_name = "cJSON_PrintUnformatted"]
pub unsafe extern "C" fn cJSON_PrintUnformatted_ffi(
    mut item: *const cJSON,
) -> *mut ::core::ffi::c_char {
    let hooks = current_global_hooks();
    print(item, false_0, &hooks) as *mut ::core::ffi::c_char
}
pub unsafe extern "C" fn cJSON_PrintBuffered(
    mut item: *const cJSON,
    mut prebuffer: ::core::ffi::c_int,
    mut fmt: cJSON_bool,
) -> *mut ::core::ffi::c_char {
    let mut p: printbuffer = printbuffer {
        buffer: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        length: 0 as size_t,
        offset: 0 as size_t,
        depth: 0 as size_t,
        noalloc: 0 as cJSON_bool,
        format: 0 as cJSON_bool,
        hooks: internal_hooks {
            allocate: None,
            deallocate: None,
            reallocate: None,
        },
    };
    if prebuffer < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let hooks = current_global_hooks();
    p.buffer = hooks.allocate.expect("non-null function pointer")(prebuffer as size_t)
        as *mut ::core::ffi::c_uchar;
    if p.buffer.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    p.length = prebuffer as size_t;
    p.offset = 0 as size_t;
    p.noalloc = false_0;
    p.format = fmt;
    p.hooks = hooks;
    if print_value(item, &raw mut p) == 0 {
        hooks.deallocate.expect("non-null function pointer")(p.buffer as *mut ::core::ffi::c_void);
        p.buffer = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return p.buffer as *mut ::core::ffi::c_char;
}
#[export_name = "cJSON_PrintBuffered"]
pub unsafe extern "C" fn cJSON_PrintBuffered_ffi(
    mut item: *const cJSON,
    mut prebuffer: ::core::ffi::c_int,
    mut fmt: cJSON_bool,
) -> *mut ::core::ffi::c_char {
    cJSON_PrintBuffered(item, prebuffer, fmt)
}
pub fn cJSON_PrintPreallocated(
    item: Option<&cJSON>,
    buffer: &mut [::core::ffi::c_char],
    format: cJSON_bool,
) -> cJSON_bool {
    let mut p: printbuffer = printbuffer {
        buffer: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        length: 0 as size_t,
        offset: 0 as size_t,
        depth: 0 as size_t,
        noalloc: 0 as cJSON_bool,
        format: 0 as cJSON_bool,
        hooks: internal_hooks {
            allocate: None,
            deallocate: None,
            reallocate: None,
        },
    };
    if buffer.is_empty() {
        return false_0;
    }
    p.buffer = buffer.as_mut_ptr() as *mut ::core::ffi::c_uchar;
    p.length = buffer.len();
    p.offset = 0 as size_t;
    p.noalloc = true_0;
    p.format = format;
    p.hooks = current_global_hooks();
    let item = match item {
        Some(item) => item as *const cJSON,
        None => ::core::ptr::null(),
    };
    return unsafe { print_value(item, &raw mut p) };
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
    cJSON_PrintPreallocated(
        unsafe { item.as_ref() },
        unsafe { ::core::slice::from_raw_parts_mut(buffer, length as usize) },
        format,
    )
}
unsafe extern "C" fn parse_value(item: *mut cJSON, input_buffer: *mut parse_buffer) -> cJSON_bool {
    if input_buffer.is_null() || (*input_buffer).content.is_null() {
        return false_0;
    }
    if !input_buffer.is_null()
        && (*input_buffer).offset.wrapping_add(4 as size_t) <= (*input_buffer).length
        && strncmp(
            (*input_buffer)
                .content
                .offset((*input_buffer).offset as isize) as *const ::core::ffi::c_char,
            b"null\0".as_ptr() as *const ::core::ffi::c_char,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        (*item).type_0 = cJSON_NULL;
        (*input_buffer).offset = (*input_buffer).offset.wrapping_add(4 as size_t);
        return true_0;
    }
    if !input_buffer.is_null()
        && (*input_buffer).offset.wrapping_add(5 as size_t) <= (*input_buffer).length
        && strncmp(
            (*input_buffer)
                .content
                .offset((*input_buffer).offset as isize) as *const ::core::ffi::c_char,
            b"false\0".as_ptr() as *const ::core::ffi::c_char,
            5 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        (*item).type_0 = cJSON_False;
        (*input_buffer).offset = (*input_buffer).offset.wrapping_add(5 as size_t);
        return true_0;
    }
    if !input_buffer.is_null()
        && (*input_buffer).offset.wrapping_add(4 as size_t) <= (*input_buffer).length
        && strncmp(
            (*input_buffer)
                .content
                .offset((*input_buffer).offset as isize) as *const ::core::ffi::c_char,
            b"true\0".as_ptr() as *const ::core::ffi::c_char,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        (*item).type_0 = cJSON_True;
        (*item).valueint = 1 as ::core::ffi::c_int;
        (*input_buffer).offset = (*input_buffer).offset.wrapping_add(4 as size_t);
        return true_0;
    }
    if !input_buffer.is_null()
        && (*input_buffer).offset.wrapping_add(0 as size_t) < (*input_buffer).length
        && *(*input_buffer)
            .content
            .offset((*input_buffer).offset as isize)
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '"' as i32
    {
        return parse_string(item, input_buffer);
    }
    if !input_buffer.is_null()
        && (*input_buffer).offset.wrapping_add(0 as size_t) < (*input_buffer).length
        && (*(*input_buffer)
            .content
            .offset((*input_buffer).offset as isize)
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '-' as i32
            || *(*input_buffer)
                .content
                .offset((*input_buffer).offset as isize)
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >= '0' as i32
                && *(*input_buffer)
                    .content
                    .offset((*input_buffer).offset as isize)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    <= '9' as i32)
    {
        return parse_number(item, input_buffer);
    }
    if !input_buffer.is_null()
        && (*input_buffer).offset.wrapping_add(0 as size_t) < (*input_buffer).length
        && *(*input_buffer)
            .content
            .offset((*input_buffer).offset as isize)
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '[' as i32
    {
        return parse_array(item, input_buffer);
    }
    if !input_buffer.is_null()
        && (*input_buffer).offset.wrapping_add(0 as size_t) < (*input_buffer).length
        && *(*input_buffer)
            .content
            .offset((*input_buffer).offset as isize)
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '{' as i32
    {
        return parse_object(item, input_buffer);
    }
    return false_0;
}
unsafe extern "C" fn print_value(
    item: *const cJSON,
    output_buffer: *mut printbuffer,
) -> cJSON_bool {
    let mut output: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if item.is_null() || output_buffer.is_null() {
        return false_0;
    }
    match (*item).type_0 & 0xff as ::core::ffi::c_int {
        cJSON_NULL => {
            output = ensure(output_buffer, 5 as size_t);
            if output.is_null() {
                return false_0;
            }
            strcpy(
                output as *mut ::core::ffi::c_char,
                b"null\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return true_0;
        }
        cJSON_False => {
            output = ensure(output_buffer, 6 as size_t);
            if output.is_null() {
                return false_0;
            }
            strcpy(
                output as *mut ::core::ffi::c_char,
                b"false\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return true_0;
        }
        cJSON_True => {
            output = ensure(output_buffer, 5 as size_t);
            if output.is_null() {
                return false_0;
            }
            strcpy(
                output as *mut ::core::ffi::c_char,
                b"true\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return true_0;
        }
        cJSON_Number => return print_number(item, output_buffer),
        cJSON_Raw => {
            let mut raw_length: size_t = 0 as size_t;
            if (*item).valuestring.is_null() {
                return false_0;
            }
            raw_length = strlen((*item).valuestring)
                .wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 1]>() as size_t);
            output = ensure(output_buffer, raw_length);
            if output.is_null() {
                return false_0;
            }
            memcpy(
                output as *mut ::core::ffi::c_void,
                (*item).valuestring as *const ::core::ffi::c_void,
                raw_length,
            );
            return true_0;
        }
        cJSON_String => return print_string(item, output_buffer),
        cJSON_Array => return print_array(item, output_buffer),
        cJSON_Object => return print_object(item, output_buffer),
        _ => return false_0,
    };
}
unsafe extern "C" fn parse_array(item: *mut cJSON, input_buffer: *mut parse_buffer) -> cJSON_bool {
    let mut c2rust_current_block: u64;
    let mut head: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut current_item: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if (*input_buffer).depth >= CJSON_NESTING_LIMIT as size_t {
        return false_0;
    }
    (*input_buffer).depth = (*input_buffer).depth.wrapping_add(1);
    if !(*(*input_buffer)
        .content
        .offset((*input_buffer).offset as isize)
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        != '[' as i32)
    {
        (*input_buffer).offset = (*input_buffer).offset.wrapping_add(1);
        buffer_skip_whitespace(input_buffer);
        if !input_buffer.is_null()
            && (*input_buffer).offset.wrapping_add(0 as size_t) < (*input_buffer).length
            && *(*input_buffer)
                .content
                .offset((*input_buffer).offset as isize)
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == ']' as i32
        {
            c2rust_current_block = 6773356538935231690;
        } else if !(!input_buffer.is_null()
            && (*input_buffer).offset.wrapping_add(0 as size_t) < (*input_buffer).length)
        {
            (*input_buffer).offset = (*input_buffer).offset.wrapping_sub(1);
            c2rust_current_block = 1336238348363633231;
        } else {
            (*input_buffer).offset = (*input_buffer).offset.wrapping_sub(1);
            loop {
                let mut new_item: *mut cJSON = cJSON_New_Item(
                    &(*input_buffer).hooks,
                    cJSON_Invalid,
                    0 as ::core::ffi::c_int,
                    0.0f64,
                    None,
                    None,
                );
                if new_item.is_null() {
                    c2rust_current_block = 1336238348363633231;
                    break;
                }
                if head.is_null() {
                    head = new_item;
                    current_item = head;
                } else {
                    (*current_item).next = new_item as *mut cJSON;
                    (*new_item).prev = current_item as *mut cJSON;
                    current_item = new_item;
                }
                (*input_buffer).offset = (*input_buffer).offset.wrapping_add(1);
                buffer_skip_whitespace(input_buffer);
                if parse_value(current_item, input_buffer) == 0 {
                    c2rust_current_block = 1336238348363633231;
                    break;
                }
                buffer_skip_whitespace(input_buffer);
                if !(!input_buffer.is_null()
                    && (*input_buffer).offset.wrapping_add(0 as size_t) < (*input_buffer).length
                    && *(*input_buffer)
                        .content
                        .offset((*input_buffer).offset as isize)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == ',' as i32)
                {
                    c2rust_current_block = 15089075282327824602;
                    break;
                }
            }
            match c2rust_current_block {
                1336238348363633231 => {}
                _ => {
                    if !(!input_buffer.is_null()
                        && (*input_buffer).offset.wrapping_add(0 as size_t)
                            < (*input_buffer).length)
                        || *(*input_buffer)
                            .content
                            .offset((*input_buffer).offset as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
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
                (*input_buffer).depth = (*input_buffer).depth.wrapping_sub(1);
                if !head.is_null() {
                    (*head).prev = current_item as *mut cJSON;
                }
                (*item).type_0 = cJSON_Array;
                (*item).child = head as *mut cJSON;
                (*input_buffer).offset = (*input_buffer).offset.wrapping_add(1);
                return true_0;
            }
        }
    }
    if !head.is_null() {
        cJSON_Delete(head);
    }
    return false_0;
}
unsafe extern "C" fn print_array(
    item: *const cJSON,
    output_buffer: *mut printbuffer,
) -> cJSON_bool {
    let mut output_pointer: *mut ::core::ffi::c_uchar =
        ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut length: size_t = 0 as size_t;
    let mut current_element: *mut cJSON = (*item).child as *mut cJSON;
    if output_buffer.is_null() {
        return false_0;
    }
    output_pointer = ensure(output_buffer, 1 as size_t);
    if output_pointer.is_null() {
        return false_0;
    }
    *output_pointer = '[' as i32 as ::core::ffi::c_uchar;
    (*output_buffer).offset = (*output_buffer).offset.wrapping_add(1);
    (*output_buffer).depth = (*output_buffer).depth.wrapping_add(1);
    while !current_element.is_null() {
        if print_value(current_element, output_buffer) == 0 {
            return false_0;
        }
        update_offset(output_buffer);
        if !(*current_element).next.is_null() {
            length = (if (*output_buffer).format != 0 {
                2 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as size_t;
            output_pointer = ensure(output_buffer, length.wrapping_add(1 as size_t));
            if output_pointer.is_null() {
                return false_0;
            }
            let c2rust_fresh22 = output_pointer;
            output_pointer = output_pointer.offset(1);
            *c2rust_fresh22 = ',' as i32 as ::core::ffi::c_uchar;
            if (*output_buffer).format != 0 {
                let c2rust_fresh23 = output_pointer;
                output_pointer = output_pointer.offset(1);
                *c2rust_fresh23 = ' ' as i32 as ::core::ffi::c_uchar;
            }
            *output_pointer = '\0' as i32 as ::core::ffi::c_uchar;
            (*output_buffer).offset = (*output_buffer).offset.wrapping_add(length);
        }
        current_element = (*current_element).next as *mut cJSON;
    }
    output_pointer = ensure(output_buffer, 2 as size_t);
    if output_pointer.is_null() {
        return false_0;
    }
    let c2rust_fresh24 = output_pointer;
    output_pointer = output_pointer.offset(1);
    *c2rust_fresh24 = ']' as i32 as ::core::ffi::c_uchar;
    *output_pointer = '\0' as i32 as ::core::ffi::c_uchar;
    (*output_buffer).depth = (*output_buffer).depth.wrapping_sub(1);
    return true_0;
}
unsafe extern "C" fn parse_object(item: *mut cJSON, input_buffer: *mut parse_buffer) -> cJSON_bool {
    let mut c2rust_current_block: u64;
    let mut head: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut current_item: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if (*input_buffer).depth >= CJSON_NESTING_LIMIT as size_t {
        return false_0;
    }
    (*input_buffer).depth = (*input_buffer).depth.wrapping_add(1);
    if !(!(!input_buffer.is_null()
        && (*input_buffer).offset.wrapping_add(0 as size_t) < (*input_buffer).length)
        || *(*input_buffer)
            .content
            .offset((*input_buffer).offset as isize)
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != '{' as i32)
    {
        (*input_buffer).offset = (*input_buffer).offset.wrapping_add(1);
        buffer_skip_whitespace(input_buffer);
        if !input_buffer.is_null()
            && (*input_buffer).offset.wrapping_add(0 as size_t) < (*input_buffer).length
            && *(*input_buffer)
                .content
                .offset((*input_buffer).offset as isize)
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '}' as i32
        {
            c2rust_current_block = 4359236900545362719;
        } else if !(!input_buffer.is_null()
            && (*input_buffer).offset.wrapping_add(0 as size_t) < (*input_buffer).length)
        {
            (*input_buffer).offset = (*input_buffer).offset.wrapping_sub(1);
            c2rust_current_block = 9990476168629568694;
        } else {
            (*input_buffer).offset = (*input_buffer).offset.wrapping_sub(1);
            loop {
                let mut new_item: *mut cJSON = cJSON_New_Item(
                    &(*input_buffer).hooks,
                    cJSON_Invalid,
                    0 as ::core::ffi::c_int,
                    0.0f64,
                    None,
                    None,
                );
                if new_item.is_null() {
                    c2rust_current_block = 9990476168629568694;
                    break;
                } else {
                    if head.is_null() {
                        head = new_item;
                        current_item = head;
                    } else {
                        (*current_item).next = new_item as *mut cJSON;
                        (*new_item).prev = current_item as *mut cJSON;
                        current_item = new_item;
                    }
                    if !(!input_buffer.is_null()
                        && (*input_buffer).offset.wrapping_add(1 as size_t)
                            < (*input_buffer).length)
                    {
                        c2rust_current_block = 9990476168629568694;
                        break;
                    } else {
                        (*input_buffer).offset = (*input_buffer).offset.wrapping_add(1);
                        buffer_skip_whitespace(input_buffer);
                        if parse_string(current_item, input_buffer) == 0 {
                            c2rust_current_block = 9990476168629568694;
                            break;
                        } else {
                            buffer_skip_whitespace(input_buffer);
                            (*current_item).string = (*current_item).valuestring;
                            (*current_item).valuestring =
                                ::core::ptr::null_mut::<::core::ffi::c_char>();
                            if !(!input_buffer.is_null()
                                && (*input_buffer).offset.wrapping_add(0 as size_t)
                                    < (*input_buffer).length)
                                || *(*input_buffer)
                                    .content
                                    .offset((*input_buffer).offset as isize)
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    != ':' as i32
                            {
                                c2rust_current_block = 9990476168629568694;
                                break;
                            } else {
                                (*input_buffer).offset = (*input_buffer).offset.wrapping_add(1);
                                buffer_skip_whitespace(input_buffer);
                                if parse_value(current_item, input_buffer) == 0 {
                                    c2rust_current_block = 9990476168629568694;
                                    break;
                                } else {
                                    buffer_skip_whitespace(input_buffer);
                                    if !(!input_buffer.is_null()
                                        && (*input_buffer).offset.wrapping_add(0 as size_t)
                                            < (*input_buffer).length
                                        && *(*input_buffer)
                                            .content
                                            .offset((*input_buffer).offset as isize)
                                            .offset(0 as ::core::ffi::c_int as isize)
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
                    if !(!input_buffer.is_null()
                        && (*input_buffer).offset.wrapping_add(0 as size_t)
                            < (*input_buffer).length)
                        || *(*input_buffer)
                            .content
                            .offset((*input_buffer).offset as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
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
                (*input_buffer).depth = (*input_buffer).depth.wrapping_sub(1);
                if !head.is_null() {
                    (*head).prev = current_item as *mut cJSON;
                }
                (*item).type_0 = cJSON_Object;
                (*item).child = head as *mut cJSON;
                (*input_buffer).offset = (*input_buffer).offset.wrapping_add(1);
                return true_0;
            }
        }
    }
    if !head.is_null() {
        cJSON_Delete(head);
    }
    return false_0;
}
unsafe extern "C" fn print_object(
    item: *const cJSON,
    output_buffer: *mut printbuffer,
) -> cJSON_bool {
    let mut output_pointer: *mut ::core::ffi::c_uchar =
        ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut length: size_t = 0 as size_t;
    let mut current_item: *mut cJSON = (*item).child as *mut cJSON;
    if output_buffer.is_null() {
        return false_0;
    }
    length = (if (*output_buffer).format != 0 {
        2 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    }) as size_t;
    output_pointer = ensure(output_buffer, length.wrapping_add(1 as size_t));
    if output_pointer.is_null() {
        return false_0;
    }
    let c2rust_fresh12 = output_pointer;
    output_pointer = output_pointer.offset(1);
    *c2rust_fresh12 = '{' as i32 as ::core::ffi::c_uchar;
    (*output_buffer).depth = (*output_buffer).depth.wrapping_add(1);
    if (*output_buffer).format != 0 {
        let c2rust_fresh13 = output_pointer;
        output_pointer = output_pointer.offset(1);
        *c2rust_fresh13 = '\n' as i32 as ::core::ffi::c_uchar;
    }
    (*output_buffer).offset = (*output_buffer).offset.wrapping_add(length);
    while !current_item.is_null() {
        if (*output_buffer).format != 0 {
            let mut i: size_t = 0;
            output_pointer = ensure(output_buffer, (*output_buffer).depth);
            if output_pointer.is_null() {
                return false_0;
            }
            i = 0 as size_t;
            while i < (*output_buffer).depth {
                let c2rust_fresh14 = output_pointer;
                output_pointer = output_pointer.offset(1);
                *c2rust_fresh14 = '\t' as i32 as ::core::ffi::c_uchar;
                i = i.wrapping_add(1);
            }
            (*output_buffer).offset = (*output_buffer).offset.wrapping_add((*output_buffer).depth);
        }
        if print_string_ptr(
            (*current_item).string as *mut ::core::ffi::c_uchar,
            output_buffer,
        ) == 0
        {
            return false_0;
        }
        update_offset(output_buffer);
        length = (if (*output_buffer).format != 0 {
            2 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) as size_t;
        output_pointer = ensure(output_buffer, length);
        if output_pointer.is_null() {
            return false_0;
        }
        let c2rust_fresh15 = output_pointer;
        output_pointer = output_pointer.offset(1);
        *c2rust_fresh15 = ':' as i32 as ::core::ffi::c_uchar;
        if (*output_buffer).format != 0 {
            let c2rust_fresh16 = output_pointer;
            output_pointer = output_pointer.offset(1);
            *c2rust_fresh16 = '\t' as i32 as ::core::ffi::c_uchar;
        }
        (*output_buffer).offset = (*output_buffer).offset.wrapping_add(length);
        if print_value(current_item, output_buffer) == 0 {
            return false_0;
        }
        update_offset(output_buffer);
        length = ((if (*output_buffer).format != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as size_t)
            .wrapping_add(
                (if !(*current_item).next.is_null() {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as size_t,
            );
        output_pointer = ensure(output_buffer, length.wrapping_add(1 as size_t));
        if output_pointer.is_null() {
            return false_0;
        }
        if !(*current_item).next.is_null() {
            let c2rust_fresh17 = output_pointer;
            output_pointer = output_pointer.offset(1);
            *c2rust_fresh17 = ',' as i32 as ::core::ffi::c_uchar;
        }
        if (*output_buffer).format != 0 {
            let c2rust_fresh18 = output_pointer;
            output_pointer = output_pointer.offset(1);
            *c2rust_fresh18 = '\n' as i32 as ::core::ffi::c_uchar;
        }
        *output_pointer = '\0' as i32 as ::core::ffi::c_uchar;
        (*output_buffer).offset = (*output_buffer).offset.wrapping_add(length);
        current_item = (*current_item).next as *mut cJSON;
    }
    output_pointer = ensure(
        output_buffer,
        if (*output_buffer).format != 0 {
            (*output_buffer).depth.wrapping_add(1 as size_t)
        } else {
            2 as size_t
        },
    );
    if output_pointer.is_null() {
        return false_0;
    }
    if (*output_buffer).format != 0 {
        let mut i_0: size_t = 0;
        i_0 = 0 as size_t;
        while i_0 < (*output_buffer).depth.wrapping_sub(1 as size_t) {
            let c2rust_fresh19 = output_pointer;
            output_pointer = output_pointer.offset(1);
            *c2rust_fresh19 = '\t' as i32 as ::core::ffi::c_uchar;
            i_0 = i_0.wrapping_add(1);
        }
    }
    let c2rust_fresh20 = output_pointer;
    output_pointer = output_pointer.offset(1);
    *c2rust_fresh20 = '}' as i32 as ::core::ffi::c_uchar;
    *output_pointer = '\0' as i32 as ::core::ffi::c_uchar;
    (*output_buffer).depth = (*output_buffer).depth.wrapping_sub(1);
    return true_0;
}
pub unsafe extern "C" fn cJSON_GetArraySize(mut array: *const cJSON) -> ::core::ffi::c_int {
    let mut child: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut size: size_t = 0 as size_t;
    if array.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    child = (*array).child as *mut cJSON;
    while !child.is_null() {
        size = size.wrapping_add(1);
        child = (*child).next as *mut cJSON;
    }
    return size as ::core::ffi::c_int;
}
#[export_name = "cJSON_GetArraySize"]
pub unsafe extern "C" fn cJSON_GetArraySize_ffi(mut array: *const cJSON) -> ::core::ffi::c_int {
    cJSON_GetArraySize(array)
}
unsafe extern "C" fn get_array_item(mut array: *const cJSON, mut index: size_t) -> *mut cJSON {
    let mut current_child: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if array.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    current_child = (*array).child as *mut cJSON;
    while !current_child.is_null() && index > 0 as size_t {
        index = index.wrapping_sub(1);
        current_child = (*current_child).next as *mut cJSON;
    }
    return current_child;
}
#[export_name = "cJSON_GetArrayItem"]
pub unsafe extern "C" fn cJSON_GetArrayItem_ffi(
    mut array: *const cJSON,
    mut index: ::core::ffi::c_int,
) -> *mut cJSON {
    if index < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<cJSON>();
    }
    get_array_item(array, index as size_t)
}
unsafe extern "C" fn get_object_item(
    object: *const cJSON,
    name: *const ::core::ffi::c_char,
    case_sensitive: cJSON_bool,
) -> *mut cJSON {
    let mut current_element: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if object.is_null() || name.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    current_element = (*object).child as *mut cJSON;
    let name_bytes = ::core::ffi::CStr::from_ptr(name).to_bytes();
    while !current_element.is_null() {
        let element_name = (*current_element).string;
        if element_name.is_null() {
            if case_sensitive != 0 {
                break;
            }
        } else {
            let element_name_bytes = ::core::ffi::CStr::from_ptr(element_name).to_bytes();
            if (case_sensitive != 0 && name_bytes == element_name_bytes)
                || (case_sensitive == 0
                    && compare_case_insensitive_bytes(name_bytes, element_name_bytes)
                        == 0 as ::core::ffi::c_int)
            {
                break;
            }
        }
        current_element = (*current_element).next as *mut cJSON;
    }
    if current_element.is_null() || (*current_element).string.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    return current_element;
}
#[export_name = "cJSON_GetObjectItem"]
pub unsafe extern "C" fn cJSON_GetObjectItem_ffi(
    object: *const cJSON,
    string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    get_object_item(object, string, false_0)
}
#[export_name = "cJSON_GetObjectItemCaseSensitive"]
pub unsafe extern "C" fn cJSON_GetObjectItemCaseSensitive_ffi(
    object: *const cJSON,
    string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    get_object_item(object, string, true_0)
}
#[export_name = "cJSON_HasObjectItem"]
pub unsafe extern "C" fn cJSON_HasObjectItem_ffi(
    mut object: *const cJSON,
    mut string: *const ::core::ffi::c_char,
) -> cJSON_bool {
    return if !get_object_item(object, string, false_0).is_null() {
        1 as cJSON_bool
    } else {
        0 as cJSON_bool
    };
}
unsafe extern "C" fn suffix_object(mut prev: *mut cJSON, mut item: *mut cJSON) {
    (*prev).next = item as *mut cJSON;
    (*item).prev = prev as *mut cJSON;
}
unsafe extern "C" fn create_reference(
    mut item: *const cJSON,
    hooks: &internal_hooks,
) -> *mut cJSON {
    let mut reference: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if item.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    reference = cJSON_New_Item(
        hooks,
        cJSON_Invalid,
        0 as ::core::ffi::c_int,
        0.0f64,
        None,
        None,
    );
    if reference.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    memcpy(
        reference as *mut ::core::ffi::c_void,
        item as *const ::core::ffi::c_void,
        ::core::mem::size_of::<cJSON>() as size_t,
    );
    (*reference).string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*reference).type_0 |= cJSON_IsReference;
    (*reference).prev = ::core::ptr::null_mut::<cJSON>();
    (*reference).next = (*reference).prev;
    return reference;
}
unsafe extern "C" fn add_item_to_array(mut array: *mut cJSON, mut item: *mut cJSON) -> cJSON_bool {
    let mut child: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if item.is_null() || array.is_null() || array == item {
        return false_0;
    }
    child = (*array).child as *mut cJSON;
    if child.is_null() {
        (*array).child = item as *mut cJSON;
        (*item).prev = item as *mut cJSON;
        (*item).next = ::core::ptr::null_mut::<cJSON>();
    } else if !(*child).prev.is_null() {
        suffix_object((*child).prev as *mut cJSON, item);
        (*(*array).child).prev = item as *mut cJSON;
    }
    return true_0;
}
#[export_name = "cJSON_AddItemToArray"]
pub unsafe extern "C" fn cJSON_AddItemToArray_ffi(
    mut array: *mut cJSON,
    mut item: *mut cJSON,
) -> cJSON_bool {
    add_item_to_array(array, item)
}
unsafe extern "C" fn add_item_to_object(
    object: *mut cJSON,
    string: *const ::core::ffi::c_char,
    item: *mut cJSON,
    hooks: *const internal_hooks,
    constant_key: cJSON_bool,
) -> cJSON_bool {
    let mut new_key: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut new_type: ::core::ffi::c_int = cJSON_Invalid;
    if object.is_null() || string.is_null() || item.is_null() || object == item {
        return false_0;
    }
    if constant_key != 0 {
        new_key = string as *mut ::core::ffi::c_char;
        new_type = (*item).type_0 | cJSON_StringIsConst;
    } else {
        new_key =
            cJSON_strdup(string as *const ::core::ffi::c_uchar, hooks) as *mut ::core::ffi::c_char;
        if new_key.is_null() {
            return false_0;
        }
        new_type = (*item).type_0 & !cJSON_StringIsConst;
    }
    if (*item).type_0 & cJSON_StringIsConst == 0 && !(*item).string.is_null() {
        (*hooks).deallocate.expect("non-null function pointer")(
            (*item).string as *mut ::core::ffi::c_void,
        );
    }
    (*item).string = new_key;
    (*item).type_0 = new_type;
    return add_item_to_array(object, item);
}
#[export_name = "cJSON_AddItemToObject"]
pub unsafe extern "C" fn cJSON_AddItemToObject_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
    mut item: *mut cJSON,
) -> cJSON_bool {
    let hooks = current_global_hooks();
    add_item_to_object(object, string, item, &hooks, false_0)
}
#[export_name = "cJSON_AddItemToObjectCS"]
pub unsafe extern "C" fn cJSON_AddItemToObjectCS_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
    mut item: *mut cJSON,
) -> cJSON_bool {
    let hooks = current_global_hooks();
    add_item_to_object(object, string, item, &hooks, true_0)
}
#[export_name = "cJSON_AddItemReferenceToArray"]
pub unsafe extern "C" fn cJSON_AddItemReferenceToArray_ffi(
    mut array: *mut cJSON,
    mut item: *mut cJSON,
) -> cJSON_bool {
    if array.is_null() {
        return false_0;
    }
    let hooks = current_global_hooks();
    return add_item_to_array(array, create_reference(item, &hooks));
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
    let hooks = current_global_hooks();
    return add_item_to_object(
        object,
        string,
        create_reference(item, &hooks),
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
    let hooks = current_global_hooks();
    if add_item_to_object(object, name, null, &hooks, false_0) != 0 {
        return null;
    }
    cJSON_Delete(null);
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddTrueToObject"]
pub unsafe extern "C" fn cJSON_AddTrueToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let mut true_item: *mut cJSON = cJSON_CreateTrue();
    let hooks = current_global_hooks();
    if add_item_to_object(object, name, true_item, &hooks, false_0) != 0 {
        return true_item;
    }
    cJSON_Delete(true_item);
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddFalseToObject"]
pub unsafe extern "C" fn cJSON_AddFalseToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let mut false_item: *mut cJSON = cJSON_CreateFalse();
    let hooks = current_global_hooks();
    if add_item_to_object(object, name, false_item, &hooks, false_0) != 0 {
        return false_item;
    }
    cJSON_Delete(false_item);
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddBoolToObject"]
pub unsafe extern "C" fn cJSON_AddBoolToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
    boolean: cJSON_bool,
) -> *mut cJSON {
    let mut bool_item: *mut cJSON = cJSON_CreateBool(boolean);
    let hooks = current_global_hooks();
    if add_item_to_object(object, name, bool_item, &hooks, false_0) != 0 {
        return bool_item;
    }
    cJSON_Delete(bool_item);
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddNumberToObject"]
pub unsafe extern "C" fn cJSON_AddNumberToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
    number: ::core::ffi::c_double,
) -> *mut cJSON {
    let mut number_item: *mut cJSON = cJSON_CreateNumber(number);
    let hooks = current_global_hooks();
    if add_item_to_object(object, name, number_item, &hooks, false_0) != 0 {
        return number_item;
    }
    cJSON_Delete(number_item);
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddStringToObject"]
pub unsafe extern "C" fn cJSON_AddStringToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
    string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let mut string_item: *mut cJSON = cJSON_CreateString(string);
    let hooks = current_global_hooks();
    if add_item_to_object(object, name, string_item, &hooks, false_0) != 0 {
        return string_item;
    }
    cJSON_Delete(string_item);
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddRawToObject"]
pub unsafe extern "C" fn cJSON_AddRawToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
    raw: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let mut raw_item: *mut cJSON = cJSON_CreateRaw(raw);
    let hooks = current_global_hooks();
    if add_item_to_object(object, name, raw_item, &hooks, false_0) != 0 {
        return raw_item;
    }
    cJSON_Delete(raw_item);
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddObjectToObject"]
pub unsafe extern "C" fn cJSON_AddObjectToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let mut object_item: *mut cJSON = cJSON_CreateObject();
    let hooks = current_global_hooks();
    if add_item_to_object(object, name, object_item, &hooks, false_0) != 0 {
        return object_item;
    }
    cJSON_Delete(object_item);
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddArrayToObject"]
pub unsafe extern "C" fn cJSON_AddArrayToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let mut array: *mut cJSON = cJSON_CreateArray();
    let hooks = current_global_hooks();
    if add_item_to_object(object, name, array, &hooks, false_0) != 0 {
        return array;
    }
    cJSON_Delete(array);
    return ::core::ptr::null_mut::<cJSON>();
}
pub unsafe extern "C" fn cJSON_DetachItemViaPointer(
    mut parent: *mut cJSON,
    item: *mut cJSON,
) -> *mut cJSON {
    if parent.is_null() || item.is_null() || item != (*parent).child && (*item).prev.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
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
    (*item).prev = ::core::ptr::null_mut::<cJSON>();
    (*item).next = ::core::ptr::null_mut::<cJSON>();
    return item;
}
#[export_name = "cJSON_DetachItemViaPointer"]
pub unsafe extern "C" fn cJSON_DetachItemViaPointer_ffi(
    mut parent: *mut cJSON,
    item: *mut cJSON,
) -> *mut cJSON {
    cJSON_DetachItemViaPointer(parent, item)
}
#[export_name = "cJSON_DetachItemFromArray"]
pub unsafe extern "C" fn cJSON_DetachItemFromArray_ffi(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
) -> *mut cJSON {
    if which < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<cJSON>();
    }
    return cJSON_DetachItemViaPointer(array, get_array_item(array, which as size_t));
}
#[export_name = "cJSON_DeleteItemFromArray"]
pub unsafe extern "C" fn cJSON_DeleteItemFromArray_ffi(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
) {
    let item = if which < 0 as ::core::ffi::c_int {
        ::core::ptr::null_mut::<cJSON>()
    } else {
        cJSON_DetachItemViaPointer(array, get_array_item(array, which as size_t))
    };
    cJSON_Delete(item);
}
#[export_name = "cJSON_DetachItemFromObject"]
pub unsafe extern "C" fn cJSON_DetachItemFromObject_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let mut to_detach: *mut cJSON = get_object_item(object, string, false_0);
    return cJSON_DetachItemViaPointer(object, to_detach);
}
#[export_name = "cJSON_DetachItemFromObjectCaseSensitive"]
pub unsafe extern "C" fn cJSON_DetachItemFromObjectCaseSensitive_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let mut to_detach: *mut cJSON = get_object_item(object, string, true_0);
    return cJSON_DetachItemViaPointer(object, to_detach);
}
#[export_name = "cJSON_DeleteItemFromObject"]
pub unsafe extern "C" fn cJSON_DeleteItemFromObject_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
) {
    let to_detach: *mut cJSON = get_object_item(object, string, false_0);
    cJSON_Delete(cJSON_DetachItemViaPointer(object, to_detach));
}
#[export_name = "cJSON_DeleteItemFromObjectCaseSensitive"]
pub unsafe extern "C" fn cJSON_DeleteItemFromObjectCaseSensitive_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
) {
    let to_detach: *mut cJSON = get_object_item(object, string, true_0);
    cJSON_Delete(cJSON_DetachItemViaPointer(object, to_detach));
}
pub unsafe extern "C" fn cJSON_InsertItemInArray(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
    mut newitem: *mut cJSON,
) -> cJSON_bool {
    let mut after_inserted: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if which < 0 as ::core::ffi::c_int || newitem.is_null() {
        return false_0;
    }
    after_inserted = get_array_item(array, which as size_t);
    if after_inserted.is_null() {
        return add_item_to_array(array, newitem);
    }
    if after_inserted != (*array).child && (*after_inserted).prev.is_null() {
        return false_0;
    }
    (*newitem).next = after_inserted as *mut cJSON;
    (*newitem).prev = (*after_inserted).prev;
    (*after_inserted).prev = newitem as *mut cJSON;
    if after_inserted == (*array).child {
        (*array).child = newitem as *mut cJSON;
    } else {
        (*(*newitem).prev).next = newitem as *mut cJSON;
    }
    return true_0;
}
#[export_name = "cJSON_InsertItemInArray"]
pub unsafe extern "C" fn cJSON_InsertItemInArray_ffi(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
    mut newitem: *mut cJSON,
) -> cJSON_bool {
    cJSON_InsertItemInArray(array, which, newitem)
}
pub unsafe extern "C" fn cJSON_ReplaceItemViaPointer(
    parent: *mut cJSON,
    item: *mut cJSON,
    mut replacement: *mut cJSON,
) -> cJSON_bool {
    if parent.is_null() || (*parent).child.is_null() || replacement.is_null() || item.is_null() {
        return false_0;
    }
    if replacement == item {
        return true_0;
    }
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
    cJSON_Delete(item);
    return true_0;
}
#[export_name = "cJSON_ReplaceItemViaPointer"]
pub unsafe extern "C" fn cJSON_ReplaceItemViaPointer_ffi(
    parent: *mut cJSON,
    item: *mut cJSON,
    mut replacement: *mut cJSON,
) -> cJSON_bool {
    cJSON_ReplaceItemViaPointer(parent, item, replacement)
}
pub unsafe extern "C" fn cJSON_ReplaceItemInArray(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
    mut newitem: *mut cJSON,
) -> cJSON_bool {
    if which < 0 as ::core::ffi::c_int {
        return false_0;
    }
    return cJSON_ReplaceItemViaPointer(array, get_array_item(array, which as size_t), newitem);
}
#[export_name = "cJSON_ReplaceItemInArray"]
pub unsafe extern "C" fn cJSON_ReplaceItemInArray_ffi(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
    mut newitem: *mut cJSON,
) -> cJSON_bool {
    cJSON_ReplaceItemInArray(array, which, newitem)
}
unsafe extern "C" fn replace_item_in_object(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
    mut replacement: *mut cJSON,
    mut case_sensitive: cJSON_bool,
) -> cJSON_bool {
    if replacement.is_null() || string.is_null() {
        return false_0;
    }
    let hooks = current_global_hooks();
    if (*replacement).type_0 & cJSON_StringIsConst == 0 && !(*replacement).string.is_null() {
        hooks.deallocate.expect("non-null function pointer")(
            (*replacement).string as *mut ::core::ffi::c_void,
        );
    }
    (*replacement).string =
        cJSON_strdup(string as *const ::core::ffi::c_uchar, &hooks) as *mut ::core::ffi::c_char;
    if (*replacement).string.is_null() {
        return false_0;
    }
    (*replacement).type_0 &= !cJSON_StringIsConst;
    return cJSON_ReplaceItemViaPointer(
        object,
        get_object_item(object, string, case_sensitive),
        replacement,
    );
}
#[export_name = "cJSON_ReplaceItemInObject"]
pub unsafe extern "C" fn cJSON_ReplaceItemInObject_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
    mut newitem: *mut cJSON,
) -> cJSON_bool {
    replace_item_in_object(object, string, newitem, false_0)
}
#[export_name = "cJSON_ReplaceItemInObjectCaseSensitive"]
pub unsafe extern "C" fn cJSON_ReplaceItemInObjectCaseSensitive_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
    mut newitem: *mut cJSON,
) -> cJSON_bool {
    replace_item_in_object(object, string, newitem, true_0)
}
pub fn cJSON_CreateNull() -> *mut cJSON {
    let hooks = current_global_hooks();
    cJSON_New_Item(
        &hooks,
        cJSON_NULL,
        0 as ::core::ffi::c_int,
        0.0f64,
        None,
        None,
    )
}
#[export_name = "cJSON_CreateNull"]
pub unsafe extern "C" fn cJSON_CreateNull_ffi() -> *mut cJSON {
    cJSON_CreateNull()
}
pub fn cJSON_CreateTrue() -> *mut cJSON {
    let hooks = current_global_hooks();
    cJSON_New_Item(
        &hooks,
        cJSON_True,
        0 as ::core::ffi::c_int,
        0.0f64,
        None,
        None,
    )
}
#[export_name = "cJSON_CreateTrue"]
pub unsafe extern "C" fn cJSON_CreateTrue_ffi() -> *mut cJSON {
    cJSON_CreateTrue()
}
pub fn cJSON_CreateFalse() -> *mut cJSON {
    let hooks = current_global_hooks();
    cJSON_New_Item(
        &hooks,
        cJSON_False,
        0 as ::core::ffi::c_int,
        0.0f64,
        None,
        None,
    )
}
#[export_name = "cJSON_CreateFalse"]
pub unsafe extern "C" fn cJSON_CreateFalse_ffi() -> *mut cJSON {
    cJSON_CreateFalse()
}
pub fn cJSON_CreateBool(boolean: cJSON_bool) -> *mut cJSON {
    let hooks = current_global_hooks();
    cJSON_New_Item(
        &hooks,
        if boolean != 0 {
            cJSON_True
        } else {
            cJSON_False
        },
        0 as ::core::ffi::c_int,
        0.0f64,
        None,
        None,
    )
}
#[export_name = "cJSON_CreateBool"]
pub unsafe extern "C" fn cJSON_CreateBool_ffi(mut boolean: cJSON_bool) -> *mut cJSON {
    cJSON_CreateBool(boolean)
}
pub fn cJSON_CreateNumber(num: ::core::ffi::c_double) -> *mut cJSON {
    let hooks = current_global_hooks();
    let valueint = if num >= INT_MAX as ::core::ffi::c_double {
        INT_MAX
    } else if num <= INT_MIN as ::core::ffi::c_double {
        INT_MIN
    } else {
        num as ::core::ffi::c_int
    };
    cJSON_New_Item(&hooks, cJSON_Number, valueint, num, None, None)
}
#[export_name = "cJSON_CreateNumber"]
pub unsafe extern "C" fn cJSON_CreateNumber_ffi(mut num: ::core::ffi::c_double) -> *mut cJSON {
    cJSON_CreateNumber(num)
}
pub unsafe extern "C" fn cJSON_CreateString(mut string: *const ::core::ffi::c_char) -> *mut cJSON {
    let hooks = current_global_hooks();
    let mut item: *mut cJSON = cJSON_New_Item(
        &hooks,
        cJSON_String,
        0 as ::core::ffi::c_int,
        0.0f64,
        None,
        None,
    );
    if !item.is_null() {
        (*item).valuestring =
            cJSON_strdup(string as *const ::core::ffi::c_uchar, &hooks) as *mut ::core::ffi::c_char;
        if (*item).valuestring.is_null() {
            cJSON_Delete(item);
            return ::core::ptr::null_mut::<cJSON>();
        }
    }
    return item;
}
#[export_name = "cJSON_CreateString"]
pub unsafe extern "C" fn cJSON_CreateString_ffi(
    mut string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    cJSON_CreateString(string)
}
pub fn cJSON_CreateStringReference(string: Option<&::core::ffi::CStr>) -> *mut cJSON {
    let hooks = current_global_hooks();
    cJSON_New_Item(
        &hooks,
        cJSON_String | cJSON_IsReference,
        0 as ::core::ffi::c_int,
        0.0f64,
        string,
        None,
    )
}
#[export_name = "cJSON_CreateStringReference"]
pub unsafe extern "C" fn cJSON_CreateStringReference_ffi(
    mut string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let string = if string.is_null() {
        None
    } else {
        Some(unsafe { ::core::ffi::CStr::from_ptr(string) })
    };
    cJSON_CreateStringReference(string)
}
pub fn cJSON_CreateObjectReference(child: Option<&cJSON>) -> *mut cJSON {
    let hooks = current_global_hooks();
    cJSON_New_Item(
        &hooks,
        cJSON_Object | cJSON_IsReference,
        0 as ::core::ffi::c_int,
        0.0f64,
        None,
        child,
    )
}
#[export_name = "cJSON_CreateObjectReference"]
pub unsafe extern "C" fn cJSON_CreateObjectReference_ffi(mut child: *const cJSON) -> *mut cJSON {
    cJSON_CreateObjectReference(unsafe { child.as_ref() })
}
pub fn cJSON_CreateArrayReference(child: Option<&cJSON>) -> *mut cJSON {
    let hooks = current_global_hooks();
    cJSON_New_Item(
        &hooks,
        cJSON_Array | cJSON_IsReference,
        0 as ::core::ffi::c_int,
        0.0f64,
        None,
        child,
    )
}
#[export_name = "cJSON_CreateArrayReference"]
pub unsafe extern "C" fn cJSON_CreateArrayReference_ffi(mut child: *const cJSON) -> *mut cJSON {
    cJSON_CreateArrayReference(unsafe { child.as_ref() })
}
pub unsafe extern "C" fn cJSON_CreateRaw(mut raw: *const ::core::ffi::c_char) -> *mut cJSON {
    let hooks = current_global_hooks();
    let mut item: *mut cJSON = cJSON_New_Item(
        &hooks,
        cJSON_Raw,
        0 as ::core::ffi::c_int,
        0.0f64,
        None,
        None,
    );
    if !item.is_null() {
        (*item).valuestring =
            cJSON_strdup(raw as *const ::core::ffi::c_uchar, &hooks) as *mut ::core::ffi::c_char;
        if (*item).valuestring.is_null() {
            cJSON_Delete(item);
            return ::core::ptr::null_mut::<cJSON>();
        }
    }
    return item;
}
#[export_name = "cJSON_CreateRaw"]
pub unsafe extern "C" fn cJSON_CreateRaw_ffi(mut raw: *const ::core::ffi::c_char) -> *mut cJSON {
    cJSON_CreateRaw(raw)
}
pub fn cJSON_CreateArray() -> *mut cJSON {
    let hooks = current_global_hooks();
    cJSON_New_Item(
        &hooks,
        cJSON_Array,
        0 as ::core::ffi::c_int,
        0.0f64,
        None,
        None,
    )
}
#[export_name = "cJSON_CreateArray"]
pub unsafe extern "C" fn cJSON_CreateArray_ffi() -> *mut cJSON {
    cJSON_CreateArray()
}
pub fn cJSON_CreateObject() -> *mut cJSON {
    let hooks = current_global_hooks();
    cJSON_New_Item(
        &hooks,
        cJSON_Object,
        0 as ::core::ffi::c_int,
        0.0f64,
        None,
        None,
    )
}
#[export_name = "cJSON_CreateObject"]
pub unsafe extern "C" fn cJSON_CreateObject_ffi() -> *mut cJSON {
    cJSON_CreateObject()
}
pub unsafe extern "C" fn cJSON_CreateIntArray(
    mut numbers: *const ::core::ffi::c_int,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    let mut i: size_t = 0 as size_t;
    let mut n: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut p: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut a: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    a = cJSON_CreateArray();
    i = 0 as size_t;
    while !a.is_null() && i < count as size_t {
        n = cJSON_CreateNumber(*numbers.offset(i as isize) as ::core::ffi::c_double);
        if n.is_null() {
            cJSON_Delete(a);
            return ::core::ptr::null_mut::<cJSON>();
        }
        if i == 0 {
            (*a).child = n as *mut cJSON;
        } else {
            suffix_object(p, n);
        }
        p = n;
        i = i.wrapping_add(1);
    }
    if !a.is_null() && !(*a).child.is_null() {
        (*(*a).child).prev = n as *mut cJSON;
    }
    return a;
}
#[export_name = "cJSON_CreateIntArray"]
pub unsafe extern "C" fn cJSON_CreateIntArray_ffi(
    mut numbers: *const ::core::ffi::c_int,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    cJSON_CreateIntArray(numbers, count)
}
pub unsafe extern "C" fn cJSON_CreateFloatArray(
    mut numbers: *const ::core::ffi::c_float,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    let mut i: size_t = 0 as size_t;
    let mut n: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut p: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut a: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    a = cJSON_CreateArray();
    i = 0 as size_t;
    while !a.is_null() && i < count as size_t {
        n = cJSON_CreateNumber(*numbers.offset(i as isize) as ::core::ffi::c_double);
        if n.is_null() {
            cJSON_Delete(a);
            return ::core::ptr::null_mut::<cJSON>();
        }
        if i == 0 {
            (*a).child = n as *mut cJSON;
        } else {
            suffix_object(p, n);
        }
        p = n;
        i = i.wrapping_add(1);
    }
    if !a.is_null() && !(*a).child.is_null() {
        (*(*a).child).prev = n as *mut cJSON;
    }
    return a;
}
#[export_name = "cJSON_CreateFloatArray"]
pub unsafe extern "C" fn cJSON_CreateFloatArray_ffi(
    mut numbers: *const ::core::ffi::c_float,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    cJSON_CreateFloatArray(numbers, count)
}
pub unsafe extern "C" fn cJSON_CreateDoubleArray(
    mut numbers: *const ::core::ffi::c_double,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    let mut i: size_t = 0 as size_t;
    let mut n: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut p: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut a: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    a = cJSON_CreateArray();
    i = 0 as size_t;
    while !a.is_null() && i < count as size_t {
        n = cJSON_CreateNumber(*numbers.offset(i as isize));
        if n.is_null() {
            cJSON_Delete(a);
            return ::core::ptr::null_mut::<cJSON>();
        }
        if i == 0 {
            (*a).child = n as *mut cJSON;
        } else {
            suffix_object(p, n);
        }
        p = n;
        i = i.wrapping_add(1);
    }
    if !a.is_null() && !(*a).child.is_null() {
        (*(*a).child).prev = n as *mut cJSON;
    }
    return a;
}
#[export_name = "cJSON_CreateDoubleArray"]
pub unsafe extern "C" fn cJSON_CreateDoubleArray_ffi(
    mut numbers: *const ::core::ffi::c_double,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    cJSON_CreateDoubleArray(numbers, count)
}
pub unsafe extern "C" fn cJSON_CreateStringArray(
    mut strings: *const *const ::core::ffi::c_char,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    let mut i: size_t = 0 as size_t;
    let mut n: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut p: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut a: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if count < 0 as ::core::ffi::c_int || strings.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    a = cJSON_CreateArray();
    i = 0 as size_t;
    while !a.is_null() && i < count as size_t {
        n = cJSON_CreateString(*strings.offset(i as isize));
        if n.is_null() {
            cJSON_Delete(a);
            return ::core::ptr::null_mut::<cJSON>();
        }
        if i == 0 {
            (*a).child = n as *mut cJSON;
        } else {
            suffix_object(p, n);
        }
        p = n;
        i = i.wrapping_add(1);
    }
    if !a.is_null() && !(*a).child.is_null() {
        (*(*a).child).prev = n as *mut cJSON;
    }
    return a;
}
#[export_name = "cJSON_CreateStringArray"]
pub unsafe extern "C" fn cJSON_CreateStringArray_ffi(
    mut strings: *const *const ::core::ffi::c_char,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    cJSON_CreateStringArray(strings, count)
}
#[export_name = "cJSON_Duplicate"]
pub unsafe extern "C" fn cJSON_Duplicate_ffi(
    mut item: *const cJSON,
    mut recurse: cJSON_bool,
) -> *mut cJSON {
    cJSON_Duplicate_rec(item, 0 as size_t, recurse)
}
pub unsafe extern "C" fn cJSON_Duplicate_rec(
    mut item: *const cJSON,
    mut depth: size_t,
    mut recurse: cJSON_bool,
) -> *mut cJSON {
    let mut c2rust_current_block: u64;
    let mut newitem: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut child: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut next: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut newchild: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let hooks = current_global_hooks();
    if !item.is_null() {
        newitem = cJSON_New_Item(
            &hooks,
            cJSON_Invalid,
            0 as ::core::ffi::c_int,
            0.0f64,
            None,
            None,
        );
        if !newitem.is_null() {
            (*newitem).type_0 = (*item).type_0 & !cJSON_IsReference;
            (*newitem).valueint = (*item).valueint;
            (*newitem).valuedouble = (*item).valuedouble;
            if !(*item).valuestring.is_null() {
                (*newitem).valuestring =
                    cJSON_strdup((*item).valuestring as *mut ::core::ffi::c_uchar, &hooks)
                        as *mut ::core::ffi::c_char;
                if (*newitem).valuestring.is_null() {
                    c2rust_current_block = 12988308604321106300;
                } else {
                    c2rust_current_block = 11812396948646013369;
                }
            } else {
                c2rust_current_block = 11812396948646013369;
            }
            match c2rust_current_block {
                12988308604321106300 => {}
                _ => {
                    if !(*item).string.is_null() {
                        (*newitem).string = if (*item).type_0 & cJSON_StringIsConst != 0 {
                            (*item).string
                        } else {
                            cJSON_strdup((*item).string as *mut ::core::ffi::c_uchar, &hooks)
                                as *mut ::core::ffi::c_char
                        };
                        if (*newitem).string.is_null() {
                            c2rust_current_block = 12988308604321106300;
                        } else {
                            c2rust_current_block = 12800627514080957624;
                        }
                    } else {
                        c2rust_current_block = 12800627514080957624;
                    }
                    match c2rust_current_block {
                        12988308604321106300 => {}
                        _ => {
                            if recurse == 0 {
                                return newitem;
                            }
                            child = (*item).child as *mut cJSON;
                            loop {
                                if child.is_null() {
                                    c2rust_current_block = 14763689060501151050;
                                    break;
                                }
                                if depth >= CJSON_CIRCULAR_LIMIT as size_t {
                                    c2rust_current_block = 12988308604321106300;
                                    break;
                                }
                                newchild = cJSON_Duplicate_rec(
                                    child,
                                    depth.wrapping_add(1 as size_t),
                                    true_0,
                                );
                                if newchild.is_null() {
                                    c2rust_current_block = 12988308604321106300;
                                    break;
                                }
                                if !next.is_null() {
                                    (*next).next = newchild as *mut cJSON;
                                    (*newchild).prev = next as *mut cJSON;
                                    next = newchild;
                                } else {
                                    (*newitem).child = newchild as *mut cJSON;
                                    next = newchild;
                                }
                                child = (*child).next as *mut cJSON;
                            }
                            match c2rust_current_block {
                                12988308604321106300 => {}
                                _ => {
                                    if !newitem.is_null() && !(*newitem).child.is_null() {
                                        (*(*newitem).child).prev = newchild as *mut cJSON;
                                    }
                                    return newitem;
                                }
                            }
                        }
                    }
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
    cJSON_Duplicate_rec(item, depth, recurse)
}
const CJSON_CHAR_NUL: ::core::ffi::c_char = b'\0' as ::core::ffi::c_char;
const CJSON_CHAR_NEWLINE: ::core::ffi::c_char = b'\n' as ::core::ffi::c_char;
const CJSON_CHAR_SPACE: ::core::ffi::c_char = b' ' as ::core::ffi::c_char;
const CJSON_CHAR_TAB: ::core::ffi::c_char = b'\t' as ::core::ffi::c_char;
const CJSON_CHAR_CARRIAGE_RETURN: ::core::ffi::c_char = b'\r' as ::core::ffi::c_char;
const CJSON_CHAR_SLASH: ::core::ffi::c_char = b'/' as ::core::ffi::c_char;
const CJSON_CHAR_STAR: ::core::ffi::c_char = b'*' as ::core::ffi::c_char;
const CJSON_CHAR_QUOTE: ::core::ffi::c_char = b'"' as ::core::ffi::c_char;
const CJSON_CHAR_BACKSLASH: ::core::ffi::c_char = b'\\' as ::core::ffi::c_char;

fn skip_oneline_comment(input: &mut usize, json: &[::core::ffi::c_char]) {
    *input += 2;
    while *input < json.len() && json[*input] != CJSON_CHAR_NUL {
        if json[*input] == CJSON_CHAR_NEWLINE {
            *input += 1;
            return;
        }
        *input += 1;
    }
}

fn skip_multiline_comment(input: &mut usize, json: &[::core::ffi::c_char]) {
    *input += 2;
    while *input < json.len() && json[*input] != CJSON_CHAR_NUL {
        if json[*input] == CJSON_CHAR_STAR
            && json.get(*input + 1).copied().unwrap_or(CJSON_CHAR_NUL) == CJSON_CHAR_SLASH
        {
            *input += 2;
            return;
        }
        *input += 1;
    }
}

fn minify_string(json: &mut [::core::ffi::c_char], input: &mut usize, output: &mut usize) {
    json[*output] = json[*input];
    *input += 1;
    *output += 1;
    while *input < json.len() && json[*input] != CJSON_CHAR_NUL {
        json[*output] = json[*input];
        if json[*input] == CJSON_CHAR_QUOTE {
            json[*output] = CJSON_CHAR_QUOTE;
            *input += 1;
            *output += 1;
            return;
        } else if json[*input] == CJSON_CHAR_BACKSLASH
            && json.get(*input + 1).copied().unwrap_or(CJSON_CHAR_NUL) == CJSON_CHAR_QUOTE
        {
            json[*output + 1] = json[*input + 1];
            *input += 1;
            *output += 1;
        }
        *input += 1;
        *output += 1;
    }
}

pub fn cJSON_Minify(json: Option<&mut [::core::ffi::c_char]>) {
    let Some(json) = json else {
        return;
    };
    let mut input = 0usize;
    let mut output = 0usize;
    while input < json.len() && json[input] != CJSON_CHAR_NUL {
        match json[input] {
            CJSON_CHAR_SPACE | CJSON_CHAR_TAB | CJSON_CHAR_CARRIAGE_RETURN | CJSON_CHAR_NEWLINE => {
                input += 1;
            }
            CJSON_CHAR_SLASH => {
                if json.get(input + 1).copied().unwrap_or(CJSON_CHAR_NUL) == CJSON_CHAR_SLASH {
                    skip_oneline_comment(&mut input, json);
                } else if json.get(input + 1).copied().unwrap_or(CJSON_CHAR_NUL) == CJSON_CHAR_STAR
                {
                    skip_multiline_comment(&mut input, json);
                } else {
                    input += 1;
                }
            }
            CJSON_CHAR_QUOTE => {
                minify_string(json, &mut input, &mut output);
            }
            _ => {
                json[output] = json[input];
                input += 1;
                output += 1;
            }
        }
    }
    if output < json.len() {
        json[output] = CJSON_CHAR_NUL;
    }
}
#[export_name = "cJSON_Minify"]
pub unsafe extern "C" fn cJSON_Minify_ffi(mut json: *mut ::core::ffi::c_char) {
    if json.is_null() {
        return;
    }
    let length = unsafe { strlen(json) }.wrapping_add(1);
    cJSON_Minify(Some(unsafe {
        ::core::slice::from_raw_parts_mut(json, length)
    }))
}
fn cJSON_has_type(item: Option<&cJSON>, type_0: ::core::ffi::c_int) -> cJSON_bool {
    item.map_or(false_0, |item| {
        (item.type_0 & 0xff as ::core::ffi::c_int == type_0) as ::core::ffi::c_int
    })
}
pub fn cJSON_IsInvalid(item: Option<&cJSON>) -> cJSON_bool {
    return cJSON_has_type(item, cJSON_Invalid);
}
#[export_name = "cJSON_IsInvalid"]
pub unsafe extern "C" fn cJSON_IsInvalid_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsInvalid(unsafe { item.as_ref() })
}
pub fn cJSON_IsFalse(item: Option<&cJSON>) -> cJSON_bool {
    return cJSON_has_type(item, cJSON_False);
}
#[export_name = "cJSON_IsFalse"]
pub unsafe extern "C" fn cJSON_IsFalse_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsFalse(unsafe { item.as_ref() })
}
pub fn cJSON_IsTrue(item: Option<&cJSON>) -> cJSON_bool {
    return cJSON_has_type(item, cJSON_True);
}
#[export_name = "cJSON_IsTrue"]
pub unsafe extern "C" fn cJSON_IsTrue_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsTrue(unsafe { item.as_ref() })
}
pub fn cJSON_IsBool(item: Option<&cJSON>) -> cJSON_bool {
    item.map_or(false_0, |item| {
        (item.type_0 & (cJSON_True | cJSON_False) != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
    })
}
#[export_name = "cJSON_IsBool"]
pub unsafe extern "C" fn cJSON_IsBool_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsBool(unsafe { item.as_ref() })
}
pub fn cJSON_IsNull(item: Option<&cJSON>) -> cJSON_bool {
    return cJSON_has_type(item, cJSON_NULL);
}
#[export_name = "cJSON_IsNull"]
pub unsafe extern "C" fn cJSON_IsNull_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsNull(unsafe { item.as_ref() })
}
pub fn cJSON_IsNumber(item: Option<&cJSON>) -> cJSON_bool {
    return cJSON_has_type(item, cJSON_Number);
}
#[export_name = "cJSON_IsNumber"]
pub unsafe extern "C" fn cJSON_IsNumber_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsNumber(unsafe { item.as_ref() })
}
pub fn cJSON_IsString(item: Option<&cJSON>) -> cJSON_bool {
    return cJSON_has_type(item, cJSON_String);
}
#[export_name = "cJSON_IsString"]
pub unsafe extern "C" fn cJSON_IsString_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsString(unsafe { item.as_ref() })
}
pub fn cJSON_IsArray(item: Option<&cJSON>) -> cJSON_bool {
    return cJSON_has_type(item, cJSON_Array);
}
#[export_name = "cJSON_IsArray"]
pub unsafe extern "C" fn cJSON_IsArray_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsArray(unsafe { item.as_ref() })
}
pub fn cJSON_IsObject(item: Option<&cJSON>) -> cJSON_bool {
    return cJSON_has_type(item, cJSON_Object);
}
#[export_name = "cJSON_IsObject"]
pub unsafe extern "C" fn cJSON_IsObject_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsObject(unsafe { item.as_ref() })
}
pub fn cJSON_IsRaw(item: Option<&cJSON>) -> cJSON_bool {
    return cJSON_has_type(item, cJSON_Raw);
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
    let a_ptr = a as *const cJSON;
    let b_ptr = b as *const cJSON;
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
            if unsafe { strcmp(a.valuestring, b.valuestring) } == 0 as ::core::ffi::c_int {
                return true_0;
            }
            return false_0;
        }
        cJSON_Array => {
            let mut a_element: *mut cJSON = a.child as *mut cJSON;
            let mut b_element: *mut cJSON = b.child as *mut cJSON;
            while !a_element.is_null() && !b_element.is_null() {
                if cJSON_Compare(
                    Some(unsafe { &*a_element }),
                    Some(unsafe { &*b_element }),
                    case_sensitive,
                ) == 0
                {
                    return false_0;
                }
                a_element = unsafe { (*a_element).next as *mut cJSON };
                b_element = unsafe { (*b_element).next as *mut cJSON };
            }
            if a_element != b_element {
                return false_0;
            }
            return true_0;
        }
        cJSON_Object => {
            let mut a_element_0: *mut cJSON = a.child as *mut cJSON;
            let mut b_element_0: *mut cJSON;
            while !a_element_0.is_null() {
                b_element_0 =
                    unsafe { get_object_item(b_ptr, (*a_element_0).string, case_sensitive) };
                if b_element_0.is_null() {
                    return false_0;
                }
                if cJSON_Compare(
                    Some(unsafe { &*a_element_0 }),
                    Some(unsafe { &*b_element_0 }),
                    case_sensitive,
                ) == 0
                {
                    return false_0;
                }
                a_element_0 = unsafe { (*a_element_0).next as *mut cJSON };
            }
            b_element_0 = b.child as *mut cJSON;
            while !b_element_0.is_null() {
                a_element_0 =
                    unsafe { get_object_item(a_ptr, (*b_element_0).string, case_sensitive) };
                if a_element_0.is_null() {
                    return false_0;
                }
                if cJSON_Compare(
                    Some(unsafe { &*b_element_0 }),
                    Some(unsafe { &*a_element_0 }),
                    case_sensitive,
                ) == 0
                {
                    return false_0;
                }
                b_element_0 = unsafe { (*b_element_0).next as *mut cJSON };
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
