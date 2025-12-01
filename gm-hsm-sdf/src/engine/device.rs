use crate::engine::symbol::{DeviceSymbols, SessionSymbols};
use crate::error::SdfErr;
use crate::Error;
use gm_hsm_sys::dll::sdf;
use gm_hsm_sys::sdf::types::SDR_OK;
use gm_hsm_sys::utils::mem::RawPtr;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::sync::Arc;
use tracing::{error, instrument, trace};

pub(crate) struct DeviceHandle {
    ptr: RawPtr,
    close_device: sdf::SDF_CloseDevice,
    _marker: PhantomData<()>,
}
impl DeviceHandle {
    pub fn new(ptr: RawPtr, close_device: sdf::SDF_CloseDevice) -> crate::Result<Self> {
        Ok(Self {
            ptr,
            close_device,
            _marker: PhantomData,
        })
    }
}
unsafe impl Send for DeviceHandle {}

unsafe impl Sync for DeviceHandle {}

impl Debug for DeviceHandle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DeviceHandle {{ ptr: {:p} }}", self.ptr)
    }
}

impl Drop for DeviceHandle {
    #[instrument]
    fn drop(&mut self) {
        let ret = unsafe { (self.close_device)(self.ptr) };
        if ret != SDR_OK {
            error!(
                "Failed to close device,handle={:p},return code: 0x{:x}",
                self.ptr, ret
            );
        } else {
            trace!("Close device success,handle={:p}", self.ptr)
        }
    }
}

pub fn open_device(symbols: DeviceSymbols) -> crate::Result<DeviceHandle> {
    let fn_close = symbols
        .close_device
        .as_ref()
        .expect("Symbol(SDF_CloseDevice) not load");
    let fn_open = symbols
        .open_device
        .as_ref()
        .expect("Symbol(SDF_OpenDevice) not load");
    let mut handle: RawPtr = std::ptr::null_mut();
    let ret = unsafe { fn_open(&mut handle) };
    if ret != SDR_OK {
        error!("Failed to open device,return code: 0x{:x}", ret);
        return Err(Error::Sdf(SdfErr::from(ret)));
    }
    if handle.is_null() {
        Err(Error::IllegalState(
            "Open device success, but handle is null".to_string(),
        ))
    } else {
        DeviceHandle::new(handle, fn_close.clone())
    }
}

pub(crate) struct SessionHandle {
    ptr: RawPtr,
    /// hold device handle
    device_handle: Arc<DeviceHandle>,
    close_session: sdf::SDF_CloseSession,

    /// Marker to inform the compiler about ownership and thread-safety.
    _marker: PhantomData<()>,
}
impl SessionHandle {
    pub fn new(
        ptr: RawPtr,
        device_handle: Arc<DeviceHandle>,
        close_session: sdf::SDF_CloseSession,
    ) -> crate::Result<Self> {
        Ok(Self {
            ptr,
            device_handle,
            close_session,
            _marker: PhantomData,
        })
    }
}

unsafe impl Send for SessionHandle {}

unsafe impl Sync for SessionHandle {}

impl Debug for SessionHandle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SessionHandle {{ ptr: {:p} }}", self.ptr)
    }
}
impl Drop for SessionHandle {
    #[instrument]
    fn drop(&mut self) {
        let ret = unsafe { (self.close_session)(self.ptr) };
        if ret != SDR_OK {
            error!(
                "Failed to close session,handle={:p},return code: 0x{:x}",
                self.ptr, ret
            );
        } else {
            trace!("Close session success,handle={:p}", self.ptr)
        }
    }
}

pub fn open_session(
    device_handle: Arc<DeviceHandle>,
    symbols: SessionSymbols,
) -> crate::Result<SessionHandle> {
    let fn_close = symbols
        .close_session
        .as_ref()
        .expect("Symbol(SDF_CloseDevice) not load");
    let fn_open = symbols
        .open_session
        .as_ref()
        .expect("Symbol(SDF_OpenDevice) not load");
    let mut handle: RawPtr = std::ptr::null_mut();
    let ret = unsafe { fn_open(device_handle.ptr, &mut handle) };
    if ret != SDR_OK {
        error!("Failed to open session,return code: 0x{:x}", ret);
        return Err(Error::Sdf(SdfErr::from(ret)));
    }
    if handle.is_null() {
        Err(Error::IllegalState(
            "Open session success, but handle is null".to_string(),
        ))
    } else {
        SessionHandle::new(handle, device_handle, fn_close.clone())
    }
}
