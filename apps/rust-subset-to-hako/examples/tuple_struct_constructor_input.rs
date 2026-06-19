pub struct BasicBlockId(pub u32);

impl BasicBlockId {
    pub fn new(id: u32) -> Self {
        BasicBlockId(id)
    }
}
