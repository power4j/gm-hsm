//! GM/T 0018-2012 types

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

/// Original C struct
/// ```c
/// typedef struct DeviceInfo_st {
/// 	unsigned char IssuerName[40];
/// 	unsigned char DeviceName[16];
/// 	unsigned char DeviceSerial[16];
/// 	unsigned int DeviceVersion;
/// 	unsigned int StandardVersion;
/// 	unsigned int AsymAlgAbility[2];
/// 	unsigned int SymAlgAbility;
/// 	unsigned int HashAlgAbility;
/// 	unsigned int BufferSize;
/// } DeviceInfo;
/// ```
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct DeviceInfo_st {
    pub IssuerName: [::std::os::raw::c_uchar; 40usize],
    pub DeviceName: [::std::os::raw::c_uchar; 16usize],
    pub DeviceSerial: [::std::os::raw::c_uchar; 16usize],
    pub DeviceVersion: ::std::os::raw::c_uint,
    pub StandardVersion: ::std::os::raw::c_uint,
    pub AsymAlgAbility: [::std::os::raw::c_uint; 2usize],
    pub SymAlgAbility: ::std::os::raw::c_uint,
    pub HashAlgAbility: ::std::os::raw::c_uint,
    pub BufferSize: ::std::os::raw::c_uint,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of DeviceInfo_st"][::std::mem::size_of::<DeviceInfo_st>() - 100usize];
    ["Alignment of DeviceInfo_st"][::std::mem::align_of::<DeviceInfo_st>() - 1usize];
    ["Offset of field: DeviceInfo_st::IssuerName"]
        [::std::mem::offset_of!(DeviceInfo_st, IssuerName) - 0usize];
    ["Offset of field: DeviceInfo_st::DeviceName"]
        [::std::mem::offset_of!(DeviceInfo_st, DeviceName) - 40usize];
    ["Offset of field: DeviceInfo_st::DeviceSerial"]
        [::std::mem::offset_of!(DeviceInfo_st, DeviceSerial) - 56usize];
    ["Offset of field: DeviceInfo_st::DeviceVersion"]
        [::std::mem::offset_of!(DeviceInfo_st, DeviceVersion) - 72usize];
    ["Offset of field: DeviceInfo_st::StandardVersion"]
        [::std::mem::offset_of!(DeviceInfo_st, StandardVersion) - 76usize];
    ["Offset of field: DeviceInfo_st::AsymAlgAbility"]
        [::std::mem::offset_of!(DeviceInfo_st, AsymAlgAbility) - 80usize];
    ["Offset of field: DeviceInfo_st::SymAlgAbility"]
        [::std::mem::offset_of!(DeviceInfo_st, SymAlgAbility) - 88usize];
    ["Offset of field: DeviceInfo_st::HashAlgAbility"]
        [::std::mem::offset_of!(DeviceInfo_st, HashAlgAbility) - 92usize];
    ["Offset of field: DeviceInfo_st::BufferSize"]
        [::std::mem::offset_of!(DeviceInfo_st, BufferSize) - 96usize];
};
pub type DeviceInfo = DeviceInfo_st;

/// Original C struct
/// ```c
/// typedef struct RSArefPublicKey_st {
/// 	unsigned int bits;
/// 	unsigned char m[RSAref_MAX_LEN];
/// 	unsigned char e[RSAref_MAX_LEN];
/// } RSArefPublicKey;
/// ```
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct RSArefPublicKey_st {
    pub bits: ::std::os::raw::c_uint,
    pub m: [::std::os::raw::c_uchar; 256usize],
    pub e: [::std::os::raw::c_uchar; 256usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of RSArefPublicKey_st"][::std::mem::size_of::<RSArefPublicKey_st>() - 516usize];
    ["Alignment of RSArefPublicKey_st"][::std::mem::align_of::<RSArefPublicKey_st>() - 1usize];
    ["Offset of field: RSArefPublicKey_st::bits"]
        [::std::mem::offset_of!(RSArefPublicKey_st, bits) - 0usize];
    ["Offset of field: RSArefPublicKey_st::m"]
        [::std::mem::offset_of!(RSArefPublicKey_st, m) - 4usize];
    ["Offset of field: RSArefPublicKey_st::e"]
        [::std::mem::offset_of!(RSArefPublicKey_st, e) - 260usize];
};
pub type RSArefPublicKey = RSArefPublicKey_st;

/// Original C struct
/// ```c
/// typedef struct RSArefPrivateKey_st {
/// 	unsigned int bits;
/// 	unsigned char m[RSAref_MAX_LEN];
/// 	unsigned char e[RSAref_MAX_LEN];
/// 	unsigned char d[RSAref_MAX_LEN];
/// 	unsigned char prime[2][RSAref_MAX_PLEN];
/// 	unsigned char pexp[2][RSAref_MAX_PLEN];
/// 	unsigned char coef[RSAref_MAX_PLEN];
/// } RSArefPrivateKey;
/// ```
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct RSArefPrivateKey_st {
    pub bits: ::std::os::raw::c_uint,
    pub m: [::std::os::raw::c_uchar; 256usize],
    pub e: [::std::os::raw::c_uchar; 256usize],
    pub d: [::std::os::raw::c_uchar; 256usize],
    pub prime: [[::std::os::raw::c_uchar; 128usize]; 2usize],
    pub pexp: [[::std::os::raw::c_uchar; 128usize]; 2usize],
    pub coef: [::std::os::raw::c_uchar; 128usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of RSArefPrivateKey_st"][::std::mem::size_of::<RSArefPrivateKey_st>() - 1412usize];
    ["Alignment of RSArefPrivateKey_st"][::std::mem::align_of::<RSArefPrivateKey_st>() - 1usize];
    ["Offset of field: RSArefPrivateKey_st::bits"]
        [::std::mem::offset_of!(RSArefPrivateKey_st, bits) - 0usize];
    ["Offset of field: RSArefPrivateKey_st::m"]
        [::std::mem::offset_of!(RSArefPrivateKey_st, m) - 4usize];
    ["Offset of field: RSArefPrivateKey_st::e"]
        [::std::mem::offset_of!(RSArefPrivateKey_st, e) - 260usize];
    ["Offset of field: RSArefPrivateKey_st::d"]
        [::std::mem::offset_of!(RSArefPrivateKey_st, d) - 516usize];
    ["Offset of field: RSArefPrivateKey_st::prime"]
        [::std::mem::offset_of!(RSArefPrivateKey_st, prime) - 772usize];
    ["Offset of field: RSArefPrivateKey_st::pexp"]
        [::std::mem::offset_of!(RSArefPrivateKey_st, pexp) - 1028usize];
    ["Offset of field: RSArefPrivateKey_st::coef"]
        [::std::mem::offset_of!(RSArefPrivateKey_st, coef) - 1284usize];
};
pub type RSArefPrivateKey = RSArefPrivateKey_st;

/// Original C struct
/// ```c
/// typedef struct ECCrefPublicKey_st {
/// 	unsigned int bits;
/// 	unsigned char x[ECCref_MAX_LEN];
/// 	unsigned char y[ECCref_MAX_LEN];
/// } ECCrefPublicKey;
/// ```
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ECCrefPublicKey_st {
    pub bits: ::std::os::raw::c_uint,
    pub x: [::std::os::raw::c_uchar; 64usize],
    pub y: [::std::os::raw::c_uchar; 64usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ECCrefPublicKey_st"][::std::mem::size_of::<ECCrefPublicKey_st>() - 132usize];
    ["Alignment of ECCrefPublicKey_st"][::std::mem::align_of::<ECCrefPublicKey_st>() - 1usize];
    ["Offset of field: ECCrefPublicKey_st::bits"]
        [::std::mem::offset_of!(ECCrefPublicKey_st, bits) - 0usize];
    ["Offset of field: ECCrefPublicKey_st::x"]
        [::std::mem::offset_of!(ECCrefPublicKey_st, x) - 4usize];
    ["Offset of field: ECCrefPublicKey_st::y"]
        [::std::mem::offset_of!(ECCrefPublicKey_st, y) - 68usize];
};
pub type ECCrefPublicKey = ECCrefPublicKey_st;

/// Original C struct
/// ```c
/// typedef struct ECCrefPrivateKey_st {
/// 	unsigned int bits;
/// 	unsigned char K[ECCref_MAX_LEN];
/// } ECCrefPrivateKey;
/// ```
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ECCrefPrivateKey_st {
    pub bits: ::std::os::raw::c_uint,
    pub K: [::std::os::raw::c_uchar; 64usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ECCrefPrivateKey_st"][::std::mem::size_of::<ECCrefPrivateKey_st>() - 68usize];
    ["Alignment of ECCrefPrivateKey_st"][::std::mem::align_of::<ECCrefPrivateKey_st>() - 1usize];
    ["Offset of field: ECCrefPrivateKey_st::bits"]
        [::std::mem::offset_of!(ECCrefPrivateKey_st, bits) - 0usize];
    ["Offset of field: ECCrefPrivateKey_st::K"]
        [::std::mem::offset_of!(ECCrefPrivateKey_st, K) - 4usize];
};
pub type ECCrefPrivateKey = ECCrefPrivateKey_st;

/// Original C struct
/// ```c
/// typedef struct ECCCipher_st {
/// 	unsigned char x[ECCref_MAX_LEN];
/// 	unsigned char y[ECCref_MAX_LEN];
/// 	unsigned char M[32];
/// 	unsigned int L;
/// 	unsigned char C;
/// } ECCCipher;
/// ```
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ECCCipher_st {
    pub x: [::std::os::raw::c_uchar; 64usize],
    pub y: [::std::os::raw::c_uchar; 64usize],
    pub M: [::std::os::raw::c_uchar; 32usize],
    pub L: ::std::os::raw::c_uint,
    pub C: [::std::os::raw::c_uchar; 1usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ECCCipher_st"][::std::mem::size_of::<ECCCipher_st>() - 165usize];
    ["Alignment of ECCCipher_st"][::std::mem::align_of::<ECCCipher_st>() - 1usize];
    ["Offset of field: ECCCipher_st::x"][::std::mem::offset_of!(ECCCipher_st, x) - 0usize];
    ["Offset of field: ECCCipher_st::y"][::std::mem::offset_of!(ECCCipher_st, y) - 64usize];
    ["Offset of field: ECCCipher_st::M"][::std::mem::offset_of!(ECCCipher_st, M) - 128usize];
    ["Offset of field: ECCCipher_st::L"][::std::mem::offset_of!(ECCCipher_st, L) - 160usize];
    ["Offset of field: ECCCipher_st::C"][::std::mem::offset_of!(ECCCipher_st, C) - 164usize];
};
pub type ECCCipher = ECCCipher_st;

/// Original C struct
/// ```c
/// typedef struct ECCSignature_st {
/// 	unsigned char r[ECCref_MAX_LEN];
/// 	unsigned char s[ECCref_MAX_LEN];
/// } ECCSignature;
/// ```
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ECCSignature_st {
    pub r: [::std::os::raw::c_uchar; 64usize],
    pub s: [::std::os::raw::c_uchar; 64usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ECCSignature_st"][::std::mem::size_of::<ECCSignature_st>() - 128usize];
    ["Alignment of ECCSignature_st"][::std::mem::align_of::<ECCSignature_st>() - 1usize];
    ["Offset of field: ECCSignature_st::r"][::std::mem::offset_of!(ECCSignature_st, r) - 0usize];
    ["Offset of field: ECCSignature_st::s"][::std::mem::offset_of!(ECCSignature_st, s) - 64usize];
};
pub type ECCSignature = ECCSignature_st;

/// Original C struct
/// ```c
/// typedef struct SDF_ENVELOPEDKEYBLOB {
/// 	unsigned long Version;
/// 	unsigned long ulSymmAlgID;
/// 	ECCCipher ECCCipehrBlob;
/// 	ECCrefPublicKey PubKey;
/// 	unsigned char cbEncryptedPrivKey[ECCref_MAX_LEN];
/// } SDF_ENVELOPEDKEYBLOB;
/// ```
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct EnvelopedKeyBlob_st {
    pub Version: ::std::os::raw::c_ulong,
    pub ulSymmAlgID: ::std::os::raw::c_ulong,
    pub ECCCipehrBlob: ECCCipher,
    pub PubKey: ECCrefPublicKey,
    pub cbEncryptedPrivKey: [::std::os::raw::c_uchar; 64usize],
}
// c_ulong is platform dependent
const ULONG_SIZE: usize = ::std::mem::size_of::<::std::os::raw::c_ulong>();

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    const EXPECTED_SIZE: usize = ULONG_SIZE * 2 + 165 + 132 + 64;
    ["Size of EnvelopedKeyBlob_st"][::std::mem::size_of::<EnvelopedKeyBlob_st>() - EXPECTED_SIZE];
    ["Alignment of EnvelopedKeyBlob_st"][::std::mem::align_of::<EnvelopedKeyBlob_st>() - 1usize];
    ["Offset of field: EnvelopedKeyBlob_st::Version"]
        [::std::mem::offset_of!(EnvelopedKeyBlob_st, Version) - 0usize];
    ["Offset of field: EnvelopedKeyBlob_st::ulSymmAlgID"]
        [::std::mem::offset_of!(EnvelopedKeyBlob_st, ulSymmAlgID) - ULONG_SIZE];
    ["Offset of field: EnvelopedKeyBlob_st::ECCCipehrBlob"]
        [::std::mem::offset_of!(EnvelopedKeyBlob_st, ECCCipehrBlob) - (ULONG_SIZE * 2)];
    ["Offset of field: EnvelopedKeyBlob_st::PubKey"]
        [::std::mem::offset_of!(EnvelopedKeyBlob_st, PubKey) - (ULONG_SIZE * 2 + 165)];
    ["Offset of field: EnvelopedKeyBlob_st::cbEncryptedPrivKey"]
        [::std::mem::offset_of!(EnvelopedKeyBlob_st, cbEncryptedPrivKey) - (ULONG_SIZE * 2 + 165 + 132)];
};
pub type EnvelopedKeyBlob = EnvelopedKeyBlob_st;
pub type PEnvelopedKeyBlob = *mut EnvelopedKeyBlob_st;

unsafe extern "C" {
    pub fn SDF_OpenDevice(
        phDeviceHandle: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_CloseDevice(hDeviceHandle: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_OpenSession(
        hDeviceHandle: *mut ::std::os::raw::c_void,
        phSessionHandle: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_CloseSession(hSessionHandle: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_GetDeviceInfo(
        hSessionHandle: *mut ::std::os::raw::c_void,
        pstDeviceInfo: *mut DeviceInfo,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_GenerateRandom(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiLength: ::std::os::raw::c_uint,
        pucRandom: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_GetPrivateKeyAccessRight(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiKeyIndex: ::std::os::raw::c_uint,
        pucPassword: *mut ::std::os::raw::c_uchar,
        uiPwdLength: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_ReleasePrivateKeyAccessRight(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiKeyIndex: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_ExportSignPublicKey_RSA(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiKeyIndex: ::std::os::raw::c_uint,
        pucPublicKey: *mut RSArefPublicKey,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_ExportEncPublicKey_RSA(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiKeyIndex: ::std::os::raw::c_uint,
        pucPublicKey: *mut RSArefPublicKey,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_GenerateKeyPair_RSA(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiKeyBits: ::std::os::raw::c_uint,
        pucPublicKey: *mut RSArefPublicKey,
        pucPrivateKey: *mut RSArefPrivateKey,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_GenerateKeyWithIPK_RSA(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiIPKIndex: ::std::os::raw::c_uint,
        uiKeyBits: ::std::os::raw::c_uint,
        pucKey: *mut ::std::os::raw::c_uchar,
        puiKeyLength: *mut ::std::os::raw::c_uint,
        phKeyHandle: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_GenerateKeyWithEPK_RSA(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiKeyBits: ::std::os::raw::c_uint,
        pucPublicKey: *mut RSArefPublicKey,
        pucKey: *mut ::std::os::raw::c_uchar,
        puiKeyLength: *mut ::std::os::raw::c_uint,
        phKeyHandle: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_ImportKeyWithISK_RSA(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiISKIndex: ::std::os::raw::c_uint,
        pucKey: *mut ::std::os::raw::c_uchar,
        uiKeyLength: ::std::os::raw::c_uint,
        phKeyHandle: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_ExchangeDigitEnvelopeBaseOnRSA(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiKeyIndex: ::std::os::raw::c_uint,
        pucPublicKey: *mut RSArefPublicKey,
        pucDEInput: *mut ::std::os::raw::c_uchar,
        uiDELength: ::std::os::raw::c_uint,
        pucDEOutput: *mut ::std::os::raw::c_uchar,
        puiDELength: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_ExportSignPublicKey_ECC(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiKeyIndex: ::std::os::raw::c_uint,
        pucPublicKey: *mut ECCrefPublicKey,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_ExportEncPublicKey_ECC(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiKeyIndex: ::std::os::raw::c_uint,
        pucPublicKey: *mut ECCrefPublicKey,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_GenerateKeyPair_ECC(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiAlgID: ::std::os::raw::c_uint,
        uiKeyBits: ::std::os::raw::c_uint,
        pucPublicKey: *mut ECCrefPublicKey,
        pucPrivateKey: *mut ECCrefPrivateKey,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_GenerateKeyWithIPK_ECC(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiIPKIndex: ::std::os::raw::c_uint,
        uiKeyBits: ::std::os::raw::c_uint,
        pucKey: *mut ECCCipher,
        phKeyHandle: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_GenerateKeyWithEPK_ECC(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiKeyBits: ::std::os::raw::c_uint,
        uiAlgID: ::std::os::raw::c_uint,
        pucPublicKey: *mut ECCrefPublicKey,
        pucKey: *mut ECCCipher,
        phKeyHandle: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_ImportKeyWithISK_ECC(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiISKIndex: ::std::os::raw::c_uint,
        pucKey: *mut ECCCipher,
        phKeyHandle: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_GenerateAgreementDataWithECC(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiISKIndex: ::std::os::raw::c_uint,
        uiKeyBits: ::std::os::raw::c_uint,
        pucSponsorID: *mut ::std::os::raw::c_uchar,
        uiSponsorIDLength: ::std::os::raw::c_uint,
        pucSponsorPublicKey: *mut ECCrefPublicKey,
        pucSponsorTmpPublicKey: *mut ECCrefPublicKey,
        phAgreementHandle: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_GenerateKeyWithECC(
        hSessionHandle: *mut ::std::os::raw::c_void,
        pucResponseID: *mut ::std::os::raw::c_uchar,
        uiResponseIDLength: ::std::os::raw::c_uint,
        pucResponsePublicKey: *mut ECCrefPublicKey,
        pucResponseTmpPublicKey: *mut ECCrefPublicKey,
        hAgreementHandle: *mut ::std::os::raw::c_void,
        phKeyHandle: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_GenerateAgreementDataAndKeyWithECC(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiISKIndex: ::std::os::raw::c_uint,
        uiKeyBits: ::std::os::raw::c_uint,
        pucResponseID: *mut ::std::os::raw::c_uchar,
        uiResponseIDLength: ::std::os::raw::c_uint,
        pucSponsorID: *mut ::std::os::raw::c_uchar,
        uiSponsorIDLength: ::std::os::raw::c_uint,
        pucSponsorPublicKey: *mut ECCrefPublicKey,
        pucSponsorTmpPublicKey: *mut ECCrefPublicKey,
        pucResponsePublicKey: *mut ECCrefPublicKey,
        pucResponseTmpPublicKey: *mut ECCrefPublicKey,
        phKeyHandle: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_ExchangeDigitEnvelopeBaseOnECC(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiKeyIndex: ::std::os::raw::c_uint,
        uiAlgID: ::std::os::raw::c_uint,
        pucPublicKey: *mut ECCrefPublicKey,
        pucEncDataIn: *mut ECCCipher,
        pucEncDataOut: *mut ECCCipher,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_GenerateKeyWithKEK(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiKeyBits: ::std::os::raw::c_uint,
        uiAlgID: ::std::os::raw::c_uint,
        uiKEKIndex: ::std::os::raw::c_uint,
        pucKey: *mut ::std::os::raw::c_uchar,
        puiKeyLength: *mut ::std::os::raw::c_uint,
        phKeyHandle: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_ImportKeyWithKEK(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiAlgID: ::std::os::raw::c_uint,
        uiKEKIndex: ::std::os::raw::c_uint,
        pucKey: *mut ::std::os::raw::c_uchar,
        uiKeyLength: ::std::os::raw::c_uint,
        phKeyHandle: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_DestroyKey(
        hSessionHandle: *mut ::std::os::raw::c_void,
        hKeyHandle: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_ExternalPublicKeyOperation_RSA(
        hSessionHandle: *mut ::std::os::raw::c_void,
        pucPublicKey: *mut RSArefPublicKey,
        pucDataInput: *mut ::std::os::raw::c_uchar,
        uiInputLength: ::std::os::raw::c_uint,
        pucDataOutput: *mut ::std::os::raw::c_uchar,
        puiOutputLength: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_InternalPublicKeyOperation_RSA(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiKeyIndex: ::std::os::raw::c_uint,
        pucDataInput: *mut ::std::os::raw::c_uchar,
        uiInputLength: ::std::os::raw::c_uint,
        pucDataOutput: *mut ::std::os::raw::c_uchar,
        puiOutputLength: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_InternalPrivateKeyOperation_RSA(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiKeyIndex: ::std::os::raw::c_uint,
        pucDataInput: *mut ::std::os::raw::c_uchar,
        uiInputLength: ::std::os::raw::c_uint,
        pucDataOutput: *mut ::std::os::raw::c_uchar,
        puiOutputLength: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_ExternalVerify_ECC(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiAlgID: ::std::os::raw::c_uint,
        pucPublicKey: *mut ECCrefPublicKey,
        pucDataInput: *mut ::std::os::raw::c_uchar,
        uiInputLength: ::std::os::raw::c_uint,
        pucSignature: *mut ECCSignature,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_InternalSign_ECC(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiISKIndex: ::std::os::raw::c_uint,
        pucData: *mut ::std::os::raw::c_uchar,
        uiDataLength: ::std::os::raw::c_uint,
        pucSignature: *mut ECCSignature,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_InternalVerify_ECC(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiIPKIndex: ::std::os::raw::c_uint,
        pucData: *mut ::std::os::raw::c_uchar,
        uiDataLength: ::std::os::raw::c_uint,
        pucSignature: *mut ECCSignature,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_ExternalEncrypt_ECC(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiAlgID: ::std::os::raw::c_uint,
        pucPublicKey: *mut ECCrefPublicKey,
        pucData: *mut ::std::os::raw::c_uchar,
        uiDataLength: ::std::os::raw::c_uint,
        pucEncData: *mut ECCCipher,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_InternalEncrypt_ECC(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiIPKIndex: ::std::os::raw::c_uint,
        uiAlgID: ::std::os::raw::c_uint,
        pucData: *mut ::std::os::raw::c_uchar,
        uiDataLength: ::std::os::raw::c_uint,
        pucEncData: *mut ECCCipher,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_InternalDecrypt_ECC(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiISKIndex: ::std::os::raw::c_uint,
        uiAlgID: ::std::os::raw::c_uint,
        pucEncData: *mut ECCCipher,
        pucData: *mut ::std::os::raw::c_uchar,
        uiDataLength: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_Encrypt(
        hSessionHandle: *mut ::std::os::raw::c_void,
        hKeyHandle: *mut ::std::os::raw::c_void,
        uiAlgID: ::std::os::raw::c_uint,
        pucIV: *mut ::std::os::raw::c_uchar,
        pucData: *mut ::std::os::raw::c_uchar,
        uiDataLength: ::std::os::raw::c_uint,
        pucEncData: *mut ::std::os::raw::c_uchar,
        puiEncDataLength: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_Decrypt(
        hSessionHandle: *mut ::std::os::raw::c_void,
        hKeyHandle: *mut ::std::os::raw::c_void,
        uiAlgID: ::std::os::raw::c_uint,
        pucIV: *mut ::std::os::raw::c_uchar,
        pucEncData: *mut ::std::os::raw::c_uchar,
        uiEncDataLength: ::std::os::raw::c_uint,
        pucData: *mut ::std::os::raw::c_uchar,
        puiDataLength: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_CalculateMAC(
        hSessionHandle: *mut ::std::os::raw::c_void,
        hKeyHandle: *mut ::std::os::raw::c_void,
        uiAlgID: ::std::os::raw::c_uint,
        pucIV: *mut ::std::os::raw::c_uchar,
        pucData: *mut ::std::os::raw::c_uchar,
        uiDataLength: ::std::os::raw::c_uint,
        pucMAC: *mut ::std::os::raw::c_uchar,
        puiMACLength: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_HashInit(
        hSessionHandle: *mut ::std::os::raw::c_void,
        uiAlgID: ::std::os::raw::c_uint,
        pucPublicKey: *mut ECCrefPublicKey,
        pucID: *mut ::std::os::raw::c_uchar,
        uiIDLength: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_HashUpdate(
        hSessionHandle: *mut ::std::os::raw::c_void,
        pucData: *mut ::std::os::raw::c_uchar,
        uiDataLength: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_HashFinal(
        hSessionHandle: *mut ::std::os::raw::c_void,
        pucHash: *mut ::std::os::raw::c_uchar,
        puiHashLength: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_CreateFile(
        hSessionHandle: *mut ::std::os::raw::c_void,
        pucFileName: *mut ::std::os::raw::c_uchar,
        uiNameLen: ::std::os::raw::c_uint,
        uiFileSize: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_ReadFile(
        hSessionHandle: *mut ::std::os::raw::c_void,
        pucFileName: *mut ::std::os::raw::c_uchar,
        uiNameLen: ::std::os::raw::c_uint,
        uiOffset: ::std::os::raw::c_uint,
        puiReadLength: *mut ::std::os::raw::c_uint,
        pucBuffer: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_WriteFile(
        hSessionHandle: *mut ::std::os::raw::c_void,
        pucFileName: *mut ::std::os::raw::c_uchar,
        uiNameLen: ::std::os::raw::c_uint,
        uiOffset: ::std::os::raw::c_uint,
        uiWriteLength: ::std::os::raw::c_uint,
        pucBuffer: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SDF_DeleteFile(
        hSessionHandle: *mut ::std::os::raw::c_void,
        pucFileName: *mut ::std::os::raw::c_uchar,
        uiNameLen: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}

pub const RSAref_MAX_BITS: usize = 2048;
pub const RSAref_MAX_LEN: usize = (RSAref_MAX_BITS + 7) / 8;
pub const RSAref_MAX_PBITS: usize = (RSAref_MAX_BITS + 1) / 2;
pub const RSAref_MAX_PLEN: usize = (RSAref_MAX_PBITS + 7) / 8;
pub const ECCref_MAX_BITS: usize = 512;
pub const ECCref_MAX_LEN: usize = (ECCref_MAX_BITS + 7) / 8;

/// for SGD_MAX_ECC_BITS_256
pub const ECCref_256_MAX_BITS: usize = 256;
pub const ECCref_256_MAX_LEN: usize = (ECCref_256_MAX_BITS + 7) / 8;

/// Check size against constants
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    const RSA_PUBLIC_KEY_SIZE: usize =
        ::std::mem::size_of::<::std::os::raw::c_uint>() + RSAref_MAX_LEN + RSAref_MAX_LEN;
    ["Check RSAref_MAX_LEN for RSArefPublicKey_st"]
        [::std::mem::size_of::<RSArefPublicKey_st>() - RSA_PUBLIC_KEY_SIZE];

    const RSA_PRIVATE_KEY_SIZE: usize =
        ::std::mem::size_of::<::std::os::raw::c_uint>() + RSAref_MAX_LEN * 3 + RSAref_MAX_PLEN * 5;
    ["Check RSAref_MAX_LEN for RSArefPrivateKey_st"]
        [::std::mem::size_of::<RSArefPrivateKey_st>() - RSA_PRIVATE_KEY_SIZE];

    const ECC_PUBLIC_KEY_SIZE: usize =
        ::std::mem::size_of::<::std::os::raw::c_uint>() + ECCref_MAX_LEN + ECCref_MAX_LEN;
    ["Check ECCref_MAX_LEN for ECCrefPublicKey_st"]
        [::std::mem::size_of::<ECCrefPublicKey_st>() - ECC_PUBLIC_KEY_SIZE];

    const ECC_PRIVATE_KEY_SIZE: usize =
        ::std::mem::size_of::<::std::os::raw::c_uint>() + ECCref_MAX_LEN;
    ["Check ECCref_MAX_LEN for ECCrefPrivateKey_st"]
        [::std::mem::size_of::<ECCrefPrivateKey_st>() - ECC_PRIVATE_KEY_SIZE];

    const ECC_CIPHER_SIZE: usize = ECCref_MAX_LEN * 2
        + 32
        + ::std::mem::size_of::<::std::os::raw::c_uint>()
        + ::std::mem::size_of::<::std::os::raw::c_char>();
    ["Check ECCref_MAX_LEN for ECCCipher_st"]
        [::std::mem::size_of::<ECCCipher_st>() - ECC_CIPHER_SIZE];

    const ECC_SIGNATURE_SIZE: usize = ECCref_MAX_LEN + ECCref_MAX_LEN;
    ["Check ECCref_MAX_LEN for ECCSignature_st"]
        [::std::mem::size_of::<ECCSignature_st>() - ECC_SIGNATURE_SIZE];
};

pub const SDR_OK: i32 = 0x0;
pub const SDR_BASE: i32 = 0x01000000;
pub const SDR_UNKNOWERR: i32 = SDR_BASE + 0x00000001;
pub const SDR_NOTSUPPORT: i32 = SDR_BASE + 0x00000002;
pub const SDR_COMMFAIL: i32 = SDR_BASE + 0x00000003;
pub const SDR_HARDFAIL: i32 = SDR_BASE + 0x00000004;
pub const SDR_OPENDEVICE: i32 = SDR_BASE + 0x00000005;
pub const SDR_OPENSESSION: i32 = SDR_BASE + 0x00000006;
pub const SDR_PARDENY: i32 = SDR_BASE + 0x00000007;
pub const SDR_KEYNOTEXIST: i32 = SDR_BASE + 0x00000008;
pub const SDR_ALGNOTSUPPORT: i32 = SDR_BASE + 0x00000009;
pub const SDR_ALGMODNOTSUPPORT: i32 = SDR_BASE + 0x0000000A;
pub const SDR_PKOPERR: i32 = SDR_BASE + 0x0000000B;
pub const SDR_SKOPERR: i32 = SDR_BASE + 0x0000000C;
pub const SDR_SIGNERR: i32 = SDR_BASE + 0x0000000D;
pub const SDR_VERIFYERR: i32 = SDR_BASE + 0x0000000E;
pub const SDR_SYMOPERR: i32 = SDR_BASE + 0x0000000F;
pub const SDR_STEPERR: i32 = SDR_BASE + 0x00000010;
pub const SDR_FILESIZEERR: i32 = SDR_BASE + 0x00000011;
pub const SDR_FILENOEXIST: i32 = SDR_BASE + 0x00000012;
pub const SDR_FILEOFSERR: i32 = SDR_BASE + 0x00000013;
pub const SDR_KEYTYPEERR: i32 = SDR_BASE + 0x00000014;
pub const SDR_KEYERR: i32 = SDR_BASE + 0x00000015;
pub const SDR_ENCDATAERR: i32 = SDR_BASE + 0x00000016;
pub const SDR_RANDERR: i32 = SDR_BASE + 0x00000017;
pub const SDR_PRKRERR: i32 = SDR_BASE + 0x00000018;
pub const SDR_MACERR: i32 = SDR_BASE + 0x00000019;
pub const SDR_FILEEXSITS: i32 = SDR_BASE + 0x0000001A;
pub const SDR_FILEWERR: i32 = SDR_BASE + 0x0000001B;
pub const SDR_NOBUFFER: i32 = SDR_BASE + 0x0000001C;
pub const SDR_INARGERR: i32 = SDR_BASE + 0x0000001D;
pub const SDR_OUTARGERR: i32 = SDR_BASE + 0x0000001E;
