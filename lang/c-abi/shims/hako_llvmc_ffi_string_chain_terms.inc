      #ifndef HAKO_LLVMC_STRING_CHAIN_TERMS_INC
      #define HAKO_LLVMC_STRING_CHAIN_TERMS_INC

      enum StringConcatPairRouteKind {
        HAKO_LLVMC_STRING_CONCAT_PAIR_ROUTE_DEFER_PAIR = 1,
        HAKO_LLVMC_STRING_CONCAT_PAIR_ROUTE_CONST_SUFFIX = 2,
        HAKO_LLVMC_STRING_CONCAT_PAIR_ROUTE_INSERT_MID_CONST = 3,
        HAKO_LLVMC_STRING_CONCAT_PAIR_ROUTE_CONCAT3_CHAIN = 4,
        HAKO_LLVMC_STRING_CONCAT_PAIR_ROUTE_CONCAT_HH = 5
      };

      enum StringConcat3RouteKind {
        HAKO_LLVMC_STRING_CONCAT3_ROUTE_CHAIN = 1,
        HAKO_LLVMC_STRING_CONCAT3_ROUTE_FALLBACK = 2
      };

      enum StringSubstringRetainedFormKind {
        HAKO_LLVMC_STRING_SUBSTRING_RETAIN_KEEP_TRANSIENT = 0,
        HAKO_LLVMC_STRING_SUBSTRING_RETAIN_RETAIN_VIEW = 1,
        HAKO_LLVMC_STRING_SUBSTRING_RETAIN_NEED_OWNED_HANDLE = 2
      };

      enum StringPostStoreUseKind {
        HAKO_LLVMC_STRING_POST_STORE_USE_NONE = 0,
        HAKO_LLVMC_STRING_POST_STORE_USE_LEN_OBSERVER = 1
      };

      auto const char* string_boundary_kind_store_name(void) {
        return "Store";
      }

      auto const char* string_post_store_use_name(
          enum StringPostStoreUseKind kind) {
        switch (kind) {
          case HAKO_LLVMC_STRING_POST_STORE_USE_LEN_OBSERVER:
            return "LenObserver";
          case HAKO_LLVMC_STRING_POST_STORE_USE_NONE:
          default:
            return "None";
        }
      }

      auto enum StringSubstringRetainedFormKind classify_string_substring_retained_form(
          int is_short_slice,
          int boundary_is_store,
          int view_allowed) {
        if (boundary_is_store && is_short_slice) {
          return HAKO_LLVMC_STRING_SUBSTRING_RETAIN_NEED_OWNED_HANDLE;
        }
        if (view_allowed) {
          return HAKO_LLVMC_STRING_SUBSTRING_RETAIN_RETAIN_VIEW;
        }
        return HAKO_LLVMC_STRING_SUBSTRING_RETAIN_KEEP_TRANSIENT;
      }

      auto const char* string_substring_retained_form_name(
          enum StringSubstringRetainedFormKind kind) {
        switch (kind) {
          case HAKO_LLVMC_STRING_SUBSTRING_RETAIN_RETAIN_VIEW:
            return "RetainView";
          case HAKO_LLVMC_STRING_SUBSTRING_RETAIN_NEED_OWNED_HANDLE:
            return "NeedOwnedHandle";
          case HAKO_LLVMC_STRING_SUBSTRING_RETAIN_KEEP_TRANSIENT:
          default:
            return "KeepTransient";
        }
      }

      auto yyjson_val* hako_llvmc_string_corridor_value_candidates(long long value) {
        char key[32];
        yyjson_val* candidates = NULL;
        if (!(fn_metadata && value)) return NULL;
        value = resolve_copy_source_reg(value);
        snprintf(key, sizeof(key), "%lld", value);
        candidates = yyjson_obj_get(fn_metadata, "string_corridor_candidates");
        if (!(candidates && yyjson_is_obj(candidates))) return NULL;
        candidates = yyjson_obj_get(candidates, key);
        if (!(candidates && yyjson_is_arr(candidates))) return NULL;
        return candidates;
      }

      auto int hako_llvmc_string_corridor_has_candidate(
          long long value,
          const char* kind) {
        yyjson_val* candidates = hako_llvmc_string_corridor_value_candidates(value);
        size_t count = candidates ? yyjson_arr_size(candidates) : 0;
        if (!(candidates && kind)) return 0;
        for (size_t i = 0; i < count; i++) {
          yyjson_val* candidate = yyjson_arr_get(candidates, i);
          const char* candidate_kind =
              candidate ? yyjson_get_str(yyjson_obj_get(candidate, "kind")) : NULL;
          if (candidate_kind && !strcmp(candidate_kind, kind)) {
            return 1;
          }
        }
        return 0;
      }

      auto yyjson_val* hako_llvmc_string_corridor_candidate_by_kind(
          long long value,
          const char* kind) {
        yyjson_val* candidates = hako_llvmc_string_corridor_value_candidates(value);
        size_t count = candidates ? yyjson_arr_size(candidates) : 0;
        if (!(candidates && kind)) return NULL;
        for (size_t i = 0; i < count; i++) {
          yyjson_val* candidate = yyjson_arr_get(candidates, i);
          const char* candidate_kind =
              candidate ? yyjson_get_str(yyjson_obj_get(candidate, "kind")) : NULL;
          if (candidate_kind && !strcmp(candidate_kind, kind)) {
            return candidate;
          }
        }
        return NULL;
      }

      auto yyjson_val* hako_llvmc_string_corridor_candidate_plan_by_kind(
          long long value,
          const char* kind) {
        yyjson_val* candidate =
            hako_llvmc_string_corridor_candidate_by_kind(value, kind);
        yyjson_val* plan = candidate ? yyjson_obj_get(candidate, "plan") : NULL;
        if (!(plan && yyjson_is_obj(plan))) return NULL;
        return plan;
      }

      auto int hako_llvmc_string_corridor_read_plan_window(
          long long value,
          const char* kind,
          long long* start,
          long long* end) {
        yyjson_val* plan =
            hako_llvmc_string_corridor_candidate_plan_by_kind(value, kind);
        yyjson_val* start_val = plan ? yyjson_obj_get(plan, "start") : NULL;
        yyjson_val* end_val = plan ? yyjson_obj_get(plan, "end") : NULL;
        if (!(start_val && end_val && yyjson_is_num(start_val) && yyjson_is_num(end_val))) {
          return 0;
        }
        if (start) *start = (long long)yyjson_get_sint(start_val);
        if (end) *end = (long long)yyjson_get_sint(end_val);
        return 1;
      }

      #endif
