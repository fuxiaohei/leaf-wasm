pub type QueryParam<'a> = (&'a str, &'a str);
pub type QueryParams<'a> = Vec<QueryParam<'a>>;

pub fn query_get<'a>(params: &'a QueryParams<'a>, key: &'a str) -> Option<&'a str> {
    for (k, v) in params {
        if *k == key {
            return Some(v);
        }
    }
    None
}

pub fn query_decode(string: &str) -> QueryParams {
    let mut v = Vec::new();
    for pair in string.split('&') {
        let mut it = pair.split('=').take(2);
        let kv = match (it.next(), it.next()) {
            (Some(k), Some(v)) => (k, v),
            _ => continue,
        };
        v.push(kv);
    }
    v
}
