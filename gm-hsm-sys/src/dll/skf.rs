#![allow(non_camel_case_types)]

use crate::define_symbol;
use crate::sgd::types::{BOOL, BYTE, CHAR, DWORD, HANDLE, LPSTR, ULONG};
use crate::skf::types::{
    BlockCipherParam, DeviceInfo, ECCCipherBlob, ECCPrivateKeyBlob, ECCPublicKeyBlob,
    ECCSignatureBlob, EnvelopedKeyBlob, FileAttribute,
};

define_symbol!(FN_SKF_WaitForDevEvent ,SKF_WaitForDevEvent ,pub fn(*mut CHAR, *mut ULONG, *mut ULONG) -> ULONG);
define_symbol!(FN_SKF_CancelWaitForDevEvent ,SKF_CancelWaitForDevEvent,pub fn() -> ULONG);
define_symbol!(FN_SKF_EnumDev ,SKF_EnumDev,pub fn(BOOL, *mut CHAR, *mut ULONG) -> ULONG);
define_symbol!(FN_SKF_GetDevState ,SKF_GetDevState, pub fn(*const CHAR, *mut ULONG) -> ULONG);
define_symbol!(FN_SKF_ConnectDev ,SKF_ConnectDev, pub fn(*const CHAR, *mut HANDLE) -> ULONG);
define_symbol!(FN_SKF_DisConnectDev ,SKF_DisConnectDev, pub fn(HANDLE) -> ULONG);
define_symbol!(FN_SKF_SetLabel ,SKF_SetLabel, pub fn(HANDLE, *const CHAR) -> ULONG);
define_symbol!(FN_SKF_GetDevInfo ,SKF_GetDevInfo, pub fn(HANDLE, *mut DeviceInfo) -> ULONG);
define_symbol!(FN_SKF_LockDev ,SKF_LockDev, pub fn(HANDLE, ULONG) -> ULONG);
define_symbol!(FN_SKF_UnlockDev ,SKF_UnlockDev,pub fn(HANDLE) -> ULONG);
define_symbol!(FN_SKF_Transmit ,SKF_Transmit, pub fn(HANDLE, *const BYTE, ULONG, *mut BYTE, *mut ULONG) -> ULONG);
define_symbol!(FN_SKF_ChangeDevAuthKey ,SKF_ChangeDevAuthKey, pub fn(HANDLE, *const BYTE, ULONG) -> ULONG);
define_symbol!(FN_SKF_DevAuth ,SKF_DevAuth, pub fn(HANDLE, *const BYTE, ULONG) -> ULONG);
define_symbol!(FN_SKF_CreateApplication ,SKF_CreateApplication, pub fn(HANDLE, LPSTR, LPSTR, DWORD, LPSTR, DWORD, DWORD, *mut HANDLE) -> ULONG);
define_symbol!(FN_SKF_OpenApplication ,SKF_OpenApplication, pub fn(HANDLE, LPSTR, *mut HANDLE) -> ULONG);
define_symbol!(FN_SKF_DeleteApplication ,SKF_DeleteApplication, pub fn(HANDLE, LPSTR) -> ULONG);
define_symbol!(FN_SKF_EnumApplication ,SKF_EnumApplication, pub fn(HANDLE, *mut CHAR, *mut ULONG) -> ULONG);

define_symbol!(FN_SKF_CloseApplication, SKF_CloseApplication, pub fn(HANDLE) -> ULONG);
define_symbol!(FN_SKF_ClearSecureState, SKF_ClearSecureState, pub fn(HANDLE) -> ULONG);
define_symbol!(FN_SKF_EnumFiles, SKF_EnumFiles, pub fn(HANDLE, *mut CHAR, *mut ULONG) -> ULONG);
define_symbol!(FN_SKF_CreateFile, SKF_CreateFile, pub fn(HANDLE, LPSTR, ULONG, ULONG, ULONG) -> ULONG);
define_symbol!(FN_SKF_DeleteFile, SKF_DeleteFile, pub fn(HANDLE, LPSTR) -> ULONG);
define_symbol!(FN_SKF_GetFileInfo, SKF_GetFileInfo, pub fn(HANDLE, LPSTR, *mut FileAttribute) -> ULONG);
define_symbol!(FN_SKF_ReadFile, SKF_ReadFile, pub fn(HANDLE, LPSTR, ULONG, ULONG, *mut BYTE, *mut ULONG) -> ULONG);
define_symbol!(FN_SKF_WriteFile, SKF_WriteFile, pub fn(HANDLE, LPSTR, ULONG, *const BYTE, ULONG) -> ULONG);
define_symbol!(FN_SKF_ChangePIN, SKF_ChangePIN, pub fn(HANDLE, ULONG, LPSTR, LPSTR, *mut ULONG) -> ULONG);
define_symbol!(FN_SKF_GetPINInfo, SKF_GetPINInfo, pub fn(HANDLE, ULONG, *mut ULONG, *mut ULONG, *mut BOOL) -> ULONG);
define_symbol!(FN_SKF_VerifyPIN, SKF_VerifyPIN, pub fn(HANDLE, ULONG, LPSTR, *mut ULONG) -> ULONG);
define_symbol!(FN_SKF_UnblockPIN, SKF_UnblockPIN, pub fn(HANDLE, LPSTR, LPSTR, *mut ULONG) -> ULONG);

define_symbol!(FN_SKF_CreateContainer, SKF_CreateContainer, pub fn(HANDLE, LPSTR, *mut HANDLE) -> ULONG);
define_symbol!(FN_SKF_DeleteContainer, SKF_DeleteContainer, pub fn(HANDLE, LPSTR) -> ULONG);
define_symbol!(FN_SKF_OpenContainer, SKF_OpenContainer, pub fn(HANDLE, LPSTR, *mut HANDLE) -> ULONG);
define_symbol!(FN_SKF_EnumContainer, SKF_EnumContainer, pub fn(HANDLE, *mut CHAR, *mut ULONG) -> ULONG);
define_symbol!(FN_SKF_CloseContainer, SKF_CloseContainer, pub fn(HANDLE) -> ULONG);
define_symbol!(FN_SKF_GetContainerType, SKF_GetContainerType, pub fn(HANDLE, *mut ULONG) -> ULONG);
define_symbol!(FN_SKF_ImportCertificate, SKF_ImportCertificate, pub fn(HANDLE, BOOL, *const BYTE, ULONG) -> ULONG);
define_symbol!(FN_SKF_ExportCertificate, SKF_ExportCertificate, pub fn(HANDLE, BOOL, *mut BYTE, *mut ULONG) -> ULONG);

define_symbol!(FN_SKF_GenRandom, SKF_GenRandom, pub fn(HANDLE, *mut BYTE, ULONG) -> ULONG);
define_symbol!(FN_SKF_CloseHandle, SKF_CloseHandle, pub fn(HANDLE) -> ULONG);
define_symbol!(FN_SKF_SetSymmKey, SKF_SetSymmKey, pub fn(HANDLE, *const BYTE, ULONG, *mut HANDLE) -> ULONG);
define_symbol!(FN_SKF_EncryptInit, SKF_EncryptInit, pub fn(HANDLE, BlockCipherParam) -> ULONG);
define_symbol!(FN_SKF_Encrypt, SKF_Encrypt, pub fn(HANDLE, *const BYTE, ULONG, *mut BYTE, *mut ULONG) -> ULONG);
define_symbol!(FN_SKF_EncryptUpdate, SKF_EncryptUpdate, pub fn(HANDLE, *const BYTE, ULONG, *mut BYTE, *mut ULONG) -> ULONG);
define_symbol!(FN_SKF_EncryptFinal, SKF_EncryptFinal, pub fn(HANDLE, *mut BYTE, *mut ULONG) -> ULONG);
define_symbol!(FN_SKF_DecryptInit, SKF_DecryptInit, pub fn(HANDLE, BlockCipherParam) -> ULONG);
define_symbol!(FN_SKF_Decrypt, SKF_Decrypt, pub fn(HANDLE, *const BYTE, ULONG, *mut BYTE, *mut ULONG) -> ULONG);
define_symbol!(FN_SKF_DecryptUpdate, SKF_DecryptUpdate, pub fn(HANDLE, *const BYTE, ULONG, *mut BYTE, *mut ULONG) -> ULONG);
define_symbol!(FN_SKF_DecryptFinal, SKF_DecryptFinal, pub fn(HANDLE, *mut BYTE, *mut ULONG) -> ULONG);
define_symbol!(FN_SKF_ExtECCEncrypt, SKF_ExtECCEncrypt, pub fn(HANDLE, *const ECCPublicKeyBlob, *const BYTE, ULONG, *mut ECCCipherBlob) -> ULONG);
define_symbol!(FN_SKF_ExtECCDecrypt, SKF_ExtECCDecrypt, pub fn(HANDLE, *const ECCPrivateKeyBlob, *const ECCCipherBlob, *mut BYTE, *mut ULONG) -> ULONG);
define_symbol!(FN_SKF_ExtECCSign, SKF_ExtECCSign, pub fn(HANDLE, *const ECCPrivateKeyBlob, *const BYTE, ULONG, *mut ECCSignatureBlob) -> ULONG);
define_symbol!(FN_SKF_ExtECCVerify, SKF_ExtECCVerify, pub fn(HANDLE, *const ECCPublicKeyBlob, *const BYTE, ULONG, *const ECCSignatureBlob) -> ULONG);
define_symbol!(FN_SKF_GenECCKeyPair, SKF_GenECCKeyPair, pub fn(HANDLE, ULONG, *mut ECCPublicKeyBlob) -> ULONG);
define_symbol!(FN_SKF_ImportECCKeyPair, SKF_ImportECCKeyPair, pub fn(HANDLE, *const EnvelopedKeyBlob) -> ULONG);
define_symbol!(FN_SKF_ECCSignData, SKF_ECCSignData, pub fn(HANDLE, *const BYTE, ULONG, *mut ECCSignatureBlob) -> ULONG);
define_symbol!(FN_SKF_ECCVerify, SKF_ECCVerify, pub fn(HANDLE, *const ECCPublicKeyBlob, *const BYTE, ULONG, *const ECCSignatureBlob) -> ULONG);
define_symbol!(FN_SKF_ECCExportSessionKey, SKF_ECCExportSessionKey, pub fn(HANDLE, ULONG, *const ECCPublicKeyBlob, *mut ECCCipherBlob, *mut HANDLE) -> ULONG);
define_symbol!(FN_SKF_GenerateAgreementDataWithECC, SKF_GenerateAgreementDataWithECC, pub fn(HANDLE, ULONG, *mut ECCPublicKeyBlob, *const BYTE, ULONG, *mut HANDLE) -> ULONG);
define_symbol!(FN_SKF_GenerateAgreementDataAndKeyWithECC, SKF_GenerateAgreementDataAndKeyWithECC, pub fn(HANDLE, ULONG, *const ECCPublicKeyBlob, *const ECCPublicKeyBlob, *mut ECCPublicKeyBlob, *const BYTE, ULONG, *const BYTE, ULONG, *mut HANDLE) -> ULONG);
define_symbol!(FN_SKF_ExportPublicKey, SKF_ExportPublicKey, pub fn(HANDLE, BOOL, *mut BYTE, *mut ULONG) -> ULONG);
define_symbol!(FN_SKF_ImportSessionKey, SKF_ImportSessionKey, pub fn(HANDLE, ULONG, *const BYTE, ULONG, *mut HANDLE) -> ULONG);
define_symbol!(FN_SKF_GenerateKeyWithECC, SKF_GenerateKeyWithECC, pub fn(HANDLE, *const ECCPublicKeyBlob, *const ECCPublicKeyBlob, *const BYTE, ULONG, *mut HANDLE) -> ULONG);
