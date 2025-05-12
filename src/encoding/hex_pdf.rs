/// Codifica una cadena UTF-8 a UTF-16BE hexadecimal PDF-safe
pub fn encode_pdf_hex(input: &str) -> String {
    let utf16: Vec<u16> = input.encode_utf16().collect();
    let mut out = String::with_capacity(utf16.len() * 4 + 2);
    out.push('<');
    for unit in utf16 {
        out.push_str(&format!("{:04X}", unit));
    }
    out.push('>');
    out
}

/// Decodifica una cadena hexadecimal PDF-safe en UTF-16BE
pub fn decode_pdf_hex(input: &str) -> Option<String> {
    let s = input.strip_prefix('<')?.strip_suffix('>')?;
    if s.len() % 4 != 0 {
        return None;
    }

    let mut u16s = Vec::with_capacity(s.len() / 4);
    for chunk in s.as_bytes().chunks(4) {
        let hex = std::str::from_utf8(chunk).ok()?;
        let code = u16::from_str_radix(hex, 16).ok()?;
        u16s.push(code);
    }

    String::from_utf16(&u16s).ok()
}
