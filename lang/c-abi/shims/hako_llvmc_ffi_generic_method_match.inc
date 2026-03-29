      enum GenericMethodEmitKind {
        HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE = 0,
        HAKO_LLVMC_GENERIC_METHOD_EMIT_BIRTH_ZERO,
        HAKO_LLVMC_GENERIC_METHOD_EMIT_SET,
        HAKO_LLVMC_GENERIC_METHOD_EMIT_GET,
        HAKO_LLVMC_GENERIC_METHOD_EMIT_LEN,
        HAKO_LLVMC_GENERIC_METHOD_EMIT_PUSH,
        HAKO_LLVMC_GENERIC_METHOD_EMIT_SUBSTRING,
        HAKO_LLVMC_GENERIC_METHOD_EMIT_HAS,
      };

      auto enum GenericMethodEmitKind classify_generic_method_emit_kind(
          const char* bname,
          const char* mname,
          const struct GenericMethodRouteState* route,
          long long a0,
          long long a1) {
        (void)route;
        if (mname && !strcmp(mname, "birth") && !a0 && !a1) {
          if ((bname && !strcmp(bname, "ArrayBox")) || (bname && !strcmp(bname, "MapBox"))) {
            return HAKO_LLVMC_GENERIC_METHOD_EMIT_BIRTH_ZERO;
          }
        }
        if (mname && !strcmp(mname, "set")) return HAKO_LLVMC_GENERIC_METHOD_EMIT_SET;
        if (mname && !strcmp(mname, "get")) return HAKO_LLVMC_GENERIC_METHOD_EMIT_GET;
        if (mname && (!strcmp(mname, "len") || !strcmp(mname, "length") || !strcmp(mname, "size"))) {
          return HAKO_LLVMC_GENERIC_METHOD_EMIT_LEN;
        }
        if (mname && !strcmp(mname, "push")) return HAKO_LLVMC_GENERIC_METHOD_EMIT_PUSH;
        if (mname && !strcmp(mname, "substring")) return HAKO_LLVMC_GENERIC_METHOD_EMIT_SUBSTRING;
        if (mname && !strcmp(mname, "has")) return HAKO_LLVMC_GENERIC_METHOD_EMIT_HAS;
        return HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE;
      }
