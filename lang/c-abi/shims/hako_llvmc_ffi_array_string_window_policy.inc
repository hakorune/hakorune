      auto int hako_llvmc_window_array_text_read_enabled(
          const char* bname,
          int recv_org,
          int runtime_array_string) {
        return runtime_array_string ||
               (bname && !strcmp(bname, "RuntimeDataBox") &&
                recv_org == ORG_ARRAY_STRING_BIRTH);
      }

      auto int hako_llvmc_window_array_read_enabled(
          const char* bname,
          int recv_org,
          int runtime_array_get,
          int runtime_array_string) {
        return (bname && !strcmp(bname, "ArrayBox")) ||
               runtime_array_get ||
               runtime_array_string ||
               (bname && !strcmp(bname, "RuntimeDataBox") &&
                (recv_org == ORG_ARRAY_BIRTH ||
                 recv_org == ORG_ARRAY_STRING_BIRTH));
      }
