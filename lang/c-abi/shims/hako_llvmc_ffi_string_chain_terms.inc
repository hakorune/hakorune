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

      #endif
