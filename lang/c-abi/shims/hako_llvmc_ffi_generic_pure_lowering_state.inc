/*
 * Generic pure lowering state and macro exposure.
 *
 * This include intentionally preserves the historical macro names consumed by
 * the lowering helper includes. It only moves the state shape out of
 * pure_compile.inc; it does not change ownership or semantics.
 */

struct GenericPureI64Const { long long reg; long long val; };
struct GenericPureStringConst { long long reg; const char* val; };
struct GenericPureKnownStringLen { long long reg; long long len; };
struct GenericPureRegMarker { long long reg; };
struct GenericPureTypeBinding { long long reg; int ty; };
struct GenericPureCopyAlias { long long reg; long long src; };
struct GenericPureReceiverForwardAlias { long long reg; long long src; };
struct GenericPureOriginBinding { long long reg; int kind; };
struct GenericPureTypedObjectBinding { long long reg; int plan_index; };
struct GenericPureBoxedSumBinding { long long reg; int plan_index; };
struct GenericPureFlattenedNestedBinding {
  long long reg;
  long long owner_reg;
};
struct GenericPureVariantBinding {
  long long reg;
  long long tag_const;
  long long tag_reg;
  long long payload_reg;
  const char* enum_name;
  int has_payload;
  int boxed_sum_plan_index;
};
struct GenericPureBlockTailLabel {
  long long bid;
  char label[96];
};
struct Incoming { long long pred; long long val_reg; };
struct PhiRec { long long dst; struct Incoming in[16]; int in_n; int explicit_type; };
struct BlockPhi { long long bid; struct PhiRec recs[24]; int rec_n; };

enum {
  GENERIC_PURE_I64_CONST_CAP = 1024,
  GENERIC_PURE_STRING_CONST_CAP = 1024,
  GENERIC_PURE_KNOWN_STRING_LEN_CAP = 4096,
  GENERIC_PURE_REG_MARKER_CAP = GENERIC_PURE_STRING_CONST_CAP,
  GENERIC_PURE_I64_CONST_BY_REG_CAP = 8192,
  GENERIC_PURE_TYPE_BINDING_CAP = 8192,
  GENERIC_PURE_COPY_ALIAS_CAP = 2048,
  GENERIC_PURE_RECEIVER_FORWARD_ALIAS_CAP = 256,
  GENERIC_PURE_ORIGIN_BINDING_CAP = 2048,
  GENERIC_PURE_TYPED_OBJECT_BINDING_CAP = 2048,
  GENERIC_PURE_BOXED_SUM_BINDING_CAP = 2048,
  GENERIC_PURE_FLATTENED_NESTED_BINDING_CAP = 512,
  GENERIC_PURE_VARIANT_BINDING_CAP = 2048,
  GENERIC_PURE_BLOCK_TAIL_LABEL_CAP = 1024,
  GENERIC_PURE_BLOCK_PHI_CAP = 512
};

struct GenericPureFunctionLoweringState {
  struct GenericPureI64Const consts[GENERIC_PURE_I64_CONST_CAP];
  size_t consts_n;
  long long const_by_reg[GENERIC_PURE_I64_CONST_BY_REG_CAP];
  unsigned char const_by_reg_present[GENERIC_PURE_I64_CONST_BY_REG_CAP];
  struct GenericPureStringConst str_consts[GENERIC_PURE_STRING_CONST_CAP];
  size_t str_consts_n;
  struct GenericPureKnownStringLen known_string_lens[GENERIC_PURE_KNOWN_STRING_LEN_CAP];
  size_t known_string_lens_n;
  char* owned_str_consts[GENERIC_PURE_STRING_CONST_CAP];
  size_t owned_str_consts_n;
  struct GenericPureRegMarker str_const_box_skip[GENERIC_PURE_REG_MARKER_CAP];
  size_t str_const_box_skip_n;
  struct GenericPureRegMarker str_const_box_emitted[GENERIC_PURE_REG_MARKER_CAP];
  size_t str_const_box_emitted_n;
  struct GenericPureTypeBinding types[GENERIC_PURE_TYPE_BINDING_CAP];
  size_t types_n;
  struct GenericPureCopyAlias copy_alias[GENERIC_PURE_COPY_ALIAS_CAP];
  size_t copy_alias_n;
  struct GenericPureReceiverForwardAlias receiver_forward_alias[GENERIC_PURE_RECEIVER_FORWARD_ALIAS_CAP];
  size_t receiver_forward_alias_n;
  struct GenericPureOriginBinding scan_origin[GENERIC_PURE_ORIGIN_BINDING_CAP];
  size_t scan_origin_n;
  struct GenericPureOriginBinding origin[GENERIC_PURE_ORIGIN_BINDING_CAP];
  size_t origin_n;
  struct GenericPureTypedObjectBinding typed_object_bindings[GENERIC_PURE_TYPED_OBJECT_BINDING_CAP];
  size_t typed_object_bindings_n;
  struct GenericPureBoxedSumBinding boxed_sum_bindings[GENERIC_PURE_BOXED_SUM_BINDING_CAP];
  size_t boxed_sum_bindings_n;
  struct GenericPureFlattenedNestedBinding flattened_nested_bindings[GENERIC_PURE_FLATTENED_NESTED_BINDING_CAP];
  size_t flattened_nested_bindings_n;
  struct GenericPureVariantBinding variant_bindings[GENERIC_PURE_VARIANT_BINDING_CAP];
  size_t variant_bindings_n;
  struct GenericPureBlockTailLabel block_tail_labels[GENERIC_PURE_BLOCK_TAIL_LABEL_CAP];
  size_t block_tail_labels_n;
  struct BlockPhi phis[GENERIC_PURE_BLOCK_PHI_CAP];
  int phi_n;
  int generic_walk_trace_budget;
  int generic_call_trace_budget;
  int generic_method_trace_budget;
  int generic_capacity_overflow;
  char generic_capacity_overflow_reason[64];
};

auto void reset_generic_pure_function_lowering_state(
    struct GenericPureFunctionLoweringState* state) {
  if (!state) return;
  memset(state, 0, sizeof(*state));
  state->generic_walk_trace_budget = 24;
  state->generic_call_trace_budget = 24;
  state->generic_method_trace_budget = 24;
}

struct GenericPureFunctionLoweringState lowering_state;
reset_generic_pure_function_lowering_state(&lowering_state);
#define consts lowering_state.consts
#define consts_n lowering_state.consts_n
#define const_by_reg lowering_state.const_by_reg
#define const_by_reg_present lowering_state.const_by_reg_present
#define str_consts lowering_state.str_consts
#define str_consts_n lowering_state.str_consts_n
#define known_string_lens lowering_state.known_string_lens
#define known_string_lens_n lowering_state.known_string_lens_n
#define owned_str_consts lowering_state.owned_str_consts
#define owned_str_consts_n lowering_state.owned_str_consts_n
#define str_const_box_skip lowering_state.str_const_box_skip
#define str_const_box_skip_n lowering_state.str_const_box_skip_n
#define str_const_box_emitted lowering_state.str_const_box_emitted
#define str_const_box_emitted_n lowering_state.str_const_box_emitted_n
#define types lowering_state.types
#define types_n lowering_state.types_n
#define copy_alias lowering_state.copy_alias
#define copy_alias_n lowering_state.copy_alias_n
#define receiver_forward_alias lowering_state.receiver_forward_alias
#define receiver_forward_alias_n lowering_state.receiver_forward_alias_n
#define scan_origin lowering_state.scan_origin
#define scan_origin_n lowering_state.scan_origin_n
#define origin lowering_state.origin
#define origin_n lowering_state.origin_n
#define typed_object_bindings lowering_state.typed_object_bindings
#define typed_object_bindings_n lowering_state.typed_object_bindings_n
#define boxed_sum_bindings lowering_state.boxed_sum_bindings
#define boxed_sum_bindings_n lowering_state.boxed_sum_bindings_n
#define flattened_nested_bindings lowering_state.flattened_nested_bindings
#define flattened_nested_bindings_n lowering_state.flattened_nested_bindings_n
#define variant_bindings lowering_state.variant_bindings
#define variant_bindings_n lowering_state.variant_bindings_n
#define block_tail_labels lowering_state.block_tail_labels
#define block_tail_labels_n lowering_state.block_tail_labels_n
#define phis lowering_state.phis
#define phi_n lowering_state.phi_n
#define generic_walk_trace_budget lowering_state.generic_walk_trace_budget
#define generic_call_trace_budget lowering_state.generic_call_trace_budget
#define generic_method_trace_budget lowering_state.generic_method_trace_budget
#define generic_capacity_overflow lowering_state.generic_capacity_overflow
#define generic_capacity_overflow_reason lowering_state.generic_capacity_overflow_reason
