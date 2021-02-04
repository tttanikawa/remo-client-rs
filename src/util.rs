use std::collections::BTreeMap;

pub fn encode_params(params: &BTreeMap<&str, &str>) -> String {
    params
        .iter()
        .map(|(k, v)| format!("{}={}", percent_encode(k), percent_encode(v)))
        .collect::<Vec<String>>()
        .join("&")
}

pub fn percent_encode(s: &str) -> percent_encoding::PercentEncode {
    use percent_encoding::*;
    const FRAGMENT: &AsciiSet = &NON_ALPHANUMERIC
        .remove(b'*')
        .remove(b'.')
        .remove(b'-')
        .remove(b'_');
    utf8_percent_encode(s, FRAGMENT)
}
