/* Sole private input lifetime. Initialize at the final address: outcomes.config
 * points into this owner. Do not copy/move an initialized invocation by value.
 * Program/definition readers retain their ordered positions in the borrowed core;
 * unread storage is not a query result. Mutable execution cursors stay local. */
#include "hako_llvmc_ffi_physical_definition_plan.inc"

struct HakoLlvmcInvocation {
  yyjson_doc* document;
  struct HakoLlvmcAllocationConfig config;
  struct NamedAllocationOutcomes outcomes;
  hako_llvmc_generic_pure_program_view program;
  struct PhysicalDefinitionPlan definitions;
};

/* Takes document ownership; config is already captured by the invocation caller. */
static void hako_llvmc_invocation_init(
    struct HakoLlvmcInvocation* invocation, yyjson_doc* document,
    struct HakoLlvmcAllocationConfig config) {
  *invocation = (struct HakoLlvmcInvocation){.document = document, .config = config};
  named_allocation_outcomes_init(
      &invocation->outcomes, yyjson_doc_get_root(document), &invocation->config);
}

static void hako_llvmc_invocation_destroy(struct HakoLlvmcInvocation* invocation) {
  named_allocation_outcomes_destroy(&invocation->outcomes);
  yyjson_doc_free(invocation->document);
  *invocation = (struct HakoLlvmcInvocation){0};
}
