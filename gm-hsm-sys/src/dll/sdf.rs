#![allow(non_camel_case_types)]

use crate::define_symbol;
use crate::sdf::types::{
    DeviceInfo, ECCCipher, ECCSignature, ECCrefPrivateKey, ECCrefPublicKey, RSArefPrivateKey,
    RSArefPublicKey,
};

// Device and session management
define_symbol!(FN_SDF_OpenDevice, SDF_OpenDevice, pub fn(*mut *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_CloseDevice, SDF_CloseDevice, pub fn(*mut ::std::os::raw::c_void) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_OpenSession, SDF_OpenSession, pub fn(*mut ::std::os::raw::c_void, *mut *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_CloseSession, SDF_CloseSession, pub fn(*mut ::std::os::raw::c_void) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_GetDeviceInfo, SDF_GetDeviceInfo, pub fn(*mut ::std::os::raw::c_void, *mut DeviceInfo) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_GenerateRandom, SDF_GenerateRandom, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint, *mut ::std::os::raw::c_uchar) -> ::std::os::raw::c_int);

// Key access control
define_symbol!(FN_SDF_GetPrivateKeyAccessRight, SDF_GetPrivateKeyAccessRight, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint, *mut ::std::os::raw::c_uchar, ::std::os::raw::c_uint) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_ReleasePrivateKeyAccessRight, SDF_ReleasePrivateKeyAccessRight, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint) -> ::std::os::raw::c_int);

// RSA key operations
define_symbol!(FN_SDF_ExportSignPublicKey_RSA, SDF_ExportSignPublicKey_RSA, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint, *mut RSArefPublicKey) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_ExportEncPublicKey_RSA, SDF_ExportEncPublicKey_RSA, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint, *mut RSArefPublicKey) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_GenerateKeyPair_RSA, SDF_GenerateKeyPair_RSA, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint, *mut RSArefPublicKey, *mut RSArefPrivateKey) -> ::std::os::raw::c_int);

// RSA key envelope operations
define_symbol!(FN_SDF_GenerateKeyWithIPK_RSA, SDF_GenerateKeyWithIPK_RSA, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint, ::std::os::raw::c_uint, *mut ::std::os::raw::c_uchar, *mut ::std::os::raw::c_uint, *mut *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_GenerateKeyWithEPK_RSA, SDF_GenerateKeyWithEPK_RSA, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint, *mut RSArefPublicKey, *mut ::std::os::raw::c_uchar, *mut ::std::os::raw::c_uint, *mut *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_ImportKeyWithISK_RSA, SDF_ImportKeyWithISK_RSA, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint, *mut ::std::os::raw::c_uchar, ::std::os::raw::c_uint, *mut *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_ExchangeDigitEnvelopeBaseOnRSA, SDF_ExchangeDigitEnvelopeBaseOnRSA, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint, *mut RSArefPublicKey, *mut ::std::os::raw::c_uchar, ::std::os::raw::c_uint, *mut ::std::os::raw::c_uchar, *mut ::std::os::raw::c_uint) -> ::std::os::raw::c_int);

// ECC key operations
define_symbol!(FN_SDF_ExportSignPublicKey_ECC, SDF_ExportSignPublicKey_ECC, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint, *mut ECCrefPublicKey) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_ExportEncPublicKey_ECC, SDF_ExportEncPublicKey_ECC, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint, *mut ECCrefPublicKey) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_GenerateKeyPair_ECC, SDF_GenerateKeyPair_ECC, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint, ::std::os::raw::c_uint, *mut ECCrefPublicKey, *mut ECCrefPrivateKey) -> ::std::os::raw::c_int);

// ECC key envelope operations
define_symbol!(FN_SDF_GenerateKeyWithIPK_ECC, SDF_GenerateKeyWithIPK_ECC, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint, ::std::os::raw::c_uint, *mut ECCCipher, *mut *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_GenerateKeyWithEPK_ECC, SDF_GenerateKeyWithEPK_ECC, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint, ::std::os::raw::c_uint, *mut ECCrefPublicKey, *mut ECCCipher, *mut *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_ImportKeyWithISK_ECC, SDF_ImportKeyWithISK_ECC, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint, *mut ECCCipher, *mut *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int);

// ECC key agreement
define_symbol!(FN_SDF_GenerateAgreementDataWithECC, SDF_GenerateAgreementDataWithECC, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint, ::std::os::raw::c_uint, *mut ::std::os::raw::c_uchar, ::std::os::raw::c_uint, *mut ECCrefPublicKey, *mut ECCrefPublicKey, *mut *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_GenerateKeyWithECC, SDF_GenerateKeyWithECC, pub fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_uchar, ::std::os::raw::c_uint, *mut ECCrefPublicKey, *mut ECCrefPublicKey, *mut ::std::os::raw::c_void, *mut *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_GenerateAgreementDataAndKeyWithECC, SDF_GenerateAgreementDataAndKeyWithECC, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint, ::std::os::raw::c_uint, *mut ::std::os::raw::c_uchar, ::std::os::raw::c_uint, *mut ::std::os::raw::c_uchar, ::std::os::raw::c_uint, *mut ECCrefPublicKey, *mut ECCrefPublicKey, *mut ECCrefPublicKey, *mut ECCrefPublicKey, *mut *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_ExchangeDigitEnvelopeBaseOnECC, SDF_ExchangeDigitEnvelopeBaseOnECC, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint, ::std::os::raw::c_uint, *mut ECCrefPublicKey, *mut ECCCipher, *mut ECCCipher) -> ::std::os::raw::c_int);

// Key management with KEK
define_symbol!(FN_SDF_GenerateKeyWithKEK, SDF_GenerateKeyWithKEK, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint, ::std::os::raw::c_uint, ::std::os::raw::c_uint, *mut ::std::os::raw::c_uchar, *mut ::std::os::raw::c_uint, *mut *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_ImportKeyWithKEK, SDF_ImportKeyWithKEK, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint, ::std::os::raw::c_uint, *mut ::std::os::raw::c_uchar, ::std::os::raw::c_uint, *mut *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_DestroyKey, SDF_DestroyKey, pub fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int);

// RSA operations
define_symbol!(FN_SDF_ExternalPublicKeyOperation_RSA, SDF_ExternalPublicKeyOperation_RSA, pub fn(*mut ::std::os::raw::c_void, *mut RSArefPublicKey, *mut ::std::os::raw::c_uchar, ::std::os::raw::c_uint, *mut ::std::os::raw::c_uchar, *mut ::std::os::raw::c_uint) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_InternalPublicKeyOperation_RSA, SDF_InternalPublicKeyOperation_RSA, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint, *mut ::std::os::raw::c_uchar, ::std::os::raw::c_uint, *mut ::std::os::raw::c_uchar, *mut ::std::os::raw::c_uint) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_InternalPrivateKeyOperation_RSA, SDF_InternalPrivateKeyOperation_RSA, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint, *mut ::std::os::raw::c_uchar, ::std::os::raw::c_uint, *mut ::std::os::raw::c_uchar, *mut ::std::os::raw::c_uint) -> ::std::os::raw::c_int);

// ECC signature operations
define_symbol!(FN_SDF_ExternalVerify_ECC, SDF_ExternalVerify_ECC, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint, *mut ECCrefPublicKey, *mut ::std::os::raw::c_uchar, ::std::os::raw::c_uint, *mut ECCSignature) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_InternalSign_ECC, SDF_InternalSign_ECC, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint, *mut ::std::os::raw::c_uchar, ::std::os::raw::c_uint, *mut ECCSignature) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_InternalVerify_ECC, SDF_InternalVerify_ECC, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint, *mut ::std::os::raw::c_uchar, ::std::os::raw::c_uint, *mut ECCSignature) -> ::std::os::raw::c_int);

// ECC encryption operations
define_symbol!(FN_SDF_ExternalEncrypt_ECC, SDF_ExternalEncrypt_ECC, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint, *mut ECCrefPublicKey, *mut ::std::os::raw::c_uchar, ::std::os::raw::c_uint, *mut ECCCipher) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_InternalEncrypt_ECC, SDF_InternalEncrypt_ECC, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint, ::std::os::raw::c_uint, *mut ::std::os::raw::c_uchar, ::std::os::raw::c_uint, *mut ECCCipher) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_InternalDecrypt_ECC, SDF_InternalDecrypt_ECC, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint, ::std::os::raw::c_uint, *mut ECCCipher, *mut ::std::os::raw::c_uchar, *mut ::std::os::raw::c_uint) -> ::std::os::raw::c_int);

// Symmetric cryptographic operations
define_symbol!(FN_SDF_Encrypt, SDF_Encrypt, pub fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_void, ::std::os::raw::c_uint, *mut ::std::os::raw::c_uchar, *mut ::std::os::raw::c_uchar, ::std::os::raw::c_uint, *mut ::std::os::raw::c_uchar, *mut ::std::os::raw::c_uint) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_Decrypt, SDF_Decrypt, pub fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_void, ::std::os::raw::c_uint, *mut ::std::os::raw::c_uchar, *mut ::std::os::raw::c_uchar, ::std::os::raw::c_uint, *mut ::std::os::raw::c_uchar, *mut ::std::os::raw::c_uint) -> ::std::os::raw::c_int);

// MAC operations
define_symbol!(FN_SDF_CalculateMAC, SDF_CalculateMAC, pub fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_void, ::std::os::raw::c_uint, *mut ::std::os::raw::c_uchar, ::std::os::raw::c_uint, *mut ::std::os::raw::c_uchar, *mut ::std::os::raw::c_uint) -> ::std::os::raw::c_int);

// Hash operations
define_symbol!(FN_SDF_HashInit, SDF_HashInit, pub fn(*mut ::std::os::raw::c_void, ::std::os::raw::c_uint, *mut *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_HashUpdate, SDF_HashUpdate, pub fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_void, *mut ::std::os::raw::c_uchar, ::std::os::raw::c_uint) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_HashFinal, SDF_HashFinal, pub fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_void, *mut ::std::os::raw::c_uchar, *mut ::std::os::raw::c_uint) -> ::std::os::raw::c_int);

// File operations
define_symbol!(FN_SDF_CreateFile, SDF_CreateFile, pub fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_char, ::std::os::raw::c_uint) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_ReadFile, SDF_ReadFile, pub fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_char, ::std::os::raw::c_uint, *mut ::std::os::raw::c_uchar, *mut ::std::os::raw::c_uint) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_WriteFile, SDF_WriteFile, pub fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_char, *mut ::std::os::raw::c_uchar, ::std::os::raw::c_uint) -> ::std::os::raw::c_int);
define_symbol!(FN_SDF_DeleteFile, SDF_DeleteFile, pub fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int);
