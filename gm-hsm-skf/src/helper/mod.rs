pub mod auth;
pub mod easy;

pub mod mem {
    use crate::{ECCEncryptedData, EnvelopedKeyData};
    use gm_hsm_sys::skf::types::ECCPublicKeyBlob;
    impl ECCEncryptedData {
        /// Convert to bytes of `ECCCipherBlob`
        pub fn blob_bytes(&self) -> Vec<u8> {
            use gm_hsm_sys::sgd::types::ULONG;

            let len = 64 + 64 + 32 + 4 + self.cipher.len();
            let mut vec: Vec<u8> = Vec::with_capacity(len);
            let cipher_len: [u8; 4] = (self.cipher.len() as ULONG).to_ne_bytes();
            vec.extend_from_slice(&self.ec_x);
            vec.extend_from_slice(&self.ec_y);
            vec.extend_from_slice(&self.hash);
            vec.extend_from_slice(&cipher_len);
            vec.extend_from_slice(&self.cipher);
            vec
        }
    }

    impl EnvelopedKeyData {
        /// Convert to bytes of `EnvelopedKeyBlob`
        pub fn blob_bytes(&self) -> Vec<u8> {
            use gm_hsm_sys::sgd::types::ULONG;

            let cipher_blob = self.ecc_cipher.blob_bytes();
            let len = 4 + 4 + 4 + 64 + std::mem::size_of::<ECCPublicKeyBlob>() + cipher_blob.len();
            let mut vec: Vec<u8> = Vec::with_capacity(len);

            // version
            let bytes: [u8; 4] = (self.version as ULONG).to_ne_bytes();
            vec.extend_from_slice(&bytes);
            // sym_alg_id
            let bytes: [u8; 4] = (self.sym_alg_id as ULONG).to_ne_bytes();
            vec.extend_from_slice(&bytes);
            // bits
            let bytes: [u8; 4] = (self.bits as ULONG).to_ne_bytes();
            vec.extend_from_slice(&bytes);
            // encrypted_pri_key
            vec.extend_from_slice(&self.encrypted_pri_key);

            // pub_key.bit_len
            let bytes: [u8; 4] = (self.pub_key.bit_len as ULONG).to_ne_bytes();
            vec.extend_from_slice(&bytes);

            // pub_key.x_coordinate
            vec.extend_from_slice(&self.pub_key.x_coordinate);

            // pub_key.y_coordinate
            vec.extend_from_slice(&self.pub_key.y_coordinate);

            // cipher
            vec.extend_from_slice(&cipher_blob);
            vec
        }
    }
    #[cfg(test)]
    mod tests {
        use crate::ECCEncryptedData;

        #[test]
        fn cipher_blob_data_test() {
            use gm_hsm_sys::skf::types::ECCCipherBlob;
            let data = ECCEncryptedData {
                ec_x: [1u8; 64],
                ec_y: [2u8; 64],
                hash: [3u8; 32],
                cipher: vec![1u8, 2u8, 3u8, 4u8, 5u8],
            };
            let mem = data.blob_bytes();
            assert_eq!(mem.len(), 64 + 64 + 32 + 4 + 5);
            unsafe {
                let blob_ptr = mem.as_ptr() as *const ECCCipherBlob;
                let blob = &*blob_ptr;

                assert_eq!(blob.x_coordinate, [1u8; 64]);
                assert_eq!(blob.y_coordinate, [2u8; 64]);
                assert_eq!(blob.hash, [3u8; 32]);
                assert_eq!(std::ptr::addr_of!(blob.cipher_len).read_unaligned(), 5);
                assert_eq!(blob.cipher, [1u8]);
            }
        }
    }
}

pub mod param {
    use crate::error::InvalidArgumentError;
    use crate::Result;
    use std::ffi::CString;

    /// Convert `&str` to `CString`
    ///
    /// ## Errors
    /// This function will return an error if conversion from `&str` to `CString` fails,The error message use `param_name` to describe the parameter.
    pub fn as_cstring(
        param_name: impl AsRef<str>,
        param_value: impl AsRef<str>,
    ) -> Result<CString> {
        let value = CString::new(param_value.as_ref()).map_err(|e| {
            InvalidArgumentError::new(
                format!("parameter '{}' is invalid", param_name.as_ref()),
                Some(anyhow::Error::new(e)),
            )
        })?;
        Ok(value)
    }
}
