      /*
       * Generic pure typed-object metadata helpers.
       *
       * These helpers publish compiler-local register facts and validate
       * TypedObjectPlan views. Prescan and lowering remain responsible for
       * declaration needs and IR emission.
       */
      auto void generic_pure_publish_typed_object_plan_index(
          long long reg,
          int typed_plan_index) {
        if (!(reg && typed_plan_index >= 0)) return;
        generic_pure_publish_i64_origin(reg, ORG_TYPED_OBJECT);
        set_typed_object_binding(reg, typed_plan_index);
      }

      auto int generic_pure_publish_typed_object_if_plan(
          long long reg,
          const char* box_name) {
        int typed_plan_index = -1;
        if (!(reg && box_name)) return 0;
        if (!find_typed_object_plan_index(box_name, &typed_plan_index)) return 0;
        generic_pure_publish_typed_object_plan_index(reg, typed_plan_index);
        return 1;
      }

      struct GenericPureTypedObjectFieldSlotView {
        int typed_plan_index;
        long long slot;
        const char* storage;
      };

      enum GenericPureTypedObjectNewPlanStatus {
        GENERIC_PURE_TYPED_OBJECT_NEW_PLAN_MISSING = 0,
        GENERIC_PURE_TYPED_OBJECT_NEW_PLAN_INVALID,
        GENERIC_PURE_TYPED_OBJECT_NEW_PLAN_OK,
      };

      struct GenericPureTypedObjectNewPlanView {
        int typed_plan_index;
        long long type_id;
        long long field_count;
      };

      auto enum GenericPureTypedObjectNewPlanStatus
      generic_pure_validate_typed_object_new_plan(
          long long dst,
          const char* box_name,
          struct GenericPureTypedObjectNewPlanView* out) {
        int typed_plan_index = -1;
        long long type_id = 0;
        long long field_count = 0;
        if (!(box_name && out)) {
          return GENERIC_PURE_TYPED_OBJECT_NEW_PLAN_MISSING;
        }
        if (!find_typed_object_plan_index(box_name, &typed_plan_index)) {
          return GENERIC_PURE_TYPED_OBJECT_NEW_PLAN_MISSING;
        }
        type_id = typed_object_plan_type_id(typed_plan_index);
        field_count = typed_object_plan_field_count(typed_plan_index);
        if (!(dst && type_id > 0 && field_count >= 0)) {
          return GENERIC_PURE_TYPED_OBJECT_NEW_PLAN_INVALID;
        }
        out->typed_plan_index = typed_plan_index;
        out->type_id = type_id;
        out->field_count = field_count;
        return GENERIC_PURE_TYPED_OBJECT_NEW_PLAN_OK;
      }

      auto int generic_pure_read_typed_object_field_slot(
          long long box_reg,
          const char* field,
          struct GenericPureTypedObjectFieldSlotView* out) {
        int typed_plan_index = -1;
        long long slot = -1;
        const char* storage = NULL;
        if (!(box_reg && field && out)) return 0;
        if (!get_typed_object_binding(box_reg, &typed_plan_index)) return 0;
        if (!typed_object_plan_field_runtime_slot_with_storage(
                typed_plan_index,
                field,
                &slot,
                &storage)) {
          return 0;
        }
        out->typed_plan_index = typed_plan_index;
        out->slot = slot;
        out->storage = storage;
        return 1;
      }
