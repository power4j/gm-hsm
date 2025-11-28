use gm_hsm_sys::define_symbol;
use gm_hsm_sys::dll::sdf;
use gm_hsm_sys::dll::symbol::SymbolBundle;
use libloading::Library;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct DeviceSymbols {
    pub open_device: Option<sdf::SDF_OpenDevice>,
    pub close_device: Option<sdf::SDF_CloseDevice>,
}

impl DeviceSymbols {
    pub fn load(lib: &Arc<Library>) -> crate::Result<Self> {
        let open_device = Some(unsafe { SymbolBundle::new(lib, b"SDF_OpenDevice\0")? });
        let close_device = Some(unsafe { SymbolBundle::new(lib, b"SDF_CloseDevice\0")? });
        Ok(Self {
            open_device,
            close_device,
        })
    }
}

#[derive(Clone)]
pub(crate) struct SessionSymbols {
    pub open_session: Option<sdf::SDF_OpenSession>,
    pub close_session: Option<sdf::SDF_CloseSession>,
}

impl SessionSymbols {
    pub fn load(lib: &Arc<Library>) -> crate::Result<Self> {
        let open_session = Some(unsafe { SymbolBundle::new(lib, b"SDF_OpenSession\0")? });
        let close_session = Some(unsafe { SymbolBundle::new(lib, b"SDF_CloseSession\0")? });
        Ok(Self {
            open_session,
            close_session,
        })
    }
}
