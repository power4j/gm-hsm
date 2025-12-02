#![allow(clippy::missing_safety_doc)]
#![allow(clippy::manual_c_str_literals)]

use std::cmp::min;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::slice;

// Allow missing safety documentation for unsafe functions in this module
// since these are internal utility functions with well-defined safety contracts

/// Raw pointer to a C structure or handle.
pub type RawPtr = *mut ::std::os::raw::c_void;

/// Represents an opaque C structure or handle (e.g., a cryptographic session or context).
///
/// **Usage Note:** This type is never instantiated in Rust; it is only used by reference
/// via a raw pointer (*mut RawHandle) to ensure type safety when interacting with FFI.
///
/// **Design Rationale:**
/// - `#[repr(C)]` ensures compatibility with C linking.
/// - `_private: [u8; 0]` makes the type opaque, preventing Rust from knowing its layout or size.
#[repr(C)]
pub struct RawHandle {
    _private: [u8; 0],
}
pub type RawHandlePtr = *mut RawHandle;

/// A safe, thread-safe wrapper for the FFI opaque handle (*mut RawHandle).
///
/// This struct takes ownership of the C resource but, in this implementation,
/// delegates resource cleanup responsibility to the caller.
///
/// **CRITICAL NOTE ON RESOURCE MANAGEMENT (DROP):**
/// This wrapper **does NOT** implement the `Drop` trait and therefore **does NOT**
/// automatically call the C library's release function (e.g., `SDF_CloseSession`).
///
/// **Recommendation:** If this handle owns a C resource that requires cleanup:
/// 1. You **MUST** define your own owning structure, embed this handle within it,
///    and implement the `Drop` trait to ensure the C resource is freed.
/// 2. Alternatively, you must manually call the C release function (less safe, prone to leaks).
pub struct ThreadSafeHandle {
    /// The raw pointer to the C resource.
    ///
    /// **Choice Rationale (*mut):** We use `*mut` (mutable) because C library functions often
    /// internally modify the resource's state (e.g., update counters, caches, or locks).
    pub ptr: RawHandlePtr,

    /// Marker to inform the compiler about ownership and thread-safety.
    _marker: PhantomData<()>,
}

/// **Safety Note (Send):** We implement `Send` because we are asserting that the underlying
/// C resource (pointed to by `ptr`) can be safely moved/transferred from one thread to another.
unsafe impl Send for ThreadSafeHandle {}

/// **Safety Note (Sync):** We implement `Sync` because we are asserting that the underlying
/// C resource can be safely accessed *concurrently* by multiple threads.
unsafe impl Sync for ThreadSafeHandle {}

/// Returns the position of the first null byte
///
/// [ptr] - The pointer to the buffer
///
/// [len] - The length of the buffer
///
/// # Examples
/// ```
/// use gm_hsm_sys::utils::mem::first_null_byte;
/// let ptr = b"Hello\0World\0".as_ptr();
/// unsafe {
///     assert_eq!(Some(5), first_null_byte(ptr, 12));
///     assert_eq!(None, first_null_byte(ptr, 5));
///     assert_eq!(Some(4), first_null_byte(ptr.add(1), 11));
///     assert_eq!(Some(0), first_null_byte(ptr.add(5), 7));
/// }
/// ```
#[inline]
#[must_use]
pub unsafe fn first_null_byte(ptr: *const u8, len: usize) -> Option<usize> {
    let slice = unsafe { slice::from_raw_parts(ptr, len) };
    slice.iter().position(|&x| x == 0)
}

/// Returns the position of the first two null byte
///
/// [ptr] - The pointer to the buffer
///
/// [len] - The length of the buffer
///
/// # Examples
/// ```
/// use gm_hsm_sys::utils::mem::first_two_null_byte;
/// let ptr = b"Hello\0World\0\0".as_ptr();
/// unsafe {
///     assert_eq!(Some(12), first_two_null_byte(ptr, 13));
///     assert_eq!(Some(11), first_two_null_byte(ptr.add(1), 12));
///     assert_eq!(None, first_two_null_byte(ptr, 12));
/// }
/// ```
#[inline]
#[must_use]
pub const unsafe fn first_two_null_byte(ptr: *const u8, len: usize) -> Option<usize> {
    let mut pos = 0;
    while pos < len {
        if *ptr.add(pos) == 0 && pos + 1 < len && *ptr.add(pos + 1) == 0 {
            return Some(pos + 1);
        }
        pos += 1;
    }
    None
}

/// Parse a C string from buffer
///
/// [ptr] - The pointer to the buffer
///
/// [len] - The length of the buffer
/// # Examples
/// ```
/// use std::ffi::CStr;
/// use gm_hsm_sys::utils::mem::parse_cstr;
/// let ptr = b"Hello\0World\0".as_ptr();
/// unsafe {
///     assert_eq!(Some(CStr::from_bytes_with_nul(b"Hello\0").unwrap()), parse_cstr(ptr, 12));
///     assert_eq!(Some(CStr::from_bytes_with_nul(b"lo\0").unwrap()), parse_cstr(ptr.add(3), 12));
///     assert_eq!(Some(CStr::from_bytes_with_nul(b"World\0").unwrap()), parse_cstr(ptr.add(6), 12));
///     assert_eq!(Some(CStr::from_bytes_with_nul(b"\0").unwrap()), parse_cstr(ptr.add(5), 1));
///     assert_eq!(None, parse_cstr(ptr, 1));
/// }
/// ```
#[inline]
#[must_use]
pub unsafe fn parse_cstr<'a>(ptr: *const u8, len: usize) -> Option<&'a CStr> {
    let slice = unsafe { slice::from_raw_parts(ptr, len) };
    CStr::from_bytes_until_nul(slice).ok()
}

/// Parse a C string from buffer, use `CStr::to_string_lossy` to convert data
///
/// [ptr] - The pointer to the buffer
///
/// [len] - The length of the buffer
#[must_use]
pub unsafe fn parse_cstr_lossy(ptr: *const u8, len: usize) -> Option<String> {
    let val = unsafe { parse_cstr(ptr, len) };
    val.map(|s| s.to_string_lossy().to_string())
}

/// Parse C string list from buffer, the list may end with two null byte
///
/// [ptr] - The pointer to the buffer
///
/// [len] - The length of the buffer
/// # Examples
/// ```
/// use std::ffi::CStr;
/// use gm_hsm_sys::utils::mem::parse_cstr_list;
/// unsafe {
///     let list = parse_cstr_list(b"Hello\0World\0\0".as_ptr(), 13);
///     assert_eq!(CStr::from_bytes_with_nul(b"Hello\0").unwrap(), *list.get(0).unwrap());
///     assert_eq!(CStr::from_bytes_with_nul(b"World\0").unwrap(), *list.get(1).unwrap());
///
///     let list = parse_cstr_list(b"Hello\0World\0".as_ptr(), 12);
///     assert_eq!(CStr::from_bytes_with_nul(b"Hello\0").unwrap(), *list.get(0).unwrap());
///     assert_eq!(CStr::from_bytes_with_nul(b"World\0").unwrap(), *list.get(1).unwrap());
///
///     let list = parse_cstr_list(b"Hello\0World".as_ptr(), 11);
///     assert_eq!(CStr::from_bytes_with_nul(b"Hello\0").unwrap(), *list.get(0).unwrap());
///
///     let list = parse_cstr_list(b"Hello".as_ptr(), 5);
///     assert!(list.is_empty());
/// }
/// ```
#[inline]
#[must_use]
pub unsafe fn parse_cstr_list<'a>(ptr: *const u8, len: usize) -> Vec<&'a CStr> {
    let mut list: Vec<&CStr> = Vec::new();
    let mut next_str = 0;
    let mut pos = 0;
    while pos < len {
        if *ptr.add(pos) == 0 {
            let bytes = slice::from_raw_parts(ptr.add(next_str), pos - next_str + 1);
            list.push(CStr::from_bytes_with_nul_unchecked(bytes));
            next_str = pos + 1;
            if next_str < len && *ptr.add(next_str) == 0 {
                break;
            }
        }
        pos += 1;
    }
    list
}

/// Parse C string list from buffer, the list may end with two null byte
///
/// [ptr] - The pointer to the buffer
///
/// [len] - The length of the buffer
#[must_use]
pub unsafe fn parse_cstr_list_lossy(ptr: *const u8, len: usize) -> Vec<String> {
    let list = unsafe { parse_cstr_list(ptr, len) };
    list.iter()
        .map(|s| s.to_string_lossy().to_string())
        .collect()
}

/// Write string to buffer
///
/// [src] - The string to write,if too long, it will be truncated
///
/// [buffer] - The buffer to write to,at least one byte to fill with null byte
///
/// ## Memory copy
///
/// - if the string is too long,it will be truncated,and the last byte will be set to null byte
/// - if the string is smaller than the buffer size,it will be filled with null byte
///
/// ## example
/// ```
/// use gm_hsm_sys::utils::mem::write_cstr;
///
/// let mut buffer = [0u8; 11];
/// unsafe {
///     write_cstr("Hello World", &mut buffer);
///}
///assert_eq!(b"Hello Worl\0", &buffer);
///```
pub unsafe fn write_cstr(src: impl AsRef<str>, buffer: &mut [u8]) {
    let src = src.as_ref().as_bytes();
    let len = min(src.len(), buffer.len());
    debug_assert!(len > 0);
    unsafe {
        std::ptr::copy(src.as_ptr(), buffer.as_mut_ptr(), len);
    }
    if len < buffer.len() {
        buffer[len] = 0;
    } else {
        buffer[len - 1] = 0;
    }
}

/// Write string to buffer
///
/// [src] - The string to write
///
/// [buffer_ptr] - The buffer to write to
///
/// [buffer_len] - The length of the buffer
pub unsafe fn write_cstr_ptr(src: impl AsRef<str>, buffer_ptr: *mut u8, buffer_len: usize) {
    let bytes = slice::from_raw_parts_mut(buffer_ptr, buffer_len);
    write_cstr(src, bytes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::manual_c_str_literals)]
    fn parse_terminated_cstr_list_test() {
        unsafe {
            let list = parse_cstr_list(b"Hello\0\0".as_ptr(), 7);
            assert_eq!(1, list.len());

            let list = parse_cstr_list(b"Hello\0World\0\0".as_ptr(), 13);
            assert_eq!(c"Hello", *list.first().unwrap());
            assert_eq!(c"World", *list.get(1).unwrap());
        }
    }
    #[test]
    fn write_cstr_test() {
        let input = "Hello World";
        let mut buffer = [0u8; 12];
        unsafe {
            write_cstr(input, &mut buffer);
        }
        assert_eq!(b"Hello World\0", &buffer);

        let mut buffer = [0u8; 11];
        unsafe {
            write_cstr(input, &mut buffer);
        }
        assert_eq!(b"Hello Worl\0", &buffer);

        let mut buffer = [0u8; 1];
        unsafe {
            write_cstr(input, &mut buffer);
        }
        assert_eq!(b"\0", &buffer);
    }
}
