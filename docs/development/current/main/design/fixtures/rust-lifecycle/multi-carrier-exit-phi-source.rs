pub struct MultiCarrierExitPhiPilot;

impl MultiCarrierExitPhiPilot {
    pub fn project_exit_carriers(exit_kind: i64) -> (i64, i64) {
        match exit_kind {
            0 => (1, 10),
            1 => (2, 20),
            2 => (3, 30),
            _ => (0, 0),
        }
    }
}
