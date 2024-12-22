#[test]
fn test() {
    use crate::*;
    use http_type::Header;
    use std::collections::HashMap;
    let headers: Header = HashMap::new();
    let data: Vec<u8> = vec![];
    let body: Vec<u8> = Compress::from(&headers).decode(&data, 1024);
    assert_eq!(body, data);
}
