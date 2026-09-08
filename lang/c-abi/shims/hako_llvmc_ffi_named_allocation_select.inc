/* Physical Named emission selection, shared by the two active walkers.
 * Borrow existing plan metadata only after builtin/alias precedence is settled.
 * No environment reads, register facts, materialization or emission here.
 * Prescan remains a separate observer; this is not source admission/preflight. */
enum GenericPureTypedObjectNewPlanStatus {
  GENERIC_PURE_TYPED_OBJECT_NEW_PLAN_MISSING = 0,
  GENERIC_PURE_TYPED_OBJECT_NEW_PLAN_INVALID,
  GENERIC_PURE_TYPED_OBJECT_NEW_PLAN_OK,
};

struct GenericPureTypedObjectNewPlanView {
  int typed_plan_index;
  long long type_id;
  long long field_count;
};

auto enum GenericPureTypedObjectNewPlanStatus
generic_pure_validate_typed_object_new_plan(
    long long dst,
    const char* box_name,
    struct GenericPureTypedObjectNewPlanView* out) {
  int typed_plan_index = -1;
  long long type_id = 0;
  long long field_count = 0;
  if (!(box_name && out)) {
    return GENERIC_PURE_TYPED_OBJECT_NEW_PLAN_MISSING;
  }
  if (!find_typed_object_plan_index(box_name, &typed_plan_index)) {
    return GENERIC_PURE_TYPED_OBJECT_NEW_PLAN_MISSING;
  }
  type_id = typed_object_plan_type_id(typed_plan_index);
  field_count = typed_object_plan_field_count(typed_plan_index);
  if (!(dst && type_id > 0 && field_count >= 0)) {
    return GENERIC_PURE_TYPED_OBJECT_NEW_PLAN_INVALID;
  }
  out->typed_plan_index = typed_plan_index;
  out->type_id = type_id;
  out->field_count = field_count;
  return GENERIC_PURE_TYPED_OBJECT_NEW_PLAN_OK;
}

enum NamedAllocationWalker {
  NAMED_ALLOCATION_GENERIC,
  NAMED_ALLOCATION_SAME_MODULE
};
enum NamedAllocationConsumer {
  NAMED_ALLOCATION_ARRAY,
  NAMED_ALLOCATION_DIRECT_ARRAY,
  NAMED_ALLOCATION_MAP,
  NAMED_ALLOCATION_FILE,
  NAMED_ALLOCATION_ALIAS_OPERAND_ZERO,
  NAMED_ALLOCATION_TYPED_OBJECT,
  NAMED_ALLOCATION_INVALID_PLAN,
  NAMED_ALLOCATION_UNSUPPORTED
};

auto enum NamedAllocationConsumer select_named_allocation_consumer(
    enum NamedAllocationWalker walker,
    const char* target,
    long long dst,
    long long arg0,
    int direct_array_exact,
    struct GenericPureTypedObjectNewPlanView* plan) {
  if (target && !strcmp(target, "DirectArrayI64"))
    return NAMED_ALLOCATION_DIRECT_ARRAY;
  if (target && !strcmp(target, "ArrayBox"))
    return direct_array_exact ? NAMED_ALLOCATION_DIRECT_ARRAY : NAMED_ALLOCATION_ARRAY;
  if (target && !strcmp(target, "MapBox"))
    return NAMED_ALLOCATION_MAP;
  if (walker == NAMED_ALLOCATION_GENERIC) {
    if (target && !strcmp(target, "FileBox")) return NAMED_ALLOCATION_FILE;
    if (target && !strcmp(target, "StringBox") && dst && arg0)
      return NAMED_ALLOCATION_ALIAS_OPERAND_ZERO;
  }
  switch (generic_pure_validate_typed_object_new_plan(dst, target, plan)) {
    case GENERIC_PURE_TYPED_OBJECT_NEW_PLAN_OK:
      return NAMED_ALLOCATION_TYPED_OBJECT;
    case GENERIC_PURE_TYPED_OBJECT_NEW_PLAN_INVALID:
      return NAMED_ALLOCATION_INVALID_PLAN;
    case GENERIC_PURE_TYPED_OBJECT_NEW_PLAN_MISSING:
      return NAMED_ALLOCATION_UNSUPPORTED;
  }
  return NAMED_ALLOCATION_UNSUPPORTED;
}
