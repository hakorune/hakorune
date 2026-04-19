      struct GenericPureNeedFlags {
        int map_birth;
        int map_set;
        int map_size;
        int map_get;
        int map_has;
        int arr_birth;
        int arr_push;
        int arr_len;
        int arr_set;
        int arr_get;
        int arr_slot_load;
        int arr_kernel_slot_concat_by_index;
        int arr_has;
        int arr_string_indexof;
        int string_const_box;
        int string_len;
        int string_substring;
        int string_substring_len;
        int string_indexof;
        int string_concat;
        int printf;
      };

      enum MirCallNeedKind {
        HAKO_LLVMC_MIR_CALL_NEED_NONE = 0,
        HAKO_LLVMC_MIR_CALL_NEED_MAP_BIRTH,
        HAKO_LLVMC_MIR_CALL_NEED_MAP_SET,
        HAKO_LLVMC_MIR_CALL_NEED_MAP_SIZE,
        HAKO_LLVMC_MIR_CALL_NEED_MAP_GET,
        HAKO_LLVMC_MIR_CALL_NEED_MAP_HAS,
        HAKO_LLVMC_MIR_CALL_NEED_ARRAY_BIRTH,
        HAKO_LLVMC_MIR_CALL_NEED_ARRAY_PUSH,
        HAKO_LLVMC_MIR_CALL_NEED_ARRAY_LEN,
        HAKO_LLVMC_MIR_CALL_NEED_ARRAY_SET,
        HAKO_LLVMC_MIR_CALL_NEED_ARRAY_GET,
        HAKO_LLVMC_MIR_CALL_NEED_ARRAY_HAS,
        HAKO_LLVMC_MIR_CALL_NEED_ARRAY_STRING_INDEXOF,
        HAKO_LLVMC_MIR_CALL_NEED_STRING_LEN,
        HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING,
        HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING_LEN,
        HAKO_LLVMC_MIR_CALL_NEED_STRING_INDEXOF,
        HAKO_LLVMC_MIR_CALL_NEED_STRING_CONCAT,
        HAKO_LLVMC_MIR_CALL_NEED_PRINTF,
      };

      auto void apply_mir_call_need_kind(
          struct GenericPureNeedFlags* needs,
          enum MirCallNeedKind kind) {
        if (!needs) return;
        switch (kind) {
          case HAKO_LLVMC_MIR_CALL_NEED_MAP_BIRTH:
            needs->map_birth = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_MAP_SET:
            needs->map_set = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_MAP_SIZE:
            needs->map_size = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_MAP_GET:
            needs->map_get = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_MAP_HAS:
            needs->map_has = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_ARRAY_BIRTH:
            needs->arr_birth = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_ARRAY_PUSH:
            needs->arr_push = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_ARRAY_LEN:
            needs->arr_len = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_ARRAY_SET:
            needs->arr_set = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_ARRAY_GET:
            needs->arr_get = 1;
            needs->arr_slot_load = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_ARRAY_HAS:
            needs->arr_has = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_ARRAY_STRING_INDEXOF:
            needs->arr_string_indexof = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_STRING_LEN:
            needs->string_len = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING:
            needs->string_substring = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING_LEN:
            needs->string_substring_len = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_STRING_INDEXOF:
            needs->string_indexof = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_STRING_CONCAT:
            needs->string_concat = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_PRINTF:
            needs->printf = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_NONE:
          default:
            break;
        }
      }

      auto enum MirCallNeedKind classify_mir_call_constructor_need_kind(
          const char* bname) {
        if (bname && !strcmp(bname, "MapBox")) {
          return HAKO_LLVMC_MIR_CALL_NEED_MAP_BIRTH;
        }
        if (bname && !strcmp(bname, "ArrayBox")) {
          return HAKO_LLVMC_MIR_CALL_NEED_ARRAY_BIRTH;
        }
        return HAKO_LLVMC_MIR_CALL_NEED_NONE;
      }

      auto enum MirCallNeedKind classify_mir_call_method_need_kind(
          const char* bname,
          const char* mname,
          enum MirCallReceiverFamilyKind recv_family) {
        if (!(bname && mname)) return HAKO_LLVMC_MIR_CALL_NEED_NONE;

        if (!strcmp(bname, "MapBox")) {
          if (!strcmp(mname, "set")) return HAKO_LLVMC_MIR_CALL_NEED_MAP_SET;
          if (!strcmp(mname, "size") || !strcmp(mname, "len") || !strcmp(mname, "length")) {
            return HAKO_LLVMC_MIR_CALL_NEED_MAP_SIZE;
          }
          if (!strcmp(mname, "get")) return HAKO_LLVMC_MIR_CALL_NEED_MAP_GET;
          if (!strcmp(mname, "has")) return HAKO_LLVMC_MIR_CALL_NEED_MAP_HAS;
          return HAKO_LLVMC_MIR_CALL_NEED_NONE;
        }

        if (!strcmp(bname, "ArrayBox")) {
          if (!strcmp(mname, "push")) return HAKO_LLVMC_MIR_CALL_NEED_ARRAY_PUSH;
          if (!strcmp(mname, "len") || !strcmp(mname, "length") || !strcmp(mname, "size")) {
            return HAKO_LLVMC_MIR_CALL_NEED_ARRAY_LEN;
          }
          if (!strcmp(mname, "set")) return HAKO_LLVMC_MIR_CALL_NEED_ARRAY_SET;
          if (!strcmp(mname, "get")) return HAKO_LLVMC_MIR_CALL_NEED_ARRAY_GET;
          return HAKO_LLVMC_MIR_CALL_NEED_NONE;
        }

        if (!strcmp(bname, "RuntimeDataBox")) {
          if (recv_family == HAKO_LLVMC_MIR_CALL_RECV_FAMILY_ARRAY_BIRTH) {
            if (!strcmp(mname, "get")) return HAKO_LLVMC_MIR_CALL_NEED_ARRAY_GET;
            if (!strcmp(mname, "push")) return HAKO_LLVMC_MIR_CALL_NEED_ARRAY_PUSH;
            if (!strcmp(mname, "len") || !strcmp(mname, "length") || !strcmp(mname, "size")) {
              return HAKO_LLVMC_MIR_CALL_NEED_ARRAY_LEN;
            }
            if (!strcmp(mname, "has")) return HAKO_LLVMC_MIR_CALL_NEED_ARRAY_HAS;
            if (!strcmp(mname, "set")) return HAKO_LLVMC_MIR_CALL_NEED_ARRAY_SET;
            return HAKO_LLVMC_MIR_CALL_NEED_NONE;
          }

          if (recv_family == HAKO_LLVMC_MIR_CALL_RECV_FAMILY_ARRAY_STRING_BIRTH) {
            if (!strcmp(mname, "get")) return HAKO_LLVMC_MIR_CALL_NEED_ARRAY_STRING_INDEXOF;
            if (!strcmp(mname, "push")) return HAKO_LLVMC_MIR_CALL_NEED_ARRAY_PUSH;
            if (!strcmp(mname, "len") || !strcmp(mname, "length") || !strcmp(mname, "size")) {
              return HAKO_LLVMC_MIR_CALL_NEED_ARRAY_LEN;
            }
            if (!strcmp(mname, "has")) return HAKO_LLVMC_MIR_CALL_NEED_ARRAY_HAS;
            if (!strcmp(mname, "set")) return HAKO_LLVMC_MIR_CALL_NEED_ARRAY_SET;
            return HAKO_LLVMC_MIR_CALL_NEED_NONE;
          }

          if (recv_family == HAKO_LLVMC_MIR_CALL_RECV_FAMILY_MAP_BIRTH) {
            if (!strcmp(mname, "get")) return HAKO_LLVMC_MIR_CALL_NEED_MAP_GET;
            if (!strcmp(mname, "size") || !strcmp(mname, "len") || !strcmp(mname, "length")) {
              return HAKO_LLVMC_MIR_CALL_NEED_MAP_SIZE;
            }
            if (!strcmp(mname, "has")) return HAKO_LLVMC_MIR_CALL_NEED_MAP_HAS;
            if (!strcmp(mname, "set")) return HAKO_LLVMC_MIR_CALL_NEED_MAP_SET;
            return HAKO_LLVMC_MIR_CALL_NEED_NONE;
          }

          if (recv_family == HAKO_LLVMC_MIR_CALL_RECV_FAMILY_STRING) {
            if (!strcmp(mname, "len") || !strcmp(mname, "length") || !strcmp(mname, "size")) {
              return HAKO_LLVMC_MIR_CALL_NEED_STRING_LEN;
            }
            if (!strcmp(mname, "substring")) return HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING;
            if (!strcmp(mname, "indexOf")) return HAKO_LLVMC_MIR_CALL_NEED_STRING_INDEXOF;
            return HAKO_LLVMC_MIR_CALL_NEED_NONE;
          }

          return HAKO_LLVMC_MIR_CALL_NEED_NONE;
        }

        if (!strcmp(bname, "StringBox")) {
          if (!strcmp(mname, "len") || !strcmp(mname, "length") || !strcmp(mname, "size")) {
            return HAKO_LLVMC_MIR_CALL_NEED_STRING_LEN;
          }
          if (!strcmp(mname, "substring")) return HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING;
          if (!strcmp(mname, "indexOf")) return HAKO_LLVMC_MIR_CALL_NEED_STRING_INDEXOF;
        }

        return HAKO_LLVMC_MIR_CALL_NEED_NONE;
      }

      auto enum MirCallNeedKind classify_mir_call_global_need_kind(
          const char* mname) {
        if (mname && !strcmp(mname, "print")) {
          return HAKO_LLVMC_MIR_CALL_NEED_PRINTF;
        }
        return HAKO_LLVMC_MIR_CALL_NEED_NONE;
      }

      auto enum MirCallNeedKind classify_mir_call_extern_need_kind(
          const char* mname) {
        if (mname &&
            (!strcmp(mname, "nyash.string.concat_hh") ||
             !strcmp(mname, "nyash.string.concat3_hhh") ||
             !strcmp(mname, "nyash.string.insert_hsi"))) {
          return HAKO_LLVMC_MIR_CALL_NEED_STRING_CONCAT;
        }
        if (mname &&
            !strcmp(mname, "nyash.string.substring_hii")) {
          return HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING;
        }
        if (mname &&
            (!strcmp(mname, "nyash.string.substring_concat_hhii") ||
             !strcmp(mname, "nyash.string.substring_concat3_hhhii"))) {
          return HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING;
        }
        if (mname && !strcmp(mname, "nyash.string.substring_len_hii")) {
          return HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING_LEN;
        }
        return HAKO_LLVMC_MIR_CALL_NEED_NONE;
      }
