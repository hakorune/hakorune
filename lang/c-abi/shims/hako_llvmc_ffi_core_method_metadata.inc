      enum CoreMethodMetadataProof {
        HAKO_LLVMC_CORE_METHOD_METADATA_PROOF_NONE = 0,
        HAKO_LLVMC_CORE_METHOD_METADATA_PROOF_CONTRACT_MANIFEST,
      };

      enum CoreMethodLoweringTierMetadata {
        HAKO_LLVMC_CORE_METHOD_LOWERING_TIER_NONE = 0,
        HAKO_LLVMC_CORE_METHOD_LOWERING_TIER_WARM_DIRECT_ABI,
        HAKO_LLVMC_CORE_METHOD_LOWERING_TIER_COLD_FALLBACK,
      };

      auto enum CoreMethodMetadataProof read_core_method_proof_metadata(
          yyjson_val* core_method,
          int* invalid) {
        const char* proof = read_str(core_method, "proof");
        if (proof && !strcmp(proof, "core_method_contract_manifest")) {
          return HAKO_LLVMC_CORE_METHOD_METADATA_PROOF_CONTRACT_MANIFEST;
        }
        if (invalid) *invalid = 1;
        return HAKO_LLVMC_CORE_METHOD_METADATA_PROOF_NONE;
      }

      auto int core_method_metadata_proof_is_contract_manifest(
          enum CoreMethodMetadataProof proof) {
        return proof == HAKO_LLVMC_CORE_METHOD_METADATA_PROOF_CONTRACT_MANIFEST;
      }

      auto enum CoreMethodLoweringTierMetadata
      read_core_method_lowering_tier_metadata(
          yyjson_val* core_method,
          int* invalid) {
        const char* tier = read_str(core_method, "lowering_tier");
        if (tier && !strcmp(tier, "warm_direct_abi")) {
          return HAKO_LLVMC_CORE_METHOD_LOWERING_TIER_WARM_DIRECT_ABI;
        }
        if (tier && !strcmp(tier, "cold_fallback")) {
          return HAKO_LLVMC_CORE_METHOD_LOWERING_TIER_COLD_FALLBACK;
        }
        if (invalid) *invalid = 1;
        return HAKO_LLVMC_CORE_METHOD_LOWERING_TIER_NONE;
      }

      auto int core_method_lowering_tier_is_warm_direct_abi(
          enum CoreMethodLoweringTierMetadata tier) {
        return tier == HAKO_LLVMC_CORE_METHOD_LOWERING_TIER_WARM_DIRECT_ABI;
      }
