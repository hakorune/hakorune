/* Sole private input lifetime. Initialize at the final address: outcomes.config
 * points into this owner. Do not copy/move an initialized invocation by value.
 * Program observation may precede core when queried; its failure is consumed at
 * the original stage. Definition reading stays late. Mutable execution cursors stay local. */
#include "hako_llvmc_ffi_physical_definition_plan.inc"

enum HakoLlvmcProgramObservation { PROGRAM_UNOBSERVED, PROGRAM_AVAILABLE, PROGRAM_UNAVAILABLE };

struct HakoLlvmcInvocation {
  yyjson_doc* document;
  struct HakoLlvmcAllocationConfig config;
  struct NamedAllocationOutcomes outcomes;
  hako_llvmc_generic_pure_program_view program;
  struct PhysicalDefinitionPlan definitions;
  enum HakoLlvmcProgramObservation program_observation;
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

/* Observe without diagnostics. Core consumes failure at its original stage. */
static int hako_llvmc_invocation_observe_program(struct HakoLlvmcInvocation* invocation) {
  if (invocation->program_observation == PROGRAM_UNOBSERVED) {
    invocation->program_observation = hako_llvmc_read_generic_pure_program_view(
        yyjson_doc_get_root(invocation->document), &invocation->program)
        ? PROGRAM_AVAILABLE : PROGRAM_UNAVAILABLE;
  }
  return invocation->program_observation == PROGRAM_AVAILABLE;
}

#include "hako_llvmc_ffi_named_query.inc"
