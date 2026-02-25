#[cfg(test)]
mod tests {
    use crate::mir::join_ir_vm_bridge::joinir_block_converter::JoinIrBlockConverter;
    use crate::mir::BasicBlockId;

    #[test]
    fn test_block_converter_exists() {
        let converter = JoinIrBlockConverter::new();
        assert_eq!(converter.current_block_id, BasicBlockId(0));
        assert_eq!(converter.next_block_id, 1);
    }
}
