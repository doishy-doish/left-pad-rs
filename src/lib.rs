const SPACES: [&str; 10] = ["", " ", "  ", "   ", "    ", "     ", "      ", "       ", "        ", "         "];

pub fn left_pad(s: &str, len: usize, ch: Option<&str>) -> String {
    let s_len = s.encode_utf16().count();
    if len <= s_len {
        return s.to_string();
    }
    let mut pad_len = len - s_len;
    let ch = match ch {
        None | Some("") => " ",
        Some(c) => c,
    };
    if ch == " " && pad_len < 10 {
        let mut out = String::with_capacity(len);
        out.push_str(SPACES[pad_len]);
        out.push_str(s);
        return out;
    }
    let mut pad = String::new();
    let mut cur = ch.to_string();
    loop {
        if pad_len & 1 == 1 {
            pad.push_str(&cur);
        }
        pad_len >>= 1;
        if pad_len > 0 {
            let doubled = cur.clone();
            cur.push_str(&doubled);
        } else {
            break;
        }
    }
    pad.push_str(s);
    pad
}
