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
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum HttpError {
        NetworkError,
        Timeout,
        InvalidUrl,
        DestinationNotAllowed,
        TooManyRequests,
        InvalidRequest,
    }
    impl HttpError {
        pub fn name(&self) -> &'static str {
            match self {
                HttpError::NetworkError => "network-error",
                HttpError::Timeout => "timeout",
                HttpError::InvalidUrl => "invalid-url",
                HttpError::DestinationNotAllowed => "destination-not-allowed",
                HttpError::TooManyRequests => "too-many-requests",
                HttpError::InvalidRequest => "invalid-request",
            }
        }
        pub fn message(&self) -> &'static str {
            match self {
                HttpError::NetworkError => "",
                HttpError::Timeout => "",
                HttpError::InvalidUrl => "",
                HttpError::DestinationNotAllowed => "",
                HttpError::TooManyRequests => "",
                HttpError::InvalidRequest => "",
            }
        }
    }
    impl core::fmt::Debug for HttpError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("HttpError")
                .field("code", &(*self as i32))
                .field("name", &self.name())
                .field("message", &self.message())
                .finish()
        }
    }
    impl core::fmt::Display for HttpError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} (error {})", self.name(), *self as i32)
        }
    }

    impl std::error::Error for HttpError {}
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum RedirectPolicy {
        Follow,
        Error,
        Manual,
    }
    impl core::fmt::Debug for RedirectPolicy {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                RedirectPolicy::Follow => f.debug_tuple("RedirectPolicy::Follow").finish(),
                RedirectPolicy::Error => f.debug_tuple("RedirectPolicy::Error").finish(),
                RedirectPolicy::Manual => f.debug_tuple("RedirectPolicy::Manual").finish(),
            }
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct FetchOptions {
        pub timeout: u32,
        pub decompress: bool,
        pub redirect: RedirectPolicy,
    }
    impl core::fmt::Debug for FetchOptions {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("FetchOptions")
                .field("timeout", &self.timeout)
                .field("decompress", &self.decompress)
                .field("redirect", &self.redirect)
                .finish()
        }
    }
    pub fn fetch(req: Request<'_>, options: FetchOptions) -> Result<Response, HttpError> {
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
            let FetchOptions {
                timeout: timeout9,
                decompress: decompress9,
                redirect: redirect9,
            } = options;

            #[repr(align(4))]
            struct RetArea([u8; 28]);
            let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr10 = ret_area.as_mut_ptr() as i32;

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
                wit_bindgen_guest_rust::rt::as_i32(timeout9),
                match decompress9 {
                    true => 1,
                    false => 0,
                },
                match redirect9 {
                    RedirectPolicy::Follow => 0,
                    RedirectPolicy::Error => 1,
                    RedirectPolicy::Manual => 2,
                },
                ptr10,
            );
            if layout6.size() != 0 {
                alloc::dealloc(result6, layout6);
            }
            match i32::from(*((ptr10 + 0) as *const u8)) {
                0 => Ok({
                    let base13 = *((ptr10 + 8) as *const i32);
                    let len13 = *((ptr10 + 12) as *const i32);
                    let mut result13 = Vec::with_capacity(len13 as usize);
                    for i in 0..len13 {
                        let base = base13 + i * 16;
                        result13.push({
                            let len11 = *((base + 4) as *const i32) as usize;
                            let len12 = *((base + 12) as *const i32) as usize;

                            (
                                String::from_utf8(Vec::from_raw_parts(
                                    *((base + 0) as *const i32) as *mut _,
                                    len11,
                                    len11,
                                ))
                                .unwrap(),
                                String::from_utf8(Vec::from_raw_parts(
                                    *((base + 8) as *const i32) as *mut _,
                                    len12,
                                    len12,
                                ))
                                .unwrap(),
                            )
                        });
                    }
                    wit_bindgen_guest_rust::rt::dealloc(base13, (len13 as usize) * 16, 4);

                    Response {
                        status: i32::from(*((ptr10 + 4) as *const u16)) as u16,
                        headers: result13,
                        body: match i32::from(*((ptr10 + 16) as *const u8)) {
                            0 => None,
                            1 => Some({
                                let len14 = *((ptr10 + 24) as *const i32) as usize;

                                Vec::from_raw_parts(
                                    *((ptr10 + 20) as *const i32) as *mut _,
                                    len14,
                                    len14,
                                )
                            }),
                            _ => panic!("invalid enum discriminant"),
                        },
                    }
                }),
                1 => Err(match i32::from(*((ptr10 + 4) as *const u8)) {
                    0 => HttpError::NetworkError,
                    1 => HttpError::Timeout,
                    2 => HttpError::InvalidUrl,
                    3 => HttpError::DestinationNotAllowed,
                    4 => HttpError::TooManyRequests,
                    5 => HttpError::InvalidRequest,
                    _ => panic!("invalid enum discriminant"),
                }),
                _ => panic!("invalid enum discriminant"),
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:http-fetch"]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 488] = [
    1, 0, 10, 104, 116, 116, 112, 45, 102, 101, 116, 99, 104, 0, 97, 115, 109, 10, 0, 1, 0, 7, 191,
    3, 15, 123, 111, 2, 115, 115, 112, 1, 115, 112, 125, 115, 107, 4, 114, 4, 6, 109, 101, 116,
    104, 111, 100, 5, 3, 117, 114, 105, 3, 7, 104, 101, 97, 100, 101, 114, 115, 2, 4, 98, 111, 100,
    121, 6, 114, 3, 6, 115, 116, 97, 116, 117, 115, 0, 7, 104, 101, 97, 100, 101, 114, 115, 2, 4,
    98, 111, 100, 121, 6, 109, 6, 13, 110, 101, 116, 119, 111, 114, 107, 45, 101, 114, 114, 111,
    114, 7, 116, 105, 109, 101, 111, 117, 116, 11, 105, 110, 118, 97, 108, 105, 100, 45, 117, 114,
    108, 23, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 45, 110, 111, 116, 45, 97, 108,
    108, 111, 119, 101, 100, 17, 116, 111, 111, 45, 109, 97, 110, 121, 45, 114, 101, 113, 117, 101,
    115, 116, 115, 15, 105, 110, 118, 97, 108, 105, 100, 45, 114, 101, 113, 117, 101, 115, 116,
    109, 3, 6, 102, 111, 108, 108, 111, 119, 5, 101, 114, 114, 111, 114, 6, 109, 97, 110, 117, 97,
    108, 114, 3, 7, 116, 105, 109, 101, 111, 117, 116, 121, 10, 100, 101, 99, 111, 109, 112, 114,
    101, 115, 115, 127, 8, 114, 101, 100, 105, 114, 101, 99, 116, 10, 106, 1, 8, 1, 9, 64, 2, 3,
    114, 101, 113, 7, 7, 111, 112, 116, 105, 111, 110, 115, 11, 0, 12, 66, 22, 2, 3, 2, 1, 0, 4, 6,
    115, 116, 97, 116, 117, 115, 0, 3, 0, 0, 2, 3, 2, 1, 2, 4, 7, 104, 101, 97, 100, 101, 114, 115,
    0, 3, 0, 1, 2, 3, 2, 1, 3, 4, 3, 117, 114, 105, 0, 3, 0, 2, 2, 3, 2, 1, 4, 4, 4, 98, 111, 100,
    121, 0, 3, 0, 3, 2, 3, 2, 1, 5, 4, 6, 109, 101, 116, 104, 111, 100, 0, 3, 0, 4, 2, 3, 2, 1, 7,
    4, 7, 114, 101, 113, 117, 101, 115, 116, 0, 3, 0, 5, 2, 3, 2, 1, 8, 4, 8, 114, 101, 115, 112,
    111, 110, 115, 101, 0, 3, 0, 6, 2, 3, 2, 1, 9, 4, 10, 104, 116, 116, 112, 45, 101, 114, 114,
    111, 114, 0, 3, 0, 7, 2, 3, 2, 1, 10, 4, 15, 114, 101, 100, 105, 114, 101, 99, 116, 45, 112,
    111, 108, 105, 99, 121, 0, 3, 0, 8, 2, 3, 2, 1, 11, 4, 13, 102, 101, 116, 99, 104, 45, 111,
    112, 116, 105, 111, 110, 115, 0, 3, 0, 9, 2, 3, 2, 1, 13, 4, 5, 102, 101, 116, 99, 104, 0, 1,
    10, 10, 15, 1, 10, 104, 116, 116, 112, 45, 102, 101, 116, 99, 104, 0, 5, 14,
];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
