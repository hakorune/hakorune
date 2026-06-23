pub struct CanonicalExplicitPhiPilot;

impl CanonicalExplicitPhiPilot {
    pub fn choose_value(flag: i64) -> i64 {
        let value;
        if flag == 1 {
            value = 10;
        } else {
            value = 20;
        }
        value
    }
}
