use libloading::{Library, Symbol};
use std::sync::Arc;

/// Helper macro to define extern function type for each platform
#[macro_export]
macro_rules! define_extern_fn_type {
    ($vis:vis $name:ident = fn($($arg:ty),*) -> $ret:ty) => {
        #[cfg(all(target_os = "windows", target_arch = "x86"))]
        $vis type $name = $crate::engine::symbol::SymbolBundle<unsafe extern "stdcall" fn($($arg),*) -> $ret>;

        #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
        $vis type $name = $crate::engine::symbol::SymbolBundle<unsafe extern "C" fn($($arg),*) -> $ret>;
    };
}

/// Symbol bundle with library pointer
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
