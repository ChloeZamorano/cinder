use std::ffi::CString;

#[inline(always)]
fn cstr(str: &str) -> *const i8
{
    return CString::new(str).expect("").as_ptr();
}