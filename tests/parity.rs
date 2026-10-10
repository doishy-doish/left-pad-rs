use left_pad_rs::left_pad;
use serde_json::Value;

#[test]
fn parity_with_original_js() {
    let raw = include_str!("../vectors.json");
    let vectors: Vec<Value> = serde_json::from_str(raw).unwrap();
    assert!(!vectors.is_empty());
    for v in &vectors {
        let s = v["str"].as_str().unwrap();
        let len = v["len"].as_i64().unwrap().max(0) as usize;
        let ch = v["ch"].as_str();
        let expected = v["expected"].as_str().unwrap();
        assert_eq!(left_pad(s, len, ch), expected, "divergence on {v}");
    }
}
