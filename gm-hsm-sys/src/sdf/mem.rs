use crate::sdf::types::ECCCipher;
use std::slice;

impl ECCCipher {
    /// Fixed size, including the 'C' field (one byte placeholder).
    pub const FIXED_SIZE_WITH_PLACEHOLDER: usize = std::mem::size_of::<Self>();

    /// Offset for the 'C' field.
    pub const C_FIELD_OFFSET: usize = std::mem::offset_of!(Self, C);

    /// The variable-length data follows the C[1] placeholder in memory.
    pub const DATA_START_OFFSET: usize = Self::C_FIELD_OFFSET + 1;

    /// Calculates the actual contiguous memory size for ECCCipher.
    ///
    /// This size is used for heap allocation (e.g., C's malloc or Rust's alloc).
    ///
    /// Total actual size = Fixed struct size (including C[1] placeholder) + Extra space needed for variable-length data (L - 1).
    pub fn total_memory_size(&self) -> usize {
        let actual_len = self.L as usize;

        // The C[1] placeholder already occupies 1 byte of space.
        // Total size = Struct's size + (Actual data length - Placeholder length).
        // Uses saturating_sub to prevent underflow when L=0.
        Self::FIXED_SIZE_WITH_PLACEHOLDER + actual_len.saturating_sub(1)
    }

    /// Returns a raw pointer to the variable-length data (C), pointing to the memory *after* the C[1] placeholder.
    ///
    /// # Safety
    ///
    /// The safety and validity of the memory pointed to by this pointer depends on the caller
    /// having allocated sufficient contiguous memory before the FFI call.
    pub fn cipher_data_ptr(&self) -> *const u8 {
        // Placeholder start pointer + 1 byte.
        unsafe { self.C.as_ptr().add(1) }
    }

    /// Returns a read-only reference (&[u8]) to the variable-length data (C).
    ///
    /// # Safety
    ///
    /// Requires the struct pointer to point to a contiguous and sufficiently large memory block,
    /// and that the value in the 'L' field does not exceed the allocated boundary.
    pub unsafe fn cipher_as_slice(&self) -> &[u8] {
        let ptr = self.cipher_data_ptr();
        let len = self.L as usize;

        // Creates the slice from the raw pointer. This is an unsafe operation.
        slice::from_raw_parts(ptr, len)
    }

    /// Returns a copy (Vec<u8>) of the variable-length data (C).
    ///
    /// # Safety
    ///
    /// Requires the struct pointer to point to a contiguous and sufficiently large memory block.
    pub unsafe fn cipher_to_vec(&self) -> Vec<u8> {
        let len = self.L as usize;
        let ptr = self.cipher_data_ptr();

        if len == 0 {
            return Vec::new();
        }

        let mut vec = Vec::with_capacity(len);

        // Copies 'len' bytes from the pointer into the Vec.
        std::ptr::copy_nonoverlapping(ptr, vec.as_mut_ptr(), len);

        vec.set_len(len);
        vec
    }
}

pub fn get_message(code: u32) -> Option<&'static str> {
    match code {
        crate::sdf::types::SDR_OK => Some("操作成功"),
        crate::sdf::types::SDR_UNKNOWERR => Some("未知错误"),
        crate::sdf::types::SDR_NOTSUPPORT => Some("不支持的接口调用"),
        crate::sdf::types::SDR_COMMFAIL => Some("与设备通信失败"),
        crate::sdf::types::SDR_HARDFAIL => Some("运算模块无响应"),
        crate::sdf::types::SDR_OPENDEVICE => Some("打开设备失败"),
        crate::sdf::types::SDR_OPENSESSION => Some("创建会话失败"),
        crate::sdf::types::SDR_PARDENY => Some("无私钥使用权限"),
        crate::sdf::types::SDR_KEYNOTEXIST => Some("不存在的密钥调用"),
        crate::sdf::types::SDR_ALGNOTSUPPORT => Some("不支持的算法调用"),
        crate::sdf::types::SDR_ALGMODNOTSUPPORT => Some("不支持的算法模式调用"),
        crate::sdf::types::SDR_PKOPERR => Some("公钥运算失败"),
        crate::sdf::types::SDR_SKOPERR => Some("私钥运算失败"),
        crate::sdf::types::SDR_SIGNERR => Some("签名运算失败"),
        crate::sdf::types::SDR_VERIFYERR => Some("验证签名失败"),
        crate::sdf::types::SDR_SYMOPERR => Some("对称算法运算失败"),
        crate::sdf::types::SDR_STEPERR => Some("多步运算步骤错误"),
        crate::sdf::types::SDR_FILESIZEERR => Some("文件长度超出限制"),
        crate::sdf::types::SDR_FILENOEXIST => Some("指定的文件不存在"),
        crate::sdf::types::SDR_FILEOFSERR => Some("文件起始位置错误"),
        crate::sdf::types::SDR_KEYTYPEERR => Some("密钥类型错误"),
        crate::sdf::types::SDR_KEYERR => Some("密钥错误"),
        crate::sdf::types::SDR_ENCDATAERR => Some("ECC 加密数据错误"),
        crate::sdf::types::SDR_RANDERR => Some("随机数产生失败"),
        crate::sdf::types::SDR_PRKRERR => Some("私钥使用权限获取失败"),
        crate::sdf::types::SDR_MACERR => Some("MAC运算失败"),
        crate::sdf::types::SDR_FILEEXSITS => Some("指定文件已存在"),
        crate::sdf::types::SDR_FILEWERR => Some("文件写入失败"),
        crate::sdf::types::SDR_NOBUFFER => Some("存储空间不足"),
        crate::sdf::types::SDR_INARGERR => Some("输入参数错误"),
        crate::sdf::types::SDR_OUTARGERR => Some("输出参数错误"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::sdf::types::ECCCipher;
    use std::alloc::{alloc, dealloc, Layout};
    use std::ptr;

    /// Helper function to allocate and initialize a continuous memory block
    /// simulating FFI data transfer.
    unsafe fn setup_test_cipher(cipher_len: usize, data_value: u8) -> (*mut ECCCipher, Layout) {
        let actual_len = cipher_len;
        // 165 bytes
        let fixed_size = ECCCipher::FIXED_SIZE_WITH_PLACEHOLDER;

        // Total allocation size = 165 + (L - 1)
        let total_alloc_size = fixed_size + actual_len.saturating_sub(1);

        let layout = Layout::from_size_align(total_alloc_size, 1).unwrap();
        let ptr = alloc(layout) as *mut ECCCipher;

        if ptr.is_null() {
            panic!("Allocation failed");
        }

        // Initialize struct fields (fixed part)
        ptr::write_bytes(ptr as *mut u8, 0x00, fixed_size);

        // Set 'L' field
        let l_ptr = ptr::addr_of_mut!((*ptr).L);
        ptr::write_unaligned(l_ptr, actual_len as ::std::os::raw::c_uint);

        // Initialize the C[1] placeholder (optional, usually ignored)
        (*ptr).C[0] = 0xFF;

        // Initialize variable length data (actual data)
        if actual_len > 0 {
            let actual_data_ptr = (*ptr).C.as_mut_ptr().add(1);
            ptr::write_bytes(actual_data_ptr, data_value, actual_len);
        }

        (ptr, layout)
    }

    #[test]
    fn test_total_memory_size() {
        unsafe {
            // Case 1: L = 10 (needs 9 extra bytes)
            let len1 = 10;
            let (ptr1, layout1) = setup_test_cipher(len1, 0xAA);
            // Expected: 165 (fixed) + 10 - 1 = 174
            assert_eq!(
                (*ptr1).total_memory_size(),
                174,
                "Test Case 1 failed for total_memory_size"
            );
            dealloc(ptr1 as *mut u8, layout1);

            // Case 2: L = 1 (Placeholder is just enough)
            let len2 = 1;
            let (ptr2, layout2) = setup_test_cipher(len2, 0xAA);
            // Expected: 165 + 1 - 1 = 165
            assert_eq!(
                (*ptr2).total_memory_size(),
                165,
                "Test Case 2 failed for total_memory_size"
            );
            dealloc(ptr2 as *mut u8, layout2);

            // Case 3: L = 0 (Should only be fixed size)
            let len3 = 0;
            let (ptr3, layout3) = setup_test_cipher(len3, 0xAA);
            // Expected: 165 + 0 - 1 (saturating_sub to 0) = 165
            assert_eq!(
                (*ptr3).total_memory_size(),
                165,
                "Test Case 3 failed for total_memory_size"
            );
            dealloc(ptr3 as *mut u8, layout3);
        }
    }

    #[test]
    fn test_cipher_data_ptr_and_slice() {
        unsafe {
            const DATA_LEN: usize = 5;
            const DATA_VAL: u8 = 0xDE;
            let (ptr, layout) = setup_test_cipher(DATA_LEN, DATA_VAL);
            let cipher_struct = &*ptr;

            // Test 1: cipher_data_ptr should point to C[1] + 1
            let expected_ptr_addr = cipher_struct.C.as_ptr().add(1) as usize;
            let actual_ptr_addr = cipher_struct.cipher_data_ptr() as usize;
            assert_eq!(
                actual_ptr_addr, expected_ptr_addr,
                "Pointer address mismatch"
            );

            // Test 2: cipher_as_slice length
            let slice = cipher_struct.cipher_as_slice();
            assert_eq!(slice.len(), DATA_LEN, "Slice length mismatch");

            // Test 3: cipher_as_slice content
            // The slice should contain DATA_VAL for all its length
            assert!(
                slice.iter().all(|&b| b == DATA_VAL),
                "Slice content mismatch"
            );

            dealloc(ptr as *mut u8, layout);
        }
    }

    #[test]
    fn test_cipher_to_vec() {
        unsafe {
            const DATA_LEN: usize = 8;
            const DATA_VAL: u8 = 0xAF;
            let (ptr, layout) = setup_test_cipher(DATA_LEN, DATA_VAL);
            let cipher_struct = &*ptr;

            // Test 1: to_vec length
            let vec = cipher_struct.cipher_to_vec();
            assert_eq!(vec.len(), DATA_LEN, "Vec length mismatch");

            // Test 2: to_vec content
            assert!(vec.iter().all(|&b| b == DATA_VAL), "Vec content mismatch");

            // Test 3: L=0 case
            let (ptr_zero, layout_zero) = setup_test_cipher(0, 0x00);
            let vec_zero = (*ptr_zero).cipher_to_vec();
            assert!(vec_zero.is_empty(), "Vec should be empty when L=0");
            dealloc(ptr_zero as *mut u8, layout_zero);

            dealloc(ptr as *mut u8, layout);
        }
    }
}
