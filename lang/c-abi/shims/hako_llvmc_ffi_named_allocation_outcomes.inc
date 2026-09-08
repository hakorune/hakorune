/* Conditional representation only; immutable reads are not frame-row takes. */
struct NamedAllocationOutcome {
  enum NamedAllocationConsumer consumer;
  struct GenericPureTypedObjectNewPlanView plan;
};
struct NamedAllocationObservation {
  yyjson_val* function;
  yyjson_val* instruction;
  struct NamedAllocationOutcome walkers[2];
};
struct NamedAllocationOutcomes {
  yyjson_val* root;
  const struct HakoLlvmcAllocationConfig* config;
  struct NamedAllocationObservation* rows;
  size_t count;
  size_t capacity;
  int storage_failed;
};

static int named_allocation_observation_compare(const void* left, const void* right) {
  const struct NamedAllocationObservation* a = left;
  const struct NamedAllocationObservation* b = right;
  uintptr_t af = (uintptr_t)a->function, bf = (uintptr_t)b->function;
  uintptr_t ai = (uintptr_t)a->instruction, bi = (uintptr_t)b->instruction;
  if (af != bf) return af < bf ? -1 : 1;
  return ai < bi ? -1 : ai > bi ? 1 : 0;
}

static int named_allocation_outcomes_append(
    struct NamedAllocationOutcomes* owner, yyjson_val* function, yyjson_val* ins) {
  if (owner->count == owner->capacity) {
    size_t capacity = owner->capacity ? owner->capacity * 2 : 16;
    if (capacity < owner->capacity || capacity > SIZE_MAX / sizeof(*owner->rows)) return 0;
    void* rows = realloc(owner->rows, capacity * sizeof(*owner->rows));
    if (!rows) return 0;
    owner->rows = rows;
    owner->capacity = capacity;
  }
  struct NamedAllocationObservation* row = &owner->rows[owner->count++];
  memset(row, 0, sizeof(*row));
  row->function = function;
  row->instruction = ins;
  const char* target = yyjson_get_str(yyjson_obj_get(ins, "type"));
  long long dst = (long long)yyjson_get_sint(yyjson_obj_get(ins, "dst"));
  yyjson_val* args = yyjson_obj_get(ins, "args");
  long long arg0 = yyjson_is_arr(args)
      ? (long long)yyjson_get_sint(yyjson_arr_get_first(args)) : 0;
  int direct = (owner->config->runtime_flags &
                HAKO_LLVMC_RUNTIME_MODE_REQUIRES_DIRECT_ARRAY_I64_EXACT) != 0;
  for (int walker = NAMED_ALLOCATION_GENERIC; walker <= NAMED_ALLOCATION_SAME_MODULE; walker++) {
    row->walkers[walker].consumer = select_named_allocation_consumer(
        owner->root, walker, target, dst,
        walker == NAMED_ALLOCATION_GENERIC ? arg0 : 0, direct,
        &row->walkers[walker].plan);
  }
  return 1;
}

/* Failure is retained, never reported ahead of existing schema/route terminals. */
static void named_allocation_outcomes_init(
    struct NamedAllocationOutcomes* owner, yyjson_val* root,
    const struct HakoLlvmcAllocationConfig* config) {
  *owner = (struct NamedAllocationOutcomes){.root = root, .config = config};
  yyjson_val* functions = yyjson_obj_get(root, "functions");
  if (!yyjson_is_arr(functions)) return;
  for (size_t fi = 0; fi < yyjson_arr_size(functions); fi++) {
    yyjson_val* fn = yyjson_arr_get(functions, fi);
    yyjson_val* blocks = yyjson_obj_get(fn, "blocks");
    if (!yyjson_is_arr(blocks)) continue;
    for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++) {
      yyjson_val* instructions = yyjson_obj_get(yyjson_arr_get(blocks, bi), "instructions");
      if (!yyjson_is_arr(instructions)) continue;
      for (size_t ii = 0; ii < yyjson_arr_size(instructions); ii++) {
        yyjson_val* ins = yyjson_arr_get(instructions, ii);
        const char* op = yyjson_get_str(yyjson_obj_get(ins, "op"));
        if (!op || strcmp(op, "newbox") || yyjson_obj_get(ins, "target")) continue;
        if (!named_allocation_outcomes_append(owner, fn, ins)) {
          owner->storage_failed = 1;
          return;
        }
      }
    }
  }
  if (owner->count > 1)
    qsort(owner->rows, owner->count, sizeof(*owner->rows), named_allocation_observation_compare);
}

static const struct NamedAllocationOutcome* named_allocation_outcomes_find(
    const struct NamedAllocationOutcomes* owner, yyjson_val* function,
    yyjson_val* instruction, enum NamedAllocationWalker walker) {
  if (owner->storage_failed || !owner->count ||
      (walker != NAMED_ALLOCATION_GENERIC && walker != NAMED_ALLOCATION_SAME_MODULE)) return NULL;
  struct NamedAllocationObservation key = {.function = function, .instruction = instruction};
  const struct NamedAllocationObservation* row = bsearch(
      &key, owner->rows, owner->count, sizeof(*owner->rows), named_allocation_observation_compare);
  return row ? &row->walkers[walker] : NULL;
}

static void named_allocation_outcomes_destroy(struct NamedAllocationOutcomes* owner) {
  free(owner->rows);
  *owner = (struct NamedAllocationOutcomes){0};
}
