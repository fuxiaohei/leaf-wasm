use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn http_main(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    // let func = syn::parse_macro_input!(item as syn::ItemFn);
    // let fun_name = func.sig.ident.clone();

    quote!(

    wit_bindgen_guest_rust::generate!("../wit/leaf-http.wit");
    struct HttpImpl;

    impl leaf_http::LeafHttp for HttpImpl {
        fn handle_request(req: leaf_http::Request) -> leaf_http::Response {
            let url = req.uri;
            let method = req.method.to_uppercase();

            let mut headers = req.headers;
            headers.push(("X-Request-Method".to_string(), method));
            headers.push(("X-Request-Url".to_string(), url));

            leaf_http::Response {
                status: 200,
                headers,
                body: req.body,
            }
        }
    }

    export_leaf_http!(HttpImpl);

    )
    .into()
}
