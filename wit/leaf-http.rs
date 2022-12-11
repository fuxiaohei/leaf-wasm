#[allow(clippy::all)]
pub mod leaf_http {
    #[allow(unused_imports)]
    use wit_bindgen_guest_rust::rt::{alloc, string::String, vec::Vec};

    pub type Status = u16;
    pub type Headers = Vec<(String, String)>;
    pub type Uri = String;
    pub type Body = Vec<u8>;
    pub type Method = String;
    #[derive(Clone)]
    pub struct Request {
        pub method: Method,
        pub uri: Uri,
        pub headers: Headers,
        pub body: Option<Body>,
    }
    impl core::fmt::Debug for Request {
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
    pub trait LeafHttp {
        fn handle_request(req: Request) -> Response;
    }

    #[doc(hidden)]
    pub unsafe fn call_handle_request<T: LeafHttp>(
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: i32,
        arg5: i32,
        arg6: i32,
        arg7: i32,
        arg8: i32,
    ) -> i32 {
        let len0 = arg1 as usize;
        let len1 = arg3 as usize;
        let base4 = arg4;
        let len4 = arg5;
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
            method: String::from_utf8(Vec::from_raw_parts(arg0 as *mut _, len0, len0)).unwrap(),
            uri: String::from_utf8(Vec::from_raw_parts(arg2 as *mut _, len1, len1)).unwrap(),
            headers: result4,
            body: match arg6 {
                0 => None,
                1 => Some({
                    let len5 = arg8 as usize;

                    Vec::from_raw_parts(arg7 as *mut _, len5, len5)
                }),
                _ => panic!("invalid enum discriminant"),
            },
        });
        let ptr7 = RET_AREA.0.as_mut_ptr() as i32;
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
    pub unsafe fn post_return_handle_request<T: LeafHttp>(arg0: i32) {
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

    #[repr(align(4))]
    struct LeafHttpRetArea([u8; 24]);
    static mut RET_AREA: LeafHttpRetArea = LeafHttpRetArea([0; 24]);
}

/// Declares the export of the component's world for the
/// given type.
#[macro_export]
macro_rules! export_leaf_http(($t:ident) => {
    const _: () = {

      #[doc(hidden)]
      #[export_name = "handle-request"]
      #[allow(non_snake_case)]
      unsafe extern "C" fn __export_exports_handle_request(arg0: i32,arg1: i32,arg2: i32,arg3: i32,arg4: i32,arg5: i32,arg6: i32,arg7: i32,arg8: i32,) -> i32 {
        leaf_http::call_handle_request::<$t>(arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,)
      }

      #[doc(hidden)]
      #[export_name = "cabi_post_handle-request"]
      #[allow(non_snake_case)]
      unsafe extern "C" fn __post_return_exports_handle_request(arg0: i32,) {
        leaf_http::post_return_handle_request::<$t>(arg0,)
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
#[link_section = "component-type:leaf-http"]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 190] = [
    1, 0, 9, 108, 101, 97, 102, 45, 104, 116, 116, 112, 0, 97, 115, 109, 10, 0, 1, 0, 7, 78, 10,
    115, 115, 111, 2, 115, 115, 112, 2, 112, 125, 107, 4, 114, 4, 6, 109, 101, 116, 104, 111, 100,
    0, 3, 117, 114, 105, 1, 7, 104, 101, 97, 100, 101, 114, 115, 3, 4, 98, 111, 100, 121, 5, 123,
    114, 3, 6, 115, 116, 97, 116, 117, 115, 7, 7, 104, 101, 97, 100, 101, 114, 115, 3, 4, 98, 111,
    100, 121, 5, 64, 1, 3, 114, 101, 113, 6, 0, 8, 11, 88, 8, 6, 115, 116, 97, 116, 117, 115, 0, 3,
    7, 7, 104, 101, 97, 100, 101, 114, 115, 0, 3, 3, 3, 117, 114, 105, 0, 3, 1, 4, 98, 111, 100,
    121, 0, 3, 4, 6, 109, 101, 116, 104, 111, 100, 0, 3, 0, 7, 114, 101, 113, 117, 101, 115, 116,
    0, 3, 6, 8, 114, 101, 115, 112, 111, 110, 115, 101, 0, 3, 8, 14, 104, 97, 110, 100, 108, 101,
    45, 114, 101, 113, 117, 101, 115, 116, 0, 3, 9,
];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
