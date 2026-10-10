use serde_json::Value;

#[path = "../src-lib.rs"]
mod lib_rs;
use lib_rs::left_pad;

#[test]
fn parity_with_original_js() {
    let raw = include_str!("../vectors.json");
    let vectors: Vec<Value> = serde_json::from_str(raw).unwrap();
    assert!(!vectors.is_empty());
    for v in &vectors {
        let s = v["str"].as_str().unwrap();
        let len = v["len"].as_u64().unwrap() as usize;
        let ch = v["ch"].as_str();
        let expected = v["expected"].as_str().unwrap();
        assert_eq!(left_pad(s, len, ch), expected, "divergence on {v}");
    }
}
