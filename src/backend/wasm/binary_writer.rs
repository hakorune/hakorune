use super::WasmError;

const WASM_MAGIC: [u8; 4] = [0x00, 0x61, 0x73, 0x6d];
const WASM_VERSION_1: [u8; 4] = [0x01, 0x00, 0x00, 0x00];

const SECTION_TYPE: u8 = 1;
const SECTION_FUNCTION: u8 = 3;
const SECTION_EXPORT: u8 = 7;
const SECTION_CODE: u8 = 10;

const OP_I32_CONST: u8 = 0x41;
const OP_END: u8 = 0x0b;
const FUNC_TYPE: u8 = 0x60;
const VALUE_TYPE_I32: u8 = 0x7f;
const EXPORT_KIND_FUNC: u8 = 0x00;

pub(crate) fn encode_u32_leb128(mut value: u32) -> Vec<u8> {
    let mut out = Vec::new();
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if value == 0 {
            break;
        }
    }
    out
}

pub(crate) fn encode_i32_leb128(mut value: i32) -> Vec<u8> {
    let mut out = Vec::new();
    loop {
        let byte = (value & 0x7f) as u8;
        value >>= 7;
        let sign_bit_set = (byte & 0x40) != 0;
        let done = (value == 0 && !sign_bit_set) || (value == -1 && sign_bit_set);
        if done {
            out.push(byte);
            break;
        } else {
            out.push(byte | 0x80);
        }
    }
    out
}

fn append_section(module: &mut Vec<u8>, section_id: u8, payload: &[u8]) {
    module.push(section_id);
    module.extend_from_slice(&encode_u32_leb128(payload.len() as u32));
    module.extend_from_slice(payload);
}

/// Build the minimum valid wasm module with:
/// - one function type: () -> i32
/// - one function exported as "main"
/// - code: i32.const <value>; end
pub(crate) fn build_minimal_main_i32_const_module(value: i32) -> Result<Vec<u8>, WasmError> {
    let mut module = Vec::new();
    module.extend_from_slice(&WASM_MAGIC);
    module.extend_from_slice(&WASM_VERSION_1);

    // Type section: [vec(1), functype(0x60), params(0), results(1 i32)]
    let type_payload = vec![0x01, FUNC_TYPE, 0x00, 0x01, VALUE_TYPE_I32];
    append_section(&mut module, SECTION_TYPE, &type_payload);

    // Function section: [vec(1), type_index(0)]
    let function_payload = vec![0x01, 0x00];
    append_section(&mut module, SECTION_FUNCTION, &function_payload);

    // Export section: export "main" -> func index 0
    let mut export_payload = vec![0x01, 0x04];
    export_payload.extend_from_slice(b"main");
    export_payload.push(EXPORT_KIND_FUNC);
    export_payload.push(0x00);
    append_section(&mut module, SECTION_EXPORT, &export_payload);

    // Code section: one body with no locals and i32.const value
    let mut body = vec![0x00, OP_I32_CONST];
    body.extend_from_slice(&encode_i32_leb128(value));
    body.push(OP_END);
    let mut code_payload = vec![0x01];
    code_payload.extend_from_slice(&encode_u32_leb128(body.len() as u32));
    code_payload.extend_from_slice(&body);
    append_section(&mut module, SECTION_CODE, &code_payload);

    if module.len() < 8 {
        return Err(WasmError::WasmValidationError(
            "binary writer emitted truncated module".to_string(),
        ));
    }
    Ok(module)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decode_u32_leb128(bytes: &[u8], start: usize) -> Option<(u32, usize)> {
        let mut result = 0u32;
        let mut shift = 0u32;
        let mut idx = start;
        loop {
            let b = *bytes.get(idx)?;
            result |= ((b & 0x7f) as u32) << shift;
            idx += 1;
            if (b & 0x80) == 0 {
                return Some((result, idx));
            }
            shift += 7;
            if shift > 35 {
                return None;
            }
        }
    }

    fn collect_section_ids(bytes: &[u8]) -> Vec<u8> {
        let mut ids = Vec::new();
        let mut idx = 8; // skip magic/version
        while idx < bytes.len() {
            let id = bytes[idx];
            idx += 1;
            let Some((len, next)) = decode_u32_leb128(bytes, idx) else {
                break;
            };
            idx = next.saturating_add(len as usize);
            ids.push(id);
        }
        ids
    }

    #[test]
    fn wasm_binary_writer_magic_version_contract() {
        let wasm = build_minimal_main_i32_const_module(7).expect("writer must succeed");
        assert!(wasm.starts_with(&WASM_MAGIC));
        assert_eq!(&wasm[4..8], &WASM_VERSION_1);
    }

    #[test]
    fn wasm_binary_writer_section_order_contract() {
        let wasm = build_minimal_main_i32_const_module(7).expect("writer must succeed");
        let ids = collect_section_ids(&wasm);
        assert_eq!(ids, vec![SECTION_TYPE, SECTION_FUNCTION, SECTION_EXPORT, SECTION_CODE]);
    }

    #[test]
    fn wasm_binary_writer_leb128_contract() {
        assert_eq!(encode_u32_leb128(0), vec![0x00]);
        assert_eq!(encode_u32_leb128(127), vec![0x7f]);
        assert_eq!(encode_u32_leb128(128), vec![0x80, 0x01]);
        assert_eq!(encode_i32_leb128(0), vec![0x00]);
        assert_eq!(encode_i32_leb128(-1), vec![0x7f]);
    }

    #[test]
    fn wasm_binary_writer_main_export_contract() {
        let wasm = build_minimal_main_i32_const_module(42).expect("writer must succeed");
        assert!(wasm.windows(4).any(|w| w == b"main"));
        assert!(wasm.starts_with(&[0x00, 0x61, 0x73, 0x6d]));
    }
}
