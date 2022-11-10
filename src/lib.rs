use std::ffi::CString;


#[allow(unused)]
enum CERR
{
	NullPointer,
}

#[allow(unused)]
#[inline(always)]
fn cstr(str: &str) -> *const i8
{
	let st= Box::new(CString::new(str).ok().unwrap());
	let st = Box::leak(st);
	return st.as_ref().as_ptr();
}

#[allow(unused)]
#[inline(always)]
fn scstr(str: &str) -> Option<*const i8>
{
	let res = CString::new(str);
	match res
	{
		Ok(s) =>
		{
			let bx = Box::new(s);
			let bx = Box::leak(bx);
			return Some(bx.as_ref().as_ptr());
		},
		Err(_) => return None,
	}
}
