use std::ffi::CString;

use libc::c_char;

#[no_mangle]
pub unsafe extern "C" fn espm_string_free(string: *mut c_char) {
    if !string.is_null() {
        CString::from_raw(string);
    }
}

#[no_mangle]
pub unsafe extern "C" fn espm_string_array_free(array: *mut *mut c_char, size: usize) {
    if array.is_null() || size == 0 {
        return;
    }

    let vec = Vec::from_raw_parts(array, size, size);
    for string in vec {
        espm_string_free(string);
    }
}
