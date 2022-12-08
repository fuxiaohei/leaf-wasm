wasmtime::component::bindgen!("./wit/leaf-http.wit");

#[test]
fn run_wasm_worker_test() {
    use super::Worker;

    let sample_wasm_file = "./tests/data/sample.wasm";

    let mut worker = Worker::new(sample_wasm_file).unwrap();

    for _ in 1..10 {
        let headers: Vec<(&str, &str)> = vec![];
        let req = Request {
            method: "GET",
            uri: "/abc",
            headers: &headers,
            body: Some("xxxyyy".as_bytes()),
        };

        let resp = worker
            .exports
            .handle_request(&mut worker.store, req)
            .unwrap();
        assert_eq!(resp.status, 200);
        assert_eq!(resp.body, Some("xxxyyy".as_bytes().to_vec()));

        let headers = resp.headers;
        for (key, value) in headers {
            if key == "X-Request-Method" {
                assert_eq!(value, "GET");
            }
            if key == "X-Request-Url" {
                assert_eq!(value, "/abc");
            }
        }
    }
}

#[tokio::test]
async fn run_worker_pool_test() {
    use super::{Manager, Pool};

    let sample_wasm_file = "./tests/data/sample.wasm";
    let mgr = Manager::new(sample_wasm_file.to_string());
    let pool = Pool::builder(mgr).build().unwrap();

    let status = pool.status();
    assert_eq!(status.size, 0);
    assert_eq!(status.available, 0);

    {
        let mut worker = pool.get().await.unwrap();
        let worker = worker.as_mut();

        let status = pool.status();
        assert_eq!(status.size, 1);
        assert_eq!(status.available, 0);

        let headers: Vec<(&str, &str)> = vec![];
        let req = Request {
            method: "GET",
            uri: "/abc",
            headers: &headers,
            body: Some("xxxyyy".as_bytes()),
        };

        let resp = worker
            .exports
            .handle_request(&mut worker.store, req)
            .unwrap();
        assert_eq!(resp.status, 200);
        assert_eq!(resp.body, Some("xxxyyy".as_bytes().to_vec()));
    }

    let status = pool.status();
    assert_eq!(status.size, 1);
    assert_eq!(status.available, 1);
}
