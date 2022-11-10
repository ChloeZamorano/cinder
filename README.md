# CINDER

Cinder is a tiny library of inline functions to write cleaner code with C bindings like ash.

## Functions

### cinder::cstr: Takes a Rust string and returns a C string in the form of a i8 pointer.
```rs
let cstring: *const i8 = cinder::cstr("Rust string");
```
### cinder::scstr: Same as cinder::cstr but it returns Result<*const i8, CERR>.
```rs
let cstring: *mut i8 = mem::uninitialized;
match cinder::scstr("Rust string")
{
	Ok(s) => cstring = s,
	Err(e) => panic!("Cinder: Rust string was null!");
}
```
