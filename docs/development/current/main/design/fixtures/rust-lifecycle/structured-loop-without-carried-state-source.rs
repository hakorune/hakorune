pub struct StructuredLoopPilot;

impl StructuredLoopPilot {
    pub fn copy_values(values: &[i64], out: &mut Vec<i64>) {
        let mut i = 0;
        while i < values.len() {
            out.push(values[i]);
            i += 1;
        }
    }
}
