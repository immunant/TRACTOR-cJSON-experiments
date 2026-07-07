extern "C" {
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
    fn strtod(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn tolower(__c: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
pub struct error {
    pub json: *const ::core::ffi::c_uchar,
    pub position: size_t,
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
static mut global_error: error = error {
    json: ::core::ptr::null::<::core::ffi::c_uchar>(),
    position: 0 as size_t,
};
#[export_name = "cJSON_GetErrorPtr"]
pub unsafe extern "C" fn cJSON_GetErrorPtr_ffi() -> *const ::core::ffi::c_char {
    return global_error.json.offset(global_error.position as isize) as *const ::core::ffi::c_char;
}
pub fn cJSON_GetStringValue(
    item: Option<&cJSON>,
) -> Option<::core::ptr::NonNull<::core::ffi::c_char>> {
    if cJSON_IsString(item) == 0 {
        return None;
    }
    return ::core::ptr::NonNull::new(item?.valuestring);
}
#[export_name = "cJSON_GetStringValue"]
pub unsafe extern "C" fn cJSON_GetStringValue_ffi(item: *const cJSON) -> *mut ::core::ffi::c_char {
    match cJSON_GetStringValue(item.as_ref()) {
        Some(value) => value.as_ptr(),
        None => ::core::ptr::null_mut::<::core::ffi::c_char>(),
    }
}
pub fn cJSON_GetNumberValue(item: Option<&cJSON>) -> ::core::ffi::c_double {
    if cJSON_IsNumber(item) == 0 {
        return 0.0f64 / 0.0f64;
    }
    return item.expect("number item checked above").valuedouble;
}
#[export_name = "cJSON_GetNumberValue"]
pub unsafe extern "C" fn cJSON_GetNumberValue_ffi(item: *const cJSON) -> ::core::ffi::c_double {
    cJSON_GetNumberValue(item.as_ref())
}
pub fn cJSON_Version() -> &'static [::core::ffi::c_char] {
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
unsafe extern "C" fn case_insensitive_strcmp(
    string1: *const ::core::ffi::c_uchar,
    string2: *const ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    if string1.is_null() || string2.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    if string1 == string2 {
        return 0 as ::core::ffi::c_int;
    }
    let string1_bytes =
        ::core::ffi::CStr::from_ptr(string1 as *const ::core::ffi::c_char).to_bytes_with_nul();
    let string2_bytes =
        ::core::ffi::CStr::from_ptr(string2 as *const ::core::ffi::c_char).to_bytes_with_nul();
    let mut index: size_t = 0;
    loop {
        let lower1 = tolower(string1_bytes[index] as ::core::ffi::c_int);
        let lower2 = tolower(string2_bytes[index] as ::core::ffi::c_int);
        if lower1 != lower2 {
            return lower1 - lower2;
        }
        if string1_bytes[index] as ::core::ffi::c_int == '\0' as i32 {
            return 0 as ::core::ffi::c_int;
        }
        index = index.wrapping_add(1);
    }
}
static mut global_hooks: internal_hooks = internal_hooks {
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
unsafe fn cJSON_strdup(
    string: *const ::core::ffi::c_uchar,
    hooks: &internal_hooks,
) -> *mut ::core::ffi::c_uchar {
    let bytes: &[::core::ffi::c_uchar];
    let mut length: size_t = 0 as size_t;
    let mut copy: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if string.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    bytes = ::core::ffi::CStr::from_ptr(string as *const ::core::ffi::c_char)
        .to_bytes_with_nul();
    length = bytes.len();
    copy = hooks.allocate.expect("non-null function pointer")(length) as *mut ::core::ffi::c_uchar;
    if copy.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    ::core::ptr::copy_nonoverlapping(bytes.as_ptr(), copy, length);
    return copy;
}
#[export_name = "cJSON_InitHooks"]
pub unsafe extern "C" fn cJSON_InitHooks_ffi(mut hooks: *mut cJSON_Hooks) {
    let mut new_hooks = internal_hooks {
        allocate: Some(malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void),
        deallocate: Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        reallocate: None,
    };
    if hooks.is_null() {
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
        global_hooks = new_hooks;
        return;
    }
    let hooks_ref = &*hooks;
    if hooks_ref.malloc_fn.is_some() {
        new_hooks.allocate = hooks_ref.malloc_fn;
    }
    if hooks_ref.free_fn.is_some() {
        new_hooks.deallocate = hooks_ref.free_fn;
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
    global_hooks = new_hooks;
}
unsafe fn cJSON_New_Item(hooks: &internal_hooks) -> *mut cJSON {
    let mut node: *mut cJSON = hooks.allocate.expect("non-null function pointer")(
        ::core::mem::size_of::<cJSON>() as size_t,
    ) as *mut cJSON;
    if !node.is_null() {
        ::core::ptr::write(
            node,
            cJSON {
                next: ::core::ptr::null_mut::<cJSON>(),
                prev: ::core::ptr::null_mut::<cJSON>(),
                child: ::core::ptr::null_mut::<cJSON>(),
                type_0: 0,
                valuestring: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                valueint: 0,
                valuedouble: 0.0f64,
                string: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
        );
    }
    return node;
}
#[export_name = "cJSON_Delete"]
pub unsafe extern "C" fn cJSON_Delete_ffi(mut item: *mut cJSON) {
    let mut next: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    while !item.is_null() {
        let item_ref = &mut *item;
        next = item_ref.next as *mut cJSON;
        if item_ref.type_0 & cJSON_IsReference == 0 && !item_ref.child.is_null() {
            cJSON_Delete_ffi(item_ref.child as *mut cJSON);
        }
        if item_ref.type_0 & cJSON_IsReference == 0 && !item_ref.valuestring.is_null() {
            global_hooks.deallocate.expect("non-null function pointer")(
                item_ref.valuestring as *mut ::core::ffi::c_void,
            );
            item_ref.valuestring = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if item_ref.type_0 & cJSON_StringIsConst == 0 && !item_ref.string.is_null() {
            global_hooks.deallocate.expect("non-null function pointer")(
                item_ref.string as *mut ::core::ffi::c_void,
            );
            item_ref.string = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        global_hooks.deallocate.expect("non-null function pointer")(
            item as *mut ::core::ffi::c_void,
        );
        item = next;
    }
}
unsafe extern "C" fn get_decimal_point() -> ::core::ffi::c_uchar {
    let lconv_ref = &*localeconv();
    return *lconv_ref.decimal_point as ::core::ffi::c_uchar;
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
    if input_buffer.is_null() {
        return false_0;
    }
    let input_buffer_ref = &mut *input_buffer;
    if input_buffer_ref.content.is_null() {
        return false_0;
    }
    let content = ::core::slice::from_raw_parts(input_buffer_ref.content, input_buffer_ref.length);
    i = 0 as size_t;
    while let Some(byte) = content
        .get(input_buffer_ref.offset.wrapping_add(i))
        .copied()
    {
        match byte {
            b'0'..=b'9' | b'+' | b'-' | b'e' | b'E' => {
                number_string_length = number_string_length.wrapping_add(1);
            }
            b'.' => {
                number_string_length = number_string_length.wrapping_add(1);
                has_decimal_point = true_0;
            }
            _ => {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    number_c_string = input_buffer_ref
        .hooks
        .allocate
        .expect("non-null function pointer")(
        number_string_length.wrapping_add(1 as size_t)
    ) as *mut ::core::ffi::c_uchar;
    if number_c_string.is_null() {
        return false_0;
    }
    let number_bytes = ::core::slice::from_raw_parts_mut(
        number_c_string,
        number_string_length.wrapping_add(1 as size_t),
    );
    let Some(source_number) = content.get(
        input_buffer_ref.offset..input_buffer_ref.offset.wrapping_add(number_string_length),
    ) else {
        input_buffer_ref
            .hooks
            .deallocate
            .expect("non-null function pointer")(number_c_string as *mut ::core::ffi::c_void);
        return false_0;
    };
    number_bytes[..number_string_length].copy_from_slice(source_number);
    number_bytes[number_string_length] = '\0' as i32 as ::core::ffi::c_uchar;
    if has_decimal_point != 0 {
        for byte in &mut number_bytes[..number_string_length] {
            if *byte == b'.' {
                *byte = decimal_point;
            }
        }
    }
    number = strtod(
        number_c_string as *const ::core::ffi::c_char,
        &raw mut after_end as *mut *mut ::core::ffi::c_char,
    );
    if number_c_string == after_end {
        input_buffer_ref
            .hooks
            .deallocate
            .expect("non-null function pointer")(
            number_c_string as *mut ::core::ffi::c_void
        );
        return false_0;
    }
    let item_ref = &mut *item;
    cJSON_SetNumberHelper(item_ref, number);
    item_ref.type_0 = cJSON_Number;
    input_buffer_ref.offset = input_buffer_ref
        .offset
        .wrapping_add(after_end.offset_from(number_c_string) as ::core::ffi::c_long as size_t);
    input_buffer_ref
        .hooks
        .deallocate
        .expect("non-null function pointer")(number_c_string as *mut ::core::ffi::c_void);
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
#[export_name = "cJSON_SetValuestring"]
pub unsafe extern "C" fn cJSON_SetValuestring_ffi(
    mut object: *mut cJSON,
    mut valuestring: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut copy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if object.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let object_ref = &mut *object;
    if object_ref.type_0 & cJSON_String == 0 || object_ref.type_0 & cJSON_IsReference != 0 {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if object_ref.valuestring.is_null() || valuestring.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let input_bytes = ::core::ffi::CStr::from_ptr(valuestring).to_bytes_with_nul();
    let v1_len = input_bytes.len().wrapping_sub(1 as size_t);
    let v2_len = ::core::ffi::CStr::from_ptr(object_ref.valuestring).to_bytes().len();
    if v1_len <= v2_len {
        let object_valuestring = object_ref.valuestring;
        if !(valuestring.wrapping_add(v1_len)
            < object_valuestring as *const ::core::ffi::c_char
            || (object_valuestring as *const ::core::ffi::c_char).wrapping_add(v2_len)
                < valuestring)
        {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        let output_slice =
            ::core::slice::from_raw_parts_mut(object_valuestring as *mut ::core::ffi::c_uchar, input_bytes.len());
        output_slice.copy_from_slice(input_bytes);
        return object_ref.valuestring;
    }
    let hooks = global_hooks;
    copy = cJSON_strdup(valuestring as *const ::core::ffi::c_uchar, &hooks)
        as *mut ::core::ffi::c_char;
    if copy.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !object_ref.valuestring.is_null() {
        cJSON_free_ffi(object_ref.valuestring as *mut ::core::ffi::c_void);
    }
    object_ref.valuestring = copy;
    return copy;
}
unsafe fn ensure(p: &mut printbuffer, mut needed: size_t) -> Option<&mut [::core::ffi::c_uchar]> {
    let mut newbuffer: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut newsize: size_t = 0 as size_t;
    let requested = needed;
    if p.buffer.is_null() {
        return None;
    }
    if p.length > 0 as size_t && p.offset >= p.length {
        return None;
    }
    if needed > INT_MAX as size_t {
        return None;
    }
    needed = needed.wrapping_add(p.offset.wrapping_add(1 as size_t));
    if needed <= p.length {
        return Some(::core::slice::from_raw_parts_mut(
            p.buffer.wrapping_add(p.offset),
            requested,
        ));
    }
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
    if p.hooks.reallocate.is_some() {
        newbuffer = p.hooks.reallocate.expect("non-null function pointer")(
            p.buffer as *mut ::core::ffi::c_void,
            newsize,
        ) as *mut ::core::ffi::c_uchar;
        if newbuffer.is_null() {
            p.hooks.deallocate.expect("non-null function pointer")(
                p.buffer as *mut ::core::ffi::c_void,
            );
            p.length = 0 as size_t;
            p.buffer = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
            return None;
        }
    } else {
        newbuffer = p.hooks.allocate.expect("non-null function pointer")(newsize)
            as *mut ::core::ffi::c_uchar;
        if newbuffer.is_null() {
            p.hooks.deallocate.expect("non-null function pointer")(
                p.buffer as *mut ::core::ffi::c_void,
            );
            p.length = 0 as size_t;
            p.buffer = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
            return None;
        }
        ::core::ptr::copy_nonoverlapping(
            p.buffer,
            newbuffer,
            p.offset.wrapping_add(1 as size_t),
        );
        p.hooks.deallocate.expect("non-null function pointer")(
            p.buffer as *mut ::core::ffi::c_void,
        );
    }
    p.length = newsize;
    p.buffer = newbuffer;
    return Some(::core::slice::from_raw_parts_mut(
        newbuffer.wrapping_add(p.offset),
        requested,
    ));
}
unsafe fn update_offset(buffer: &mut printbuffer) {
    if buffer.buffer.is_null() {
        return;
    }
    if buffer.length <= buffer.offset {
        return;
    }
    let remaining = buffer.length.wrapping_sub(buffer.offset);
    let bytes = ::core::slice::from_raw_parts(buffer.buffer.wrapping_add(buffer.offset), remaining);
    if let Some(written) = bytes.iter().position(|byte| *byte == 0) {
        buffer.offset = buffer.offset.wrapping_add(written);
    } else {
        buffer.offset = buffer.length;
    }
}
fn compare_double(
    mut a: ::core::ffi::c_double,
    mut b: ::core::ffi::c_double,
) -> cJSON_bool {
    let mut maxVal: ::core::ffi::c_double = if a.abs() > b.abs() { a.abs() } else { b.abs() };
    return ((a - b).abs() <= maxVal * DBL_EPSILON) as ::core::ffi::c_int;
}
unsafe fn print_number(
    item: &cJSON,
    output_buffer: &mut printbuffer,
) -> cJSON_bool {
    let mut d: ::core::ffi::c_double = item.valuedouble;
    let mut length: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
    if d != d || d - d != d - d && !(d != d) {
        length = sprintf(
            &raw mut number_buffer as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
            b"null\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else if d == item.valueint as ::core::ffi::c_double {
        length = sprintf(
            &raw mut number_buffer as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
            b"%d\0".as_ptr() as *const ::core::ffi::c_char,
            item.valueint,
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
    let output_length = length as size_t;
    let Some(output_slice) = ensure(
        output_buffer,
        output_length.wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 1]>() as size_t),
    ) else {
        return false_0;
    };
    for (index, output_byte) in output_slice[..output_length].iter_mut().enumerate() {
        if number_buffer[index] as ::core::ffi::c_int == decimal_point as ::core::ffi::c_int {
            *output_byte = '.' as i32 as ::core::ffi::c_uchar;
        } else {
            *output_byte = number_buffer[index];
        }
    }
    output_slice[output_length] = '\0' as i32 as ::core::ffi::c_uchar;
    output_buffer.offset = output_buffer.offset.wrapping_add(output_length);
    return true_0;
}
fn parse_hex4(input: &[::core::ffi::c_uchar]) -> ::core::ffi::c_uint {
    let mut h: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut i: size_t = 0 as size_t;
    i = 0 as size_t;
    while i < 4 as size_t {
        if input[i as usize] as ::core::ffi::c_int >= '0' as i32
            && input[i as usize] as ::core::ffi::c_int <= '9' as i32
        {
            h = h.wrapping_add(
                (input[i as usize] as ::core::ffi::c_uint)
                    .wrapping_sub('0' as i32 as ::core::ffi::c_uint),
            );
        } else if input[i as usize] as ::core::ffi::c_int >= 'A' as i32
            && input[i as usize] as ::core::ffi::c_int <= 'F' as i32
        {
            h = h.wrapping_add(
                (10 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_add(input[i as usize] as ::core::ffi::c_uint)
                    .wrapping_sub('A' as i32 as ::core::ffi::c_uint),
            );
        } else if input[i as usize] as ::core::ffi::c_int >= 'a' as i32
            && input[i as usize] as ::core::ffi::c_int <= 'f' as i32
        {
            h = h.wrapping_add(
                (10 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_add(input[i as usize] as ::core::ffi::c_uint)
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
fn utf8_bytes_from_codepoint(
    mut codepoint: ::core::ffi::c_ulong,
) -> Option<([::core::ffi::c_uchar; 4], ::core::ffi::c_uchar)> {
    let utf8_length: ::core::ffi::c_uchar;
    let first_byte_mark: ::core::ffi::c_uchar;
    if codepoint < 0x80 as ::core::ffi::c_ulong {
        utf8_length = 1 as ::core::ffi::c_uchar;
        first_byte_mark = 0 as ::core::ffi::c_uchar;
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

    let mut utf8 = [0 as ::core::ffi::c_uchar; 4];
    let mut utf8_position =
        (utf8_length as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    while utf8_position as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        utf8[utf8_position as usize] =
            ((codepoint | 0x80 as ::core::ffi::c_ulong) & 0xbf as ::core::ffi::c_ulong)
                as ::core::ffi::c_uchar;
        codepoint >>= 6 as ::core::ffi::c_int;
        utf8_position = utf8_position.wrapping_sub(1);
    }
    if utf8_length as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
        utf8[0] = ((codepoint | first_byte_mark as ::core::ffi::c_ulong)
            & 0xff as ::core::ffi::c_ulong) as ::core::ffi::c_uchar;
    } else {
        utf8[0] = (codepoint & 0x7f as ::core::ffi::c_ulong) as ::core::ffi::c_uchar;
    }
    return Some((utf8, utf8_length));
}
fn utf16_literal_to_utf8(
    input: &[::core::ffi::c_uchar],
    output: &mut [::core::ffi::c_uchar],
) -> Option<(::core::ffi::c_uchar, size_t)> {
    let mut c2rust_current_block: u64;
    let mut codepoint: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    let mut first_code: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut sequence_length: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
    if input.len() >= 6 {
        first_code = parse_hex4(&input[2..6]);
        if !(first_code >= 0xdc00 as ::core::ffi::c_uint
            && first_code <= 0xdfff as ::core::ffi::c_uint)
        {
            if first_code >= 0xd800 as ::core::ffi::c_uint
                && first_code <= 0xdbff as ::core::ffi::c_uint
            {
                let mut second_code: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                sequence_length = 12 as ::core::ffi::c_uchar;
                if input.len() < 12 {
                    c2rust_current_block = 2136517548508416331;
                } else if input[6] != b'\\' || input[7] != b'u'
                {
                    c2rust_current_block = 2136517548508416331;
                } else {
                    second_code = parse_hex4(&input[8..12]);
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
                    if let Some((utf8, utf8_length)) = utf8_bytes_from_codepoint(codepoint) {
                        if output.len() < utf8_length as usize {
                            return None;
                        }
                        output[..utf8_length as usize]
                            .copy_from_slice(&utf8[..utf8_length as usize]);
                        return Some((sequence_length, utf8_length as size_t));
                    }
                }
            }
        }
    }
    return None;
}
unsafe extern "C" fn parse_string(item: *mut cJSON, input_buffer: *mut parse_buffer) -> cJSON_bool {
    let mut c2rust_current_block: u64;
    if input_buffer.is_null() {
        return false_0;
    }
    let input_buffer_ref = &mut *input_buffer;
    if input_buffer_ref.content.is_null() {
        return false_0;
    }
    let content = ::core::slice::from_raw_parts(input_buffer_ref.content, input_buffer_ref.length);
    let mut input_index = input_buffer_ref.offset.wrapping_add(1 as size_t);
    let mut input_end_index = input_index;
    let mut output: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if content.get(input_buffer_ref.offset).copied() == Some(b'"') {
        let mut allocation_length: size_t = 0 as size_t;
        let mut skipped_bytes: size_t = 0 as size_t;
        loop {
            if !(input_end_index < input_buffer_ref.length
                && content.get(input_end_index).copied() != Some(b'"'))
            {
                c2rust_current_block = 11812396948646013369;
                break;
            }
            if content.get(input_end_index).copied() == Some(b'\\') {
                if input_end_index.wrapping_add(1 as size_t) >= input_buffer_ref.length {
                    c2rust_current_block = 4600858903266057594;
                    break;
                }
                skipped_bytes = skipped_bytes.wrapping_add(1);
                input_end_index = input_end_index.wrapping_add(1);
            }
            input_end_index = input_end_index.wrapping_add(1);
        }
        match c2rust_current_block {
            4600858903266057594 => {}
            _ => {
                if input_end_index < input_buffer_ref.length
                    && content.get(input_end_index).copied() == Some(b'"')
                {
                    allocation_length =
                        input_end_index.wrapping_sub(input_buffer_ref.offset).wrapping_sub(skipped_bytes);
                    output = input_buffer_ref
                        .hooks
                        .allocate
                        .expect("non-null function pointer")(
                        allocation_length.wrapping_add(::core::mem::size_of::<
                            [::core::ffi::c_char; 1],
                        >() as size_t),
                    ) as *mut ::core::ffi::c_uchar;
                    if !output.is_null() {
                        let output_slice = ::core::slice::from_raw_parts_mut(
                            output,
                            allocation_length
                                .wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 1]>()
                                    as size_t),
                        );
                        let mut output_index: size_t = 0 as size_t;
                        loop {
                            if !(input_index < input_end_index) {
                                c2rust_current_block = 7828949454673616476;
                                break;
                            }
                            if content.get(input_index).copied() != Some(b'\\') {
                                output_slice[output_index] = content[input_index];
                                input_index = input_index.wrapping_add(1);
                                output_index = output_index.wrapping_add(1);
                            } else {
                                let mut sequence_length: ::core::ffi::c_uchar =
                                    2 as ::core::ffi::c_uchar;
                                if input_end_index.wrapping_sub(input_index) < 1 as size_t {
                                    c2rust_current_block = 4600858903266057594;
                                    break;
                                }
                                match content[input_index.wrapping_add(1 as size_t)]
                                    as ::core::ffi::c_int
                                {
                                    98 => {
                                        output_slice[output_index] =
                                            '\u{8}' as i32 as ::core::ffi::c_uchar;
                                        output_index = output_index.wrapping_add(1);
                                    }
                                    102 => {
                                        output_slice[output_index] =
                                            '\u{c}' as i32 as ::core::ffi::c_uchar;
                                        output_index = output_index.wrapping_add(1);
                                    }
                                    110 => {
                                        output_slice[output_index] =
                                            '\n' as i32 as ::core::ffi::c_uchar;
                                        output_index = output_index.wrapping_add(1);
                                    }
                                    114 => {
                                        output_slice[output_index] =
                                            '\r' as i32 as ::core::ffi::c_uchar;
                                        output_index = output_index.wrapping_add(1);
                                    }
                                    116 => {
                                        output_slice[output_index] =
                                            '\t' as i32 as ::core::ffi::c_uchar;
                                        output_index = output_index.wrapping_add(1);
                                    }
                                    34 | 92 | 47 => {
                                        output_slice[output_index] =
                                            content[input_index.wrapping_add(1 as size_t)];
                                        output_index = output_index.wrapping_add(1);
                                    }
                                    117 => {
                                        match utf16_literal_to_utf8(
                                            &content[input_index..input_end_index],
                                            &mut output_slice[output_index..],
                                        ) {
                                            Some((input_length, utf8_length)) => {
                                                sequence_length = input_length;
                                                output_index =
                                                    output_index.wrapping_add(utf8_length);
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
                                input_index = input_index.wrapping_add(sequence_length as size_t);
                            }
                        }
                        match c2rust_current_block {
                            4600858903266057594 => {}
                            _ => {
                                output_slice[output_index] =
                                    '\0' as i32 as ::core::ffi::c_uchar;
                                let item_ref = &mut *item;
                                item_ref.type_0 = cJSON_String;
                                item_ref.valuestring = output as *mut ::core::ffi::c_char;
                                input_buffer_ref.offset = input_end_index;
                                input_buffer_ref.offset = input_buffer_ref.offset.wrapping_add(1);
                                return true_0;
                            }
                        }
                    }
                }
            }
        }
    }
    if !output.is_null() {
        input_buffer_ref
            .hooks
            .deallocate
            .expect("non-null function pointer")(output as *mut ::core::ffi::c_void);
        output = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    input_buffer_ref.offset = input_index;
    return false_0;
}
unsafe extern "C" fn print_string_ptr(
    input: *const ::core::ffi::c_uchar,
    output_buffer: &mut printbuffer,
) -> cJSON_bool {
    let mut output_length: size_t = 0 as size_t;
    let mut escape_characters: size_t = 0 as size_t;
    if input.is_null() {
        let Some(output_slice) = ensure(
            output_buffer,
            ::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t,
        ) else {
            return false_0;
        };
        output_slice.copy_from_slice(b"\"\"\0");
        return true_0;
    }
    let input_bytes =
        ::core::ffi::CStr::from_ptr(input as *const ::core::ffi::c_char).to_bytes();
    for byte in input_bytes {
        match *byte {
            b'"' | b'\\' | b'\x08' | b'\x0c' | b'\n' | b'\r' | b'\t' => {
                escape_characters = escape_characters.wrapping_add(1);
            }
            _ => {
                if *byte < 32 {
                    escape_characters = escape_characters.wrapping_add(5 as size_t);
                }
            }
        }
    }
    output_length = input_bytes.len().wrapping_add(escape_characters);
    let Some(output_slice) = ensure(
        output_buffer,
        output_length.wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t),
    ) else {
        return false_0;
    };
    if escape_characters == 0 as size_t {
        output_slice[0] = b'"';
        output_slice[1..1 + input_bytes.len()].copy_from_slice(input_bytes);
        output_slice[output_length.wrapping_add(1 as size_t)] = b'"';
        output_slice[output_length.wrapping_add(2 as size_t)] = b'\0';
        return true_0;
    }
    output_slice[0] = b'"';
    let mut output_index = 1 as size_t;
    for byte in input_bytes {
        if *byte > 31 && *byte != b'"' && *byte != b'\\' {
            output_slice[output_index] = *byte;
        } else {
            output_slice[output_index] = b'\\';
            output_index = output_index.wrapping_add(1);
            match *byte {
                b'\\' => output_slice[output_index] = b'\\',
                b'"' => output_slice[output_index] = b'"',
                b'\x08' => output_slice[output_index] = b'b',
                b'\x0c' => output_slice[output_index] = b'f',
                b'\n' => output_slice[output_index] = b'n',
                b'\r' => output_slice[output_index] = b'r',
                b'\t' => output_slice[output_index] = b't',
                _ => {
                    output_slice[output_index] = b'u';
                    output_slice[output_index.wrapping_add(1)] = b'0';
                    output_slice[output_index.wrapping_add(2)] = b'0';
                    output_slice[output_index.wrapping_add(3)] = hex_digit(byte >> 4);
                    output_slice[output_index.wrapping_add(4)] = hex_digit(byte & 0x0f);
                    output_index = output_index.wrapping_add(4);
                }
            }
        }
        output_index = output_index.wrapping_add(1);
    }
    output_slice[output_length.wrapping_add(1 as size_t)] = b'"';
    output_slice[output_length.wrapping_add(2 as size_t)] = b'\0';
    return true_0;
}
fn hex_digit(value: ::core::ffi::c_uchar) -> ::core::ffi::c_uchar {
    match value {
        0..=9 => b'0' + value,
        _ => b'a' + value - 10,
    }
}
fn buffer_skip_whitespace_ref<'a>(
    buffer_ref: &'a mut parse_buffer,
    content: &[::core::ffi::c_uchar],
) -> &'a mut parse_buffer {
    if !(buffer_ref.offset.wrapping_add(0 as size_t) < buffer_ref.length) {
        return buffer_ref;
    }
    while buffer_ref.offset.wrapping_add(0 as size_t) < buffer_ref.length
        && content
            .get(buffer_ref.offset)
            .copied()
            .unwrap_or(33 as ::core::ffi::c_uchar) as ::core::ffi::c_int
            <= 32 as ::core::ffi::c_int
    {
        buffer_ref.offset = buffer_ref.offset.wrapping_add(1);
    }
    if buffer_ref.offset == buffer_ref.length {
        buffer_ref.offset = buffer_ref.offset.wrapping_sub(1);
    }
    return buffer_ref;
}
fn skip_utf8_bom<'a>(
    buffer: &'a mut parse_buffer,
    content: &[::core::ffi::c_uchar],
) -> Option<&'a mut parse_buffer> {
    if buffer.content.is_null() || buffer.offset != 0 as size_t {
        return None;
    }
    if buffer.offset.wrapping_add(4 as size_t) < buffer.length
        && content.get(buffer.offset..buffer.offset.wrapping_add(3 as size_t))
            == Some(b"\xEF\xBB\xBF".as_slice())
    {
        buffer.offset = buffer.offset.wrapping_add(3 as size_t);
    }
    return Some(buffer);
}
#[export_name = "cJSON_ParseWithOpts"]
pub unsafe extern "C" fn cJSON_ParseWithOpts_ffi(
    mut value: *const ::core::ffi::c_char,
    mut return_parse_end: *mut *const ::core::ffi::c_char,
    mut require_null_terminated: cJSON_bool,
) -> *mut cJSON {
    let mut buffer_length: size_t = 0;
    if value.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    buffer_length = ::core::ffi::CStr::from_ptr(value).to_bytes_with_nul().len();
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
    global_error.json = ::core::ptr::null::<::core::ffi::c_uchar>();
    global_error.position = 0 as size_t;
    if !(value.is_null() || 0 as size_t == buffer_length) {
        buffer.content = value as *const ::core::ffi::c_uchar;
        buffer.length = buffer_length;
        buffer.offset = 0 as size_t;
        buffer.hooks = global_hooks;
        item = cJSON_New_Item(&buffer.hooks);
        if !item.is_null() {
            let input_slice =
                ::core::slice::from_raw_parts(value as *const ::core::ffi::c_uchar, buffer_length);
            let parsed = match skip_utf8_bom(&mut buffer, input_slice) {
                Some(buffer_ref) => {
                    buffer_skip_whitespace_ref(buffer_ref, input_slice);
                    parse_value(&mut *item, &mut buffer)
                }
                None => false_0,
            };
            if !(parsed == 0) {
                if require_null_terminated != 0 {
                    buffer_skip_whitespace_ref(&mut buffer, input_slice);
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
        cJSON_Delete_ffi(item);
    }
    if !value.is_null() {
        let mut local_error: error = error {
            json: ::core::ptr::null::<::core::ffi::c_uchar>(),
            position: 0,
        };
        local_error.json = value as *const ::core::ffi::c_uchar;
        local_error.position = 0 as size_t;
        if buffer.offset < buffer.length {
            local_error.position = buffer.offset;
        } else if buffer.length > 0 as size_t {
            local_error.position = buffer.length.wrapping_sub(1 as size_t);
        }
        if !return_parse_end.is_null() {
            *return_parse_end = (local_error.json as *const ::core::ffi::c_char)
                .offset(local_error.position as isize);
        }
        global_error = local_error;
    }
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_Parse"]
pub unsafe extern "C" fn cJSON_Parse_ffi(mut value: *const ::core::ffi::c_char) -> *mut cJSON {
    return cJSON_ParseWithOpts_ffi(
        value,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        0 as cJSON_bool,
    );
}
#[export_name = "cJSON_ParseWithLength"]
pub unsafe extern "C" fn cJSON_ParseWithLength_ffi(
    mut value: *const ::core::ffi::c_char,
    mut buffer_length: size_t,
) -> *mut cJSON {
    return cJSON_ParseWithLengthOpts_ffi(
        value,
        buffer_length,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        0 as cJSON_bool,
    );
}
unsafe fn print(
    item: Option<&cJSON>,
    mut format: cJSON_bool,
    hooks_ref: &internal_hooks,
) -> *mut ::core::ffi::c_uchar {
    let mut c2rust_current_block: u64;
    let default_buffer_size: size_t = 256 as size_t;
    let mut buffer = printbuffer {
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
    };
    let mut printed: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    buffer.buffer = hooks_ref.allocate.expect("non-null function pointer")(default_buffer_size)
        as *mut ::core::ffi::c_uchar;
    buffer.length = default_buffer_size;
    buffer.format = format;
    buffer.hooks = *hooks_ref;
    if !buffer.buffer.is_null() {
        if let Some(item_ref) = item {
            if print_value(item_ref, &mut buffer) != 0 {
            update_offset(&mut buffer);
            if hooks_ref.reallocate.is_some() {
                printed = hooks_ref.reallocate.expect("non-null function pointer")(
                    buffer.buffer as *mut ::core::ffi::c_void,
                    buffer.offset.wrapping_add(1 as size_t),
                ) as *mut ::core::ffi::c_uchar;
                if printed.is_null() {
                    c2rust_current_block = 15492938347856135346;
                } else {
                    buffer.buffer = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                    c2rust_current_block = 7149356873433890176;
                }
            } else {
                printed = hooks_ref.allocate.expect("non-null function pointer")(
                    buffer.offset.wrapping_add(1 as size_t)
                ) as *mut ::core::ffi::c_uchar;
                if printed.is_null() {
                    c2rust_current_block = 15492938347856135346;
                } else {
                    let copy_length = if buffer.length < buffer.offset.wrapping_add(1 as size_t) {
                        buffer.length
                    } else {
                        buffer.offset.wrapping_add(1 as size_t)
                    };
                    ::core::ptr::copy_nonoverlapping(
                        buffer.buffer,
                        printed,
                        copy_length,
                    );
                    let printed_slice = ::core::slice::from_raw_parts_mut(
                        printed,
                        buffer.offset.wrapping_add(1 as size_t),
                    );
                    printed_slice[buffer.offset] = '\0' as i32 as ::core::ffi::c_uchar;
                    hooks_ref.deallocate.expect("non-null function pointer")(
                        buffer.buffer as *mut ::core::ffi::c_void,
                    );
                    buffer.buffer = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                    c2rust_current_block = 7149356873433890176;
                }
            }
            match c2rust_current_block {
                15492938347856135346 => {}
                _ => return printed,
            }
        }
        }
    }
    if !buffer.buffer.is_null() {
        hooks_ref.deallocate.expect("non-null function pointer")(
            buffer.buffer as *mut ::core::ffi::c_void,
        );
        buffer.buffer = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    if !printed.is_null() {
        hooks_ref.deallocate.expect("non-null function pointer")(
            printed as *mut ::core::ffi::c_void,
        );
        printed = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
}
#[export_name = "cJSON_Print"]
pub unsafe extern "C" fn cJSON_Print_ffi(mut item: *const cJSON) -> *mut ::core::ffi::c_char {
    let hooks = global_hooks;
    return print(item.as_ref(), true_0, &hooks) as *mut ::core::ffi::c_char;
}
#[export_name = "cJSON_PrintUnformatted"]
pub unsafe extern "C" fn cJSON_PrintUnformatted_ffi(
    mut item: *const cJSON,
) -> *mut ::core::ffi::c_char {
    let hooks = global_hooks;
    return print(item.as_ref(), false_0, &hooks) as *mut ::core::ffi::c_char;
}
#[export_name = "cJSON_PrintBuffered"]
pub unsafe extern "C" fn cJSON_PrintBuffered_ffi(
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
    p.buffer = global_hooks.allocate.expect("non-null function pointer")(prebuffer as size_t)
        as *mut ::core::ffi::c_uchar;
    if p.buffer.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    p.length = prebuffer as size_t;
    p.offset = 0 as size_t;
    p.noalloc = false_0;
    p.format = fmt;
    p.hooks = global_hooks;
    if item.is_null() || print_value(&*item, &mut p) == 0 {
        global_hooks.deallocate.expect("non-null function pointer")(
            p.buffer as *mut ::core::ffi::c_void,
        );
        p.buffer = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return p.buffer as *mut ::core::ffi::c_char;
}
#[export_name = "cJSON_PrintPreallocated"]
pub unsafe extern "C" fn cJSON_PrintPreallocated_ffi(
    mut item: *mut cJSON,
    mut buffer: *mut ::core::ffi::c_char,
    length: ::core::ffi::c_int,
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
    if length < 0 as ::core::ffi::c_int || buffer.is_null() {
        return false_0;
    }
    p.buffer = buffer as *mut ::core::ffi::c_uchar;
    p.length = length as size_t;
    p.offset = 0 as size_t;
    p.noalloc = true_0;
    p.format = format;
    p.hooks = global_hooks;
    if item.is_null() {
        return false_0;
    }
    return print_value(&*item, &mut p);
}
unsafe fn parse_value(item_ref: &mut cJSON, buffer_ref: &mut parse_buffer) -> cJSON_bool {
    if buffer_ref.content.is_null() {
        return false_0;
    }
    let content = ::core::slice::from_raw_parts(buffer_ref.content, buffer_ref.length);
    if content.get(buffer_ref.offset..buffer_ref.offset.wrapping_add(4 as size_t))
        == Some(b"null".as_slice())
    {
        item_ref.type_0 = cJSON_NULL;
        buffer_ref.offset = buffer_ref.offset.wrapping_add(4 as size_t);
        return true_0;
    }
    if content.get(buffer_ref.offset..buffer_ref.offset.wrapping_add(5 as size_t))
        == Some(b"false".as_slice())
    {
        item_ref.type_0 = cJSON_False;
        buffer_ref.offset = buffer_ref.offset.wrapping_add(5 as size_t);
        return true_0;
    }
    if content.get(buffer_ref.offset..buffer_ref.offset.wrapping_add(4 as size_t))
        == Some(b"true".as_slice())
    {
        item_ref.type_0 = cJSON_True;
        item_ref.valueint = 1 as ::core::ffi::c_int;
        buffer_ref.offset = buffer_ref.offset.wrapping_add(4 as size_t);
        return true_0;
    }
    match content.get(buffer_ref.offset).copied() {
        Some(b'"') => return parse_string(item_ref as *mut cJSON, buffer_ref as *mut parse_buffer),
        Some(byte) if byte == b'-' || byte >= b'0' && byte <= b'9' => {
            return parse_number(item_ref as *mut cJSON, buffer_ref as *mut parse_buffer);
        }
        Some(b'[') => return parse_array(item_ref as *mut cJSON, buffer_ref as *mut parse_buffer),
        Some(b'{') => return parse_object(item_ref as *mut cJSON, buffer_ref as *mut parse_buffer),
        _ => {}
    }
    return false_0;
}
unsafe fn print_value(
    item_ref: &cJSON,
    output_buffer: &mut printbuffer,
) -> cJSON_bool {
    match item_ref.type_0 & 0xff as ::core::ffi::c_int {
        cJSON_NULL => {
            let Some(output_slice) = ensure(output_buffer, 5 as size_t) else {
                return false_0;
            };
            output_slice.copy_from_slice(b"null\0");
            return true_0;
        }
        cJSON_False => {
            let Some(output_slice) = ensure(output_buffer, 6 as size_t) else {
                return false_0;
            };
            output_slice.copy_from_slice(b"false\0");
            return true_0;
        }
        cJSON_True => {
            let Some(output_slice) = ensure(output_buffer, 5 as size_t) else {
                return false_0;
            };
            output_slice.copy_from_slice(b"true\0");
            return true_0;
        }
        cJSON_Number => return print_number(item_ref, output_buffer),
        cJSON_Raw => {
            if item_ref.valuestring.is_null() {
                return false_0;
            }
            let raw_bytes = ::core::ffi::CStr::from_ptr(item_ref.valuestring).to_bytes_with_nul();
            let raw_length = raw_bytes.len();
            let Some(output_slice) = ensure(output_buffer, raw_length) else {
                return false_0;
            };
            output_slice.copy_from_slice(raw_bytes);
            return true_0;
        }
        cJSON_String => {
            return print_string_ptr(
                item_ref.valuestring as *mut ::core::ffi::c_uchar,
                output_buffer,
            );
        }
        cJSON_Array => return print_array(item_ref, output_buffer),
        cJSON_Object => return print_object(item_ref, output_buffer),
        _ => return false_0,
    };
}
unsafe extern "C" fn parse_array(item: *mut cJSON, input_buffer: *mut parse_buffer) -> cJSON_bool {
    let mut c2rust_current_block: u64;
    let mut head: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut current_item: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let input_buffer_ref = &mut *input_buffer;
    let content = ::core::slice::from_raw_parts(input_buffer_ref.content, input_buffer_ref.length);
    if input_buffer_ref.depth >= CJSON_NESTING_LIMIT as size_t {
        return false_0;
    }
    input_buffer_ref.depth = input_buffer_ref.depth.wrapping_add(1);
    if content.get(input_buffer_ref.offset).copied() == Some(b'[') {
        input_buffer_ref.offset = input_buffer_ref.offset.wrapping_add(1);
        buffer_skip_whitespace_ref(input_buffer_ref, content);
        if content.get(input_buffer_ref.offset).copied() == Some(b']') {
            c2rust_current_block = 6773356538935231690;
        } else if content.get(input_buffer_ref.offset).is_none() {
            input_buffer_ref.offset = input_buffer_ref.offset.wrapping_sub(1);
            c2rust_current_block = 1336238348363633231;
        } else {
            input_buffer_ref.offset = input_buffer_ref.offset.wrapping_sub(1);
            loop {
                let mut new_item: *mut cJSON = cJSON_New_Item(&input_buffer_ref.hooks);
                if new_item.is_null() {
                    c2rust_current_block = 1336238348363633231;
                    break;
                }
                let new_item_ref = &mut *new_item;
                if head.is_null() {
                    head = new_item;
                    current_item = head;
                } else {
                    let current_ref = &mut *current_item;
                    current_ref.next = new_item as *mut cJSON;
                    new_item_ref.prev = current_item as *mut cJSON;
                    current_item = new_item;
                }
                input_buffer_ref.offset = input_buffer_ref.offset.wrapping_add(1);
                buffer_skip_whitespace_ref(input_buffer_ref, content);
                if parse_value(new_item_ref, input_buffer_ref) == 0 {
                    c2rust_current_block = 1336238348363633231;
                    break;
                }
                buffer_skip_whitespace_ref(input_buffer_ref, content);
                if content.get(input_buffer_ref.offset).copied() != Some(b',') {
                    c2rust_current_block = 15089075282327824602;
                    break;
                }
            }
            match c2rust_current_block {
                1336238348363633231 => {}
                _ => {
                    if content.get(input_buffer_ref.offset).copied() != Some(b']') {
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
                if !head.is_null() {
                    let head_ref = &mut *head;
                    head_ref.prev = current_item as *mut cJSON;
                }
                let item_ref = &mut *item;
                item_ref.type_0 = cJSON_Array;
                item_ref.child = head as *mut cJSON;
                input_buffer_ref.offset = input_buffer_ref.offset.wrapping_add(1);
                return true_0;
            }
        }
    }
    if !head.is_null() {
        cJSON_Delete_ffi(head);
    }
    return false_0;
}
unsafe fn print_array(
    item_ref: &cJSON,
    output_buffer: &mut printbuffer,
) -> cJSON_bool {
    let mut length: size_t = 0 as size_t;
    let mut current_element: *mut cJSON = item_ref.child as *mut cJSON;
    let output_buffer_ref = output_buffer;
    let Some(output_slice) = ensure(output_buffer_ref, 1 as size_t) else {
        return false_0;
    };
    output_slice[0] = b'[';
    output_buffer_ref.offset = output_buffer_ref.offset.wrapping_add(1);
    output_buffer_ref.depth = output_buffer_ref.depth.wrapping_add(1);
    while !current_element.is_null() {
        let current_ref = &*current_element;
        if print_value(current_ref, output_buffer_ref) == 0 {
            return false_0;
        }
        update_offset(output_buffer_ref);
        if !current_ref.next.is_null() {
            let formatted = output_buffer_ref.format != 0;
            length = (if formatted {
                2 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as size_t;
            let Some(output_slice) =
                ensure(output_buffer_ref, length.wrapping_add(1 as size_t))
            else {
                return false_0;
            };
            output_slice[0] = b',';
            if formatted {
                output_slice[1] = b' ';
            }
            output_slice[length] = b'\0';
            output_buffer_ref.offset = output_buffer_ref.offset.wrapping_add(length);
        }
        current_element = current_ref.next as *mut cJSON;
    }
    let Some(output_slice) = ensure(output_buffer_ref, 2 as size_t) else {
        return false_0;
    };
    output_slice[0] = b']';
    output_slice[1] = b'\0';
    output_buffer_ref.depth = output_buffer_ref.depth.wrapping_sub(1);
    return true_0;
}
unsafe extern "C" fn parse_object(item: *mut cJSON, input_buffer: *mut parse_buffer) -> cJSON_bool {
    let mut c2rust_current_block: u64;
    let mut head: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut current_item: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let input_buffer_ref = &mut *input_buffer;
    let content = ::core::slice::from_raw_parts(input_buffer_ref.content, input_buffer_ref.length);
    if input_buffer_ref.depth >= CJSON_NESTING_LIMIT as size_t {
        return false_0;
    }
    input_buffer_ref.depth = input_buffer_ref.depth.wrapping_add(1);
    if content.get(input_buffer_ref.offset).copied() == Some(b'{') {
        input_buffer_ref.offset = input_buffer_ref.offset.wrapping_add(1);
        buffer_skip_whitespace_ref(input_buffer_ref, content);
        if content.get(input_buffer_ref.offset).copied() == Some(b'}') {
            c2rust_current_block = 4359236900545362719;
        } else if content.get(input_buffer_ref.offset).is_none() {
            input_buffer_ref.offset = input_buffer_ref.offset.wrapping_sub(1);
            c2rust_current_block = 9990476168629568694;
        } else {
            input_buffer_ref.offset = input_buffer_ref.offset.wrapping_sub(1);
            loop {
                let mut new_item: *mut cJSON = cJSON_New_Item(&input_buffer_ref.hooks);
                if new_item.is_null() {
                    c2rust_current_block = 9990476168629568694;
                    break;
                } else {
                    let new_item_ref = &mut *new_item;
                    if head.is_null() {
                        head = new_item;
                        current_item = head;
                    } else {
                        let current_ref = &mut *current_item;
                        current_ref.next = new_item as *mut cJSON;
                        new_item_ref.prev = current_item as *mut cJSON;
                        current_item = new_item;
                    }
                    if !(input_buffer_ref.offset.wrapping_add(1 as size_t) < input_buffer_ref.length)
                    {
                        c2rust_current_block = 9990476168629568694;
                        break;
                    } else {
                        input_buffer_ref.offset = input_buffer_ref.offset.wrapping_add(1);
                        buffer_skip_whitespace_ref(input_buffer_ref, content);
                        if parse_string(current_item, input_buffer) == 0 {
                            c2rust_current_block = 9990476168629568694;
                            break;
                        } else {
                            buffer_skip_whitespace_ref(input_buffer_ref, content);
                            let current_ref = &mut *current_item;
                            current_ref.string = current_ref.valuestring;
                            current_ref.valuestring =
                                ::core::ptr::null_mut::<::core::ffi::c_char>();
                            if content.get(input_buffer_ref.offset).copied() != Some(b':') {
                                c2rust_current_block = 9990476168629568694;
                                break;
                            } else {
                                input_buffer_ref.offset =
                                    input_buffer_ref.offset.wrapping_add(1);
                                buffer_skip_whitespace_ref(input_buffer_ref, content);
                                if parse_value(current_ref, input_buffer_ref) == 0 {
                                    c2rust_current_block = 9990476168629568694;
                                    break;
                                } else {
                                    buffer_skip_whitespace_ref(input_buffer_ref, content);
                                    if content.get(input_buffer_ref.offset).copied() != Some(b',') {
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
                    if content.get(input_buffer_ref.offset).copied() != Some(b'}') {
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
                if !head.is_null() {
                    let head_ref = &mut *head;
                    head_ref.prev = current_item as *mut cJSON;
                }
                let item_ref = &mut *item;
                item_ref.type_0 = cJSON_Object;
                item_ref.child = head as *mut cJSON;
                input_buffer_ref.offset = input_buffer_ref.offset.wrapping_add(1);
                return true_0;
            }
        }
    }
    if !head.is_null() {
        cJSON_Delete_ffi(head);
    }
    return false_0;
}
unsafe fn print_object(
    item_ref: &cJSON,
    output_buffer: &mut printbuffer,
) -> cJSON_bool {
    let mut length: size_t = 0 as size_t;
    let mut current_item: *mut cJSON = item_ref.child as *mut cJSON;
    let output_buffer_ref = output_buffer;
    let formatted = output_buffer_ref.format != 0;
    length = (if formatted {
        2 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    }) as size_t;
    let Some(output_slice) = ensure(output_buffer_ref, length.wrapping_add(1 as size_t)) else {
        return false_0;
    };
    output_slice[0] = b'{';
    if formatted {
        output_slice[1] = b'\n';
    }
    output_buffer_ref.depth = output_buffer_ref.depth.wrapping_add(1);
    output_buffer_ref.offset = output_buffer_ref.offset.wrapping_add(length);
    while !current_item.is_null() {
        let current_ref = &*current_item;
        if output_buffer_ref.format != 0 {
            let Some(output_slice) = ensure(output_buffer_ref, output_buffer_ref.depth) else {
                return false_0;
            };
            output_slice.fill(b'\t');
            output_buffer_ref.offset = output_buffer_ref.offset.wrapping_add(output_buffer_ref.depth);
        }
        if print_string_ptr(
            current_ref.string as *mut ::core::ffi::c_uchar,
            output_buffer_ref,
        ) == 0
        {
            return false_0;
        }
        update_offset(output_buffer_ref);
        let formatted = output_buffer_ref.format != 0;
        length = (if formatted {
            2 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) as size_t;
        let Some(output_slice) = ensure(output_buffer_ref, length) else {
            return false_0;
        };
        output_slice[0] = b':';
        if formatted {
            output_slice[1] = b'\t';
        }
        output_buffer_ref.offset = output_buffer_ref.offset.wrapping_add(length);
        if print_value(current_ref, output_buffer_ref) == 0 {
            return false_0;
        }
        update_offset(output_buffer_ref);
        let formatted = output_buffer_ref.format != 0;
        length = ((if formatted {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as size_t)
            .wrapping_add(
                (if !current_ref.next.is_null() {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as size_t,
            );
        let Some(output_slice) = ensure(output_buffer_ref, length.wrapping_add(1 as size_t)) else {
            return false_0;
        };
        let mut output_index: size_t = 0;
        if !current_ref.next.is_null() {
            output_slice[output_index] = b',';
            output_index = output_index.wrapping_add(1);
        }
        if formatted {
            output_slice[output_index] = b'\n';
            output_index = output_index.wrapping_add(1);
        }
        output_slice[output_index] = b'\0';
        output_buffer_ref.offset = output_buffer_ref.offset.wrapping_add(length);
        current_item = current_ref.next as *mut cJSON;
    }
    let formatted = output_buffer_ref.format != 0;
    let depth = output_buffer_ref.depth;
    let output_length = if formatted {
        depth.wrapping_add(1 as size_t)
    } else {
        2 as size_t
    };
    let Some(output_slice) = ensure(output_buffer_ref, output_length) else {
        return false_0;
    };
    let mut output_index: size_t = 0;
    if formatted {
        let mut i_0: size_t = 0;
        i_0 = 0 as size_t;
        while i_0 < depth.wrapping_sub(1 as size_t) {
            output_slice[output_index] = b'\t';
            output_index = output_index.wrapping_add(1);
            i_0 = i_0.wrapping_add(1);
        }
    }
    output_slice[output_index] = b'}';
    output_index = output_index.wrapping_add(1);
    output_slice[output_index] = b'\0';
    output_buffer_ref.depth = output_buffer_ref.depth.wrapping_sub(1);
    return true_0;
}
#[export_name = "cJSON_GetArraySize"]
pub unsafe extern "C" fn cJSON_GetArraySize_ffi(mut array: *const cJSON) -> ::core::ffi::c_int {
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
#[export_name = "cJSON_GetArrayItem"]
pub unsafe extern "C" fn cJSON_GetArrayItem_ffi(
    mut array: *const cJSON,
    mut index: ::core::ffi::c_int,
) -> *mut cJSON {
    let mut current_child: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if index < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<cJSON>();
    }
    if array.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    current_child = (*array).child as *mut cJSON;
    while !current_child.is_null() && index as size_t > 0 as size_t {
        index -= 1;
        current_child = (*current_child).next as *mut cJSON;
    }
    return current_child;
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
    current_element = (&*object).child as *mut cJSON;
    if case_sensitive != 0 {
        while !current_element.is_null() {
            let current_ref = &*current_element;
            if current_ref.string.is_null()
                || strcmp(name, current_ref.string) == 0 as ::core::ffi::c_int
            {
                break;
            }
            current_element = current_ref.next as *mut cJSON;
        }
    } else {
        while !current_element.is_null() {
            let current_ref = &*current_element;
            if case_insensitive_strcmp(
                name as *const ::core::ffi::c_uchar,
                current_ref.string as *const ::core::ffi::c_uchar,
            ) == 0 as ::core::ffi::c_int
            {
                break;
            }
            current_element = current_ref.next as *mut cJSON;
        }
    }
    if current_element.is_null() || (&*current_element).string.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    return current_element;
}
#[export_name = "cJSON_GetObjectItem"]
pub unsafe extern "C" fn cJSON_GetObjectItem_ffi(
    object: *const cJSON,
    string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    return get_object_item(object, string, false_0);
}
#[export_name = "cJSON_GetObjectItemCaseSensitive"]
pub unsafe extern "C" fn cJSON_GetObjectItemCaseSensitive_ffi(
    object: *const cJSON,
    string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    return get_object_item(object, string, true_0);
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
unsafe fn suffix_object(prev: *mut cJSON, item: *mut cJSON) {
    (*prev).next = item as *mut cJSON;
    (*item).prev = prev as *mut cJSON;
}
unsafe fn create_reference(item: Option<&cJSON>, hooks: &internal_hooks) -> *mut cJSON {
    let mut reference: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let Some(item_ref) = item else {
        return ::core::ptr::null_mut::<cJSON>();
    };
    reference = cJSON_New_Item(hooks);
    if reference.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let reference_ref = &mut *reference;
    *reference_ref = *item_ref;
    reference_ref.string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    reference_ref.type_0 |= cJSON_IsReference;
    reference_ref.prev = ::core::ptr::null_mut::<cJSON>();
    reference_ref.next = reference_ref.prev;
    return reference;
}
unsafe extern "C" fn add_item_to_array(array: *mut cJSON, item: *mut cJSON) -> cJSON_bool {
    let mut child: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if item.is_null() || array.is_null() || array == item {
        return false_0;
    }
    let array_ref = &mut *array;
    let item_ref = &mut *item;
    child = array_ref.child as *mut cJSON;
    if child.is_null() {
        array_ref.child = item as *mut cJSON;
        item_ref.prev = item as *mut cJSON;
        item_ref.next = ::core::ptr::null_mut::<cJSON>();
    } else {
        let child_ref = &mut *child;
        if !child_ref.prev.is_null() {
            suffix_object(child_ref.prev as *mut cJSON, item);
            child_ref.prev = item as *mut cJSON;
        }
    }
    return true_0;
}
#[export_name = "cJSON_AddItemToArray"]
pub unsafe extern "C" fn cJSON_AddItemToArray_ffi(
    mut array: *mut cJSON,
    mut item: *mut cJSON,
) -> cJSON_bool {
    return add_item_to_array(array, item);
}
unsafe extern "C" fn add_item_to_object(
    object: *mut cJSON,
    string: *const ::core::ffi::c_char,
    item: *mut cJSON,
    hooks: &internal_hooks,
    constant_key: cJSON_bool,
) -> cJSON_bool {
    let mut new_key: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut new_type: ::core::ffi::c_int = cJSON_Invalid;
    if object.is_null() || string.is_null() || item.is_null() || object == item {
        return false_0;
    }
    let item_ref = &mut *item;
    if constant_key != 0 {
        new_key = string as *mut ::core::ffi::c_char;
        new_type = item_ref.type_0 | cJSON_StringIsConst;
    } else {
        new_key =
            cJSON_strdup(string as *const ::core::ffi::c_uchar, hooks) as *mut ::core::ffi::c_char;
        if new_key.is_null() {
            return false_0;
        }
        new_type = item_ref.type_0 & !cJSON_StringIsConst;
    }
    if item_ref.type_0 & cJSON_StringIsConst == 0 && !item_ref.string.is_null() {
        hooks
            .deallocate
            .expect("non-null function pointer")(item_ref.string as *mut ::core::ffi::c_void);
    }
    item_ref.string = new_key;
    item_ref.type_0 = new_type;
    return add_item_to_array(object, item);
}
#[export_name = "cJSON_AddItemToObject"]
pub unsafe extern "C" fn cJSON_AddItemToObject_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
    mut item: *mut cJSON,
) -> cJSON_bool {
    let hooks = global_hooks;
    return add_item_to_object(object, string, item, &hooks, false_0);
}
#[export_name = "cJSON_AddItemToObjectCS"]
pub unsafe extern "C" fn cJSON_AddItemToObjectCS_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
    mut item: *mut cJSON,
) -> cJSON_bool {
    let hooks = global_hooks;
    return add_item_to_object(object, string, item, &hooks, true_0);
}
#[export_name = "cJSON_AddItemReferenceToArray"]
pub unsafe extern "C" fn cJSON_AddItemReferenceToArray_ffi(
    mut array: *mut cJSON,
    mut item: *mut cJSON,
) -> cJSON_bool {
    if array.is_null() {
        return false_0;
    }
    let hooks = global_hooks;
    return add_item_to_array(array, create_reference(item.as_ref(), &hooks));
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
    let hooks = global_hooks;
    return add_item_to_object(
        object,
        string,
        create_reference(item.as_ref(), &hooks),
        &hooks,
        false_0,
    );
}
#[export_name = "cJSON_AddNullToObject"]
pub unsafe extern "C" fn cJSON_AddNullToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let mut null: *mut cJSON = cJSON_CreateNull_ffi();
    let hooks = global_hooks;
    if add_item_to_object(object, name, null, &hooks, false_0) != 0 {
        return null;
    }
    cJSON_Delete_ffi(null);
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddTrueToObject"]
pub unsafe extern "C" fn cJSON_AddTrueToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let mut true_item: *mut cJSON = cJSON_CreateTrue_ffi();
    let hooks = global_hooks;
    if add_item_to_object(object, name, true_item, &hooks, false_0) != 0 {
        return true_item;
    }
    cJSON_Delete_ffi(true_item);
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddFalseToObject"]
pub unsafe extern "C" fn cJSON_AddFalseToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let mut false_item: *mut cJSON = cJSON_CreateFalse_ffi();
    let hooks = global_hooks;
    if add_item_to_object(object, name, false_item, &hooks, false_0) != 0 {
        return false_item;
    }
    cJSON_Delete_ffi(false_item);
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddBoolToObject"]
pub unsafe extern "C" fn cJSON_AddBoolToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
    boolean: cJSON_bool,
) -> *mut cJSON {
    let mut bool_item: *mut cJSON = cJSON_CreateBool_ffi(boolean);
    let hooks = global_hooks;
    if add_item_to_object(object, name, bool_item, &hooks, false_0) != 0 {
        return bool_item;
    }
    cJSON_Delete_ffi(bool_item);
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddNumberToObject"]
pub unsafe extern "C" fn cJSON_AddNumberToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
    number: ::core::ffi::c_double,
) -> *mut cJSON {
    let mut number_item: *mut cJSON = cJSON_CreateNumber_ffi(number);
    let hooks = global_hooks;
    if add_item_to_object(object, name, number_item, &hooks, false_0) != 0 {
        return number_item;
    }
    cJSON_Delete_ffi(number_item);
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddStringToObject"]
pub unsafe extern "C" fn cJSON_AddStringToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
    string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let mut string_item: *mut cJSON = cJSON_CreateString_ffi(string);
    let hooks = global_hooks;
    if add_item_to_object(object, name, string_item, &hooks, false_0) != 0 {
        return string_item;
    }
    cJSON_Delete_ffi(string_item);
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddRawToObject"]
pub unsafe extern "C" fn cJSON_AddRawToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
    raw: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let mut raw_item: *mut cJSON = cJSON_CreateRaw_ffi(raw);
    let hooks = global_hooks;
    if add_item_to_object(object, name, raw_item, &hooks, false_0) != 0 {
        return raw_item;
    }
    cJSON_Delete_ffi(raw_item);
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddObjectToObject"]
pub unsafe extern "C" fn cJSON_AddObjectToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let mut object_item: *mut cJSON = cJSON_CreateObject_ffi();
    let hooks = global_hooks;
    if add_item_to_object(object, name, object_item, &hooks, false_0) != 0 {
        return object_item;
    }
    cJSON_Delete_ffi(object_item);
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_AddArrayToObject"]
pub unsafe extern "C" fn cJSON_AddArrayToObject_ffi(
    object: *mut cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let mut array: *mut cJSON = cJSON_CreateArray_ffi();
    let hooks = global_hooks;
    if add_item_to_object(object, name, array, &hooks, false_0) != 0 {
        return array;
    }
    cJSON_Delete_ffi(array);
    return ::core::ptr::null_mut::<cJSON>();
}
#[export_name = "cJSON_DetachItemViaPointer"]
pub unsafe extern "C" fn cJSON_DetachItemViaPointer_ffi(
    mut parent: *mut cJSON,
    item: *mut cJSON,
) -> *mut cJSON {
    if parent.is_null() || item.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let parent_ref = &mut *parent;
    let item_ref = &mut *item;
    if item != parent_ref.child && item_ref.prev.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    if item != parent_ref.child {
        let prev_ref = &mut *item_ref.prev;
        prev_ref.next = item_ref.next;
    }
    if !item_ref.next.is_null() {
        let next_ref = &mut *item_ref.next;
        next_ref.prev = item_ref.prev;
    }
    if item == parent_ref.child {
        parent_ref.child = item_ref.next;
    } else if item_ref.next.is_null() {
        let child_ref = &mut *parent_ref.child;
        child_ref.prev = item_ref.prev;
    }
    item_ref.prev = ::core::ptr::null_mut::<cJSON>();
    item_ref.next = ::core::ptr::null_mut::<cJSON>();
    return item;
}
#[export_name = "cJSON_DetachItemFromArray"]
pub unsafe extern "C" fn cJSON_DetachItemFromArray_ffi(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
) -> *mut cJSON {
    let mut current_child: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if which < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<cJSON>();
    }
    if array.is_null() {
        return cJSON_DetachItemViaPointer_ffi(array, ::core::ptr::null_mut::<cJSON>());
    }
    current_child = (*array).child as *mut cJSON;
    while !current_child.is_null() && which as size_t > 0 as size_t {
        which -= 1;
        current_child = (*current_child).next as *mut cJSON;
    }
    return cJSON_DetachItemViaPointer_ffi(array, current_child);
}
#[export_name = "cJSON_DeleteItemFromArray"]
pub unsafe extern "C" fn cJSON_DeleteItemFromArray_ffi(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
) {
    cJSON_Delete_ffi(cJSON_DetachItemFromArray_ffi(array, which));
}
#[export_name = "cJSON_DetachItemFromObject"]
pub unsafe extern "C" fn cJSON_DetachItemFromObject_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let mut to_detach: *mut cJSON = get_object_item(object, string, false_0);
    return cJSON_DetachItemViaPointer_ffi(object, to_detach);
}
#[export_name = "cJSON_DetachItemFromObjectCaseSensitive"]
pub unsafe extern "C" fn cJSON_DetachItemFromObjectCaseSensitive_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let mut to_detach: *mut cJSON = get_object_item(object, string, true_0);
    return cJSON_DetachItemViaPointer_ffi(object, to_detach);
}
#[export_name = "cJSON_DeleteItemFromObject"]
pub unsafe extern "C" fn cJSON_DeleteItemFromObject_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
) {
    cJSON_Delete_ffi(cJSON_DetachItemFromObject_ffi(object, string));
}
#[export_name = "cJSON_DeleteItemFromObjectCaseSensitive"]
pub unsafe extern "C" fn cJSON_DeleteItemFromObjectCaseSensitive_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
) {
    cJSON_Delete_ffi(cJSON_DetachItemFromObjectCaseSensitive_ffi(
        object, string,
    ));
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
    if !array.is_null() {
        let array_ref = &mut *array;
        after_inserted = array_ref.child as *mut cJSON;
        while !after_inserted.is_null() && which as size_t > 0 as size_t {
            which -= 1;
            let after_ref = &*after_inserted;
            after_inserted = after_ref.next as *mut cJSON;
        }
    }
    if after_inserted.is_null() {
        return add_item_to_array(array, newitem);
    }
    let array_ref = &mut *array;
    let after_ref = &mut *after_inserted;
    if after_inserted != array_ref.child && after_ref.prev.is_null() {
        return false_0;
    }
    let new_ref = &mut *newitem;
    new_ref.next = after_inserted as *mut cJSON;
    new_ref.prev = after_ref.prev;
    after_ref.prev = newitem as *mut cJSON;
    if after_inserted == array_ref.child {
        array_ref.child = newitem as *mut cJSON;
    } else {
        let prev_ref = &mut *new_ref.prev;
        prev_ref.next = newitem as *mut cJSON;
    }
    return true_0;
}
#[export_name = "cJSON_ReplaceItemViaPointer"]
pub unsafe extern "C" fn cJSON_ReplaceItemViaPointer_ffi(
    parent: *mut cJSON,
    item: *mut cJSON,
    replacement: *mut cJSON,
) -> cJSON_bool {
    if parent.is_null() || replacement.is_null() || item.is_null() {
        return false_0;
    }
    let parent_ref = &mut *parent;
    if parent_ref.child.is_null() {
        return false_0;
    }
    if replacement == item {
        return true_0;
    }
    let item_ref = &mut *item;
    let replacement_ref = &mut *replacement;
    replacement_ref.next = item_ref.next;
    replacement_ref.prev = item_ref.prev;
    if !replacement_ref.next.is_null() {
        let next_ref = &mut *replacement_ref.next;
        next_ref.prev = replacement as *mut cJSON;
    }
    if parent_ref.child == item {
        if replacement_ref.prev == parent_ref.child {
            replacement_ref.prev = replacement as *mut cJSON;
        }
        parent_ref.child = replacement as *mut cJSON;
    } else {
        if !replacement_ref.prev.is_null() {
            let prev_ref = &mut *replacement_ref.prev;
            prev_ref.next = replacement as *mut cJSON;
        }
        if replacement_ref.next.is_null() {
            let child_ref = &mut *parent_ref.child;
            child_ref.prev = replacement as *mut cJSON;
        }
    }
    item_ref.next = ::core::ptr::null_mut::<cJSON>();
    item_ref.prev = ::core::ptr::null_mut::<cJSON>();
    cJSON_Delete_ffi(item);
    return true_0;
}
#[export_name = "cJSON_ReplaceItemInArray"]
pub unsafe extern "C" fn cJSON_ReplaceItemInArray_ffi(
    mut array: *mut cJSON,
    mut which: ::core::ffi::c_int,
    mut newitem: *mut cJSON,
) -> cJSON_bool {
    let mut current_child: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if which < 0 as ::core::ffi::c_int {
        return false_0;
    }
    if !array.is_null() {
        current_child = (*array).child as *mut cJSON;
        while !current_child.is_null() && which as size_t > 0 as size_t {
            which -= 1;
            current_child = (*current_child).next as *mut cJSON;
        }
    }
    return cJSON_ReplaceItemViaPointer_ffi(array, current_child, newitem);
}
unsafe extern "C" fn replace_item_in_object(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
    mut replacement: *mut cJSON,
    mut case_sensitive: cJSON_bool,
    hooks: &internal_hooks,
) -> cJSON_bool {
    if replacement.is_null() || string.is_null() {
        return false_0;
    }
    let replacement_ref = &mut *replacement;
    if replacement_ref.type_0 & cJSON_StringIsConst == 0 && !replacement_ref.string.is_null() {
        cJSON_free_ffi(replacement_ref.string as *mut ::core::ffi::c_void);
    }
    replacement_ref.string = cJSON_strdup(string as *const ::core::ffi::c_uchar, hooks)
        as *mut ::core::ffi::c_char;
    if replacement_ref.string.is_null() {
        return false_0;
    }
    replacement_ref.type_0 &= !cJSON_StringIsConst;
    return cJSON_ReplaceItemViaPointer_ffi(
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
    let hooks = global_hooks;
    return replace_item_in_object(object, string, newitem, false_0, &hooks);
}
#[export_name = "cJSON_ReplaceItemInObjectCaseSensitive"]
pub unsafe extern "C" fn cJSON_ReplaceItemInObjectCaseSensitive_ffi(
    mut object: *mut cJSON,
    mut string: *const ::core::ffi::c_char,
    mut newitem: *mut cJSON,
) -> cJSON_bool {
    let hooks = global_hooks;
    return replace_item_in_object(object, string, newitem, true_0, &hooks);
}
#[export_name = "cJSON_CreateNull"]
pub unsafe extern "C" fn cJSON_CreateNull_ffi() -> *mut cJSON {
    let hooks = global_hooks;
    let mut item: *mut cJSON = cJSON_New_Item(&hooks);
    if !item.is_null() {
        (*item).type_0 = cJSON_NULL;
    }
    return item;
}
#[export_name = "cJSON_CreateTrue"]
pub unsafe extern "C" fn cJSON_CreateTrue_ffi() -> *mut cJSON {
    let hooks = global_hooks;
    let mut item: *mut cJSON = cJSON_New_Item(&hooks);
    if !item.is_null() {
        (*item).type_0 = cJSON_True;
    }
    return item;
}
#[export_name = "cJSON_CreateFalse"]
pub unsafe extern "C" fn cJSON_CreateFalse_ffi() -> *mut cJSON {
    let hooks = global_hooks;
    let mut item: *mut cJSON = cJSON_New_Item(&hooks);
    if !item.is_null() {
        (*item).type_0 = cJSON_False;
    }
    return item;
}
#[export_name = "cJSON_CreateBool"]
pub unsafe extern "C" fn cJSON_CreateBool_ffi(mut boolean: cJSON_bool) -> *mut cJSON {
    let hooks = global_hooks;
    let mut item: *mut cJSON = cJSON_New_Item(&hooks);
    if !item.is_null() {
        (*item).type_0 = if boolean != 0 {
            cJSON_True
        } else {
            cJSON_False
        };
    }
    return item;
}
#[export_name = "cJSON_CreateNumber"]
pub unsafe extern "C" fn cJSON_CreateNumber_ffi(mut num: ::core::ffi::c_double) -> *mut cJSON {
    let hooks = global_hooks;
    let mut item: *mut cJSON = cJSON_New_Item(&hooks);
    if !item.is_null() {
        (*item).type_0 = cJSON_Number;
        cJSON_SetNumberHelper(&mut *item, num);
    }
    return item;
}
#[export_name = "cJSON_CreateString"]
pub unsafe extern "C" fn cJSON_CreateString_ffi(
    mut string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let hooks = global_hooks;
    let mut item: *mut cJSON = cJSON_New_Item(&hooks);
    if !item.is_null() {
        (*item).type_0 = cJSON_String;
        let hooks = global_hooks;
        (*item).valuestring =
            cJSON_strdup(string as *const ::core::ffi::c_uchar, &hooks) as *mut ::core::ffi::c_char;
        if (*item).valuestring.is_null() {
            cJSON_Delete_ffi(item);
            return ::core::ptr::null_mut::<cJSON>();
        }
    }
    return item;
}
#[export_name = "cJSON_CreateStringReference"]
pub unsafe extern "C" fn cJSON_CreateStringReference_ffi(
    mut string: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let hooks = global_hooks;
    let mut item: *mut cJSON = cJSON_New_Item(&hooks);
    if !item.is_null() {
        (*item).type_0 = cJSON_String | cJSON_IsReference;
        (*item).valuestring = string as *mut ::core::ffi::c_char;
    }
    return item;
}
#[export_name = "cJSON_CreateObjectReference"]
pub unsafe extern "C" fn cJSON_CreateObjectReference_ffi(mut child: *const cJSON) -> *mut cJSON {
    let hooks = global_hooks;
    let mut item: *mut cJSON = cJSON_New_Item(&hooks);
    if !item.is_null() {
        (*item).type_0 = cJSON_Object | cJSON_IsReference;
        (*item).child = child as *mut cJSON;
    }
    return item;
}
#[export_name = "cJSON_CreateArrayReference"]
pub unsafe extern "C" fn cJSON_CreateArrayReference_ffi(mut child: *const cJSON) -> *mut cJSON {
    let hooks = global_hooks;
    let mut item: *mut cJSON = cJSON_New_Item(&hooks);
    if !item.is_null() {
        (*item).type_0 = cJSON_Array | cJSON_IsReference;
        (*item).child = child as *mut cJSON;
    }
    return item;
}
#[export_name = "cJSON_CreateRaw"]
pub unsafe extern "C" fn cJSON_CreateRaw_ffi(mut raw: *const ::core::ffi::c_char) -> *mut cJSON {
    let hooks = global_hooks;
    let mut item: *mut cJSON = cJSON_New_Item(&hooks);
    if !item.is_null() {
        (*item).type_0 = cJSON_Raw;
        let hooks = global_hooks;
        (*item).valuestring =
            cJSON_strdup(raw as *const ::core::ffi::c_uchar, &hooks) as *mut ::core::ffi::c_char;
        if (*item).valuestring.is_null() {
            cJSON_Delete_ffi(item);
            return ::core::ptr::null_mut::<cJSON>();
        }
    }
    return item;
}
#[export_name = "cJSON_CreateArray"]
pub unsafe extern "C" fn cJSON_CreateArray_ffi() -> *mut cJSON {
    let hooks = global_hooks;
    let mut item: *mut cJSON = cJSON_New_Item(&hooks);
    if !item.is_null() {
        (*item).type_0 = cJSON_Array;
    }
    return item;
}
#[export_name = "cJSON_CreateObject"]
pub unsafe extern "C" fn cJSON_CreateObject_ffi() -> *mut cJSON {
    let hooks = global_hooks;
    let mut item: *mut cJSON = cJSON_New_Item(&hooks);
    if !item.is_null() {
        (*item).type_0 = cJSON_Object;
    }
    return item;
}
#[export_name = "cJSON_CreateIntArray"]
pub unsafe extern "C" fn cJSON_CreateIntArray_ffi(
    mut numbers: *const ::core::ffi::c_int,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    let mut n: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut p: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut a: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let number_slice = ::core::slice::from_raw_parts(numbers, count as usize);
    a = cJSON_CreateArray_ffi();
    if a.is_null() {
        return a;
    }
    for (i, number) in number_slice.iter().enumerate() {
        n = cJSON_CreateNumber_ffi(*number as ::core::ffi::c_double);
        if n.is_null() {
            cJSON_Delete_ffi(a);
            return ::core::ptr::null_mut::<cJSON>();
        }
        if i == 0 {
            let a_ref = &mut *a;
            a_ref.child = n as *mut cJSON;
        } else {
            suffix_object(p, n);
        }
        p = n;
    }
    let a_ref = &mut *a;
    if !a_ref.child.is_null() {
        let child_ref = &mut *a_ref.child;
        child_ref.prev = n as *mut cJSON;
    }
    return a;
}
#[export_name = "cJSON_CreateFloatArray"]
pub unsafe extern "C" fn cJSON_CreateFloatArray_ffi(
    mut numbers: *const ::core::ffi::c_float,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    let mut n: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut p: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut a: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let number_slice = ::core::slice::from_raw_parts(numbers, count as usize);
    a = cJSON_CreateArray_ffi();
    if a.is_null() {
        return a;
    }
    for (i, number) in number_slice.iter().enumerate() {
        n = cJSON_CreateNumber_ffi(*number as ::core::ffi::c_double);
        if n.is_null() {
            cJSON_Delete_ffi(a);
            return ::core::ptr::null_mut::<cJSON>();
        }
        if i == 0 {
            let a_ref = &mut *a;
            a_ref.child = n as *mut cJSON;
        } else {
            suffix_object(p, n);
        }
        p = n;
    }
    let a_ref = &mut *a;
    if !a_ref.child.is_null() {
        let child_ref = &mut *a_ref.child;
        child_ref.prev = n as *mut cJSON;
    }
    return a;
}
#[export_name = "cJSON_CreateDoubleArray"]
pub unsafe extern "C" fn cJSON_CreateDoubleArray_ffi(
    mut numbers: *const ::core::ffi::c_double,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    let mut n: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut p: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut a: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if count < 0 as ::core::ffi::c_int || numbers.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let number_slice = ::core::slice::from_raw_parts(numbers, count as usize);
    a = cJSON_CreateArray_ffi();
    if a.is_null() {
        return a;
    }
    for (i, number) in number_slice.iter().enumerate() {
        n = cJSON_CreateNumber_ffi(*number);
        if n.is_null() {
            cJSON_Delete_ffi(a);
            return ::core::ptr::null_mut::<cJSON>();
        }
        if i == 0 {
            let a_ref = &mut *a;
            a_ref.child = n as *mut cJSON;
        } else {
            suffix_object(p, n);
        }
        p = n;
    }
    let a_ref = &mut *a;
    if !a_ref.child.is_null() {
        let child_ref = &mut *a_ref.child;
        child_ref.prev = n as *mut cJSON;
    }
    return a;
}
#[export_name = "cJSON_CreateStringArray"]
pub unsafe extern "C" fn cJSON_CreateStringArray_ffi(
    mut strings: *const *const ::core::ffi::c_char,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    let mut n: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut p: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut a: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if count < 0 as ::core::ffi::c_int || strings.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let string_slice = ::core::slice::from_raw_parts(strings, count as usize);
    a = cJSON_CreateArray_ffi();
    if a.is_null() {
        return a;
    }
    for (i, string) in string_slice.iter().enumerate() {
        n = cJSON_CreateString_ffi(*string);
        if n.is_null() {
            cJSON_Delete_ffi(a);
            return ::core::ptr::null_mut::<cJSON>();
        }
        if i == 0 {
            let a_ref = &mut *a;
            a_ref.child = n as *mut cJSON;
        } else {
            suffix_object(p, n);
        }
        p = n;
    }
    let a_ref = &mut *a;
    if !a_ref.child.is_null() {
        let child_ref = &mut *a_ref.child;
        child_ref.prev = n as *mut cJSON;
    }
    return a;
}
#[export_name = "cJSON_Duplicate"]
pub unsafe extern "C" fn cJSON_Duplicate_ffi(
    mut item: *const cJSON,
    mut recurse: cJSON_bool,
) -> *mut cJSON {
    return cJSON_Duplicate_rec_ffi(item, 0 as size_t, recurse);
}
#[export_name = "cJSON_Duplicate_rec"]
pub unsafe extern "C" fn cJSON_Duplicate_rec_ffi(
    mut item: *const cJSON,
    mut depth: size_t,
    mut recurse: cJSON_bool,
) -> *mut cJSON {
    let mut newitem: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut child: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut next: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut newchild: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if item.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let item_ref = &*item;
    let hooks = global_hooks;
    newitem = cJSON_New_Item(&hooks);
    if newitem.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let newitem_ref = &mut *newitem;
    newitem_ref.type_0 = item_ref.type_0 & !cJSON_IsReference;
    newitem_ref.valueint = item_ref.valueint;
    newitem_ref.valuedouble = item_ref.valuedouble;
    if !item_ref.valuestring.is_null() {
        let hooks = global_hooks;
        newitem_ref.valuestring =
            cJSON_strdup(item_ref.valuestring as *mut ::core::ffi::c_uchar, &hooks)
                as *mut ::core::ffi::c_char;
        if newitem_ref.valuestring.is_null() {
            cJSON_Delete_ffi(newitem);
            return ::core::ptr::null_mut::<cJSON>();
        }
    }
    if !item_ref.string.is_null() {
        newitem_ref.string = if item_ref.type_0 & cJSON_StringIsConst != 0 {
            item_ref.string
        } else {
            let hooks = global_hooks;
            cJSON_strdup(item_ref.string as *mut ::core::ffi::c_uchar, &hooks)
                as *mut ::core::ffi::c_char
        };
        if newitem_ref.string.is_null() {
            cJSON_Delete_ffi(newitem);
            return ::core::ptr::null_mut::<cJSON>();
        }
    }
    if recurse == 0 {
        return newitem;
    }
    child = item_ref.child as *mut cJSON;
    while !child.is_null() {
        if depth >= CJSON_CIRCULAR_LIMIT as size_t {
            cJSON_Delete_ffi(newitem);
            return ::core::ptr::null_mut::<cJSON>();
        }
        let child_ref = &*child;
        newchild = cJSON_Duplicate_rec_ffi(child, depth.wrapping_add(1 as size_t), true_0);
        if newchild.is_null() {
            cJSON_Delete_ffi(newitem);
            return ::core::ptr::null_mut::<cJSON>();
        }
        if !next.is_null() {
            let next_ref = &mut *next;
            let newchild_ref = &mut *newchild;
            next_ref.next = newchild as *mut cJSON;
            newchild_ref.prev = next as *mut cJSON;
            next = newchild;
        } else {
            newitem_ref.child = newchild as *mut cJSON;
            next = newchild;
        }
        child = child_ref.next as *mut cJSON;
    }
    if !newitem_ref.child.is_null() {
        let child_ref = &mut *newitem_ref.child;
        child_ref.prev = newchild as *mut cJSON;
    }
    return newitem;
}
fn char_byte(byte: u8) -> ::core::ffi::c_char {
    byte as ::core::ffi::c_char
}
fn minify_byte(input: &[::core::ffi::c_char], index: size_t) -> ::core::ffi::c_char {
    input.get(index).copied().unwrap_or(0)
}
fn skip_oneline_comment(input: &mut size_t, json: &[::core::ffi::c_char]) {
    *input = input.wrapping_add(2);
    while minify_byte(json, *input) != 0 {
        if minify_byte(json, *input) == char_byte(b'\n') {
            *input = input.wrapping_add(1);
            return;
        }
        *input = input.wrapping_add(1);
    }
}
fn skip_multiline_comment(input: &mut size_t, json: &[::core::ffi::c_char]) {
    *input = input.wrapping_add(2);
    while minify_byte(json, *input) != 0 {
        if minify_byte(json, *input) == char_byte(b'*')
            && minify_byte(json, input.wrapping_add(1)) == char_byte(b'/')
        {
            *input = input.wrapping_add(2);
            return;
        }
        *input = input.wrapping_add(1);
    }
}
fn minify_string(
    input: &mut size_t,
    output: &mut size_t,
    json: &mut [::core::ffi::c_char],
) {
    let current = minify_byte(json, *input);
    if let Some(slot) = json.get_mut(*output) {
        *slot = current;
    }
    *input = input.wrapping_add(1);
    *output = output.wrapping_add(1);
    while minify_byte(json, *input) != 0 {
        let current = minify_byte(json, *input);
        if let Some(slot) = json.get_mut(*output) {
            *slot = current;
        }
        if current == char_byte(b'"') {
            if let Some(slot) = json.get_mut(*output) {
                *slot = char_byte(b'"');
            }
            *input = input.wrapping_add(1);
            *output = output.wrapping_add(1);
            return;
        } else if current == char_byte(b'\\')
            && minify_byte(json, input.wrapping_add(1)) == char_byte(b'"')
        {
            let escaped_quote = minify_byte(json, input.wrapping_add(1));
            if let Some(slot) = json.get_mut(output.wrapping_add(1)) {
                *slot = escaped_quote;
            }
            *input = input.wrapping_add(1);
            *output = output.wrapping_add(1);
        }
        *input = input.wrapping_add(1);
        *output = output.wrapping_add(1);
    }
}
fn minify_in_place(json: &mut [::core::ffi::c_char]) {
    let mut input: size_t = 0;
    let mut output: size_t = 0;
    while minify_byte(json, input) != 0 {
        match minify_byte(json, input) {
            byte if byte == char_byte(b' ')
                || byte == char_byte(b'\t')
                || byte == char_byte(b'\r')
                || byte == char_byte(b'\n') =>
            {
                input = input.wrapping_add(1);
            }
            byte if byte == char_byte(b'/') => {
                if minify_byte(json, input.wrapping_add(1)) == char_byte(b'/') {
                    skip_oneline_comment(&mut input, json);
                } else if minify_byte(json, input.wrapping_add(1)) == char_byte(b'*') {
                    skip_multiline_comment(&mut input, json);
                } else {
                    input = input.wrapping_add(1);
                }
            }
            byte if byte == char_byte(b'"') => {
                minify_string(&mut input, &mut output, json);
            }
            _ => {
                let current = minify_byte(json, input);
                if let Some(slot) = json.get_mut(output) {
                    *slot = current;
                }
                input = input.wrapping_add(1);
                output = output.wrapping_add(1);
            }
        }
    }
    if let Some(slot) = json.get_mut(output) {
        *slot = 0;
    }
}
#[export_name = "cJSON_Minify"]
pub unsafe extern "C" fn cJSON_Minify_ffi(json: *mut ::core::ffi::c_char) {
    if json.is_null() {
        return;
    }
    let json_length = ::core::ffi::CStr::from_ptr(json).to_bytes_with_nul().len();
    let json = ::core::slice::from_raw_parts_mut(json, json_length);
    minify_in_place(json);
}
fn cJSON_type(item: Option<&cJSON>) -> Option<::core::ffi::c_int> {
    item.map(|item| item.type_0 & 0xff as ::core::ffi::c_int)
}
pub fn cJSON_IsInvalid(item: Option<&cJSON>) -> cJSON_bool {
    return (cJSON_type(item) == Some(cJSON_Invalid)) as ::core::ffi::c_int;
}
#[export_name = "cJSON_IsInvalid"]
pub unsafe extern "C" fn cJSON_IsInvalid_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsInvalid(item.as_ref())
}
pub fn cJSON_IsFalse(item: Option<&cJSON>) -> cJSON_bool {
    return (cJSON_type(item) == Some(cJSON_False)) as ::core::ffi::c_int;
}
#[export_name = "cJSON_IsFalse"]
pub unsafe extern "C" fn cJSON_IsFalse_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsFalse(item.as_ref())
}
pub fn cJSON_IsTrue(item: Option<&cJSON>) -> cJSON_bool {
    return (cJSON_type(item) == Some(cJSON_True)) as ::core::ffi::c_int;
}
#[export_name = "cJSON_IsTrue"]
pub unsafe extern "C" fn cJSON_IsTrue_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsTrue(item.as_ref())
}
pub fn cJSON_IsBool(item: Option<&cJSON>) -> cJSON_bool {
    return item
        .map(|item| item.type_0 & (cJSON_True | cJSON_False) != 0 as ::core::ffi::c_int)
        .unwrap_or(false)
        as ::core::ffi::c_int;
}
#[export_name = "cJSON_IsBool"]
pub unsafe extern "C" fn cJSON_IsBool_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsBool(item.as_ref())
}
pub fn cJSON_IsNull(item: Option<&cJSON>) -> cJSON_bool {
    return (cJSON_type(item) == Some(cJSON_NULL)) as ::core::ffi::c_int;
}
#[export_name = "cJSON_IsNull"]
pub unsafe extern "C" fn cJSON_IsNull_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsNull(item.as_ref())
}
pub fn cJSON_IsNumber(item: Option<&cJSON>) -> cJSON_bool {
    return (cJSON_type(item) == Some(cJSON_Number)) as ::core::ffi::c_int;
}
#[export_name = "cJSON_IsNumber"]
pub unsafe extern "C" fn cJSON_IsNumber_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsNumber(item.as_ref())
}
pub fn cJSON_IsString(item: Option<&cJSON>) -> cJSON_bool {
    return (cJSON_type(item) == Some(cJSON_String)) as ::core::ffi::c_int;
}
#[export_name = "cJSON_IsString"]
pub unsafe extern "C" fn cJSON_IsString_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsString(item.as_ref())
}
pub fn cJSON_IsArray(item: Option<&cJSON>) -> cJSON_bool {
    return (cJSON_type(item) == Some(cJSON_Array)) as ::core::ffi::c_int;
}
#[export_name = "cJSON_IsArray"]
pub unsafe extern "C" fn cJSON_IsArray_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsArray(item.as_ref())
}
pub fn cJSON_IsObject(item: Option<&cJSON>) -> cJSON_bool {
    return (cJSON_type(item) == Some(cJSON_Object)) as ::core::ffi::c_int;
}
#[export_name = "cJSON_IsObject"]
pub unsafe extern "C" fn cJSON_IsObject_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsObject(item.as_ref())
}
pub fn cJSON_IsRaw(item: Option<&cJSON>) -> cJSON_bool {
    return (cJSON_type(item) == Some(cJSON_Raw)) as ::core::ffi::c_int;
}
#[export_name = "cJSON_IsRaw"]
pub unsafe extern "C" fn cJSON_IsRaw_ffi(item: *const cJSON) -> cJSON_bool {
    cJSON_IsRaw(item.as_ref())
}
#[export_name = "cJSON_Compare"]
pub unsafe extern "C" fn cJSON_Compare_ffi(
    a: *const cJSON,
    b: *const cJSON,
    case_sensitive: cJSON_bool,
) -> cJSON_bool {
    if a.is_null() || b.is_null() {
        return false_0;
    }
    let a_ref = &*a;
    let b_ref = &*b;
    let item_type = a_ref.type_0 & 0xff as ::core::ffi::c_int;
    if item_type != b_ref.type_0 & 0xff as ::core::ffi::c_int {
        return false_0;
    }
    match item_type {
        cJSON_False | cJSON_True | cJSON_NULL | cJSON_Number | cJSON_String | cJSON_Raw
        | cJSON_Array | cJSON_Object => {}
        _ => return false_0,
    }
    if a == b {
        return true_0;
    }
    match item_type {
        cJSON_False | cJSON_True | cJSON_NULL => return true_0,
        cJSON_Number => {
            if compare_double(a_ref.valuedouble, b_ref.valuedouble) != 0 {
                return true_0;
            }
            return false_0;
        }
        cJSON_String | cJSON_Raw => {
            if a_ref.valuestring.is_null() || b_ref.valuestring.is_null() {
                return false_0;
            }
            if strcmp(a_ref.valuestring, b_ref.valuestring) == 0 as ::core::ffi::c_int {
                return true_0;
            }
            return false_0;
        }
        cJSON_Array => {
            let mut a_element: *mut cJSON = a_ref.child as *mut cJSON;
            let mut b_element: *mut cJSON = b_ref.child as *mut cJSON;
            while !a_element.is_null() && !b_element.is_null() {
                if cJSON_Compare_ffi(a_element, b_element, case_sensitive) == 0 {
                    return false_0;
                }
                let a_element_ref = &*a_element;
                let b_element_ref = &*b_element;
                a_element = a_element_ref.next as *mut cJSON;
                b_element = b_element_ref.next as *mut cJSON;
            }
            if a_element != b_element {
                return false_0;
            }
            return true_0;
        }
        cJSON_Object => {
            let mut a_element_0: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
            let mut b_element_0: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
            a_element_0 = a_ref.child as *mut cJSON;
            while !a_element_0.is_null() {
                let a_element_ref = &*a_element_0;
                b_element_0 = get_object_item(b, a_element_ref.string, case_sensitive);
                if b_element_0.is_null() {
                    return false_0;
                }
                if cJSON_Compare_ffi(a_element_0, b_element_0, case_sensitive) == 0 {
                    return false_0;
                }
                a_element_0 = a_element_ref.next as *mut cJSON;
            }
            b_element_0 = b_ref.child as *mut cJSON;
            while !b_element_0.is_null() {
                let b_element_ref = &*b_element_0;
                a_element_0 = get_object_item(a, b_element_ref.string, case_sensitive);
                if a_element_0.is_null() {
                    return false_0;
                }
                if cJSON_Compare_ffi(b_element_0, a_element_0, case_sensitive) == 0 {
                    return false_0;
                }
                b_element_0 = b_element_ref.next as *mut cJSON;
            }
            return true_0;
        }
        _ => return false_0,
    };
}
#[export_name = "cJSON_malloc"]
pub unsafe extern "C" fn cJSON_malloc_ffi(mut size: size_t) -> *mut ::core::ffi::c_void {
    return global_hooks.allocate.expect("non-null function pointer")(size);
}
#[export_name = "cJSON_free"]
pub unsafe extern "C" fn cJSON_free_ffi(mut object: *mut ::core::ffi::c_void) {
    global_hooks.deallocate.expect("non-null function pointer")(object);
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __DBL_EPSILON__: ::core::ffi::c_double = 2.2204460492503131e-16f64;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const INT_MIN: ::core::ffi::c_int = -2147483647 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
pub const DBL_EPSILON: ::core::ffi::c_double = __DBL_EPSILON__;
