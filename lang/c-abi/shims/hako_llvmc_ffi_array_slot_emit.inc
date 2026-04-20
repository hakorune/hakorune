      enum HakoLlvmcArraySlotEmitDemand {
        HAKO_LLVMC_ARRAY_SLOT_DEMAND_READ_HANDLE = 0,
        HAKO_LLVMC_ARRAY_SLOT_DEMAND_READ_TEXT_REF,
        HAKO_LLVMC_ARRAY_SLOT_DEMAND_READ_TEXT_LEN,
        HAKO_LLVMC_ARRAY_SLOT_DEMAND_READ_TEXT_INDEXOF,
        HAKO_LLVMC_ARRAY_SLOT_DEMAND_STORE_I64,
        HAKO_LLVMC_ARRAY_SLOT_DEMAND_STORE_HANDLE,
        HAKO_LLVMC_ARRAY_SLOT_DEMAND_STORE_TEXT_PUBLIC,
        HAKO_LLVMC_ARRAY_SLOT_DEMAND_STORE_TEXT_KERNEL,
      };

      auto void hako_llvmc_emit_i64_call_result(
          FILE* f,
          const char* symbol,
          long long dst,
          const char* args) {
        if (dst) {
          fprintf(f, "  %%r%lld = call i64 @\"%s\"(%s)\n", dst, symbol, args);
          set_type(dst, T_I64);
        } else {
          fprintf(f, "  call i64 @\"%s\"(%s)\n", symbol, args);
        }
      }

      auto void hako_llvmc_emit_array_slot_load(
          FILE* f,
          long long dst,
          const char* args,
          enum HakoLlvmcArraySlotEmitDemand demand) {
        hako_llvmc_emit_i64_call_result(f, "nyash.array.slot_load_hi", dst, args);
        if (dst && demand == HAKO_LLVMC_ARRAY_SLOT_DEMAND_READ_TEXT_REF) {
          set_origin(dst, ORG_STRING);
          set_scan_origin(dst, ORG_STRING);
        }
      }

      auto void hako_llvmc_emit_array_string_len(
          FILE* f,
          long long dst,
          const char* args) {
        hako_llvmc_emit_i64_call_result(f, "nyash.array.string_len_hi", dst, args);
      }

      auto void hako_llvmc_emit_array_string_indexof(
          FILE* f,
          long long dst,
          const char* args) {
        hako_llvmc_emit_i64_call_result(f, "nyash.array.string_indexof_hih", dst, args);
      }

      auto void hako_llvmc_emit_array_string_indexof_const_utf8(
          FILE* f,
          long long dst,
          const char* args) {
        hako_llvmc_emit_i64_call_result(f, "nyash.array.string_indexof_hisi", dst, args);
      }

      auto void hako_llvmc_emit_array_slot_store(
          FILE* f,
          long long dst,
          const char* args,
          enum HakoLlvmcArraySlotEmitDemand demand) {
        const char* symbol = "nyash.array.slot_store_hih";
        switch (demand) {
          case HAKO_LLVMC_ARRAY_SLOT_DEMAND_STORE_I64:
            symbol = "nyash.array.slot_store_hii";
            break;
          case HAKO_LLVMC_ARRAY_SLOT_DEMAND_STORE_TEXT_PUBLIC:
            symbol = "nyash.array.set_his";
            break;
          case HAKO_LLVMC_ARRAY_SLOT_DEMAND_STORE_TEXT_KERNEL:
            symbol = "nyash.array.kernel_slot_store_hi";
            break;
          case HAKO_LLVMC_ARRAY_SLOT_DEMAND_STORE_HANDLE:
          default:
            symbol = "nyash.array.slot_store_hih";
            break;
        }
        hako_llvmc_emit_i64_call_result(f, symbol, dst, args);
      }
