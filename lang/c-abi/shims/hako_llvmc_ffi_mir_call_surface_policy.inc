      enum MirCallConstructorSurfaceKind {
        HAKO_LLVMC_MIR_CALL_CONSTRUCTOR_SURFACE_NONE = 0,
        HAKO_LLVMC_MIR_CALL_CONSTRUCTOR_SURFACE_MAP_BOX,
        HAKO_LLVMC_MIR_CALL_CONSTRUCTOR_SURFACE_ARRAY_BOX,
      };

      enum MirCallGlobalSurfaceKind {
        HAKO_LLVMC_MIR_CALL_GLOBAL_SURFACE_NONE = 0,
        HAKO_LLVMC_MIR_CALL_GLOBAL_SURFACE_PRINT,
      };

      enum MirCallStringExternSurfaceKind {
        HAKO_LLVMC_MIR_CALL_STRING_EXTERN_SURFACE_NONE = 0,
        HAKO_LLVMC_MIR_CALL_STRING_EXTERN_SURFACE_CONCAT_HH,
        HAKO_LLVMC_MIR_CALL_STRING_EXTERN_SURFACE_CONCAT3_HHH,
      };

      auto enum MirCallConstructorSurfaceKind
      classify_mir_call_constructor_surface(const char* bname) {
        if (bname && !strcmp(bname, "MapBox")) {
          return HAKO_LLVMC_MIR_CALL_CONSTRUCTOR_SURFACE_MAP_BOX;
        }
        if (bname && !strcmp(bname, "ArrayBox")) {
          return HAKO_LLVMC_MIR_CALL_CONSTRUCTOR_SURFACE_ARRAY_BOX;
        }
        return HAKO_LLVMC_MIR_CALL_CONSTRUCTOR_SURFACE_NONE;
      }

      auto enum MirCallGlobalSurfaceKind classify_mir_call_global_surface(
          const char* mname) {
        if (mname && !strcmp(mname, "print")) {
          return HAKO_LLVMC_MIR_CALL_GLOBAL_SURFACE_PRINT;
        }
        return HAKO_LLVMC_MIR_CALL_GLOBAL_SURFACE_NONE;
      }

      auto enum MirCallStringExternSurfaceKind
      classify_mir_call_string_extern_surface(const char* mname) {
        if (mname && !strcmp(mname, "nyash.string.concat_hh")) {
          return HAKO_LLVMC_MIR_CALL_STRING_EXTERN_SURFACE_CONCAT_HH;
        }
        if (mname && !strcmp(mname, "nyash.string.concat3_hhh")) {
          return HAKO_LLVMC_MIR_CALL_STRING_EXTERN_SURFACE_CONCAT3_HHH;
        }
        return HAKO_LLVMC_MIR_CALL_STRING_EXTERN_SURFACE_NONE;
      }
