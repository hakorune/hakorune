/* One function owns all indexed flow storage. No semantic ledger or cache. */
typedef struct {
  hako_lv4_state *states, *scratch;
  signed char* faults;
  unsigned char* queued;
  size_t *queue, head, tail, count, stride, pending, entry;
} hako_lv4_worklist;

static void lv4_work_close(hako_lv4_worklist* work) {
  free(work->states); free(work->scratch); free(work->faults); free(work->queue); free(work->queued);
  memset(work, 0, sizeof(*work));
}

static int lv4_work_open(hako_lv4_worklist* work, size_t count, size_t stride, size_t entry) {
  if (!count || entry >= count || !stride || count > SIZE_MAX / stride || stride > SIZE_MAX / 3) return 0;
  work->states = lv4_checked_calloc(count * stride, sizeof(*work->states));
  work->scratch = lv4_checked_calloc(stride * 3, sizeof(*work->scratch));
  work->faults = lv4_checked_calloc(count, sizeof(*work->faults));
  work->queue = lv4_checked_calloc(count, sizeof(*work->queue));
  work->queued = lv4_checked_calloc(count, sizeof(*work->queued));
  if (!work->queued || !work->states || !work->scratch || !work->faults || !work->queue) {
    lv4_work_close(work); return 0;
  }
  work->count = count; work->stride = stride; work->entry = entry;
  memset(work->faults, -1, count * sizeof(*work->faults));
  work->faults[entry] = 0;
  work->queue[work->tail++] = entry; work->tail %= count;
  work->queued[entry] = 1; work->pending = 1;
  return 1;
}

static int lv4_work_enqueue(hako_lv4_worklist* work, size_t target) {
  if (work->queued[target]) return 1;
  if (work->pending >= work->count) return 0;
  work->queue[work->tail] = target; work->tail = (work->tail + 1) % work->count;
  work->queued[target] = 1; work->pending++;
  return 1;
}

static hako_lv4_value* lv4_text_phi_input(hako_lv4_function_index* fn,
    yyjson_val* phi, uint32_t predecessor) {
  yyjson_val* inputs = yyjson_obj_get(phi, "inputs");
  for (size_t i = 0; i < yyjson_arr_size(inputs); i++) {
    yyjson_val* row = yyjson_arr_get(inputs, i);
    if (lv4_u(row, "block") == predecessor) return lv4_index_value(fn, lv4_u(row, "value"));
  }
  return NULL;
}

/* Snapshot validation, all clears, then all installs: parallel affine Phi
 * transfer, including swaps. Copies lend the SAME current obligation slot. */
static int lv4_text_edge(hako_lv4_function_index* fn, uint32_t predecessor,
    yyjson_val* target, const hako_lv4_state* before, hako_lv4_state* after) {
  yyjson_val* rows = yyjson_obj_get(target, "instructions");
  size_t count = lv4_state_count(fn);
  hako_lv4_state reserved_top = lv4_string_domain_top(before, count, LV4_STRING_HOME_BASE, LV4_STRING_LIMIT);
  for (size_t i = 0; i < yyjson_arr_size(rows); i++) {
    yyjson_val* ins = yyjson_obj_get(yyjson_arr_get(rows, i), "instruction");
    if (!lv4_is(ins, "op", "phi")) break;
    hako_lv4_value* dest = lv4_index_value(fn, lv4_u(ins, "dst"));
    if (!dest || dest->kind != LV4_TEXT_OPERAND) continue;
    hako_lv4_value* input = lv4_text_phi_input(fn, ins, predecessor);
    if (!lv4_text_readable(fn, input, before)) return 0;
    int source = lv4_text_origin(fn, input), overwritten = !before[dest->origin];
    if (source >= 0 && ((size_t)source >= count || (before[source] & LV4_STRING_REPLACEMENT))) return 0;
    for (size_t j = 0; j < yyjson_arr_size(rows); j++) {
      yyjson_val* other = yyjson_obj_get(yyjson_arr_get(rows, j), "instruction");
      if (!lv4_is(other, "op", "phi")) break;
      hako_lv4_value* other_dest = lv4_index_value(fn, lv4_u(other, "dst"));
      if (!other_dest || other_dest->kind != LV4_TEXT_OPERAND) continue;
      int from = lv4_text_origin(fn, lv4_text_phi_input(fn, other, predecessor));
      if (j < i && source >= 0 && from == source) return 0;
      if (from == dest->origin) overwritten = 1;
    }
    if (!overwritten) return 0;
  }
  for (size_t i = 0; i < yyjson_arr_size(rows); i++) {
    yyjson_val* ins = yyjson_obj_get(yyjson_arr_get(rows, i), "instruction");
    if (!lv4_is(ins, "op", "phi")) break;
    hako_lv4_value* dest = lv4_index_value(fn, lv4_u(ins, "dst"));
    if (!dest || dest->kind != LV4_TEXT_OPERAND) continue;
    int source = lv4_text_origin(fn, lv4_text_phi_input(fn, ins, predecessor));
    if (source >= 0) after[source] = 0;
  }
  for (size_t i = 0; i < yyjson_arr_size(rows); i++) {
    yyjson_val* ins = yyjson_obj_get(yyjson_arr_get(rows, i), "instruction");
    if (!lv4_is(ins, "op", "phi")) break;
    hako_lv4_value* dest = lv4_index_value(fn, lv4_u(ins, "dst"));
    if (!dest || dest->kind != LV4_TEXT_OPERAND) continue;
    int source = lv4_text_origin(fn, lv4_text_phi_input(fn, ins, predecessor));
    hako_lv4_state state;
    if (source < 0) {
      state = ++reserved_top;
      if (state >= LV4_STRING_LIMIT) return 0;
      state |= LV4_STRING_STATIC;
    } else {
      state = before[source];
      if (lv4_string_rank(state) <= LV4_STRING_HOME_BASE || lv4_string_rank(state) >= LV4_STRING_LIMIT) return 0;
      if (!(state & LV4_STRING_POSSIBLE)) state |= LV4_STRING_OWNED;
    }
    after[dest->origin] = state;
  }
  return 1;
}

/* A join of the same sole live Home at two acquisition ranks (a replacement
 * End that succeeded versus faulted) orders nothing: no other rank exists. */
static int lv4_sole_home_rank_join(const hako_lv4_state* saved, const hako_lv4_state* state,
    size_t count, size_t slot) {
  if (slot >= count) return 0;
  hako_lv4_state a = lv4_string_rank(saved[slot]), b = lv4_string_rank(state[slot]);
  if (a <= LV4_STRING_HOME_BASE || a >= LV4_STRING_LIMIT || b <= LV4_STRING_HOME_BASE || b >= LV4_STRING_LIMIT ||
      (saved[slot] ^ a) != (state[slot] ^ b)) return 0;
  for (size_t j = 0; j < count; j++) {
    hako_lv4_state other = lv4_string_rank(saved[j]);
    if (j == slot) continue;
    if (saved[j] != state[j] || (other > LV4_STRING_RANK_BASE && other < LV4_STRING_LIMIT)) return 0;
  }
  return 1;
}

static int lv4_indexed_deliver(hako_lv4_worklist* work, size_t target,
    const hako_lv4_state* state, int faulted) {
  if (target >= work->count || target == work->entry) return 0;
  hako_lv4_state* saved = work->states + target * work->stride;
  if (work->faults[target] >= 0) {
    if (work->faults[target] != faulted) return 0;
    int changed = 0;
    for (size_t i = 0; i < work->stride; i++) {
      if (saved[i] == state[i]) continue;
      if (lv4_sole_home_rank_join(saved, state, work->count, i)) continue;
      if (i < work->count || !(saved[i] & LV4_STRING_POSSIBLE) || !(state[i] & LV4_STRING_POSSIBLE) ||
          ((saved[i] ^ state[i]) & ~LV4_STRING_POSSIBLE)) return 0;
      hako_lv4_state joined = saved[i] | state[i];
      changed |= joined != saved[i]; saved[i] = joined;
    }
    return !changed || lv4_work_enqueue(work, target);
  }
  memcpy(saved, state, work->stride * sizeof(*state));
  work->faults[target] = (signed char)faulted;
  return lv4_work_enqueue(work, target);
}
