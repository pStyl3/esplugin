
use libc::{c_char, uint32_t};

use form_id::FormId;
use ffi::helpers::*;
use ffi::constants::*;

#[no_mangle]
pub extern "C" fn espm_formid_new(
    formid_ptr_ptr: *mut *const FormId,
    parent_plugin_name: *const c_char,
    masters: *const *const c_char,
    masters_count: u8,
    raw_form_id: uint32_t,
) -> u32 {
    let rust_name = match to_str(parent_plugin_name) {
        Ok(x) => x,
        Err(x) => return x,
    };
    let rust_masters = match to_str_vec(masters, masters_count as isize) {
        Ok(x) => x,
        Err(x) => return x,
    };

    let formid = FormId::new(rust_name, &rust_masters, raw_form_id);
    unsafe {
        *formid_ptr_ptr = Box::into_raw(Box::new(formid));
    };

    ESPM_OK
}

#[no_mangle]
pub extern "C" fn espm_formid_free(formid_ptr: *mut FormId) {
    if !formid_ptr.is_null() {
        unsafe {
            Box::from_raw(formid_ptr);
        }
    }
}

#[no_mangle]
pub extern "C" fn espm_formid_plugin_name(
    name: *mut *mut c_char,
    formid_ptr: *const FormId,
) -> u32 {
    if name.is_null() || formid_ptr.is_null() {
        ESPM_ERROR_NULL_POINTER
    } else {
        let formid = unsafe { &*formid_ptr };
        let c_string = match to_c_string(&formid.plugin_name) {
            Ok(x) => x,
            Err(x) => return x,
        };

        unsafe {
            *name = c_string;
        }

        ESPM_OK
    }
}
