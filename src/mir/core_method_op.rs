/*!
 * MIR-side CoreMethodOp carrier vocabulary.
 *
 * The CoreMethodContract `.hako` box owns the semantic rows. This module only
 * defines the narrow MIR carrier vocabulary used after a method has already
 * been resolved to a compiler contract row.
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreMethodOp {
    ArrayLen,
    ArrayGet,
    ArrayHas,
    ArraySet,
    ArrayPush,
    MapGet,
    MapSet,
    MapHas,
    MapLen,
    StringLen,
    StringSubstring,
    StringIndexOf,
}

impl CoreMethodOp {
    pub const ALL: &'static [Self] = &[
        Self::ArrayLen,
        Self::ArrayGet,
        Self::ArrayHas,
        Self::ArraySet,
        Self::ArrayPush,
        Self::MapGet,
        Self::MapSet,
        Self::MapHas,
        Self::MapLen,
        Self::StringLen,
        Self::StringSubstring,
        Self::StringIndexOf,
    ];

    pub fn as_manifest_name(self) -> &'static str {
        match self {
            Self::ArrayLen => "ArrayLen",
            Self::ArrayGet => "ArrayGet",
            Self::ArrayHas => "ArrayHas",
            Self::ArraySet => "ArraySet",
            Self::ArrayPush => "ArrayPush",
            Self::MapGet => "MapGet",
            Self::MapSet => "MapSet",
            Self::MapHas => "MapHas",
            Self::MapLen => "MapLen",
            Self::StringLen => "StringLen",
            Self::StringSubstring => "StringSubstring",
            Self::StringIndexOf => "StringIndexOf",
        }
    }

    pub fn from_manifest_name(name: &str) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|op| op.as_manifest_name() == name)
    }
}

impl std::fmt::Display for CoreMethodOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_manifest_name())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreMethodOpProof {
    CoreMethodContractManifest,
}

impl std::fmt::Display for CoreMethodOpProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CoreMethodContractManifest => f.write_str("core_method_contract_manifest"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreMethodLoweringTier {
    WarmDirectAbi,
    ColdFallback,
}

impl CoreMethodLoweringTier {
    pub const ALL: &'static [Self] = &[Self::WarmDirectAbi, Self::ColdFallback];

    pub fn as_manifest_name(self) -> &'static str {
        match self {
            Self::WarmDirectAbi => "warm_direct_abi",
            Self::ColdFallback => "cold_fallback",
        }
    }

    pub fn from_manifest_name(name: &str) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|tier| tier.as_manifest_name() == name)
    }

    pub fn is_warm_direct_abi(self) -> bool {
        self == Self::WarmDirectAbi
    }
}

impl std::fmt::Display for CoreMethodLoweringTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_manifest_name())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoreMethodOpCarrier {
    pub op: CoreMethodOp,
    pub proof: CoreMethodOpProof,
    pub lowering_tier: CoreMethodLoweringTier,
}

impl CoreMethodOpCarrier {
    pub fn manifest(op: CoreMethodOp, lowering_tier: CoreMethodLoweringTier) -> Self {
        Self {
            op,
            proof: CoreMethodOpProof::CoreMethodContractManifest,
            lowering_tier,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn manifest_core_ops_are_known_by_mir_carrier() {
        let manifest = include_str!(
            "../../lang/src/runtime/meta/generated/core_method_contract_manifest.json"
        );
        let parsed: serde_json::Value = serde_json::from_str(manifest).expect("manifest json");
        let rows = parsed["rows"].as_array().expect("manifest rows");
        let manifest_ops = rows
            .iter()
            .map(|row| row["core_op"].as_str().expect("core_op"))
            .collect::<BTreeSet<_>>();
        let mir_ops = CoreMethodOp::ALL
            .iter()
            .map(|op| op.as_manifest_name())
            .collect::<BTreeSet<_>>();

        assert_eq!(manifest_ops, mir_ops);
    }

    #[test]
    fn manifest_lowering_tiers_are_known_by_mir_carrier() {
        let manifest = include_str!(
            "../../lang/src/runtime/meta/generated/core_method_contract_manifest.json"
        );
        let parsed: serde_json::Value = serde_json::from_str(manifest).expect("manifest json");
        let rows = parsed["rows"].as_array().expect("manifest rows");
        let manifest_tiers = rows
            .iter()
            .map(|row| row["lowering_tier"].as_str().expect("lowering_tier"))
            .collect::<BTreeSet<_>>();
        let mir_tiers = CoreMethodLoweringTier::ALL
            .iter()
            .map(|tier| tier.as_manifest_name())
            .collect::<BTreeSet<_>>();

        for tier in manifest_tiers {
            assert!(
                mir_tiers.contains(tier),
                "CoreMethodContract lowering tier is missing from MIR vocabulary: {tier}"
            );
        }
    }

    #[test]
    fn map_has_manifest_tier_matches_first_carrier_contract() {
        let manifest = include_str!(
            "../../lang/src/runtime/meta/generated/core_method_contract_manifest.json"
        );
        let parsed: serde_json::Value = serde_json::from_str(manifest).expect("manifest json");
        let rows = parsed["rows"].as_array().expect("manifest rows");
        let map_has = rows
            .iter()
            .find(|row| row["core_op"].as_str() == Some("MapHas"))
            .expect("MapHas manifest row");
        let tier = map_has["lowering_tier"].as_str().expect("lowering_tier");

        assert_eq!(
            CoreMethodLoweringTier::from_manifest_name(tier),
            Some(CoreMethodLoweringTier::WarmDirectAbi)
        );
    }

    #[test]
    fn carrier_formats_stable_metadata_tokens() {
        let carrier = CoreMethodOpCarrier::manifest(
            CoreMethodOp::MapHas,
            CoreMethodLoweringTier::WarmDirectAbi,
        );

        assert_eq!(carrier.op.to_string(), "MapHas");
        assert_eq!(carrier.proof.to_string(), "core_method_contract_manifest");
        assert_eq!(carrier.lowering_tier.to_string(), "warm_direct_abi");
        assert!(carrier.lowering_tier.is_warm_direct_abi());
        assert_eq!(
            CoreMethodLoweringTier::from_manifest_name("warm_direct_abi"),
            Some(CoreMethodLoweringTier::WarmDirectAbi)
        );
        assert_eq!(CoreMethodLoweringTier::from_manifest_name("unknown"), None);
        assert_eq!(
            CoreMethodOp::from_manifest_name("MapHas"),
            Some(CoreMethodOp::MapHas)
        );
        assert_eq!(CoreMethodOp::from_manifest_name("Unknown"), None);
    }
}
