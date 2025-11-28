use gm_hsm_sys::dll::skf;
use gm_hsm_sys::dll::symbol::SymbolBundle;
use libloading::Library;
use std::sync::Arc;

#[derive(Default)]
pub(crate) struct ModMag {
    pub enum_dev: Option<skf::SKF_EnumDev>,
    pub wait_plug_event: Option<skf::SKF_WaitForDevEvent>,
    pub cancel_wait_plug_event: Option<skf::SKF_CancelWaitForDevEvent>,
    pub get_dev_state: Option<skf::SKF_GetDevState>,
    pub connect_dev: Option<skf::SKF_ConnectDev>,
}

impl ModMag {
    pub fn load_symbols(lib: &Arc<Library>) -> crate::Result<Self> {
        let enum_dev = Some(unsafe { SymbolBundle::new(lib, b"SKF_EnumDev\0")? });
        let wait_plug_event = Some(unsafe { SymbolBundle::new(lib, b"SKF_WaitForDevEvent\0")? });
        let cancel_wait_plug_event =
            Some(unsafe { SymbolBundle::new(lib, b"SKF_CancelWaitForDevEvent\0")? });
        let get_dev_state = Some(unsafe { SymbolBundle::new(lib, b"SKF_GetDevState\0")? });
        let connect_dev = Some(unsafe { SymbolBundle::new(lib, b"SKF_ConnectDev\0")? });
        let holder = Self {
            enum_dev,
            wait_plug_event,
            cancel_wait_plug_event,
            get_dev_state,
            connect_dev,
        };
        Ok(holder)
    }
}

#[derive(Default)]
pub(crate) struct ModDev {
    pub dev_set_label: Option<skf::SKF_SetLabel>,
    pub dev_dis_connect: Option<skf::SKF_DisConnectDev>,
    pub dev_get_info: Option<skf::SKF_GetDevInfo>,
    pub dev_lock: Option<skf::SKF_LockDev>,
    pub dev_unlock: Option<skf::SKF_UnlockDev>,
    pub dev_transmit: Option<skf::SKF_Transmit>,
    pub dev_auth: Option<skf::SKF_DevAuth>,
    pub dev_change_auth_key: Option<skf::SKF_ChangeDevAuthKey>,
    pub app_create: Option<skf::SKF_CreateApplication>,
    pub app_open: Option<skf::SKF_OpenApplication>,
    pub app_delete: Option<skf::SKF_DeleteApplication>,
    pub app_enum: Option<skf::SKF_EnumApplication>,
    pub gen_random: Option<skf::SKF_GenRandom>,
    pub sym_key_import: Option<skf::SKF_SetSymmKey>,
    pub ecc_ext_encrypt: Option<skf::SKF_ExtECCEncrypt>,
    pub ecc_ext_decrypt: Option<skf::SKF_ExtECCDecrypt>,
    pub ecc_ext_sign: Option<skf::SKF_ExtECCSign>,
    pub ecc_ext_verify: Option<skf::SKF_ExtECCVerify>,
    pub ecc_verify: Option<skf::SKF_ECCVerify>,
    pub ecc_gen_sk: Option<skf::SKF_GenerateKeyWithECC>,
}

impl ModDev {
    pub fn load_symbols(lib: &Arc<Library>) -> crate::Result<Self> {
        let dev_set_label = Some(unsafe { SymbolBundle::new(lib, b"SKF_SetLabel\0")? });
        let dev_dis_connect = Some(unsafe { SymbolBundle::new(lib, b"SKF_DisConnectDev\0")? });
        let dev_get_info = Some(unsafe { SymbolBundle::new(lib, b"SKF_GetDevInfo\0")? });
        let dev_lock = Some(unsafe { SymbolBundle::new(lib, b"SKF_LockDev\0")? });
        let dev_unlock = Some(unsafe { SymbolBundle::new(lib, b"SKF_UnlockDev\0")? });
        let dev_transmit = Some(unsafe { SymbolBundle::new(lib, b"SKF_Transmit\0")? });
        let dev_auth = Some(unsafe { SymbolBundle::new(lib, b"SKF_DevAuth\0")? });
        let dev_change_auth_key =
            Some(unsafe { SymbolBundle::new(lib, b"SKF_ChangeDevAuthKey\0")? });
        let app_create = Some(unsafe { SymbolBundle::new(lib, b"SKF_CreateApplication\0")? });
        let app_open = Some(unsafe { SymbolBundle::new(lib, b"SKF_OpenApplication\0")? });
        let app_delete = Some(unsafe { SymbolBundle::new(lib, b"SKF_DeleteApplication\0")? });
        let app_enum = Some(unsafe { SymbolBundle::new(lib, b"SKF_EnumApplication\0")? });
        let gen_random = Some(unsafe { SymbolBundle::new(lib, b"SKF_GenRandom\0")? });
        let sym_key_import = Some(unsafe { SymbolBundle::new(lib, b"SKF_SetSymmKey\0")? });
        let ecc_ext_encrypt = Some(unsafe { SymbolBundle::new(lib, b"SKF_ExtECCEncrypt\0")? });
        let ecc_ext_decrypt = Some(unsafe { SymbolBundle::new(lib, b"SKF_ExtECCDecrypt\0")? });
        let ecc_ext_sign = Some(unsafe { SymbolBundle::new(lib, b"SKF_ExtECCSign\0")? });
        let ecc_ext_verify = Some(unsafe { SymbolBundle::new(lib, b"SKF_ExtECCVerify\0")? });
        let ecc_verify = Some(unsafe { SymbolBundle::new(lib, b"SKF_ECCVerify\0")? });
        let ecc_gen_sk = Some(unsafe { SymbolBundle::new(lib, b"SKF_GenerateKeyWithECC\0")? });

        let holder = Self {
            dev_set_label,
            dev_dis_connect,
            dev_get_info,
            dev_lock,
            dev_unlock,
            dev_transmit,
            dev_auth,
            dev_change_auth_key,
            app_create,
            app_open,
            app_delete,
            app_enum,
            gen_random,
            sym_key_import,
            ecc_ext_encrypt,
            ecc_ext_decrypt,
            ecc_ext_sign,
            ecc_ext_verify,
            ecc_verify,
            ecc_gen_sk,
        };
        Ok(holder)
    }
}

#[derive(Default)]
pub(crate) struct ModApp {
    pub app_close: Option<skf::SKF_CloseApplication>,
    pub app_clear_secure_state: Option<skf::SKF_ClearSecureState>,
    pub file_get_list: Option<skf::SKF_EnumFiles>,
    pub file_create: Option<skf::SKF_CreateFile>,
    pub file_delete: Option<skf::SKF_DeleteFile>,
    pub file_get_info: Option<skf::SKF_GetFileInfo>,
    pub file_read: Option<skf::SKF_ReadFile>,
    pub file_write: Option<skf::SKF_WriteFile>,
    pub container_get_list: Option<skf::SKF_EnumContainer>,
    pub container_create: Option<skf::SKF_CreateContainer>,
    pub container_delete: Option<skf::SKF_DeleteContainer>,
    pub container_open: Option<skf::SKF_OpenContainer>,
    pub pin_change: Option<skf::SKF_ChangePIN>,
    pub pin_get_info: Option<skf::SKF_GetPINInfo>,
    pub pin_verify: Option<skf::SKF_VerifyPIN>,
    pub pin_unblock: Option<skf::SKF_UnblockPIN>,
}

impl ModApp {
    pub fn load_symbols(lib: &Arc<Library>) -> crate::Result<Self> {
        let app_close = Some(unsafe { SymbolBundle::new(lib, b"SKF_CloseApplication\0")? });
        let app_clear_secure_state =
            Some(unsafe { SymbolBundle::new(lib, b"SKF_ClearSecureState\0")? });
        let file_get_list = Some(unsafe { SymbolBundle::new(lib, b"SKF_EnumFiles\0")? });
        let file_create = Some(unsafe { SymbolBundle::new(lib, b"SKF_CreateFile\0")? });
        let file_delete = Some(unsafe { SymbolBundle::new(lib, b"SKF_DeleteFile\0")? });
        let file_get_info = Some(unsafe { SymbolBundle::new(lib, b"SKF_GetFileInfo\0")? });
        let file_read = Some(unsafe { SymbolBundle::new(lib, b"SKF_ReadFile\0")? });
        let file_write = Some(unsafe { SymbolBundle::new(lib, b"SKF_WriteFile\0")? });
        let container_get_list = Some(unsafe { SymbolBundle::new(lib, b"SKF_EnumContainer\0")? });
        let container_create = Some(unsafe { SymbolBundle::new(lib, b"SKF_CreateContainer\0")? });
        let container_delete = Some(unsafe { SymbolBundle::new(lib, b"SKF_DeleteContainer\0")? });
        let container_open = Some(unsafe { SymbolBundle::new(lib, b"SKF_OpenContainer\0")? });

        let pin_change = Some(unsafe { SymbolBundle::new(lib, b"SKF_ChangePIN\0")? });
        let pin_get_info = Some(unsafe { SymbolBundle::new(lib, b"SKF_GetPINInfo\0")? });
        let pin_verify = Some(unsafe { SymbolBundle::new(lib, b"SKF_VerifyPIN\0")? });
        let pin_unblock = Some(unsafe { SymbolBundle::new(lib, b"SKF_UnblockPIN\0")? });

        let holder = Self {
            app_close,
            file_get_list,
            file_create,
            file_delete,
            file_get_info,
            file_read,
            file_write,
            container_get_list,
            container_create,
            container_delete,
            container_open,
            app_clear_secure_state,
            pin_change,
            pin_get_info,
            pin_verify,
            pin_unblock,
        };
        Ok(holder)
    }
}

#[derive(Default)]
pub(crate) struct ModContainer {
    pub ct_close: Option<skf::SKF_CloseContainer>,
    pub ct_get_type: Option<skf::SKF_GetContainerType>,
    pub ct_imp_cert: Option<skf::SKF_ImportCertificate>,
    pub ct_exp_cert: Option<skf::SKF_ExportCertificate>,
    pub ct_ecc_gen_pair: Option<skf::SKF_GenECCKeyPair>,
    pub ct_ecc_imp_pair: Option<skf::SKF_ImportECCKeyPair>,
    pub ct_ecc_sign: Option<skf::SKF_ECCSignData>,
    pub ct_sk_gen_agreement: Option<skf::SKF_GenerateAgreementDataWithECC>,
    pub ct_sk_gen_agreement_and_key: Option<skf::SKF_GenerateAgreementDataAndKeyWithECC>,
    pub ct_ecc_exp_pub_key: Option<skf::SKF_ExportPublicKey>,
    pub ct_sk_imp: Option<skf::SKF_ImportSessionKey>,
    pub ct_sk_exp: Option<skf::SKF_ECCExportSessionKey>,
}

impl ModContainer {
    pub fn load_symbols(lib: &Arc<Library>) -> crate::Result<Self> {
        let ct_close = Some(unsafe { SymbolBundle::new(lib, b"SKF_CloseContainer\0")? });
        let ct_get_type = Some(unsafe { SymbolBundle::new(lib, b"SKF_GetContainerType\0")? });
        let ct_imp_cert = Some(unsafe { SymbolBundle::new(lib, b"SKF_ImportCertificate\0")? });
        let ct_exp_cert = Some(unsafe { SymbolBundle::new(lib, b"SKF_ExportCertificate\0")? });
        let ct_ecc_gen_pair = Some(unsafe { SymbolBundle::new(lib, b"SKF_GenECCKeyPair\0")? });
        let ct_ecc_imp_pair = Some(unsafe { SymbolBundle::new(lib, b"SKF_ImportECCKeyPair\0")? });
        let ct_ecc_sign = Some(unsafe { SymbolBundle::new(lib, b"SKF_ECCSignData\0")? });
        let ct_sk_gen_agreement =
            Some(unsafe { SymbolBundle::new(lib, b"SKF_GenerateAgreementDataWithECC\0")? });
        let ct_sk_gen_agreement_and_key =
            Some(unsafe { SymbolBundle::new(lib, b"SKF_GenerateAgreementDataAndKeyWithECC\0")? });
        let ct_ecc_exp_pub_key = Some(unsafe { SymbolBundle::new(lib, b"SKF_ExportPublicKey\0")? });
        let ct_sk_imp = Some(unsafe { SymbolBundle::new(lib, b"SKF_ImportSessionKey\0")? });
        let ct_sk_exp = Some(unsafe { SymbolBundle::new(lib, b"SKF_ECCExportSessionKey\0")? });
        let holder = Self {
            ct_close,
            ct_get_type,
            ct_imp_cert,
            ct_exp_cert,
            ct_ecc_gen_pair,
            ct_ecc_imp_pair,
            ct_ecc_sign,
            ct_sk_gen_agreement,
            ct_sk_gen_agreement_and_key,
            ct_ecc_exp_pub_key,
            ct_sk_imp,
            ct_sk_exp,
        };
        Ok(holder)
    }
}

#[derive(Default)]
pub(crate) struct ModBlockCipher {
    pub encrypt_init: Option<skf::SKF_EncryptInit>,
    pub encrypt: Option<skf::SKF_Encrypt>,
    pub encrypt_update: Option<skf::SKF_EncryptUpdate>,
    pub encrypt_final: Option<skf::SKF_EncryptFinal>,
    pub decrypt_init: Option<skf::SKF_DecryptInit>,
    pub decrypt: Option<skf::SKF_Decrypt>,
    pub decrypt_update: Option<skf::SKF_DecryptUpdate>,
    pub decrypt_final: Option<skf::SKF_DecryptFinal>,
}

impl ModBlockCipher {
    pub fn load_symbols(lib: &Arc<Library>) -> crate::Result<Self> {
        let encrypt_init = Some(unsafe { SymbolBundle::new(lib, b"SKF_EncryptInit\0")? });
        let encrypt = Some(unsafe { SymbolBundle::new(lib, b"SKF_Encrypt\0")? });
        let encrypt_update = Some(unsafe { SymbolBundle::new(lib, b"SKF_EncryptUpdate\0")? });
        let encrypt_final = Some(unsafe { SymbolBundle::new(lib, b"SKF_EncryptFinal\0")? });

        let decrypt_init = Some(unsafe { SymbolBundle::new(lib, b"SKF_DecryptInit\0")? });
        let decrypt = Some(unsafe { SymbolBundle::new(lib, b"SKF_Decrypt\0")? });
        let decrypt_update = Some(unsafe { SymbolBundle::new(lib, b"SKF_DecryptUpdate\0")? });
        let decrypt_final = Some(unsafe { SymbolBundle::new(lib, b"SKF_DecryptFinal\0")? });
        let holder = Self {
            encrypt_init,
            encrypt,
            encrypt_update,
            encrypt_final,
            decrypt_init,
            decrypt,
            decrypt_update,
            decrypt_final,
        };
        Ok(holder)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::LibLoader;

    #[test]
    #[ignore]
    fn sym_mod_ctl_test() {
        let lib = Arc::new(LibLoader::env_lookup().unwrap());
        assert!(ModMag::load_symbols(&lib).is_ok());
    }

    #[test]
    #[ignore]
    fn sym_mod_dev_test() {
        let lib = Arc::new(LibLoader::env_lookup().unwrap());
        assert!(ModDev::load_symbols(&lib).is_ok());
    }
}
