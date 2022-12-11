#[allow(clippy::all)]
pub mod http_fetch {
    #[allow(unused_imports)]
    use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};

    pub type Status = u16;
    pub type HeadersParam<'a> = &'a [(&'a str, &'a str)];
    pub type HeadersResult = Vec<(String, String)>;
    pub type Uri<'a> = &'a str;
    pub type BodyParam<'a> = &'a [u8];
    pub type BodyResult = Vec<u8>;
    pub type Method<'a> = &'a str;
    #[derive(Clone)]
    pub struct Request<'a> {
        pub method: Method<'a>,
        pub uri: Uri<'a>,
        pub headers: HeadersParam<'a>,
        pub body: Option<BodyParam<'a>>,
    }
    impl<'a> core::fmt::Debug for Request<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Request")
                .field("method", &self.method)
                .field("uri", &self.uri)
                .field("headers", &self.headers)
                .field("body", &self.body)
                .finish()
        }
    }
    #[derive(Clone)]
    pub struct Response {
        pub status: Status,
        pub headers: HeadersResult,
        pub body: Option<BodyResult>,
    }
    impl core::fmt::Debug for Response {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Response")
                .field("status", &self.status)
                .field("headers", &self.headers)
                .field("body", &self.body)
                .finish()
        }
    }
    pub fn fetch(req: Request<'_>) -> Response {
        unsafe {
            let Request {
                method: method0,
                uri: uri0,
                headers: headers0,
                body: body0,
            } = req;
            let vec1 = method0;
            let ptr1 = vec1.as_ptr() as i32;
            let len1 = vec1.len() as i32;
            let vec2 = uri0;
            let ptr2 = vec2.as_ptr() as i32;
            let len2 = vec2.len() as i32;
            let vec6 = headers0;
            let len6 = vec6.len() as i32;
            let layout6 = alloc::Layout::from_size_align_unchecked(vec6.len() * 16, 4);
            let result6 = if layout6.size() != 0 {
                let ptr = alloc::alloc(layout6);
                if ptr.is_null() {
                    alloc::handle_alloc_error(layout6);
                }
                ptr
            } else {
                core::ptr::null_mut()
            };
            for (i, e) in vec6.into_iter().enumerate() {
                let base = result6 as i32 + (i as i32) * 16;
                {
                    let (t3_0, t3_1) = e;
                    let vec4 = t3_0;
                    let ptr4 = vec4.as_ptr() as i32;
                    let len4 = vec4.len() as i32;
                    *((base + 4) as *mut i32) = len4;
                    *((base + 0) as *mut i32) = ptr4;
                    let vec5 = t3_1;
                    let ptr5 = vec5.as_ptr() as i32;
                    let len5 = vec5.len() as i32;
                    *((base + 12) as *mut i32) = len5;
                    *((base + 8) as *mut i32) = ptr5;
                }
            }
            let (result8_0, result8_1, result8_2) = match body0 {
                Some(e) => {
                    let vec7 = e;
                    let ptr7 = vec7.as_ptr() as i32;
                    let len7 = vec7.len() as i32;

                    (1i32, ptr7, len7)
                }
                None => (0i32, 0i32, 0i32),
            };
            #[repr(align(4))]
            struct RetArea([u8; 24]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr9 = ret_area.as_mut_ptr() as i32;

            #[link(wasm_import_module = "http-fetch")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "fetch")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "http-fetch_fetch")]
                fn wit_import(
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                );
            }
            wit_import(
                ptr1,
                len1,
                ptr2,
                len2,
                result6 as i32,
                len6,
                result8_0,
                result8_1,
                result8_2,
                ptr9,
            );
            let base12 = *((ptr9 + 4) as *const i32);
            let len12 = *((ptr9 + 8) as *const i32);
            let mut result12 = Vec::with_capacity(len12 as usize);
            for i in 0..len12 {
                let base = base12 + i * 16;
                result12.push({
                    let len10 = *((base + 4) as *const i32) as usize;
                    let len11 = *((base + 12) as *const i32) as usize;

                    (
                        String::from_utf8(Vec::from_raw_parts(
                            *((base + 0) as *const i32) as *mut _,
                            len10,
                            len10,
                        ))
                        .unwrap(),
                        String::from_utf8(Vec::from_raw_parts(
                            *((base + 8) as *const i32) as *mut _,
                            len11,
                            len11,
                        ))
                        .unwrap(),
                    )
                });
            }
            wit_bindgen_guest_rust::rt::dealloc(base12, (len12 as usize) * 16, 4);
            if layout6.size() != 0 {
                alloc::dealloc(result6, layout6);
            }
            Response {
                status: i32::from(*((ptr9 + 0) as *const u16)) as u16,
                headers: result12,
                body: match i32::from(*((ptr9 + 12) as *const u8)) {
                    0 => None,
                    1 => Some({
                        let len13 = *((ptr9 + 20) as *const i32) as usize;

                        Vec::from_raw_parts(*((ptr9 + 16) as *const i32) as *mut _, len13, len13)
                    }),
                    _ => panic!("invalid enum discriminant"),
                },
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:leaf-http-fetch"]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 259] = [
    1, 0, 15, 108, 101, 97, 102, 45, 104, 116, 116, 112, 45, 102, 101, 116, 99, 104, 0, 97, 115,
    109, 10, 0, 1, 0, 7, 213, 1, 11, 123, 111, 2, 115, 115, 112, 1, 115, 112, 125, 115, 107, 4,
    114, 4, 6, 109, 101, 116, 104, 111, 100, 5, 3, 117, 114, 105, 3, 7, 104, 101, 97, 100, 101,
    114, 115, 2, 4, 98, 111, 100, 121, 6, 114, 3, 6, 115, 116, 97, 116, 117, 115, 0, 7, 104, 101,
    97, 100, 101, 114, 115, 2, 4, 98, 111, 100, 121, 6, 64, 1, 3, 114, 101, 113, 7, 0, 8, 66, 16,
    2, 3, 2, 1, 0, 4, 6, 115, 116, 97, 116, 117, 115, 0, 3, 0, 0, 2, 3, 2, 1, 2, 4, 7, 104, 101,
    97, 100, 101, 114, 115, 0, 3, 0, 1, 2, 3, 2, 1, 3, 4, 3, 117, 114, 105, 0, 3, 0, 2, 2, 3, 2, 1,
    4, 4, 4, 98, 111, 100, 121, 0, 3, 0, 3, 2, 3, 2, 1, 5, 4, 6, 109, 101, 116, 104, 111, 100, 0,
    3, 0, 4, 2, 3, 2, 1, 7, 4, 7, 114, 101, 113, 117, 101, 115, 116, 0, 3, 0, 5, 2, 3, 2, 1, 8, 4,
    8, 114, 101, 115, 112, 111, 110, 115, 101, 0, 3, 0, 6, 2, 3, 2, 1, 9, 4, 5, 102, 101, 116, 99,
    104, 0, 1, 7, 10, 15, 1, 10, 104, 116, 116, 112, 45, 102, 101, 116, 99, 104, 0, 5, 10,
];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
