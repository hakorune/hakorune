use crate::config::env::joinir_vm_bridge_enabled;

/// JoinIR VM ブリッジの環境フラグ
#[derive(Debug, Clone, Copy)]
pub struct JoinIrEnvFlags {
    pub vm_bridge: bool,
}

impl JoinIrEnvFlags {
    /// 現在の環境変数からフラグを取得
    pub fn from_env() -> Self {
        Self {
            vm_bridge: joinir_vm_bridge_enabled(),
        }
    }

    /// JoinIR ブリッジが有効かどうか
    pub fn is_bridge_enabled(&self) -> bool {
        self.vm_bridge
    }
}
