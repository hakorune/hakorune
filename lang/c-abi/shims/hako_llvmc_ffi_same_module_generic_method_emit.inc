/*
 * Same-module generic method body emit.
 *
 * This file now dispatches into smaller method-family includes. The
 * behavior stays the same; the responsibility is just split for readability.
 */
#include "hako_llvmc_ffi_same_module_generic_method_string_emit.inc"
#include "hako_llvmc_ffi_same_module_generic_method_collection_emit.inc"

      auto int same_module_function_emit_generic_method_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          const struct LoweringPlanGenericMethodView* method_view,
          long long dst,
          long long recv,
          yyjson_val* args) {
        int rc = same_module_function_emit_method_len_mir_call(
            f,
            bid,
            ii,
            method_view,
            dst,
            recv);
        if (rc != 1) {
          rc = same_module_function_emit_method_push_mir_call(
              f,
              bid,
              ii,
              method_view,
              dst,
              recv,
              args);
        }
        if (rc != 1) {
          rc = same_module_function_emit_method_get_mir_call(
              f,
              bid,
              ii,
              method_view,
              dst,
              recv,
              args);
        }
        if (rc != 1) {
          rc = same_module_function_emit_method_keys_mir_call(
              f,
              bid,
              ii,
              method_view,
              dst,
              recv,
              args);
        }
        if (rc != 1) {
          rc = same_module_function_emit_method_set_mir_call(
              f,
              bid,
              ii,
              method_view,
              dst,
              recv,
              args);
        }
        if (rc != 1) {
          rc = same_module_function_emit_method_has_mir_call(
              f,
              bid,
              ii,
              method_view,
              dst,
              recv,
              args);
        }
        if (rc != 1) {
          rc = same_module_function_emit_method_delete_mir_call(
              f,
              bid,
              ii,
              method_view,
              dst,
              recv,
              args);
        }
        if (rc != 1) {
          rc = same_module_function_emit_method_substring_mir_call(
              f,
              bid,
              ii,
              method_view,
              dst,
              recv,
              args);
        }
        if (rc != 1) {
          rc = same_module_function_emit_method_indexof_mir_call(
              f,
              bid,
              ii,
              method_view,
              dst,
              recv,
              args);
        }
        if (rc != 1) {
          rc = same_module_function_emit_method_lastindexof_mir_call(
              f,
              bid,
              ii,
              method_view,
              dst,
              recv,
              args);
        }
        if (rc != 1) {
          rc = same_module_function_emit_method_contains_mir_call(
              f,
              bid,
              ii,
              method_view,
              dst,
              recv,
              args);
        }
        return rc;
      }
