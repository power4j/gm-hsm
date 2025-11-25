pub mod types {
    #[derive(Copy, Clone, Debug, Default)]
    #[repr(C)]
    pub struct Void {
        _inner: [u8; 0],
        _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
    }

    pub type INT8 = i8;
    pub type INT16 = i16;
    pub type INT32 = i32;
    pub type UINT8 = u8;
    pub type UINT16 = u16;
    pub type UINT32 = u32;
    pub type SHORT = INT16;
    pub type LONG = INT32;
    pub type UINT = INT32;
    pub type USHORT = UINT16;
    pub type ULONG = UINT32;

    pub type BOOL = UINT32;
    pub type BYTE = UINT8;
    pub type CHAR = UINT8;

    pub type WORD = UINT16;

    pub type DWORD = UINT32;
    pub type FLAGS = UINT32;

    pub type LPSTR = *const CHAR;
    pub type HANDLE = *const Void;

    pub const TRUE: BOOL = 0x00000001;
    pub const FALSE: BOOL = 0x00000000;
}
