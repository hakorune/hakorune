/* Sole private input lifetime. Initialize at the final address: outcomes.config
 * points into this owner. Do not copy/move an initialized invocation by value.
 * Program observation may precede core when queried; its failure is consumed at
 * the original stage. Definition reading stays late. Mutable execution cursors stay local. */
#include "hako_llvmc_ffi_physical_definition_plan.inc"

enum HakoLlvmcProgramObservation { PROGRAM_UNOBSERVED, PROGRAM_AVAILABLE, PROGRAM_UNAVAILABLE };

struct HakoLlvmcLegacyCapiState {
  int target_machine_enabled;
  int opt_level;
};

struct HakoLlvmcInvocation {
  yyjson_doc* document;
  struct HakoLlvmcAllocationConfig config;
  struct HakoLlvmcPhysicalOptions options;
  enum HakoLlvmcIngressProfile ingress_profile;
  struct NamedAllocationOutcomes outcomes;
  hako_llvmc_generic_pure_program_view program;
  struct PhysicalDefinitionPlan definitions;
  enum HakoLlvmcProgramObservation program_observation;
  struct HakoStaticV2Index static_v2;
  struct HakoLlvmcPublishedCallRows published_call_rows;
  struct HakoLlvmcLegacyCapiState legacy_capi;
};

/* Takes document ownership; config is already captured by the invocation caller. */
static void hako_llvmc_invocation_init(
    struct HakoLlvmcInvocation* invocation, yyjson_doc* document,
    struct HakoLlvmcAllocationConfig config,
    enum HakoLlvmcIngressProfile ingress_profile) {
  *invocation = (struct HakoLlvmcInvocation){
      .document = document,
      .config = config,
      .options = {0},
      .ingress_profile = ingress_profile};
  named_allocation_outcomes_init(
      &invocation->outcomes, yyjson_doc_get_root(document), &invocation->config);
}

static void hako_llvmc_invocation_destroy(struct HakoLlvmcInvocation* invocation) {
  hako_llvmc_published_static_method_rows_end(&invocation->published_call_rows);
  hako_static_v2_clear(&invocation->static_v2);
  hako_llvmc_physical_options_destroy(&invocation->options);
  named_allocation_outcomes_destroy(&invocation->outcomes);
  yyjson_doc_free(invocation->document);
  *invocation = (struct HakoLlvmcInvocation){0};
}

/* Observe without diagnostics. Core consumes failure at its original stage. */
static int hako_llvmc_invocation_observe_program(struct HakoLlvmcInvocation* invocation) {
  if (invocation->program_observation == PROGRAM_UNOBSERVED) {
    invocation->program_observation = hako_llvmc_read_generic_pure_program_view(
        yyjson_doc_get_root(invocation->document), &invocation->program)
        ? PROGRAM_AVAILABLE : PROGRAM_UNAVAILABLE;
  }
  return invocation->program_observation == PROGRAM_AVAILABLE;
}

static int hako_llvmc_legacy_capi_opt_level(const char* value) {
  if (value && value[0] == '3') return 3;
  if (value && value[0] == '2') return 2;
  if (value && value[0] == '1') return 1;
  return 0;
}

/* Capture the retained legacy CAPI selector only for the old lowering lane.
 * Presence, including an empty HAKO value, intentionally keeps HAKO first. */
static void hako_llvmc_invocation_capture_legacy_capi(
    struct HakoLlvmcInvocation* invocation, int eligible) {
  const char* level;
  if (!invocation) return;
  invocation->legacy_capi = (struct HakoLlvmcLegacyCapiState){0};
  if (!eligible) return;
#if !defined(_WIN32)
  {
    const char* tm = getenv("HAKO_CAPI_TM");
    if (!(tm && tm[0] == '1')) return;
    invocation->legacy_capi.target_machine_enabled = 1;
    level = getenv("HAKO_LLVM_OPT_LEVEL");
    if (!level) level = getenv("NYASH_LLVM_OPT_LEVEL");
    if (!level) level = "0";
    invocation->legacy_capi.opt_level = hako_llvmc_legacy_capi_opt_level(level);
  }
#else
  (void)level;
#endif
}

#include "hako_llvmc_ffi_named_query.inc"
