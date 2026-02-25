pub(in crate::mir::builder) fn normalize_span_line_col(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = String::with_capacity(input.len());
    let mut i = 0usize;
    while i < bytes.len() {
        const LINE: &[u8] = b"line: ";
        const COL: &[u8] = b"column: ";

        if bytes[i..].starts_with(LINE) {
            out.push_str("line: ");
            i += LINE.len();
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            out.push('1');
            continue;
        }
        if bytes[i..].starts_with(COL) {
            out.push_str("column: ");
            i += COL.len();
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            out.push('1');
            continue;
        }

        out.push(bytes[i] as char);
        i += 1;
    }
    out
}
