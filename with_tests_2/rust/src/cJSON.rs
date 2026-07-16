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

impl cJSON {
    fn null() -> Self {
        Self {
            next: ::core::ptr::null_mut::<Self>(),
            prev: ::core::ptr::null_mut::<Self>(),
            child: ::core::ptr::null_mut::<Self>(),
            type_0: cJSON_NULL,
            valuestring: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            valueint: 0,
            valuedouble: 0.0,
            string: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        }
    }
}

#[derive(Default)]
struct owned_strings {
    value: Option<Vec<u8>>,
    key: Option<Vec<u8>>,
}

static OWNED_STRINGS: std::sync::LazyLock<
    std::sync::Mutex<std::collections::HashMap<usize, owned_strings>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(std::collections::HashMap::new()));

fn node_id(item: &cJSON) -> usize {
    std::ptr::from_ref(item).addr()
}

fn set_owned_valuestring(item: &mut cJSON, bytes: Vec<u8>) {
    debug_assert_eq!(bytes.last(), Some(&0));
    let mut strings = OWNED_STRINGS.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let entry = strings.entry(node_id(item)).or_default();
    entry.value = Some(bytes);
    item.valuestring = entry
        .value
        .as_mut()
        .expect("owned value string was just set")
        .as_mut_ptr()
        .cast();
}

fn set_owned_string(item: &mut cJSON, bytes: Vec<u8>) {
    debug_assert_eq!(bytes.last(), Some(&0));
    let mut strings = OWNED_STRINGS.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let entry = strings.entry(node_id(item)).or_default();
    entry.key = Some(bytes);
    item.string = entry
        .key
        .as_mut()
        .expect("owned key string was just set")
        .as_mut_ptr()
        .cast();
}

fn move_valuestring_to_string(item: &mut cJSON) {
    let mut strings = OWNED_STRINGS.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let entry = strings.entry(node_id(item)).or_default();
    entry.key = entry.value.take();
    item.string = ::core::ptr::null_mut();
    if let Some(bytes) = entry.key.as_mut() {
        item.string = bytes.as_mut_ptr().cast();
    }
    item.valuestring = ::core::ptr::null_mut();
}

fn clear_owned_string(item: &cJSON) {
    let mut strings = OWNED_STRINGS.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let id = node_id(item);
    if let Some(entry) = strings.get_mut(&id) {
        entry.key = None;
        if entry.value.is_none() {
            strings.remove(&id);
        }
    }
}

fn owned_valuestring_len(item: &cJSON) -> Option<usize> {
    OWNED_STRINGS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&node_id(item))?
        .value
        .as_ref()
        .map(Vec::len)
}

fn replace_owned_valuestring_if_fits(item: &cJSON, bytes: &[u8]) -> Option<bool> {
    let mut strings = OWNED_STRINGS.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let storage = strings.get_mut(&node_id(item))?.value.as_mut()?;
    if bytes.len() > storage.len() {
        return Some(false);
    }
    storage[..bytes.len()].copy_from_slice(bytes);
    Some(true)
}

fn release_owned_strings(item: &cJSON) {
    let owned = OWNED_STRINGS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&node_id(item));
    let Some(owned) = owned else {
        return;
    };
    if let Some(mut value) = owned.value {
        if item.valuestring != value.as_mut_ptr().cast() {
            std::mem::forget(value);
        }
    }
    if let Some(mut key) = owned.key {
        if item.string != key.as_mut_ptr().cast() {
            std::mem::forget(key);
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]

pub struct cJSON_Hooks {
    pub malloc_fn:
        Option<extern "C" fn(crate::__stddef_size_t_h::size_t) -> *mut ::core::ffi::c_void>,
    pub free_fn: Option<extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
}

pub type cJSON_bool = ::core::ffi::c_int;

pub const CJSON_NESTING_LIMIT: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;

pub const CJSON_CIRCULAR_LIMIT: ::core::ffi::c_int = 10000 as ::core::ffi::c_int;
pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;

pub use crate::float_h::DBL_EPSILON;


pub use crate::internal::__DBL_EPSILON__;
pub use crate::internal::__INT_MAX__;
pub use crate::limits_h::INT_MAX;
pub use crate::limits_h::INT_MIN;

pub use crate::stdlib::lconv;
pub use crate::stdlib::localeconv;













#[derive(Copy, Clone)]
#[repr(C)]

pub struct internal_hooks {
    pub allocate:
        Option<extern "C" fn(crate::__stddef_size_t_h::size_t) -> *mut ::core::ffi::c_void>,
    pub deallocate: Option<extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct parse_buffer<'a> {
    pub content: &'a [u8],
    pub length: crate::__stddef_size_t_h::size_t,
    pub offset: crate::__stddef_size_t_h::size_t,
    pub depth: crate::__stddef_size_t_h::size_t,
    pub hooks: internal_hooks,
}

impl parse_buffer<'_> {
    fn remaining(&self) -> &[u8] {
        self.content.get(self.offset..).unwrap_or_default()
    }

    fn current(&self) -> Option<u8> {
        self.content.get(self.offset).copied()
    }

    fn starts_with(&self, prefix: &[u8]) -> bool {
        self.remaining().starts_with(prefix)
    }
}
enum print_storage<'a> {
    Owned(Vec<u8>),
    Borrowed(&'a mut [u8]),
}

pub struct printbuffer<'a> {
    storage: print_storage<'a>,
    pub offset: crate::__stddef_size_t_h::size_t,
    pub depth: crate::__stddef_size_t_h::size_t,
    pub format: crate::src::cJSON::cJSON_bool,
}

impl<'a> printbuffer<'a> {
    fn owned(capacity: usize, format: cJSON_bool) -> Self {
        Self {
            storage: print_storage::Owned(vec![0; capacity]),
            offset: 0,
            depth: 0,
            format,
        }
    }

    fn borrowed(buffer: &'a mut [u8], format: cJSON_bool) -> Self {
        Self {
            storage: print_storage::Borrowed(buffer),
            offset: 0,
            depth: 0,
            format,
        }
    }

    fn write(&mut self, bytes: &[u8]) -> bool {
        let Some(end) = self.offset.checked_add(bytes.len()) else {
            return false;
        };
        let Some(required) = end.checked_add(1) else {
            return false;
        };
        if required > INT_MAX as usize {
            return false;
        }

        match &mut self.storage {
            print_storage::Owned(buffer) => {
                if required > buffer.len() {
                    let new_length = required
                        .checked_mul(2)
                        .unwrap_or(INT_MAX as usize)
                        .min(INT_MAX as usize);
                    buffer.resize(new_length, 0);
                }
                buffer[self.offset..end].copy_from_slice(bytes);
                buffer[end] = 0;
            }
            print_storage::Borrowed(buffer) => {
                let Some(output) = buffer.get_mut(self.offset..required) else {
                    return false;
                };
                output[..bytes.len()].copy_from_slice(bytes);
                output[bytes.len()] = 0;
            }
        }
        self.offset = end;
        true
    }

    fn into_bytes(self) -> Option<Vec<u8>> {
        match self.storage {
            print_storage::Owned(mut buffer) => {
                buffer.truncate(self.offset.checked_add(1)?);
                Some(buffer)
            }
            print_storage::Borrowed(_) => None,
        }
    }
}

pub const true_0: crate::src::cJSON::cJSON_bool = 1 as ::core::ffi::c_int;

pub const false_0: crate::src::cJSON::cJSON_bool = 0 as ::core::ffi::c_int;

struct global_error_state {
    json: std::sync::atomic::AtomicUsize,
    position: std::sync::atomic::AtomicUsize,
}

static global_error: global_error_state = global_error_state {
    json: std::sync::atomic::AtomicUsize::new(0),
    position: std::sync::atomic::AtomicUsize::new(0),
};
#[export_name = "cJSON_GetErrorPtr"]

pub unsafe extern "C" fn cJSON_GetErrorPtr_ffi() -> *const ::core::ffi::c_char {
    let json = global_error.json.load(std::sync::atomic::Ordering::Relaxed);
    let position = global_error.position.load(std::sync::atomic::Ordering::Relaxed);
    return (json as *const ::core::ffi::c_uchar).wrapping_add(position)
        as *const ::core::ffi::c_char;
}
#[export_name = "cJSON_GetStringValue"]

pub unsafe extern "C" fn cJSON_GetStringValue_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> *mut ::core::ffi::c_char {
    let Some(item) = (unsafe { item.as_ref() }) else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    if cJSON_IsString(Some(item)) == 0 {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    item.valuestring
}
pub fn cJSON_GetNumberValue(item: Option<&cJSON>) -> ::core::ffi::c_double {
    if cJSON_IsNumber(item) == 0 {
        return 0.0f64 / 0.0f64;
    }
    return item.unwrap().valuedouble;
}
#[export_name = "cJSON_GetNumberValue"]

pub unsafe extern "C" fn cJSON_GetNumberValue_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> ::core::ffi::c_double {
    cJSON_GetNumberValue(unsafe { item.as_ref() })
}
fn cjson_version() -> &'static ::std::ffi::CStr {
    ::std::ffi::CStr::from_bytes_with_nul(b"1.7.19\0").expect("version string is nul terminated")
}
#[export_name = "cJSON_Version"]

pub unsafe extern "C" fn cJSON_Version_ffi() -> *const ::core::ffi::c_char {
    cjson_version().as_ptr()
}
fn case_insensitive_strcmp(string1: &[u8], string2: &[u8]) -> ::core::ffi::c_int {
    for (&byte1, &byte2) in string1.iter().zip(string2.iter()) {
        let lowercase1 = byte1.to_ascii_lowercase();
        let lowercase2 = byte2.to_ascii_lowercase();
        if lowercase1 != lowercase2 {
            return lowercase1 as ::core::ffi::c_int - lowercase2 as ::core::ffi::c_int;
        }
    }

    string1.len() as ::core::ffi::c_int - string2.len() as ::core::ffi::c_int
}

static global_hooks: std::sync::Mutex<internal_hooks> = std::sync::Mutex::new(internal_hooks {
    allocate: Some(
        crate::stdlib::malloc
            as extern "C" fn(crate::__stddef_size_t_h::size_t) -> *mut ::core::ffi::c_void,
    ),
    deallocate: Some(crate::stdlib::free as extern "C" fn(*mut ::core::ffi::c_void) -> ()),
});

fn current_hooks() -> internal_hooks {
    *global_hooks
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn cJSON_InitHooks(hooks: Option<cJSON_Hooks>) {
    let mut hooks_guard = global_hooks
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    hooks_guard.allocate = Some(
        crate::stdlib::malloc
            as extern "C" fn(crate::__stddef_size_t_h::size_t) -> *mut ::core::ffi::c_void,
    );
    hooks_guard.deallocate =
        Some(crate::stdlib::free as extern "C" fn(*mut ::core::ffi::c_void) -> ());

    if let Some(hooks) = hooks {
        if hooks.malloc_fn.is_some() {
            hooks_guard.allocate = hooks.malloc_fn;
        }
        if hooks.free_fn.is_some() {
            hooks_guard.deallocate = hooks.free_fn;
        }
    }

}

fn allocation_succeeds(hooks: &internal_hooks, length: usize) -> bool {
    let allocation = hooks.allocate.expect("non-null function pointer")(length);
    if allocation.is_null() {
        return false;
    }
    hooks.deallocate.expect("non-null function pointer")(allocation);
    true
}

fn duplicate_c_string(string: &std::ffi::CStr, hooks: &internal_hooks) -> Option<Vec<u8>> {
    let bytes = string.to_bytes_with_nul();
    allocation_succeeds(hooks, bytes.len()).then(|| bytes.to_vec())
}

fn duplicate_bytes_as_c_string(bytes: &[u8], hooks: &internal_hooks) -> Option<Vec<u8>> {
    let mut copy = Vec::with_capacity(bytes.len().saturating_add(1));
    copy.extend_from_slice(bytes);
    copy.push(0);
    allocation_succeeds(hooks, copy.len()).then_some(copy)
}
#[export_name = "cJSON_InitHooks"]

pub unsafe extern "C" fn cJSON_InitHooks_ffi(mut hooks: *mut crate::src::cJSON::cJSON_Hooks) {
    cJSON_InitHooks(unsafe { hooks.as_ref().copied() })
}
#[export_name = "cJSON_Delete"]
pub unsafe extern "C" fn cJSON_Delete_ffi(mut item: *mut crate::src::cJSON::cJSON) {
    while !item.is_null() {
        let mut node = Box::from_raw(item);
        item = node.next;
        let child = if node.type_0 & crate::src::cJSON::cJSON_IsReference == 0
            && !node.child.is_null()
        {
            Some(node.child)
        } else {
            None
        };

        node.next = ::core::ptr::null_mut();
        node.prev = ::core::ptr::null_mut();
        node.child = ::core::ptr::null_mut();

        if let Some(child) = child {
            cJSON_Delete_ffi(child);
        }
        release_owned_strings(&node);
    }
}
fn parse_json_number(input: &[u8]) -> Option<(::core::ffi::c_double, usize)> {
    let mut offset = 0usize;

    if input.get(offset) == Some(&b'-') {
        offset += 1;
    }

    let integer_start = offset;
    while input.get(offset).is_some_and(u8::is_ascii_digit) {
        offset += 1;
    }
    if offset == integer_start {
        return None;
    }

    if input.get(offset) == Some(&b'.') {
        offset += 1;
        while input.get(offset).is_some_and(u8::is_ascii_digit) {
            offset += 1;
        }
    }

    if matches!(input.get(offset), Some(b'e' | b'E')) {
        let exponent_start = offset;
        offset += 1;
        if matches!(input.get(offset), Some(b'+' | b'-')) {
            offset += 1;
        }

        let digits_start = offset;
        while input.get(offset).is_some_and(u8::is_ascii_digit) {
            offset += 1;
        }
        if offset == digits_start {
            offset = exponent_start;
        }
    }

    let number = std::str::from_utf8(&input[..offset])
        .ok()?
        .parse::<::core::ffi::c_double>()
        .ok()?;
    Some((number, offset))
}

fn parse_number(item: &mut cJSON, input_buffer: &mut parse_buffer<'_>) -> cJSON_bool {
    let Some((number, consumed)) = parse_json_number(input_buffer.remaining()) else {
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
    input_buffer.offset = input_buffer.offset.wrapping_add(consumed);
    true_0
}
pub fn cJSON_SetNumberHelper(
    object: &mut crate::src::cJSON::cJSON,
    number: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if number >= crate::limits_h::INT_MAX as ::core::ffi::c_double {
        object.valueint = crate::limits_h::INT_MAX;
    } else if number <= crate::limits_h::INT_MIN as ::core::ffi::c_double {
        object.valueint = crate::limits_h::INT_MIN;
    } else {
        object.valueint = number as ::core::ffi::c_int;
    }
    object.valuedouble = number;
    object.valuedouble
}
#[export_name = "cJSON_SetNumberHelper"]

pub unsafe extern "C" fn cJSON_SetNumberHelper_ffi(
    object: *mut crate::src::cJSON::cJSON,
    number: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    cJSON_SetNumberHelper(
        unsafe { object.as_mut() }.expect("cJSON_SetNumberHelper requires a non-null object"),
        number,
    )
}
#[export_name = "cJSON_SetValuestring"]

pub unsafe extern "C" fn cJSON_SetValuestring_ffi(
    object: *mut crate::src::cJSON::cJSON,
    valuestring: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if object.is_null()
        || (*object).type_0 & crate::src::cJSON::cJSON_String == 0
        || (*object).type_0 & crate::src::cJSON::cJSON_IsReference != 0
        || (*object).valuestring.is_null()
        || valuestring.is_null()
    {
        return ::core::ptr::null_mut();
    }

    let storage_start = (*object).valuestring as usize;
    let Some(storage_len) = owned_valuestring_len(&*object) else {
        return ::core::ptr::null_mut();
    };
    let source_start = valuestring as usize;
    if source_start >= storage_start && source_start < storage_start.saturating_add(storage_len) {
        return ::core::ptr::null_mut();
    }

    let bytes = std::ffi::CStr::from_ptr(valuestring).to_bytes_with_nul().to_vec();
    let object = &mut *object;
    match replace_owned_valuestring_if_fits(object, &bytes) {
        Some(true) => return object.valuestring,
        Some(false) => {}
        None => return ::core::ptr::null_mut(),
    }

    let Some(copy) = duplicate_c_string(
        std::ffi::CStr::from_bytes_with_nul(&bytes).expect("C string bytes retain terminator"),
        &current_hooks(),
    ) else {
        return ::core::ptr::null_mut();
    };
    set_owned_valuestring(object, copy);
    object.valuestring
}
fn compare_double(
    a: ::core::ffi::c_double,
    b: ::core::ffi::c_double,
) -> crate::src::cJSON::cJSON_bool {
    let maxVal: ::core::ffi::c_double = if a.abs() > b.abs() {
        a.abs()
    } else {
        b.abs()
    };
    return ((a - b).abs() <= maxVal * crate::float_h::DBL_EPSILON)
        as ::core::ffi::c_int;
}

fn decimal_exponent(value: ::core::ffi::c_double) -> i32 {
    let bits = value.abs().to_bits();
    let exponent_bits = ((bits >> 52) & 0x7ff) as i32;
    let fraction_bits = bits & ((1_u64 << 52) - 1);
    let significand = if exponent_bits == 0 {
        fraction_bits
    } else {
        fraction_bits | (1_u64 << 52)
    };
    let binary_exponent = if exponent_bits == 0 {
        -1074
    } else {
        exponent_bits - 1023 - 52
    };
    let mut exponent = value.abs().log10().floor() as i32;

    while compare_to_power_of_ten(significand, binary_exponent, exponent)
        == ::std::cmp::Ordering::Less
    {
        exponent -= 1;
    }
    while compare_to_power_of_ten(significand, binary_exponent, exponent + 1)
        != ::std::cmp::Ordering::Less
    {
        exponent += 1;
    }
    exponent
}

fn compare_to_power_of_ten(
    significand: u64,
    binary_exponent: i32,
    decimal_exponent: i32,
) -> ::std::cmp::Ordering {
    let mut value_numerator = num_bigint::BigUint::from(significand);
    let mut value_denominator = num_bigint::BigUint::from(1_u8);
    if binary_exponent >= 0 {
        value_numerator <<= binary_exponent as usize;
    } else {
        value_denominator <<= (-binary_exponent) as usize;
    }

    let mut power_numerator = num_bigint::BigUint::from(1_u8);
    let mut power_denominator = num_bigint::BigUint::from(1_u8);
    if decimal_exponent >= 0 {
        power_numerator *= num_bigint::BigUint::from(5_u8).pow(decimal_exponent as u32);
        power_numerator <<= decimal_exponent as usize;
    } else {
        let negative_exponent = (-decimal_exponent) as usize;
        power_denominator *= num_bigint::BigUint::from(5_u8).pow(negative_exponent as u32);
        power_denominator <<= negative_exponent;
    }

    (value_numerator * power_denominator).cmp(&(power_numerator * value_denominator))
}
fn rounded_significant_digits(
    value: ::core::ffi::c_double,
    decimal_exponent: i32,
    precision: usize,
) -> (String, i32) {
    let bits = value.abs().to_bits();
    let exponent_bits = ((bits >> 52) & 0x7ff) as i32;
    let fraction_bits = bits & ((1_u64 << 52) - 1);
    let significand = if exponent_bits == 0 {
        fraction_bits
    } else {
        fraction_bits | (1_u64 << 52)
    };
    let binary_exponent = if exponent_bits == 0 {
        -1074
    } else {
        exponent_bits - 1023 - 52
    };
    let scale = precision as i32 - decimal_exponent - 1;

    let mut numerator = num_bigint::BigUint::from(significand);
    let mut denominator = num_bigint::BigUint::from(1_u8);
    if binary_exponent >= 0 {
        numerator <<= binary_exponent as usize;
    } else {
        denominator <<= (-binary_exponent) as usize;
    }
    if scale >= 0 {
        numerator *= num_bigint::BigUint::from(5_u8).pow(scale as u32);
        numerator <<= scale as usize;
    } else {
        let negative_scale = (-scale) as usize;
        denominator *= num_bigint::BigUint::from(5_u8).pow(negative_scale as u32);
        denominator <<= negative_scale;
    }

    let mut rounded = &numerator / &denominator;
    let remainder = numerator % &denominator;
    let twice_remainder = &remainder << 1_usize;
    if twice_remainder > denominator
        || (twice_remainder == denominator
            && (&rounded % 2_u8) == num_bigint::BigUint::from(1_u8))
    {
        rounded += 1_u8;
    }

    let mut digits = rounded.to_str_radix(10);
    let mut adjusted_exponent = decimal_exponent;
    if digits.len() > precision {
        digits.pop();
        adjusted_exponent += 1;
    }
    (digits, adjusted_exponent)
}

fn trim_significant_zeros(mut digits: String) -> String {
    while digits.ends_with('0') {
        digits.pop();
    }
    digits
}

fn render_significant_digits(
    value: ::core::ffi::c_double,
    precision: usize,
) -> String {
    let (digits, decimal_exponent) =
        rounded_significant_digits(value, decimal_exponent(value), precision);
    let digits = trim_significant_zeros(digits);
    let mut rendered = String::new();
    if value.is_sign_negative() {
        rendered.push('-');
    }

    if decimal_exponent < -4 || decimal_exponent >= precision as i32 {
        let mut characters = digits.chars();
        rendered.push(
            characters
                .next()
                .expect("a finite nonzero number has a significant digit"),
        );
        let fractional = characters.as_str();
        if !fractional.is_empty() {
            rendered.push('.');
            rendered.push_str(fractional);
        }
        rendered.push('e');
        if decimal_exponent >= 0 {
            rendered.push('+');
        } else {
            rendered.push('-');
        }
        let exponent_digits = decimal_exponent.unsigned_abs().to_string();
        if exponent_digits.len() == 1 {
            rendered.push('0');
        }
        rendered.push_str(&exponent_digits);
    } else {
        if decimal_exponent < 0 {
            rendered.push_str("0.");
            for _ in 0..(-decimal_exponent - 1) {
                rendered.push('0');
            }
            rendered.push_str(&digits);
        } else {
            let decimal_position = (decimal_exponent + 1) as usize;
            if decimal_position >= digits.len() {
                rendered.push_str(&digits);
                for _ in digits.len()..decimal_position {
                    rendered.push('0');
                }
            } else {
                rendered.push_str(&digits[..decimal_position]);
                rendered.push('.');
                rendered.push_str(&digits[decimal_position..]);
            }
        }
    }

    rendered
}

fn print_number(
    valueint: ::core::ffi::c_int,
    valuedouble: ::core::ffi::c_double,
    output_buffer: &mut printbuffer<'_>,
    _decimal_point: ::core::ffi::c_uchar,
) -> crate::src::cJSON::cJSON_bool {
    let number = if !valuedouble.is_finite() {
        "null".to_owned()
    } else if valuedouble == valueint as ::core::ffi::c_double {
        valueint.to_string()
    } else {
        let candidate = render_significant_digits(valuedouble, 15);
        if matches!(candidate.parse::<::core::ffi::c_double>(), Ok(test) if compare_double(test, valuedouble) != 0) {
            candidate
        } else {
            render_significant_digits(valuedouble, 17)
        }
    };

    output_buffer.write(number.as_bytes()) as cJSON_bool
}
fn parse_hex4(input: &[u8]) -> ::core::ffi::c_uint {
    if input.len() < 4 {
        return 0;
    }

    let mut value = 0;
    for byte in &input[..4] {
        let digit = match *byte {
            b'0'..=b'9' => (*byte - b'0') as ::core::ffi::c_uint,
            b'A'..=b'F' => (*byte - b'A' + 10) as ::core::ffi::c_uint,
            b'a'..=b'f' => (*byte - b'a' + 10) as ::core::ffi::c_uint,
            _ => return 0,
        };
        value = (value << 4) | digit;
    }
    value
}

fn decode_utf16_literal(input: &[u8], output: &mut [u8]) -> Option<(usize, usize)> {
    if input.len() < 6 {
        return None;
    }

    let first_code = parse_hex4(&input[2..6]);
    if (0xdc00..=0xdfff).contains(&first_code) {
        return None;
    }

    let (codepoint, sequence_length) = if (0xd800..=0xdbff).contains(&first_code) {
        if input.len() < 12 || input[6] != b'\\' || input[7] != b'u' {
            return None;
        }

        let second_code = parse_hex4(&input[8..12]);
        if !(0xdc00..=0xdfff).contains(&second_code) {
            return None;
        }

        (
            0x10000 + ((first_code & 0x3ff) << 10 | (second_code & 0x3ff)),
            12,
        )
    } else {
        (first_code, 6)
    };

    let character = char::from_u32(codepoint)?;
    let mut encoded = [0; 4];
    let encoded = character.encode_utf8(&mut encoded).as_bytes();
    let destination = output.get_mut(..encoded.len())?;
    destination.copy_from_slice(encoded);
    Some((sequence_length, encoded.len()))
}

fn decode_json_string(input: &[u8]) -> Result<(Vec<u8>, usize, usize), usize> {
    if input.first() != Some(&b'"') {
        return Err(1);
    }

    let mut input_end = 1;
    let mut skipped_bytes = 0;
    while input_end < input.len() && input[input_end] != b'"' {
        if input[input_end] == b'\\' {
            if input_end + 1 >= input.len() {
                return Err(1);
            }
            skipped_bytes += 1;
            input_end += 1;
        }
        input_end += 1;
    }
    if input_end >= input.len() {
        return Err(1);
    }

    let mut output = Vec::with_capacity(input_end - 1);
    let mut input_pointer = 1;
    while input_pointer < input_end {
        let byte = input[input_pointer];
        if byte != b'\\' {
            output.push(byte);
            input_pointer += 1;
            continue;
        }

        let Some(&escape) = input.get(input_pointer + 1) else {
            return Err(input_pointer);
        };
        match escape {
            b'b' => output.push(b'\x08'),
            b'f' => output.push(b'\x0c'),
            b'n' => output.push(b'\n'),
            b'r' => output.push(b'\r'),
            b't' => output.push(b'\t'),
            b'"' | b'\\' | b'/' => output.push(escape),
            b'u' => {
                let mut encoded = [0; 4];
                let Some((sequence_length, output_length)) =
                    decode_utf16_literal(&input[input_pointer..input_end], &mut encoded)
                else {
                    return Err(input_pointer);
                };
                output.extend_from_slice(&encoded[..output_length]);
                input_pointer += sequence_length;
                continue;
            }
            _ => return Err(input_pointer),
        }
        input_pointer += 2;
    }

    Ok((output, input_end + 1, input_end - skipped_bytes))
}

struct parsed_json_string {
    bytes: Vec<u8>,
    consumed: usize,
}

fn decode_parsed_string(input_buffer: &mut parse_buffer<'_>) -> Option<parsed_json_string> {
    let input = input_buffer.remaining();
    let (bytes, consumed, _) = match decode_json_string(input) {
        Ok(decoded) => decoded,
        Err(error_offset) => {
            input_buffer.offset = input_buffer.offset.wrapping_add(error_offset);
            return None;
        }
    };

    Some(parsed_json_string {
        bytes,
        consumed,
    })
}

fn parse_string(
    item: &mut cJSON,
    input_buffer: &mut parse_buffer<'_>,
) -> crate::src::cJSON::cJSON_bool {
    let Some(parsed) = decode_parsed_string(input_buffer) else {
        return false_0;
    };
    let Some(bytes) = duplicate_bytes_as_c_string(&parsed.bytes, &input_buffer.hooks) else {
        input_buffer.offset = input_buffer.offset.wrapping_add(1);
        return false_0;
    };
    item.type_0 = crate::src::cJSON::cJSON_String;
    set_owned_valuestring(item, bytes);
    input_buffer.offset = input_buffer.offset.wrapping_add(parsed.consumed);
    true_0
}

fn print_string(
    input: &[u8],
    output_buffer: &mut printbuffer<'_>,
) -> crate::src::cJSON::cJSON_bool {
    let mut escaped = Vec::with_capacity(input.len().saturating_add(2));
    escaped.push(b'"');

    for &byte in input {
        match byte {
            b'"' => escaped.extend_from_slice(b"\\\""),
            b'\\' => escaped.extend_from_slice(b"\\\\"),
            b'\x08' => escaped.extend_from_slice(b"\\b"),
            b'\x0c' => escaped.extend_from_slice(b"\\f"),
            b'\n' => escaped.extend_from_slice(b"\\n"),
            b'\r' => escaped.extend_from_slice(b"\\r"),
            b'\t' => escaped.extend_from_slice(b"\\t"),
            0..=31 => {
                const HEX: &[u8; 16] = b"0123456789abcdef";
                escaped.extend_from_slice(b"\\u00");
                escaped.push(HEX[(byte >> 4) as usize]);
                escaped.push(HEX[(byte & 0x0f) as usize]);
            }
            _ => escaped.push(byte),
        }
    }

    escaped.push(b'"');
    output_buffer.write(&escaped) as cJSON_bool
}

fn skip_whitespace(buffer: &mut parse_buffer<'_>) {
    if buffer.offset >= buffer.length {
        return;
    }
    while buffer.offset < buffer.length && buffer.current().is_some_and(|byte| byte <= 32) {
        buffer.offset = buffer.offset.wrapping_add(1);
    }
    if buffer.offset == buffer.length {
        buffer.offset = buffer.offset.wrapping_sub(1);
    }
}


fn skip_bom(buffer: &mut parse_buffer<'_>) {
    if buffer.offset != 0 {
        return;
    }
    if buffer.offset.wrapping_add(4) < buffer.length && buffer.starts_with(b"\xEF\xBB\xBF") {
        buffer.offset = buffer.offset.wrapping_add(3);
    }
}

#[export_name = "cJSON_ParseWithOpts"]

pub unsafe extern "C" fn cJSON_ParseWithOpts_ffi(
    mut value: *const ::core::ffi::c_char,
    mut return_parse_end: *mut *const ::core::ffi::c_char,
    mut require_null_terminated: crate::src::cJSON::cJSON_bool,
) -> *mut crate::src::cJSON::cJSON {
    if value.is_null() {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    }
    let buffer_length = crate::stdlib::strlen(value)
        .wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 1]>());
    cJSON_ParseWithLengthOpts_ffi(value, buffer_length, return_parse_end, require_null_terminated)
}
#[export_name = "cJSON_ParseWithLengthOpts"]

pub unsafe extern "C" fn cJSON_ParseWithLengthOpts_ffi(
    mut value: *const ::core::ffi::c_char,
    mut buffer_length: crate::__stddef_size_t_h::size_t,
    mut return_parse_end: *mut *const ::core::ffi::c_char,
    mut require_null_terminated: crate::src::cJSON::cJSON_bool,
) -> *mut crate::src::cJSON::cJSON {
    let mut buffer: parse_buffer = parse_buffer {
        content: &[],
        length: 0 as crate::__stddef_size_t_h::size_t,
        offset: 0 as crate::__stddef_size_t_h::size_t,
        depth: 0 as crate::__stddef_size_t_h::size_t,
        hooks: internal_hooks {
            allocate: None,
            deallocate: None,
        },
    };
    let mut item: Option<parsed_tree> = None;
    global_error.json.store(
        0,
        std::sync::atomic::Ordering::Relaxed,
    );
    global_error
        .position
        .store(0 as crate::__stddef_size_t_h::size_t, std::sync::atomic::Ordering::Relaxed);
    '_fail: {
        if !(value.is_null() || 0 as crate::__stddef_size_t_h::size_t == buffer_length) {
            buffer.content = ::core::slice::from_raw_parts(value.cast::<u8>(), buffer_length);
            buffer.length = buffer_length;
            buffer.offset = 0 as crate::__stddef_size_t_h::size_t;
            buffer.hooks = current_hooks();
            item = parsed_tree::new(&buffer.hooks);
            if let Some(parsed) = item.as_mut() {
                skip_bom(&mut buffer);
                skip_whitespace(&mut buffer);
                if parse_value(parsed, 0, &mut buffer) != 0 {
                    if require_null_terminated != 0 {
                        skip_whitespace(&mut buffer);
                        if buffer.offset >= buffer.length
                            || buffer.current() != Some(b'\0')
                        {
                            break '_fail;
                        }
                    }
                    if !return_parse_end.is_null() {
                        *return_parse_end = buffer.content.as_ptr().wrapping_add(buffer.offset)
                            as *const ::core::ffi::c_char;
                    }
                    let mut parsed = item.take().expect("parsed item is present");
                    let mut nodes = Vec::with_capacity(parsed.nodes.len());
                    for node in &mut parsed.nodes {
                        nodes.push(node.as_mut() as *mut cJSON);
                    }
                    let parsed_tree { nodes: owned_nodes, children } = &mut parsed;
                    for index in 0..owned_nodes.len() {
                        let child_indexes = &children[index];
                        let head = match child_indexes.first() {
                            Some(child) => nodes[*child],
                            None => ::core::ptr::null_mut(),
                        };
                        let tail = match child_indexes.last() {
                            Some(child) => nodes[*child],
                            None => ::core::ptr::null_mut(),
                        };
                        owned_nodes[index].child = head;
                        for (position, child) in child_indexes.iter().enumerate() {
                            let child_node = owned_nodes[*child].as_mut();
                            child_node.prev = if position == 0 {
                                tail
                            } else {
                                nodes[child_indexes[position - 1]]
                            };
                            child_node.next = match child_indexes.get(position + 1) {
                                Some(next) => nodes[*next],
                                None => ::core::ptr::null_mut(),
                            };
                        }
                    }
                    let root = nodes[0];
                    for node in parsed.nodes {
                        let _ = Box::into_raw(node);
                    }
                    return root;
                }
            }
        }
    }
    if let Some(parsed) = item.as_ref() {
        parsed.release_owned_strings();
    }
    if !value.is_null() {
        let mut error_position = 0 as crate::__stddef_size_t_h::size_t;
        if buffer.offset < buffer.length {
            error_position = buffer.offset;
        } else if buffer.length > 0 as crate::__stddef_size_t_h::size_t {
            error_position = buffer
                .length
                .wrapping_sub(1 as crate::__stddef_size_t_h::size_t);
        }
        if !return_parse_end.is_null() {
            *return_parse_end = value.wrapping_add(error_position);
        }
        global_error.json.store(
            value as usize,
            std::sync::atomic::Ordering::Relaxed,
        );
        global_error.position.store(
            error_position,
            std::sync::atomic::Ordering::Relaxed,
        );
    }
    return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
}
#[export_name = "cJSON_Parse"]

pub unsafe extern "C" fn cJSON_Parse_ffi(
    mut value: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    cJSON_ParseWithOpts_ffi(
        value,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        0 as crate::src::cJSON::cJSON_bool,
    )
}
#[export_name = "cJSON_ParseWithLength"]

pub unsafe extern "C" fn cJSON_ParseWithLength_ffi(
    mut value: *const ::core::ffi::c_char,
    mut buffer_length: crate::__stddef_size_t_h::size_t,
) -> *mut crate::src::cJSON::cJSON {
    cJSON_ParseWithLengthOpts_ffi(
        value,
        buffer_length,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        0 as crate::src::cJSON::cJSON_bool,
    )
}
enum printable_node_kind {
    Null,
    False,
    True,
    Number {
        valueint: ::core::ffi::c_int,
        valuedouble: ::core::ffi::c_double,
    },
    Raw(Vec<u8>),
    String(Vec<u8>),
    Array,
    Object,
}

struct printable_node {
    kind: printable_node_kind,
    key: Vec<u8>,
    first_child: Option<usize>,
    next: Option<usize>,
}

struct printable_document {
    nodes: Vec<printable_node>,
}

enum comparable_node_kind {
    Null,
    False,
    True,
    Number(::core::ffi::c_double),
    Raw(Option<Vec<u8>>),
    String(Option<Vec<u8>>),
    Array,
    Object,
}

struct comparable_node {
    kind: comparable_node_kind,
    key: Option<Vec<u8>>,
    first_child: Option<usize>,
    next: Option<usize>,
}

struct comparable_document {
    nodes: Vec<comparable_node>,
}

impl comparable_document {
    fn object_item(&self, parent: usize, name: &[u8], case_sensitive: bool) -> Option<usize> {
        let mut current = self.nodes.get(parent)?.first_child;
        while let Some(index) = current {
            let item = self.nodes.get(index)?;
            if let Some(key) = item.key.as_deref() {
                let matches = if case_sensitive {
                    key == name
                } else {
                    case_insensitive_strcmp(name, key) == 0
                };
                if matches {
                    return Some(index);
                }
            }
            current = item.next;
        }
        None
    }

    fn compare(&self, other: &Self, case_sensitive: bool) -> bool {
        self.compare_nodes(other, 0, 0, case_sensitive)
    }

    fn compare_nodes(
        &self,
        other: &Self,
        left: usize,
        right: usize,
        case_sensitive: bool,
    ) -> bool {
        let Some(left_item) = self.nodes.get(left) else {
            return false;
        };
        let Some(right_item) = other.nodes.get(right) else {
            return false;
        };

        match (&left_item.kind, &right_item.kind) {
            (comparable_node_kind::False, comparable_node_kind::False)
            | (comparable_node_kind::True, comparable_node_kind::True)
            | (comparable_node_kind::Null, comparable_node_kind::Null) => true,
            (comparable_node_kind::Number(left), comparable_node_kind::Number(right)) => {
                compare_double(*left, *right) != 0
            }
            (comparable_node_kind::Raw(Some(left)), comparable_node_kind::Raw(Some(right)))
            | (comparable_node_kind::String(Some(left)), comparable_node_kind::String(Some(right))) => {
                left == right
            }
            (comparable_node_kind::Array, comparable_node_kind::Array) => {
                let mut left_child = left_item.first_child;
                let mut right_child = right_item.first_child;
                while let (Some(left_index), Some(right_index)) = (left_child, right_child) {
                    if !self.compare_nodes(other, left_index, right_index, case_sensitive) {
                        return false;
                    }
                    let Some(left_node) = self.nodes.get(left_index) else {
                        return false;
                    };
                    let Some(right_node) = other.nodes.get(right_index) else {
                        return false;
                    };
                    left_child = left_node.next;
                    right_child = right_node.next;
                }
                left_child == right_child
            }
            (comparable_node_kind::Object, comparable_node_kind::Object) => {
                if !self.compare_object_nodes(other, left, right, case_sensitive) {
                    return false;
                }
                other.compare_object_nodes(self, right, left, case_sensitive)
            }
            _ => false,
        }
    }

    fn compare_object_nodes(
        &self,
        other: &Self,
        left: usize,
        right: usize,
        case_sensitive: bool,
    ) -> bool {
        let Some(left_item) = self.nodes.get(left) else {
            return false;
        };
        let mut current = left_item.first_child;
        while let Some(left_index) = current {
            let Some(left_node) = self.nodes.get(left_index) else {
                return false;
            };
            let Some(key) = left_node.key.as_deref() else {
                return false;
            };
            let Some(right_index) = other.object_item(right, key, case_sensitive) else {
                return false;
            };
            if !self.compare_nodes(other, left_index, right_index, case_sensitive) {
                return false;
            }
            current = left_node.next;
        }
        true
    }
}


struct lookup_node<'a> {
    item: &'a cJSON,
    key: Option<Vec<u8>>,
}

struct lookup_document<'a> {
    nodes: Vec<lookup_node<'a>>,
}

impl<'a> lookup_document<'a> {
    fn array_size(&self) -> ::core::ffi::c_int {
        self.nodes.len() as ::core::ffi::c_int
    }

    fn array_item(&self, index: ::core::ffi::c_int) -> Option<&'a cJSON> {
        if index < 0 {
            return None;
        }
        self.nodes.get(index as usize).map(|node| node.item)
    }

    fn object_item(&self, name: &[u8], case_sensitive: bool) -> Option<&'a cJSON> {
        self.nodes.iter().find_map(|node| {
            let key = node.key.as_deref()?;
            let matches = if case_sensitive {
                key == name
            } else {
                case_insensitive_strcmp(name, key) == 0
            };
            matches.then_some(node.item)
        })
    }
}

macro_rules! snapshot_lookup_document {
    ($item:expr) => {{
        let mut nodes = Vec::new();
        let mut current = $item.child;
        while let Some(child) = current.as_ref() {
            let key = if child.string.is_null() {
                None
            } else {
                Some(std::ffi::CStr::from_ptr(child.string).to_bytes().to_vec())
            };
            nodes.push(lookup_node { item: child, key });
            current = child.next;
        }
        lookup_document { nodes }
    }};
}

macro_rules! snapshot_print_node {
    ($item:expr) => {{
        let item = $item;
        let kind = match item.type_0 & 0xff {
            cJSON_NULL => Some(printable_node_kind::Null),
            cJSON_False => Some(printable_node_kind::False),
            cJSON_True => Some(printable_node_kind::True),
            cJSON_Number => Some(printable_node_kind::Number {
                valueint: item.valueint,
                valuedouble: item.valuedouble,
            }),
            cJSON_Raw if !item.valuestring.is_null() => Some(printable_node_kind::Raw(
                std::ffi::CStr::from_ptr(item.valuestring).to_bytes().to_vec(),
            )),
            cJSON_String => Some(printable_node_kind::String(
                if item.valuestring.is_null() {
                    &[][..]
                } else {
                    std::ffi::CStr::from_ptr(item.valuestring).to_bytes()
                }
                .to_vec(),
            )),
            cJSON_Array => Some(printable_node_kind::Array),
            cJSON_Object => Some(printable_node_kind::Object),
            _ => None,
        };
        match kind {
            Some(kind) => Some(printable_node {
                kind,
                key: if item.string.is_null() {
                    Vec::new()
                } else {
                    std::ffi::CStr::from_ptr(item.string).to_bytes().to_vec()
                },
                first_child: None,
                next: None,
            }),
            None => None,
        }
    }};
}

macro_rules! snapshot_print_document {
    ($root:expr) => {{
        'snapshot: {
            let root = $root;
            let Some(root_node) = snapshot_print_node!(root) else {
                break 'snapshot None;
            };
            let mut document = printable_document {
                nodes: vec![root_node],
            };
            let mut pending = vec![(0usize, root.child)];
            while let Some((parent, mut current)) = pending.pop() {
                let mut previous: Option<usize> = None;
                while let Some(child) = current.as_ref() {
                    let index = document.nodes.len();
                    let Some(node) = snapshot_print_node!(child) else {
                        break 'snapshot None;
                    };
                    document.nodes.push(node);
                    if let Some(previous) = previous {
                        document.nodes[previous].next = Some(index);
                    } else {
                        document.nodes[parent].first_child = Some(index);
                    }
                    pending.push((index, child.child));
                    previous = Some(index);
                    current = child.next;
                }
            }
            Some(document)
        }
    }};
}


macro_rules! snapshot_compare_node {
    ($item:expr) => {{
        let item = $item;
        let kind = match item.type_0 & 0xff {
            cJSON_NULL => Some(comparable_node_kind::Null),
            cJSON_False => Some(comparable_node_kind::False),
            cJSON_True => Some(comparable_node_kind::True),
            cJSON_Number => Some(comparable_node_kind::Number(item.valuedouble)),
            cJSON_Raw => Some(comparable_node_kind::Raw(if item.valuestring.is_null() {
                None
            } else {
                Some(std::ffi::CStr::from_ptr(item.valuestring).to_bytes().to_vec())
            })),
            cJSON_String => Some(comparable_node_kind::String(if item.valuestring.is_null() {
                None
            } else {
                Some(std::ffi::CStr::from_ptr(item.valuestring).to_bytes().to_vec())
            })),
            cJSON_Array => Some(comparable_node_kind::Array),
            cJSON_Object => Some(comparable_node_kind::Object),
            _ => None,
        };
        match kind {
            Some(kind) => Some(comparable_node {
                kind,
                key: if item.string.is_null() {
                    None
                } else {
                    Some(std::ffi::CStr::from_ptr(item.string).to_bytes().to_vec())
                },
                first_child: None,
                next: None,
            }),
            None => None,
        }
    }};
}

macro_rules! snapshot_compare_document {
    ($root:expr) => {{
        'snapshot: {
            let root = $root;
            let Some(root_node) = snapshot_compare_node!(root) else {
                break 'snapshot None;
            };
            let mut document = comparable_document {
                nodes: vec![root_node],
            };
            let mut pending = vec![(0usize, root.child)];
            while let Some((parent, mut current)) = pending.pop() {
                let mut previous: Option<usize> = None;
                while let Some(child) = current.as_ref() {
                    let index = document.nodes.len();
                    let Some(node) = snapshot_compare_node!(child) else {
                        break 'snapshot None;
                    };
                    document.nodes.push(node);
                    if let Some(previous) = previous {
                        document.nodes[previous].next = Some(index);
                    } else {
                        document.nodes[parent].first_child = Some(index);
                    }
                    pending.push((index, child.child));
                    previous = Some(index);
                    current = child.next;
                }
            }
            Some(document)
        }
    }};
}

fn print(
    document: &printable_document,
    format: crate::src::cJSON::cJSON_bool,
    initial_capacity: usize,
    decimal_point: ::core::ffi::c_uchar,
) -> Option<Vec<u8>> {
    let mut buffer = printbuffer::owned(initial_capacity, format);
    (print_value(document, 0, &mut buffer, decimal_point) != 0).then(|| buffer.into_bytes()).flatten()
}

#[export_name = "cJSON_Print"]
pub unsafe extern "C" fn cJSON_Print_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> *mut ::core::ffi::c_char {
    let decimal_point = *(*crate::stdlib::localeconv()).decimal_point as ::core::ffi::c_uchar;
    let Some(item) = item.as_ref() else {
        return ::core::ptr::null_mut();
    };
    let Some(snapshot) = snapshot_print_document!(item) else {
        return ::core::ptr::null_mut();
    };
    let Some(bytes) = print(&snapshot, true_0, 256, decimal_point) else {
        return ::core::ptr::null_mut();
    };
    let output = current_hooks().allocate.expect("non-null function pointer")(bytes.len()).cast::<::core::ffi::c_char>();
    if output.is_null() {
        return ::core::ptr::null_mut();
    }
    ::core::ptr::copy_nonoverlapping(bytes.as_ptr(), output.cast::<u8>(), bytes.len());
    output
}

#[export_name = "cJSON_PrintUnformatted"]
pub unsafe extern "C" fn cJSON_PrintUnformatted_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> *mut ::core::ffi::c_char {
    let decimal_point = *(*crate::stdlib::localeconv()).decimal_point as ::core::ffi::c_uchar;
    let Some(item) = item.as_ref() else {
        return ::core::ptr::null_mut();
    };
    let Some(snapshot) = snapshot_print_document!(item) else {
        return ::core::ptr::null_mut();
    };
    let Some(bytes) = print(&snapshot, false_0, 256, decimal_point) else {
        return ::core::ptr::null_mut();
    };
    let output = current_hooks().allocate.expect("non-null function pointer")(bytes.len()).cast::<::core::ffi::c_char>();
    if output.is_null() {
        return ::core::ptr::null_mut();
    }
    ::core::ptr::copy_nonoverlapping(bytes.as_ptr(), output.cast::<u8>(), bytes.len());
    output
}

#[export_name = "cJSON_PrintBuffered"]
pub unsafe extern "C" fn cJSON_PrintBuffered_ffi(
    item: *const crate::src::cJSON::cJSON,
    prebuffer: ::core::ffi::c_int,
    format: crate::src::cJSON::cJSON_bool,
) -> *mut ::core::ffi::c_char {
    if prebuffer < 0 {
        return ::core::ptr::null_mut();
    }
    let decimal_point = *(*crate::stdlib::localeconv()).decimal_point as ::core::ffi::c_uchar;
    let Some(item) = item.as_ref() else {
        return ::core::ptr::null_mut();
    };
    let Some(snapshot) = snapshot_print_document!(item) else {
        return ::core::ptr::null_mut();
    };
    let Some(bytes) = print(&snapshot, format, prebuffer as usize, decimal_point) else {
        return ::core::ptr::null_mut();
    };
    let output = current_hooks().allocate.expect("non-null function pointer")(bytes.len()).cast::<::core::ffi::c_char>();
    if output.is_null() {
        return ::core::ptr::null_mut();
    }
    ::core::ptr::copy_nonoverlapping(bytes.as_ptr(), output.cast::<u8>(), bytes.len());
    output
}

#[export_name = "cJSON_PrintPreallocated"]
pub unsafe extern "C" fn cJSON_PrintPreallocated_ffi(
    item: *mut crate::src::cJSON::cJSON,
    buffer: *mut ::core::ffi::c_char,
    length: ::core::ffi::c_int,
    format: crate::src::cJSON::cJSON_bool,
) -> crate::src::cJSON::cJSON_bool {
    if length < 0 || buffer.is_null() {
        return false_0;
    }
    let decimal_point = *(*crate::stdlib::localeconv()).decimal_point as ::core::ffi::c_uchar;
    let output = std::slice::from_raw_parts_mut(buffer.cast::<u8>(), length as usize);
    let mut print_buffer = printbuffer::borrowed(output, format);
    match item.as_ref() {
        Some(item) => match snapshot_print_document!(item) {
            Some(snapshot) => print_value(&snapshot, 0, &mut print_buffer, decimal_point),
            None => false_0,
        },
        None => false_0,
    }
}

enum value_parse_action {
    Complete(cJSON_bool),
    String,
    Number,
    Array,
    Object,
}

fn parse_value_safe(item: &mut cJSON, input_buffer: &mut parse_buffer<'_>) -> value_parse_action {
    if input_buffer.starts_with(b"null") {
        item.type_0 = crate::src::cJSON::cJSON_NULL;
        input_buffer.offset = input_buffer.offset.wrapping_add(4);
        return value_parse_action::Complete(true_0);
    }
    if input_buffer.starts_with(b"false") {
        item.type_0 = crate::src::cJSON::cJSON_False;
        input_buffer.offset = input_buffer.offset.wrapping_add(5);
        return value_parse_action::Complete(true_0);
    }
    if input_buffer.starts_with(b"true") {
        item.type_0 = crate::src::cJSON::cJSON_True;
        item.valueint = 1;
        input_buffer.offset = input_buffer.offset.wrapping_add(4);
        return value_parse_action::Complete(true_0);
    }

    match input_buffer.current() {
        Some(b'"') => value_parse_action::String,
        Some(b'-' | b'0'..=b'9') => value_parse_action::Number,
        Some(b'[') => value_parse_action::Array,
        Some(b'{') => value_parse_action::Object,
        _ => value_parse_action::Complete(false_0),
    }
}
struct parsed_tree {
    nodes: Vec<Box<cJSON>>,
    children: Vec<Vec<usize>>,
}

impl parsed_tree {
    fn new(hooks: &internal_hooks) -> Option<Self> {
        if !allocation_succeeds(hooks, ::core::mem::size_of::<cJSON>()) {
            return None;
        }
        let mut nodes = Vec::new();
        nodes.push(Box::new(cJSON::null()));
        let mut children = Vec::new();
        children.push(Vec::new());
        Some(Self { nodes, children })
    }

    fn add_node(&mut self, hooks: &internal_hooks) -> Option<usize> {
        if !allocation_succeeds(hooks, ::core::mem::size_of::<cJSON>()) {
            return None;
        }
        let index = self.nodes.len();
        self.nodes.push(Box::new(cJSON::null()));
        self.children.push(Vec::new());
        Some(index)
    }

    fn node_mut(&mut self, index: usize) -> Option<&mut cJSON> {
        self.nodes.get_mut(index).map(Box::as_mut)
    }

    fn append_child(&mut self, parent: usize, child: usize) -> bool {
        let Some(children) = self.children.get_mut(parent) else {
            return false;
        };
        children.push(child);
        true
    }

    fn release_owned_strings(&self) {
        for node in &self.nodes {
            crate::src::cJSON::release_owned_strings(node.as_ref());
        }
    }
}

fn print_value(
    document: &printable_document,
    index: usize,
    output_buffer: &mut printbuffer<'_>,
    decimal_point: ::core::ffi::c_uchar,
) -> crate::src::cJSON::cJSON_bool {
    let Some(item) = document.nodes.get(index) else {
        return false_0;
    };
    match &item.kind {
        printable_node_kind::Null => output_buffer.write(b"null") as cJSON_bool,
        printable_node_kind::False => output_buffer.write(b"false") as cJSON_bool,
        printable_node_kind::True => output_buffer.write(b"true") as cJSON_bool,
        printable_node_kind::Number {
            valueint,
            valuedouble,
        } => print_number(*valueint, *valuedouble, output_buffer, decimal_point),
        printable_node_kind::Raw(raw) => output_buffer.write(raw) as cJSON_bool,
        printable_node_kind::String(string) => print_string(string, output_buffer),
        printable_node_kind::Array => print_array(document, item.first_child, output_buffer, decimal_point),
        printable_node_kind::Object => print_object(document, item.first_child, output_buffer, decimal_point),
    }
}

fn parse_value(
    tree: &mut parsed_tree,
    index: usize,
    input_buffer: &mut parse_buffer<'_>,
) -> crate::src::cJSON::cJSON_bool {
    let Some(item) = tree.node_mut(index) else {
        return false_0;
    };
    match parse_value_safe(item, input_buffer) {
        value_parse_action::Complete(result) => result,
        value_parse_action::String => {
            let Some(item) = tree.node_mut(index) else {
                return false_0;
            };
            parse_string(item, input_buffer)
        }
        value_parse_action::Number => {
            let Some(item) = tree.node_mut(index) else {
                return false_0;
            };
            parse_number(item, input_buffer)
        }
        value_parse_action::Array => parse_array(tree, index, input_buffer),
        value_parse_action::Object => parse_object(tree, index, input_buffer),
    }
}

fn parse_array(
    tree: &mut parsed_tree,
    index: usize,
    input_buffer: &mut parse_buffer<'_>,
) -> crate::src::cJSON::cJSON_bool {
    if input_buffer.depth >= crate::src::cJSON::CJSON_NESTING_LIMIT as usize
        || input_buffer.current() != Some(b'[')
    {
        return false_0;
    }

    input_buffer.depth = input_buffer.depth.wrapping_add(1);
    let result = 'parse: {
        input_buffer.offset = input_buffer.offset.wrapping_add(1);
        skip_whitespace(input_buffer);
        if input_buffer.current() == Some(b']') {
            let Some(item) = tree.node_mut(index) else {
                break 'parse false_0;
            };
            item.type_0 = crate::src::cJSON::cJSON_Array;
            input_buffer.offset = input_buffer.offset.wrapping_add(1);
            break 'parse true_0;
        }
        if input_buffer.offset >= input_buffer.length {
            input_buffer.offset = input_buffer.offset.wrapping_sub(1);
            break 'parse false_0;
        }

        input_buffer.offset = input_buffer.offset.wrapping_sub(1);
        loop {
            let Some(child) = tree.add_node(&input_buffer.hooks) else {
                break 'parse false_0;
            };
            input_buffer.offset = input_buffer.offset.wrapping_add(1);
            skip_whitespace(input_buffer);
            if parse_value(tree, child, input_buffer) == 0 {
                break 'parse false_0;
            }
            if !tree.append_child(index, child) {
                break 'parse false_0;
            }
            skip_whitespace(input_buffer);
            if input_buffer.current() != Some(b',') {
                break;
            }
        }

        if input_buffer.current() != Some(b']') {
            break 'parse false_0;
        }
        let Some(item) = tree.node_mut(index) else {
            break 'parse false_0;
        };
        item.type_0 = crate::src::cJSON::cJSON_Array;
        input_buffer.offset = input_buffer.offset.wrapping_add(1);
        true_0
    };
    input_buffer.depth = input_buffer.depth.wrapping_sub(1);
    result
}

fn parse_object(
    tree: &mut parsed_tree,
    index: usize,
    input_buffer: &mut parse_buffer<'_>,
) -> crate::src::cJSON::cJSON_bool {
    if input_buffer.depth >= crate::src::cJSON::CJSON_NESTING_LIMIT as usize
        || input_buffer.current() != Some(b'{')
    {
        return false_0;
    }

    input_buffer.depth = input_buffer.depth.wrapping_add(1);
    let result = 'parse: {
        input_buffer.offset = input_buffer.offset.wrapping_add(1);
        skip_whitespace(input_buffer);
        if input_buffer.current() == Some(b'}') {
            let Some(item) = tree.node_mut(index) else {
                break 'parse false_0;
            };
            item.type_0 = crate::src::cJSON::cJSON_Object;
            input_buffer.offset = input_buffer.offset.wrapping_add(1);
            break 'parse true_0;
        }
        if input_buffer.offset >= input_buffer.length {
            input_buffer.offset = input_buffer.offset.wrapping_sub(1);
            break 'parse false_0;
        }

        input_buffer.offset = input_buffer.offset.wrapping_sub(1);
        loop {
            let Some(child) = tree.add_node(&input_buffer.hooks) else {
                break 'parse false_0;
            };
            if input_buffer.offset.wrapping_add(1) >= input_buffer.length {
                break 'parse false_0;
            }
            input_buffer.offset = input_buffer.offset.wrapping_add(1);
            skip_whitespace(input_buffer);
            let Some(item) = tree.node_mut(child) else {
                break 'parse false_0;
            };
            if parse_string(item, input_buffer) == 0 {
                break 'parse false_0;
            }
            skip_whitespace(input_buffer);
            let Some(item) = tree.node_mut(child) else {
                break 'parse false_0;
            };
            move_valuestring_to_string(item);
            if input_buffer.current() != Some(b':') {
                break 'parse false_0;
            }
            input_buffer.offset = input_buffer.offset.wrapping_add(1);
            skip_whitespace(input_buffer);
            if parse_value(tree, child, input_buffer) == 0 {
                break 'parse false_0;
            }
            if !tree.append_child(index, child) {
                break 'parse false_0;
            }
            skip_whitespace(input_buffer);
            if input_buffer.current() != Some(b',') {
                break;
            }
        }

        if input_buffer.current() != Some(b'}') {
            break 'parse false_0;
        }
        let Some(item) = tree.node_mut(index) else {
            break 'parse false_0;
        };
        item.type_0 = crate::src::cJSON::cJSON_Object;
        input_buffer.offset = input_buffer.offset.wrapping_add(1);
        true_0
    };
    input_buffer.depth = input_buffer.depth.wrapping_sub(1);
    result
}

fn print_array(
    document: &printable_document,
    mut current: Option<usize>,
    output_buffer: &mut printbuffer<'_>,
    decimal_point: ::core::ffi::c_uchar,
) -> crate::src::cJSON::cJSON_bool {
    if !output_buffer.write(b"[") {
        return false_0;
    }
    output_buffer.depth = output_buffer.depth.wrapping_add(1);
    while let Some(index) = current {
        if print_value(document, index, output_buffer, decimal_point) == 0 {
            return false_0;
        }
        let Some(item) = document.nodes.get(index) else {
            return false_0;
        };
        current = item.next;
        if current.is_some()
            && !output_buffer.write(if output_buffer.format != 0 { b", " } else { b"," })
        {
            return false_0;
        }
    }
    if !output_buffer.write(b"]") {
        return false_0;
    }
    output_buffer.depth = output_buffer.depth.wrapping_sub(1);
    true_0
}

fn print_object(
    document: &printable_document,
    mut current: Option<usize>,
    output_buffer: &mut printbuffer<'_>,
    decimal_point: ::core::ffi::c_uchar,
) -> crate::src::cJSON::cJSON_bool {
    if !output_buffer.write(if output_buffer.format != 0 { b"{\n" } else { b"{" }) {
        return false_0;
    }
    output_buffer.depth = output_buffer.depth.wrapping_add(1);
    while let Some(index) = current {
        let Some(item) = document.nodes.get(index) else {
            return false_0;
        };
        if output_buffer.format != 0
            && !output_buffer.write(&vec![b'\t'; output_buffer.depth])
        {
            return false_0;
        }
        if print_string(&item.key, output_buffer) == 0 {
            return false_0;
        }
        if !output_buffer.write(if output_buffer.format != 0 { b":\t" } else { b":" }) {
            return false_0;
        }
        if print_value(document, index, output_buffer, decimal_point) == 0 {
            return false_0;
        }
        current = item.next;
        if current.is_some() && !output_buffer.write(b",") {
            return false_0;
        }
        if output_buffer.format != 0 && !output_buffer.write(b"\n") {
            return false_0;
        }
    }
    if output_buffer.format != 0
        && !output_buffer.write(&vec![b'\t'; output_buffer.depth.wrapping_sub(1)])
    {
        return false_0;
    }
    if !output_buffer.write(b"}") {
        return false_0;
    }
    output_buffer.depth = output_buffer.depth.wrapping_sub(1);
    true_0
}

#[export_name = "cJSON_GetArraySize"]

pub unsafe extern "C" fn cJSON_GetArraySize_ffi(
    array: *const crate::src::cJSON::cJSON,
) -> ::core::ffi::c_int {
    let Some(array) = array.as_ref() else {
        return 0;
    };
    snapshot_lookup_document!(array).array_size()
}
#[export_name = "cJSON_GetArrayItem"]

pub unsafe extern "C" fn cJSON_GetArrayItem_ffi(
    array: *const crate::src::cJSON::cJSON,
    index: ::core::ffi::c_int,
) -> *mut crate::src::cJSON::cJSON {
    let Some(array) = array.as_ref() else {
        return ::core::ptr::null_mut();
    };
    let Some(item) = snapshot_lookup_document!(array).array_item(index) else {
        return ::core::ptr::null_mut();
    };
    item as *const cJSON as *mut cJSON
}
#[export_name = "cJSON_GetObjectItem"]

pub unsafe extern "C" fn cJSON_GetObjectItem_ffi(
    object: *const crate::src::cJSON::cJSON,
    string: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    let Some(object) = object.as_ref() else {
        return ::core::ptr::null_mut();
    };
    let Some(string) = string.as_ref() else {
        return ::core::ptr::null_mut();
    };
    let name = ::std::ffi::CStr::from_ptr(string).to_bytes();
    let Some(item) = snapshot_lookup_document!(object).object_item(name, false) else {
        return ::core::ptr::null_mut();
    };
    item as *const cJSON as *mut cJSON
}
#[export_name = "cJSON_GetObjectItemCaseSensitive"]

pub unsafe extern "C" fn cJSON_GetObjectItemCaseSensitive_ffi(
    object: *const crate::src::cJSON::cJSON,
    string: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    let Some(object) = object.as_ref() else {
        return ::core::ptr::null_mut();
    };
    let Some(string) = string.as_ref() else {
        return ::core::ptr::null_mut();
    };
    let name = ::std::ffi::CStr::from_ptr(string).to_bytes();
    let Some(item) = snapshot_lookup_document!(object).object_item(name, true) else {
        return ::core::ptr::null_mut();
    };
    item as *const cJSON as *mut cJSON
}
#[export_name = "cJSON_HasObjectItem"]

pub unsafe extern "C" fn cJSON_HasObjectItem_ffi(
    object: *const crate::src::cJSON::cJSON,
    string: *const ::core::ffi::c_char,
) -> crate::src::cJSON::cJSON_bool {
    let Some(object) = object.as_ref() else {
        return false_0;
    };
    let Some(string) = string.as_ref() else {
        return false_0;
    };
    let name = ::std::ffi::CStr::from_ptr(string).to_bytes();
    (snapshot_lookup_document!(object).object_item(name, false).is_some()) as cJSON_bool
}
#[export_name = "cJSON_AddItemToArray"]
pub unsafe extern "C" fn cJSON_AddItemToArray_ffi(
    mut array: *mut crate::src::cJSON::cJSON,
    mut item: *mut crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    let mut child: *mut crate::src::cJSON::cJSON =
        ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    if item.is_null() || array.is_null() || array == item {
        return false_0;
    }
    child = (*array).child as *mut crate::src::cJSON::cJSON;
    if child.is_null() {
        (*array).child = item as *mut crate::src::cJSON::cJSON;
        (*item).prev = item as *mut crate::src::cJSON::cJSON;
        (*item).next = ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    } else if !(*child).prev.is_null() {
        (*(*child).prev).next = item as *mut crate::src::cJSON::cJSON;
        (*item).prev = (*child).prev as *mut crate::src::cJSON::cJSON;
        (*(*array).child).prev = item as *mut crate::src::cJSON::cJSON;
    }
    return true_0;
}
#[export_name = "cJSON_AddItemToObject"]

pub unsafe extern "C" fn cJSON_AddItemToObject_ffi(
    object: *mut crate::src::cJSON::cJSON,
    string: *const ::core::ffi::c_char,
    item: *mut crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    if object.is_null() || string.is_null() || item.is_null() || object == item {
        return false_0;
    }
    let Some(key) = duplicate_c_string(std::ffi::CStr::from_ptr(string), &current_hooks()) else {
        return false_0;
    };
    let item = &mut *item;
    clear_owned_string(item);
    set_owned_string(item, key);
    item.type_0 &= !crate::src::cJSON::cJSON_StringIsConst;
    cJSON_AddItemToArray_ffi(object, item)
}
#[export_name = "cJSON_AddItemToObjectCS"]

pub unsafe extern "C" fn cJSON_AddItemToObjectCS_ffi(
    object: *mut crate::src::cJSON::cJSON,
    string: *const ::core::ffi::c_char,
    item: *mut crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    if object.is_null() || string.is_null() || item.is_null() || object == item {
        return false_0;
    }
    let item = &mut *item;
    clear_owned_string(item);
    item.string = string.cast_mut();
    item.type_0 |= crate::src::cJSON::cJSON_StringIsConst;
    cJSON_AddItemToArray_ffi(object, item)
}
#[export_name = "cJSON_AddItemReferenceToArray"]

pub unsafe extern "C" fn cJSON_AddItemReferenceToArray_ffi(
    array: *mut crate::src::cJSON::cJSON,
    item: *mut crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    if array.is_null() {
        return false_0;
    }
    let reference = if item.is_null() {
        ::core::ptr::null_mut()
    } else {
        let reference = cJSON_CreateNull_ffi();
        if !reference.is_null() {
            (*reference).next = ::core::ptr::null_mut();
            (*reference).prev = ::core::ptr::null_mut();
            (*reference).child = (*item).child;
            (*reference).type_0 = (*item).type_0 | crate::src::cJSON::cJSON_IsReference;
            (*reference).valuestring = (*item).valuestring;
            (*reference).valueint = (*item).valueint;
            (*reference).valuedouble = (*item).valuedouble;
            (*reference).string = ::core::ptr::null_mut();
        }
        reference
    };
    cJSON_AddItemToArray_ffi(array, reference)
}
#[export_name = "cJSON_AddItemReferenceToObject"]

pub unsafe extern "C" fn cJSON_AddItemReferenceToObject_ffi(
    object: *mut crate::src::cJSON::cJSON,
    string: *const ::core::ffi::c_char,
    item: *mut crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    if object.is_null() || string.is_null() {
        return false_0;
    }
    let reference = if item.is_null() {
        ::core::ptr::null_mut()
    } else {
        let reference = cJSON_CreateNull_ffi();
        if !reference.is_null() {
            (*reference).next = ::core::ptr::null_mut();
            (*reference).prev = ::core::ptr::null_mut();
            (*reference).child = (*item).child;
            (*reference).type_0 = (*item).type_0 | crate::src::cJSON::cJSON_IsReference;
            (*reference).valuestring = (*item).valuestring;
            (*reference).valueint = (*item).valueint;
            (*reference).valuedouble = (*item).valuedouble;
            (*reference).string = ::core::ptr::null_mut();
        }
        reference
    };
    cJSON_AddItemToObject_ffi(object, string, reference)
}
#[export_name = "cJSON_AddNullToObject"]

pub unsafe extern "C" fn cJSON_AddNullToObject_ffi(
    object: *mut crate::src::cJSON::cJSON,
    name: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    let mut null: *mut crate::src::cJSON::cJSON = cJSON_CreateNull_ffi();
    if cJSON_AddItemToObject_ffi(object, name, null) != 0 {
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
    if cJSON_AddItemToObject_ffi(object, name, true_item) != 0 {
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
    if cJSON_AddItemToObject_ffi(object, name, false_item) != 0 {
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
    if cJSON_AddItemToObject_ffi(object, name, bool_item) != 0 {
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
    if cJSON_AddItemToObject_ffi(object, name, number_item) != 0 {
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
    if cJSON_AddItemToObject_ffi(object, name, string_item) != 0 {
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
    if cJSON_AddItemToObject_ffi(object, name, raw_item) != 0 {
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
    if cJSON_AddItemToObject_ffi(object, name, object_item) != 0 {
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
    if cJSON_AddItemToObject_ffi(object, name, array) != 0 {
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
    if which < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    }
    return cJSON_DetachItemViaPointer_ffi(
        array,
        cJSON_GetArrayItem_ffi(array, which),
    );
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
    let mut to_detach: *mut crate::src::cJSON::cJSON = cJSON_GetObjectItem_ffi(object, string);
    return cJSON_DetachItemViaPointer_ffi(object, to_detach);
}
#[export_name = "cJSON_DetachItemFromObjectCaseSensitive"]

pub unsafe extern "C" fn cJSON_DetachItemFromObjectCaseSensitive_ffi(
    mut object: *mut crate::src::cJSON::cJSON,
    mut string: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    let mut to_detach: *mut crate::src::cJSON::cJSON =
        cJSON_GetObjectItemCaseSensitive_ffi(object, string);
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
    after_inserted = cJSON_GetArrayItem_ffi(array, which);
    if after_inserted.is_null() {
        return cJSON_AddItemToArray_ffi(array, newitem);
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
    if which < 0 as ::core::ffi::c_int {
        return false_0;
    }
    cJSON_ReplaceItemViaPointer_ffi(
        array,
        cJSON_GetArrayItem_ffi(array, which),
        newitem,
    )
}
#[export_name = "cJSON_ReplaceItemInObject"]

pub unsafe extern "C" fn cJSON_ReplaceItemInObject_ffi(
    object: *mut crate::src::cJSON::cJSON,
    string: *const ::core::ffi::c_char,
    newitem: *mut crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    if newitem.is_null() || string.is_null() {
        return false_0;
    }
    let Some(key) = duplicate_c_string(std::ffi::CStr::from_ptr(string), &current_hooks()) else {
        return false_0;
    };
    let newitem = &mut *newitem;
    clear_owned_string(newitem);
    set_owned_string(newitem, key);
    newitem.type_0 &= !crate::src::cJSON::cJSON_StringIsConst;
    cJSON_ReplaceItemViaPointer_ffi(object, cJSON_GetObjectItem_ffi(object, string), newitem)
}
#[export_name = "cJSON_ReplaceItemInObjectCaseSensitive"]

pub unsafe extern "C" fn cJSON_ReplaceItemInObjectCaseSensitive_ffi(
    object: *mut crate::src::cJSON::cJSON,
    string: *const ::core::ffi::c_char,
    newitem: *mut crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    if newitem.is_null() || string.is_null() {
        return false_0;
    }
    let Some(key) = duplicate_c_string(std::ffi::CStr::from_ptr(string), &current_hooks()) else {
        return false_0;
    };
    let newitem = &mut *newitem;
    clear_owned_string(newitem);
    set_owned_string(newitem, key);
    newitem.type_0 &= !crate::src::cJSON::cJSON_StringIsConst;
    cJSON_ReplaceItemViaPointer_ffi(object, cJSON_GetObjectItemCaseSensitive_ffi(object, string), newitem)
}
#[export_name = "cJSON_CreateNull"]

pub unsafe extern "C" fn cJSON_CreateNull_ffi() -> *mut crate::src::cJSON::cJSON {
    let hooks = current_hooks();
    let allocation_probe = hooks.allocate.expect("non-null function pointer")(
        ::core::mem::size_of::<crate::src::cJSON::cJSON>(),
    );
    if allocation_probe.is_null() {
        return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    }
    hooks.deallocate.expect("non-null function pointer")(allocation_probe);

    Box::into_raw(Box::new(cJSON::null()))
}
#[export_name = "cJSON_CreateTrue"]

pub unsafe extern "C" fn cJSON_CreateTrue_ffi() -> *mut crate::src::cJSON::cJSON {
    let mut item: *mut crate::src::cJSON::cJSON = cJSON_CreateNull_ffi();
    if !item.is_null() {
        (*item).type_0 = crate::src::cJSON::cJSON_True;
    }
    return item;
}
#[export_name = "cJSON_CreateFalse"]

pub unsafe extern "C" fn cJSON_CreateFalse_ffi() -> *mut crate::src::cJSON::cJSON {
    let mut item: *mut crate::src::cJSON::cJSON = cJSON_CreateNull_ffi();
    if !item.is_null() {
        (*item).type_0 = crate::src::cJSON::cJSON_False;
    }
    return item;
}
#[export_name = "cJSON_CreateBool"]

pub unsafe extern "C" fn cJSON_CreateBool_ffi(
    mut boolean: crate::src::cJSON::cJSON_bool,
) -> *mut crate::src::cJSON::cJSON {
    let mut item: *mut crate::src::cJSON::cJSON = cJSON_CreateNull_ffi();
    if !item.is_null() {
        (*item).type_0 = if boolean != 0 {
            crate::src::cJSON::cJSON_True
        } else {
            crate::src::cJSON::cJSON_False
        };
    }
    return item;
}
#[export_name = "cJSON_CreateNumber"]

pub unsafe extern "C" fn cJSON_CreateNumber_ffi(
    mut num: ::core::ffi::c_double,
) -> *mut crate::src::cJSON::cJSON {
    let mut item: *mut crate::src::cJSON::cJSON = cJSON_CreateNull_ffi();
    if !item.is_null() {
        (*item).type_0 = crate::src::cJSON::cJSON_Number;
        (*item).valuedouble = num;
        if num >= crate::limits_h::INT_MAX as ::core::ffi::c_double {
            (*item).valueint = crate::limits_h::INT_MAX;
        } else if num <= crate::limits_h::INT_MIN as ::core::ffi::c_double {
            (*item).valueint = crate::limits_h::INT_MIN;
        } else {
            (*item).valueint = num as ::core::ffi::c_int;
        }
    }
    return item;
}
#[export_name = "cJSON_CreateString"]
pub unsafe extern "C" fn cJSON_CreateString_ffi(
    string: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    let item = cJSON_CreateNull_ffi();
    if item.is_null() || string.is_null() {
        cJSON_Delete_ffi(item);
        return ::core::ptr::null_mut();
    }
    let Some(value) = duplicate_c_string(std::ffi::CStr::from_ptr(string), &current_hooks()) else {
        cJSON_Delete_ffi(item);
        return ::core::ptr::null_mut();
    };
    (*item).type_0 = crate::src::cJSON::cJSON_String;
    set_owned_valuestring(&mut *item, value);
    item
}
#[export_name = "cJSON_CreateStringReference"]
pub unsafe extern "C" fn cJSON_CreateStringReference_ffi(
    mut string: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    let mut item: *mut crate::src::cJSON::cJSON = cJSON_CreateNull_ffi();
    if !item.is_null() {
        (*item).type_0 = crate::src::cJSON::cJSON_String | crate::src::cJSON::cJSON_IsReference;
        (*item).valuestring = string as *mut ::core::ffi::c_char;
    }
    return item;
}
#[export_name = "cJSON_CreateObjectReference"]
pub unsafe extern "C" fn cJSON_CreateObjectReference_ffi(
    mut child: *const crate::src::cJSON::cJSON,
) -> *mut crate::src::cJSON::cJSON {
    let mut item: *mut crate::src::cJSON::cJSON = cJSON_CreateNull_ffi();
    if !item.is_null() {
        (*item).type_0 = crate::src::cJSON::cJSON_Object | crate::src::cJSON::cJSON_IsReference;
        (*item).child = child as *mut crate::src::cJSON::cJSON;
    }
    return item;
}
#[export_name = "cJSON_CreateArrayReference"]
pub unsafe extern "C" fn cJSON_CreateArrayReference_ffi(
    mut child: *const crate::src::cJSON::cJSON,
) -> *mut crate::src::cJSON::cJSON {
    let mut item: *mut crate::src::cJSON::cJSON = cJSON_CreateNull_ffi();
    if !item.is_null() {
        (*item).type_0 = crate::src::cJSON::cJSON_Array | crate::src::cJSON::cJSON_IsReference;
        (*item).child = child as *mut crate::src::cJSON::cJSON;
    }
    return item;
}
#[export_name = "cJSON_CreateRaw"]
pub unsafe extern "C" fn cJSON_CreateRaw_ffi(
    raw: *const ::core::ffi::c_char,
) -> *mut crate::src::cJSON::cJSON {
    let item = cJSON_CreateNull_ffi();
    if item.is_null() || raw.is_null() {
        cJSON_Delete_ffi(item);
        return ::core::ptr::null_mut();
    }
    let Some(value) = duplicate_c_string(std::ffi::CStr::from_ptr(raw), &current_hooks()) else {
        cJSON_Delete_ffi(item);
        return ::core::ptr::null_mut();
    };
    (*item).type_0 = crate::src::cJSON::cJSON_Raw;
    set_owned_valuestring(&mut *item, value);
    item
}
#[export_name = "cJSON_CreateArray"]

pub unsafe extern "C" fn cJSON_CreateArray_ffi() -> *mut crate::src::cJSON::cJSON {
    let mut item: *mut crate::src::cJSON::cJSON = cJSON_CreateNull_ffi();
    if !item.is_null() {
        (*item).type_0 = crate::src::cJSON::cJSON_Array;
    }
    return item;
}
#[export_name = "cJSON_CreateObject"]

pub unsafe extern "C" fn cJSON_CreateObject_ffi() -> *mut crate::src::cJSON::cJSON {
    let mut item: *mut crate::src::cJSON::cJSON = cJSON_CreateNull_ffi();
    if !item.is_null() {
        (*item).type_0 = crate::src::cJSON::cJSON_Object;
    }
    return item;
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
    i = 0 as crate::__stddef_size_t_h::size_t;
    while !a.is_null() && i < count as crate::__stddef_size_t_h::size_t {
        n = cJSON_CreateNumber_ffi(*numbers.offset(i as isize) as ::core::ffi::c_double);
        if n.is_null() {
            cJSON_Delete_ffi(a);
            return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
        }
        if i == 0 {
            (*a).child = n as *mut crate::src::cJSON::cJSON;
        } else {
            (*p).next = n as *mut crate::src::cJSON::cJSON;
            (*n).prev = p as *mut crate::src::cJSON::cJSON;
        }
        p = n;
        i = i.wrapping_add(1);
    }
    if !a.is_null() && !(*a).child.is_null() {
        (*(*a).child).prev = n as *mut crate::src::cJSON::cJSON;
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
    i = 0 as crate::__stddef_size_t_h::size_t;
    while !a.is_null() && i < count as crate::__stddef_size_t_h::size_t {
        n = cJSON_CreateNumber_ffi(*numbers.offset(i as isize) as ::core::ffi::c_double);
        if n.is_null() {
            cJSON_Delete_ffi(a);
            return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
        }
        if i == 0 {
            (*a).child = n as *mut crate::src::cJSON::cJSON;
        } else {
            (*p).next = n as *mut crate::src::cJSON::cJSON;
            (*n).prev = p as *mut crate::src::cJSON::cJSON;
        }
        p = n;
        i = i.wrapping_add(1);
    }
    if !a.is_null() && !(*a).child.is_null() {
        (*(*a).child).prev = n as *mut crate::src::cJSON::cJSON;
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
    i = 0 as crate::__stddef_size_t_h::size_t;
    while !a.is_null() && i < count as crate::__stddef_size_t_h::size_t {
        n = cJSON_CreateNumber_ffi(*numbers.offset(i as isize));
        if n.is_null() {
            cJSON_Delete_ffi(a);
            return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
        }
        if i == 0 {
            (*a).child = n as *mut crate::src::cJSON::cJSON;
        } else {
            (*p).next = n as *mut crate::src::cJSON::cJSON;
            (*n).prev = p as *mut crate::src::cJSON::cJSON;
        }
        p = n;
        i = i.wrapping_add(1);
    }
    if !a.is_null() && !(*a).child.is_null() {
        (*(*a).child).prev = n as *mut crate::src::cJSON::cJSON;
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
    i = 0 as crate::__stddef_size_t_h::size_t;
    while !a.is_null() && i < count as crate::__stddef_size_t_h::size_t {
        n = cJSON_CreateString_ffi(*strings.offset(i as isize));
        if n.is_null() {
            cJSON_Delete_ffi(a);
            return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
        }
        if i == 0 {
            (*a).child = n as *mut crate::src::cJSON::cJSON;
        } else {
            (*p).next = n as *mut crate::src::cJSON::cJSON;
            (*n).prev = p as *mut crate::src::cJSON::cJSON;
        }
        p = n;
        i = i.wrapping_add(1);
    }
    if !a.is_null() && !(*a).child.is_null() {
        (*(*a).child).prev = n as *mut crate::src::cJSON::cJSON;
    }
    return a;
}

#[export_name = "cJSON_Duplicate"]

pub unsafe extern "C" fn cJSON_Duplicate_ffi(
    mut item: *const crate::src::cJSON::cJSON,
    mut recurse: crate::src::cJSON::cJSON_bool,
) -> *mut crate::src::cJSON::cJSON {
    cJSON_Duplicate_rec_ffi(item, 0 as crate::__stddef_size_t_h::size_t, recurse)
}
#[export_name = "cJSON_Duplicate_rec"]

pub unsafe extern "C" fn cJSON_Duplicate_rec_ffi(
    mut item: *const crate::src::cJSON::cJSON,
    mut depth: crate::__stddef_size_t_h::size_t,
    mut recurse: crate::src::cJSON::cJSON_bool,
) -> *mut crate::src::cJSON::cJSON {
    let mut newitem: *mut crate::src::cJSON::cJSON =
        ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    let mut child: *mut crate::src::cJSON::cJSON =
        ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    let mut next: *mut crate::src::cJSON::cJSON =
        ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    let mut newchild: *mut crate::src::cJSON::cJSON =
        ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
    '_fail: {
        if !item.is_null() {
            newitem = cJSON_CreateNull_ffi();
            if !newitem.is_null() {
                (*newitem).type_0 = (*item).type_0 & !crate::src::cJSON::cJSON_IsReference;
                (*newitem).valueint = (*item).valueint;
                (*newitem).valuedouble = (*item).valuedouble;
                if !(*item).valuestring.is_null() {
                    let Some(value) = duplicate_c_string(
                        std::ffi::CStr::from_ptr((*item).valuestring),
                        &current_hooks(),
                    ) else {
                        break '_fail;
                    };
                    set_owned_valuestring(&mut *newitem, value);
                }
                if !(*item).string.is_null() {
                    if (*item).type_0 & crate::src::cJSON::cJSON_StringIsConst != 0 {
                        (*newitem).string = (*item).string;
                    } else {
                        let Some(key) = duplicate_c_string(
                            std::ffi::CStr::from_ptr((*item).string),
                            &current_hooks(),
                        ) else {
                            break '_fail;
                        };
                        set_owned_string(&mut *newitem, key);
                    }
                }
                if recurse == 0 {
                    return newitem;
                }
                child = (*item).child as *mut crate::src::cJSON::cJSON;
                while !child.is_null() {
                    if depth
                        >= crate::src::cJSON::CJSON_CIRCULAR_LIMIT
                            as crate::__stddef_size_t_h::size_t
                    {
                        break '_fail;
                    }
                    newchild = cJSON_Duplicate_rec_ffi(
                        child,
                        depth.wrapping_add(1 as crate::__stddef_size_t_h::size_t),
                        true_0,
                    );
                    if newchild.is_null() {
                        break '_fail;
                    }
                    if !next.is_null() {
                        (*next).next = newchild as *mut crate::src::cJSON::cJSON;
                        (*newchild).prev = next as *mut crate::src::cJSON::cJSON;
                        next = newchild;
                    } else {
                        (*newitem).child = newchild as *mut crate::src::cJSON::cJSON;
                        next = newchild;
                    }
                    child = (*child).next as *mut crate::src::cJSON::cJSON;
                }
                if !newitem.is_null() && !(*newitem).child.is_null() {
                    (*(*newitem).child).prev = newchild as *mut crate::src::cJSON::cJSON;
                }
                return newitem;
            }
        }
    }
    if !newitem.is_null() {
        cJSON_Delete_ffi(newitem);
    }
    return ::core::ptr::null_mut::<crate::src::cJSON::cJSON>();
}
fn minify_json_in_place(json: &mut [u8]) {
    let Some(terminator) = json.iter().position(|byte| *byte == 0) else {
        return;
    };
    let mut input = 0;
    let mut output = 0;

    while input < terminator {
        match json[input] {
            b' ' | b'\t' | b'\r' | b'\n' => input += 1,
            b'/' if json.get(input + 1) == Some(&b'/') => {
                input += 2;
                while input < terminator && json[input] != b'\n' {
                    input += 1;
                }
                if input < terminator {
                    input += 1;
                }
            }
            b'/' if json.get(input + 1) == Some(&b'*') => {
                input += 2;
                while input < terminator {
                    if json[input] == b'*' && json.get(input + 1) == Some(&b'/') {
                        input += 2;
                        break;
                    }
                    input += 1;
                }
            }
            b'/' => input += 1,
            b'"' => {
                json[output] = json[input];
                input += 1;
                output += 1;
                while input < terminator {
                    json[output] = json[input];
                    if json[input] == b'"' {
                        input += 1;
                        output += 1;
                        break;
                    }
                    if json[input] == b'\\' && json.get(input + 1) == Some(&b'"') {
                        json[output + 1] = json[input + 1];
                        input += 1;
                        output += 1;
                    }
                    input += 1;
                    output += 1;
                }
            }
            _ => {
                json[output] = json[input];
                input += 1;
                output += 1;
            }
        }
    }
    json[output] = 0;
}

#[export_name = "cJSON_Minify"]

pub unsafe extern "C" fn cJSON_Minify_ffi(json: *mut ::core::ffi::c_char) {
    if json.is_null() {
        return;
    }
    let length = unsafe { std::ffi::CStr::from_ptr(json).to_bytes_with_nul().len() };
    let json = unsafe { std::slice::from_raw_parts_mut(json.cast::<u8>(), length) };
    minify_json_in_place(json);
}
fn cjson_has_type(item: Option<&cJSON>, type_: ::core::ffi::c_int) -> cJSON_bool {
    match item {
        Some(item) => ((item.type_0 & 0xff) == type_) as ::core::ffi::c_int,
        None => false_0,
    }
}

pub fn cJSON_IsInvalid(item: Option<&cJSON>) -> cJSON_bool {
    cjson_has_type(item, cJSON_Invalid)
}
#[export_name = "cJSON_IsInvalid"]

pub unsafe extern "C" fn cJSON_IsInvalid_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    cJSON_IsInvalid(unsafe { item.as_ref() })
}
pub fn cJSON_IsFalse(item: Option<&cJSON>) -> cJSON_bool {
    cjson_has_type(item, cJSON_False)
}
#[export_name = "cJSON_IsFalse"]

pub unsafe extern "C" fn cJSON_IsFalse_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    cJSON_IsFalse(unsafe { item.as_ref() })
}
pub fn cJSON_IsTrue(item: Option<&cJSON>) -> cJSON_bool {
    cjson_has_type(item, cJSON_True)
}
#[export_name = "cJSON_IsTrue"]

pub unsafe extern "C" fn cJSON_IsTrue_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    cJSON_IsTrue(unsafe { item.as_ref() })
}
pub fn cJSON_IsBool(item: Option<&cJSON>) -> cJSON_bool {
    item.map_or(false_0, |item| {
        ((item.type_0 & (cJSON_True | cJSON_False)) != 0) as ::core::ffi::c_int
    })
}
#[export_name = "cJSON_IsBool"]

pub unsafe extern "C" fn cJSON_IsBool_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    cJSON_IsBool(unsafe { item.as_ref() })
}
pub fn cJSON_IsNull(item: Option<&cJSON>) -> cJSON_bool {
    cjson_has_type(item, cJSON_NULL)
}
#[export_name = "cJSON_IsNull"]

pub unsafe extern "C" fn cJSON_IsNull_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    cJSON_IsNull(unsafe { item.as_ref() })
}
pub fn cJSON_IsNumber(item: Option<&cJSON>) -> cJSON_bool {
    cjson_has_type(item, cJSON_Number)
}
#[export_name = "cJSON_IsNumber"]

pub unsafe extern "C" fn cJSON_IsNumber_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    cJSON_IsNumber(unsafe { item.as_ref() })
}
pub fn cJSON_IsString(item: Option<&cJSON>) -> cJSON_bool {
    cjson_has_type(item, cJSON_String)
}
#[export_name = "cJSON_IsString"]

pub unsafe extern "C" fn cJSON_IsString_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    cJSON_IsString(unsafe { item.as_ref() })
}
pub fn cJSON_IsArray(item: Option<&cJSON>) -> cJSON_bool {
    cjson_has_type(item, cJSON_Array)
}
#[export_name = "cJSON_IsArray"]

pub unsafe extern "C" fn cJSON_IsArray_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    cJSON_IsArray(unsafe { item.as_ref() })
}
pub fn cJSON_IsObject(item: Option<&cJSON>) -> cJSON_bool {
    cjson_has_type(item, cJSON_Object)
}
#[export_name = "cJSON_IsObject"]

pub unsafe extern "C" fn cJSON_IsObject_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    cJSON_IsObject(unsafe { item.as_ref() })
}
pub fn cJSON_IsRaw(item: Option<&cJSON>) -> cJSON_bool {
    cjson_has_type(item, cJSON_Raw)
}
#[export_name = "cJSON_IsRaw"]

pub unsafe extern "C" fn cJSON_IsRaw_ffi(
    item: *const crate::src::cJSON::cJSON,
) -> crate::src::cJSON::cJSON_bool {
    cJSON_IsRaw(unsafe { item.as_ref() })
}
#[export_name = "cJSON_Compare"]
pub unsafe extern "C" fn cJSON_Compare_ffi(
    a: *const crate::src::cJSON::cJSON,
    b: *const crate::src::cJSON::cJSON,
    case_sensitive: crate::src::cJSON::cJSON_bool,
) -> crate::src::cJSON::cJSON_bool {
    let Some(a) = a.as_ref() else {
        return false_0;
    };
    let Some(b) = b.as_ref() else {
        return false_0;
    };
    let Some(a_document) = snapshot_compare_document!(a) else {
        return false_0;
    };
    let Some(b_document) = snapshot_compare_document!(b) else {
        return false_0;
    };
    a_document.compare(&b_document, case_sensitive != 0) as cJSON_bool
}
#[export_name = "cJSON_malloc"]

pub unsafe extern "C" fn cJSON_malloc_ffi(
    size: crate::__stddef_size_t_h::size_t,
) -> *mut ::core::ffi::c_void {
    current_hooks().allocate.expect("non-null function pointer")(size)
}
#[export_name = "cJSON_free"]

pub unsafe extern "C" fn cJSON_free_ffi(mut object: *mut ::core::ffi::c_void) {
    current_hooks().deallocate.expect("non-null function pointer")(object);
}
