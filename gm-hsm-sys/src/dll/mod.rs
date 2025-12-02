pub mod sdf;
pub mod skf;

pub mod symbol {

    use libloading::{Library, Symbol};
    use std::sync::Arc;

    /// Defines a public or private type alias for an external function pointer.
    ///
    /// This macro automatically selects the correct Calling Convention (ABI) based
    /// on the target platform to ensure proper Foreign Function Interface (FFI)
    /// compatibility.
    ///
    /// # Platform ABI Selection:
    /// - **Windows x86 (32-bit):** Uses the `stdcall` convention, typical for the Windows API.
    /// - **All others (e.g., Linux, macOS, Windows x64):** Uses the standard `C` convention.
    ///
    /// # Syntax
    /// `define_extern_fn!($vis $name = fn($($arg:ty),*) -> $ret:ty)`
    ///
    /// # Examples
    /// ```rust
    /// use gm_hsm_sys::define_extern_fn;
    /// define_extern_fn!(pub Win32GetProcAddress = fn(isize, *const u8) -> isize);
    /// // Win32GetProcAddress is now an alias for the correct unsafe extern "..." fn(...) type.
    ///
    /// // This type can then be used directly or within a wrapper struct:
    /// // type SymbolGetProcAddress = SymbolBundle<Win32GetProcAddress>;
    /// ```
    #[macro_export]
    macro_rules! define_extern_fn {
    ($vis:vis $name:ident = fn($($arg:ty),*) -> $ret:ty) => {
        #[cfg(all(target_os = "windows", target_arch = "x86"))]
        $vis type $name = unsafe extern "stdcall" fn($($arg),*) -> $ret;

        #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
        $vis type $name = unsafe extern "C" fn($($arg),*) -> $ret;
    };
}

    /// Defines a pair of type aliases for a Foreign Function Interface (FFI) symbol.
    ///
    /// This macro ensures FFI compatibility across different platforms by defining
    /// the raw function pointer type with the correct Calling Convention (ABI),
    /// and then wrapping it in a Symbol Bundle structure.
    ///
    /// # Macro Parameters:
    /// * `$fn_name:ident`: The name for the raw FFI function pointer type alias (e.g., `FnWin32GetProcAddress`).
    /// * `$bundle_name:ident`: The name for the final SymbolBundle wrapper type alias (e.g., `Win32GetProcAddress`).
    /// * `$vis:vis`: The visibility modifier (e.g., `pub`, `pub(crate)`).
    /// * `fn($($arg:ty),*) -> $ret:ty`: The function signature.
    ///
    /// # Example Usage:
    /// ```rust
    /// use gm_hsm_sys::define_symbol;
    /// define_symbol!(FnWin32GetProcAddress, Win32GetProcAddress, pub(crate) fn(isize, *const u8) -> isize);
    ///
    /// // Expands (e.g., on Windows x64) to:
    /// // pub(crate) type FnWin32GetProcAddress = unsafe extern "C" fn(isize, *const u8) -> isize;
    /// // pub(crate) type Win32GetProcAddress = $crate::dll::symbol::SymbolBundle<FnWin32GetProcAddress>;
    /// ```
    #[macro_export]
    macro_rules! define_symbol {
    ($fn_name:ident, $bundle_name:ident, $vis:vis fn($($arg:ty),*) -> $ret:ty) => {

            #[cfg(all(target_os = "windows", target_arch = "x86"))]
            $vis type $fn_name = unsafe extern "stdcall" fn($($arg),*) -> $ret;

            #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
            $vis type $fn_name = unsafe extern "C" fn($($arg),*) -> $ret;

            $vis type $bundle_name = $crate::dll::symbol::SymbolBundle<$fn_name>;
    };
}

    /// Symbol bundle with library pointer
    #[derive(Clone)]
    pub struct SymbolBundle<T: 'static> {
        _lib: Arc<Library>,
        symbol: Symbol<'static, T>,
    }
    impl<T> SymbolBundle<T> {
        /// Get a pointer to a function or static variable by symbol name.
        pub unsafe fn new(
            lib: &Arc<Library>,
            sym: &[u8],
        ) -> Result<SymbolBundle<T>, libloading::Error> {
            let lc = lib.clone();
            unsafe {
                let symbol: Symbol<T> = lib.get(sym)?;
                let bundle = SymbolBundle {
                    _lib: lc,
                    symbol: std::mem::transmute(symbol),
                };
                Ok(bundle)
            }
        }
    }
    impl<T> std::ops::Deref for SymbolBundle<T> {
        type Target = Symbol<'static, T>;
        fn deref(&self) -> &Self::Target {
            &self.symbol
        }
    }
}
