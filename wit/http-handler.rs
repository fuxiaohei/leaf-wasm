#[allow(clippy::all)]
pub mod http_handler {
    pub type Uri = String;
    pub type Status = u16;
    pub type RequestId = u64;
    pub type Method = String;
    pub type Headers = Vec<(String, String)>;
    pub type Body = Vec<u8>;
    #[derive(Clone)]
    pub struct Response {
        pub status: Status,
        pub headers: Headers,
        pub body: Option<Body>,
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
    #[derive(Clone)]
    pub struct Request {
        pub request_id: RequestId,
        pub method: Method,
        pub uri: Uri,
        pub headers: Headers,
        pub body: Option<Body>,
    }
    impl core::fmt::Debug for Request {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Request")
                .field("request-id", &self.request_id)
                .field("method", &self.method)
                .field("uri", &self.uri)
                .field("headers", &self.headers)
                .field("body", &self.body)
                .finish()
        }
    }
    pub trait HttpHandler {
        fn handle_request(req: Request) -> Response;
    }

    #[doc(hidden)]
    pub unsafe fn call_handle_request<T: HttpHandler>(
        arg0: i64,
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: i32,
        arg5: i32,
        arg6: i32,
        arg7: i32,
        arg8: i32,
        arg9: i32,
    ) -> i32 {
        let len0 = arg2 as usize;
        let len1 = arg4 as usize;
        let base4 = arg5;
        let len4 = arg6;
        let mut result4 = Vec::with_capacity(len4 as usize);
        for i in 0..len4 {
            let base = base4 + i * 16;
            result4.push({
                let len2 = *((base + 4) as *const i32) as usize;
                let len3 = *((base + 12) as *const i32) as usize;

                (
                    String::from_utf8(Vec::from_raw_parts(
                        *((base + 0) as *const i32) as *mut _,
                        len2,
                        len2,
                    ))
                    .unwrap(),
                    String::from_utf8(Vec::from_raw_parts(
                        *((base + 8) as *const i32) as *mut _,
                        len3,
                        len3,
                    ))
                    .unwrap(),
                )
            });
        }
        wit_bindgen_guest_rust::rt::dealloc(base4, (len4 as usize) * 16, 4);
        let result6 = T::handle_request(Request {
            request_id: arg0 as u64,
            method: String::from_utf8(Vec::from_raw_parts(arg1 as *mut _, len0, len0)).unwrap(),
            uri: String::from_utf8(Vec::from_raw_parts(arg3 as *mut _, len1, len1)).unwrap(),
            headers: result4,
            body: match arg7 {
                0 => None,
                1 => Some({
                    let len5 = arg9 as usize;

                    Vec::from_raw_parts(arg8 as *mut _, len5, len5)
                }),
                _ => panic!("invalid enum discriminant"),
            },
        });
        let ptr7 = _RET_AREA.0.as_mut_ptr() as i32;
        let Response {
            status: status8,
            headers: headers8,
            body: body8,
        } = result6;
        *((ptr7 + 0) as *mut u16) = (wit_bindgen_guest_rust::rt::as_i32(status8)) as u16;
        let vec12 = headers8;
        let len12 = vec12.len() as i32;
        let layout12 = alloc::Layout::from_size_align_unchecked(vec12.len() * 16, 4);
        let result12 = if layout12.size() != 0 {
            let ptr = alloc::alloc(layout12);
            if ptr.is_null() {
                alloc::handle_alloc_error(layout12);
            }
            ptr
        } else {
            core::ptr::null_mut()
        };
        for (i, e) in vec12.into_iter().enumerate() {
            let base = result12 as i32 + (i as i32) * 16;
            {
                let (t9_0, t9_1) = e;
                let vec10 = (t9_0.into_bytes()).into_boxed_slice();
                let ptr10 = vec10.as_ptr() as i32;
                let len10 = vec10.len() as i32;
                core::mem::forget(vec10);
                *((base + 4) as *mut i32) = len10;
                *((base + 0) as *mut i32) = ptr10;
                let vec11 = (t9_1.into_bytes()).into_boxed_slice();
                let ptr11 = vec11.as_ptr() as i32;
                let len11 = vec11.len() as i32;
                core::mem::forget(vec11);
                *((base + 12) as *mut i32) = len11;
                *((base + 8) as *mut i32) = ptr11;
            }
        }
        *((ptr7 + 8) as *mut i32) = len12;
        *((ptr7 + 4) as *mut i32) = result12 as i32;
        match body8 {
            Some(e) => {
                *((ptr7 + 12) as *mut u8) = (1i32) as u8;
                let vec13 = (e).into_boxed_slice();
                let ptr13 = vec13.as_ptr() as i32;
                let len13 = vec13.len() as i32;
                core::mem::forget(vec13);
                *((ptr7 + 20) as *mut i32) = len13;
                *((ptr7 + 16) as *mut i32) = ptr13;
            }
            None => {
                *((ptr7 + 12) as *mut u8) = (0i32) as u8;
            }
        };
        ptr7
    }

    #[doc(hidden)]
    pub unsafe fn post_return_handle_request<T: HttpHandler>(arg0: i32) {
        let base0 = *((arg0 + 4) as *const i32);
        let len0 = *((arg0 + 8) as *const i32);
        for i in 0..len0 {
            let base = base0 + i * 16;
            {
                wit_bindgen_guest_rust::rt::dealloc(
                    *((base + 0) as *const i32),
                    (*((base + 4) as *const i32)) as usize,
                    1,
                );
                wit_bindgen_guest_rust::rt::dealloc(
                    *((base + 8) as *const i32),
                    (*((base + 12) as *const i32)) as usize,
                    1,
                );
            }
        }
        wit_bindgen_guest_rust::rt::dealloc(base0, (len0 as usize) * 16, 4);
        match i32::from(*((arg0 + 12) as *const u8)) {
            0 => (),
            _ => {
                let base1 = *((arg0 + 16) as *const i32);
                let len1 = *((arg0 + 20) as *const i32);
                wit_bindgen_guest_rust::rt::dealloc(base1, (len1 as usize) * 1, 1);
            }
        }
    }

    #[allow(unused_imports)]
    use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};

    #[repr(align(4))]
    struct _RetArea([u8; 24]);
    static mut _RET_AREA: _RetArea = _RetArea([0; 24]);
}

/// Declares the export of the component's world for the
/// given type.
#[macro_export]
macro_rules! export_http_handler(($t:ident) => {
    const _: () = {

      #[doc(hidden)]
      #[export_name = "http-handler#handle-request"]
      #[allow(non_snake_case)]
      unsafe extern "C" fn __export_http_handler_handle_request(arg0: i64,arg1: i32,arg2: i32,arg3: i32,arg4: i32,arg5: i32,arg6: i32,arg7: i32,arg8: i32,arg9: i32,) -> i32 {
        http_handler::call_handle_request::<$t>(arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,arg9,)
      }

      #[doc(hidden)]
      #[export_name = "cabi_post_http-handler#handle-request"]
      #[allow(non_snake_case)]
      unsafe extern "C" fn __post_return_http_handler_handle_request(arg0: i32,) {
        http_handler::post_return_handle_request::<$t>(arg0,)
      }

    };

    #[used]
    #[doc(hidden)]
    #[cfg(target_arch = "wasm32")]
    static __FORCE_SECTION_REF: fn() = __force_section_ref;
    #[doc(hidden)]
    #[cfg(target_arch = "wasm32")]
    fn __force_section_ref() {
      __link_section()
    }
  });

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:http-handler"]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 750] = [
    2, 0, 12, 104, 116, 116, 112, 45, 104, 97, 110, 100, 108, 101, 114, 12, 104, 116, 116, 112, 45,
    104, 97, 110, 100, 108, 101, 114, 12, 104, 116, 116, 112, 45, 104, 97, 110, 100, 108, 101, 114,
    0, 97, 115, 109, 11, 0, 1, 0, 7, 231, 4, 1, 65, 4, 1, 66, 20, 1, 115, 4, 3, 117, 114, 105, 0,
    3, 0, 0, 1, 123, 4, 6, 115, 116, 97, 116, 117, 115, 0, 3, 0, 2, 1, 119, 4, 10, 114, 101, 113,
    117, 101, 115, 116, 45, 105, 100, 0, 3, 0, 4, 1, 115, 4, 6, 109, 101, 116, 104, 111, 100, 0, 3,
    0, 6, 1, 111, 2, 115, 115, 1, 112, 8, 4, 7, 104, 101, 97, 100, 101, 114, 115, 0, 3, 0, 9, 1,
    112, 125, 4, 4, 98, 111, 100, 121, 0, 3, 0, 11, 1, 107, 12, 1, 114, 3, 6, 115, 116, 97, 116,
    117, 115, 3, 7, 104, 101, 97, 100, 101, 114, 115, 10, 4, 98, 111, 100, 121, 13, 4, 8, 114, 101,
    115, 112, 111, 110, 115, 101, 0, 3, 0, 14, 1, 114, 5, 10, 114, 101, 113, 117, 101, 115, 116,
    45, 105, 100, 5, 6, 109, 101, 116, 104, 111, 100, 7, 3, 117, 114, 105, 1, 7, 104, 101, 97, 100,
    101, 114, 115, 10, 4, 98, 111, 100, 121, 13, 4, 7, 114, 101, 113, 117, 101, 115, 116, 0, 3, 0,
    16, 1, 64, 1, 3, 114, 101, 113, 17, 0, 15, 4, 14, 104, 97, 110, 100, 108, 101, 45, 114, 101,
    113, 117, 101, 115, 116, 0, 1, 18, 4, 20, 104, 116, 116, 112, 45, 104, 97, 110, 100, 108, 101,
    114, 45, 101, 120, 112, 111, 114, 116, 115, 38, 112, 107, 103, 58, 47, 104, 116, 116, 112, 45,
    104, 97, 110, 100, 108, 101, 114, 47, 104, 116, 116, 112, 45, 104, 97, 110, 100, 108, 101, 114,
    45, 101, 120, 112, 111, 114, 116, 115, 5, 0, 1, 65, 2, 1, 66, 20, 1, 115, 4, 3, 117, 114, 105,
    0, 3, 0, 0, 1, 123, 4, 6, 115, 116, 97, 116, 117, 115, 0, 3, 0, 2, 1, 119, 4, 10, 114, 101,
    113, 117, 101, 115, 116, 45, 105, 100, 0, 3, 0, 4, 1, 115, 4, 6, 109, 101, 116, 104, 111, 100,
    0, 3, 0, 6, 1, 111, 2, 115, 115, 1, 112, 8, 4, 7, 104, 101, 97, 100, 101, 114, 115, 0, 3, 0, 9,
    1, 112, 125, 4, 4, 98, 111, 100, 121, 0, 3, 0, 11, 1, 107, 12, 1, 114, 3, 6, 115, 116, 97, 116,
    117, 115, 3, 7, 104, 101, 97, 100, 101, 114, 115, 10, 4, 98, 111, 100, 121, 13, 4, 8, 114, 101,
    115, 112, 111, 110, 115, 101, 0, 3, 0, 14, 1, 114, 5, 10, 114, 101, 113, 117, 101, 115, 116,
    45, 105, 100, 5, 6, 109, 101, 116, 104, 111, 100, 7, 3, 117, 114, 105, 1, 7, 104, 101, 97, 100,
    101, 114, 115, 10, 4, 98, 111, 100, 121, 13, 4, 7, 114, 101, 113, 117, 101, 115, 116, 0, 3, 0,
    16, 1, 64, 1, 3, 114, 101, 113, 17, 0, 15, 4, 14, 104, 97, 110, 100, 108, 101, 45, 114, 101,
    113, 117, 101, 115, 116, 0, 1, 18, 4, 12, 104, 116, 116, 112, 45, 104, 97, 110, 100, 108, 101,
    114, 38, 112, 107, 103, 58, 47, 104, 116, 116, 112, 45, 104, 97, 110, 100, 108, 101, 114, 47,
    104, 116, 116, 112, 45, 104, 97, 110, 100, 108, 101, 114, 45, 101, 120, 112, 111, 114, 116,
    115, 5, 0, 4, 12, 104, 116, 116, 112, 45, 104, 97, 110, 100, 108, 101, 114, 30, 112, 107, 103,
    58, 47, 104, 116, 116, 112, 45, 104, 97, 110, 100, 108, 101, 114, 47, 104, 116, 116, 112, 45,
    104, 97, 110, 100, 108, 101, 114, 4, 1, 0, 45, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115,
    1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 1, 13, 119, 105, 116, 45, 99,
    111, 109, 112, 111, 110, 101, 110, 116, 5, 48, 46, 53, 46, 49, 11, 34, 1, 12, 104, 116, 116,
    112, 45, 104, 97, 110, 100, 108, 101, 114, 17, 112, 107, 103, 58, 47, 104, 116, 116, 112, 45,
    104, 97, 110, 100, 108, 101, 114, 3, 0,
];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
