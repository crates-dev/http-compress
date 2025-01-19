#[test]
fn test() {
    use crate::*;
    use std::{borrow::Cow, collections::HashMap};

    let headers: HashMap<_, _> = HashMap::new();
    let data: Vec<u8> = vec![];
    let body: Cow<'_, Vec<u8>> = Compress::from(&headers).decode(&data, 1024);
    assert_eq!(*body, data);
}
