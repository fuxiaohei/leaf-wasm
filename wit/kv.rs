#[allow(clippy::all)]
pub mod kv_imports {
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum OpResult {
        Success,
        Error,
    }
    impl OpResult {
        pub fn name(&self) -> &'static str {
            match self {
                OpResult::Success => "success",
                OpResult::Error => "error",
            }
        }
        pub fn message(&self) -> &'static str {
            match self {
                OpResult::Success => "",
                OpResult::Error => "",
            }
        }
    }
    impl core::fmt::Debug for OpResult {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("OpResult")
                .field("code", &(*self as i32))
                .field("name", &self.name())
                .field("message", &self.message())
                .finish()
        }
    }
    impl core::fmt::Display for OpResult {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} (error {})", self.name(), *self as i32)
        }
    }

    impl std::error::Error for OpResult {}
    #[allow(clippy::all)]
    pub fn set(key: &str, value: &str, ttl: u32) -> OpResult {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            let vec0 = key;
            let ptr0 = vec0.as_ptr() as i32;
            let len0 = vec0.len() as i32;
            let vec1 = value;
            let ptr1 = vec1.as_ptr() as i32;
            let len1 = vec1.len() as i32;

            #[link(wasm_import_module = "kv-imports")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "set")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "kv-imports_set")]
                fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32) -> i32;
            }
            let ret = wit_import(
                ptr0,
                len0,
                ptr1,
                len1,
                wit_bindgen_guest_rust::rt::as_i32(ttl),
            );
            match ret {
                0 => OpResult::Success,
                1 => OpResult::Error,
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    pub fn get(key: &str) -> Result<String, OpResult> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(4))]
            struct RetArea([u8; 12]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let vec0 = key;
            let ptr0 = vec0.as_ptr() as i32;
            let len0 = vec0.len() as i32;
            let ptr1 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "kv-imports")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "get")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "kv-imports_get")]
                fn wit_import(_: i32, _: i32, _: i32);
            }
            wit_import(ptr0, len0, ptr1);
            match i32::from(*((ptr1 + 0) as *const u8)) {
                0 => Ok({
                    let len2 = *((ptr1 + 8) as *const i32) as usize;

                    String::from_utf8(Vec::from_raw_parts(
                        *((ptr1 + 4) as *const i32) as *mut _,
                        len2,
                        len2,
                    ))
                    .unwrap()
                }),
                1 => Err(match i32::from(*((ptr1 + 4) as *const u8)) {
                    0 => OpResult::Success,
                    1 => OpResult::Error,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
    #[allow(clippy::all)]
    pub fn list_values() -> Result<Vec<String>, OpResult> {
        #[allow(unused_imports)]
        use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};
        unsafe {
            #[repr(align(4))]
            struct RetArea([u8; 12]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[link(wasm_import_module = "kv-imports")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "list-values")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "kv-imports_list-values")]
                fn wit_import(_: i32);
            }
            wit_import(ptr0);
            match i32::from(*((ptr0 + 0) as *const u8)) {
                0 => Ok({
                    let base2 = *((ptr0 + 4) as *const i32);
                    let len2 = *((ptr0 + 8) as *const i32);
                    let mut result2 = Vec::with_capacity(len2 as usize);
                    for i in 0..len2 {
                        let base = base2 + i * 8;
                        result2.push({
                            let len1 = *((base + 4) as *const i32) as usize;

                            String::from_utf8(Vec::from_raw_parts(
                                *((base + 0) as *const i32) as *mut _,
                                len1,
                                len1,
                            ))
                            .unwrap()
                        });
                    }
                    wit_bindgen_guest_rust::rt::dealloc(base2, (len2 as usize) * 8, 4);

                    result2
                }),
                1 => Err(match i32::from(*((ptr0 + 4) as *const u8)) {
                    0 => OpResult::Success,
                    1 => OpResult::Error,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:kv"]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 412] = [
    2, 0, 2, 107, 118, 2, 107, 118, 2, 107, 118, 0, 97, 115, 109, 11, 0, 1, 0, 7, 199, 2, 1, 65, 4,
    1, 66, 11, 1, 109, 2, 7, 115, 117, 99, 99, 101, 115, 115, 5, 101, 114, 114, 111, 114, 4, 9,
    111, 112, 45, 114, 101, 115, 117, 108, 116, 0, 3, 0, 0, 1, 64, 3, 3, 107, 101, 121, 115, 5,
    118, 97, 108, 117, 101, 115, 3, 116, 116, 108, 121, 0, 1, 4, 3, 115, 101, 116, 0, 1, 2, 1, 106,
    1, 115, 1, 1, 1, 64, 1, 3, 107, 101, 121, 115, 0, 3, 4, 3, 103, 101, 116, 0, 1, 4, 1, 112, 115,
    1, 106, 1, 5, 1, 1, 1, 64, 0, 0, 6, 4, 11, 108, 105, 115, 116, 45, 118, 97, 108, 117, 101, 115,
    0, 1, 7, 4, 10, 107, 118, 45, 105, 109, 112, 111, 114, 116, 115, 18, 112, 107, 103, 58, 47,
    107, 118, 47, 107, 118, 45, 105, 109, 112, 111, 114, 116, 115, 5, 0, 1, 65, 2, 1, 66, 11, 1,
    109, 2, 7, 115, 117, 99, 99, 101, 115, 115, 5, 101, 114, 114, 111, 114, 4, 9, 111, 112, 45,
    114, 101, 115, 117, 108, 116, 0, 3, 0, 0, 1, 64, 3, 3, 107, 101, 121, 115, 5, 118, 97, 108,
    117, 101, 115, 3, 116, 116, 108, 121, 0, 1, 4, 3, 115, 101, 116, 0, 1, 2, 1, 106, 1, 115, 1, 1,
    1, 64, 1, 3, 107, 101, 121, 115, 0, 3, 4, 3, 103, 101, 116, 0, 1, 4, 1, 112, 115, 1, 106, 1, 5,
    1, 1, 1, 64, 0, 0, 6, 4, 11, 108, 105, 115, 116, 45, 118, 97, 108, 117, 101, 115, 0, 1, 7, 3,
    10, 107, 118, 45, 105, 109, 112, 111, 114, 116, 115, 18, 112, 107, 103, 58, 47, 107, 118, 47,
    107, 118, 45, 105, 109, 112, 111, 114, 116, 115, 5, 0, 4, 2, 107, 118, 10, 112, 107, 103, 58,
    47, 107, 118, 47, 107, 118, 4, 1, 0, 45, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12,
    112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 1, 13, 119, 105, 116, 45, 99, 111,
    109, 112, 111, 110, 101, 110, 116, 5, 48, 46, 53, 46, 49, 11, 14, 1, 2, 107, 118, 7, 112, 107,
    103, 58, 47, 107, 118, 3, 0,
];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
