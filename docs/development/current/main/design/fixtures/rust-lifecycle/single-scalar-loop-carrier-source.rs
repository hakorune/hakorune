pub struct SingleScalarLoopCarrierPilot;

impl SingleScalarLoopCarrierPilot {
    pub fn sum_values(values: &[i64]) -> i64 {
        let mut i = 0;
        let mut sum = 0;
        while i < values.len() {
            sum += values[i];
            i += 1;
        }
        sum
    }
}
