/// Pre-expand line-head `@name[: Type] = expr` into `local name[: Type] = expr`.
/// Minimal, safe, no semantics change. Applies only at line head (after spaces/tabs).
pub fn preexpand_at_local(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    for line in src.lines() {
        let bytes = line.as_bytes();
        let mut i = 0;
        while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b'\t') {
            i += 1;
        }
        if i < bytes.len() && bytes[i] == b'@' {
            // parse identifier
            let mut j = i + 1;
            if j < bytes.len() && ((bytes[j] as char).is_ascii_alphabetic() || bytes[j] == b'_') {
                j += 1;
                while j < bytes.len() {
                    let c = bytes[j] as char;
                    if c.is_ascii_alphanumeric() || c == '_' {
                        j += 1;
                    } else {
                        break;
                    }
                }
                let mut k = j;
                while k < bytes.len() && (bytes[k] == b' ' || bytes[k] == b'\t') {
                    k += 1;
                }
                if k < bytes.len() && bytes[k] == b':' {
                    k += 1;
                    while k < bytes.len() && (bytes[k] == b' ' || bytes[k] == b'\t') {
                        k += 1;
                    }
                    if k < bytes.len()
                        && ((bytes[k] as char).is_ascii_alphabetic() || bytes[k] == b'_')
                    {
                        k += 1;
                        while k < bytes.len() {
                            let c = bytes[k] as char;
                            if c.is_ascii_alphanumeric() || c == '_' {
                                k += 1;
                            } else {
                                break;
                            }
                        }
                    }
                }
                let mut eqp = k;
                while eqp < bytes.len() && (bytes[eqp] == b' ' || bytes[eqp] == b'\t') {
                    eqp += 1;
                }
                if eqp < bytes.len() && bytes[eqp] == b'=' {
                    out.push_str(&line[..i]);
                    out.push_str("local ");
                    out.push_str(&line[i + 1..eqp]);
                    out.push_str(" =");
                    out.push_str(&line[eqp + 1..]);
                    out.push('\n');
                    continue;
                }
            }
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}
