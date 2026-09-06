#[test]
fn published_array_element_writes_are_typed_rows_for_all_four_kinds() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "array-writes/0".to_owned(),
            params: Vec::new(),
            return_type: MirType::Void,
            effects: EffectMask::WRITE,
        },
        BasicBlockId::new(0),
    );
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry block");
    for (site_id, kind, index) in [
        (1, crate::mir::ArrayElementWriteKind::LiteralAppend, None),
        (2, crate::mir::ArrayElementWriteKind::Push, None),
        (
            3,
            crate::mir::ArrayElementWriteKind::Set,
            Some(ValueId::new(2)),
        ),
        (
            4,
            crate::mir::ArrayElementWriteKind::Insert,
            Some(ValueId::new(3)),
        ),
    ] {
        block.add_instruction(
            crate::mir::array_element_write::instruction(
                crate::mir::ArrayWriteSiteId::new(site_id),
                None,
                kind,
                crate::mir::ArrayWriteProducerKind::MethodCall,
                ValueId::new(1),
                index,
                ValueId::new(4),
            )
            .expect("valid array write"),
        );
    }
    let mut module = MirModule::new("array-write-view".to_owned());
    module.add_function(function);

    let view = PublishedMirBackendView::try_new(&module).expect("typed array-write view");
    assert_eq!(view.route(), PublishedStaticMethodRouteV1::CanonicalTyped);
    let writes = view.array_element_writes();
    assert_eq!(writes.len(), 4);
    assert_eq!(
        writes[0].kind(),
        crate::mir::ArrayElementWriteKind::LiteralAppend
    );
    assert_eq!(writes[1].kind(), crate::mir::ArrayElementWriteKind::Push);
    assert_eq!(writes[2].kind(), crate::mir::ArrayElementWriteKind::Set);
    assert_eq!(writes[2].index(), Some(ValueId::new(2)));
    assert_eq!(writes[3].kind(), crate::mir::ArrayElementWriteKind::Insert);
    assert_eq!(writes[3].receiver(), ValueId::new(1));
    assert_eq!(writes[3].value(), ValueId::new(4));

    let frame = PublishedStaticMethodCFrameV1::from_view(&view).expect("array-write C frame");
    assert_eq!(frame.len(), 4);
    assert_eq!(
        frame.as_slice()[0].kind,
        PublishedCallKindV1::ArrayLiteralAppend as u32
    );
    assert_eq!(
        frame.as_slice()[1].kind,
        PublishedCallKindV1::ArrayPush as u32
    );
    assert_eq!(
        frame.as_slice()[2].kind,
        PublishedCallKindV1::ArraySet as u32
    );
    assert_eq!(
        frame.as_slice()[3].kind,
        PublishedCallKindV1::ArrayInsert as u32
    );
    assert_eq!(frame.as_slice()[2].receiver, 1);
    assert_eq!(frame.as_slice()[2].index, 2);
    assert_eq!(frame.as_slice()[2].value, 4);
    assert_eq!(frame.as_slice()[2].flags, 2);
    assert!(frame.as_slice().iter().all(|row| row.dst == 0));
    assert!(frame.as_slice().iter().all(|row| row.flags & 1 == 0));
}

#[test]
fn published_array_element_writes_reject_invalid_kind_index_shape() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "array-write-invalid/0".to_owned(),
            params: Vec::new(),
            return_type: MirType::Void,
            effects: EffectMask::WRITE,
        },
        BasicBlockId::new(0),
    );
    function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry block")
        .add_instruction(MirInstruction::ArrayElementWrite {
            site_id: crate::mir::ArrayWriteSiteId::new(1),
            dst: None,
            kind: crate::mir::ArrayElementWriteKind::Set,
            producer: crate::mir::ArrayWriteProducerKind::IndexAssignment,
            receiver: ValueId::new(1),
            index: None,
            value: ValueId::new(2),
        });
    let mut module = MirModule::new("array-write-invalid".to_owned());
    module.add_function(function);
    assert!(matches!(
        PublishedMirBackendView::try_new(&module).unwrap_err(),
        PublishedMirBackendViewErrorV1::ArrayElementWriteShapeMismatch { .. }
    ));
}

#[test]
fn published_array_write_typed_contract_rejects_before_object() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "typed-array-write/1".to_owned(),
            params: vec![MirType::Unknown],
            return_type: MirType::Void,
            effects: EffectMask::WRITE,
        },
        BasicBlockId::new(0),
    );
    function.metadata.declared_param_decls = vec![crate::mir::function::MirParamDecl {
        name: "bytes".to_owned(),
        declared_type_name: Some("Array<u8>".to_owned()),
        implicit_receiver: false,
    }];
    function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry block")
        .add_instruction(
            crate::mir::array_element_write::instruction(
                crate::mir::ArrayWriteSiteId::new(1),
                None,
                crate::mir::ArrayElementWriteKind::Push,
                crate::mir::ArrayWriteProducerKind::MethodCall,
                ValueId::new(1),
                None,
                ValueId::new(2),
            )
            .expect("valid array write"),
        );
    let mut module = MirModule::new("typed-array-write".to_owned());
    module.add_function(function);
    crate::mir::semantic_refresh::refresh_and_validate_for_boundary(
        &mut module,
        crate::mir::ContractRefreshBoundary::Verifier,
    )
    .expect("complete input contracts before publishing the borrowed view");

    let output = std::env::temp_dir().join("hakorune-typed-array-write-reject.o");
    let _ = std::fs::remove_file(&output);
    let error = crate::host_providers::llvm_codegen::try_compile_published_static_method_object(
        &module,
        output.to_str().expect("temporary path is UTF-8"),
    )
    .expect_err("typed array must reject before object transport");
    assert!(error.contains(crate::mir::typed_array_backend_capability::BACKEND_UNSUPPORTED_TAG));
    assert!(
        !output.exists(),
        "rejected module must not create an object"
    );

    let exe_output = std::env::temp_dir().join("hakorune-typed-array-write-reject");
    let _ = std::fs::remove_file(&exe_output);
    let error = crate::host_providers::llvm_codegen::emit_published_static_method_exe(
        &module,
        exe_output.to_str().expect("temporary path is UTF-8"),
        None,
        None,
    )
    .expect_err("typed array must reject before executable transport");
    assert!(error.contains(crate::mir::typed_array_backend_capability::BACKEND_UNSUPPORTED_TAG));
    assert!(
        !exe_output.exists(),
        "rejected module must not create an executable"
    );
}

#[test]
fn published_array_element_writes_compile_object_without_void_result_leak() {
    std::thread::Builder::new()
        .name("published-array-write-object".to_owned())
        .stack_size(32 * 1024 * 1024)
        .spawn(|| {
            compile_array_element_writes_object_on_large_stack();
        })
        .expect("spawn large-stack object compile test")
        .join()
        .expect("large-stack object compile test");
}

fn compile_array_element_writes_object_on_large_stack() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "array-write-object/0".to_owned(),
            params: Vec::new(),
            // The selected-C generic emitter currently materializes every
            // function as i64.  Keep the write operations themselves
            // destination-free (their logical result is Void), while using
            // local constants for the physical operands.
            return_type: MirType::Integer,
            effects: EffectMask::WRITE,
        },
        BasicBlockId::new(0),
    );
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry block");
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "ArrayBox".to_owned(),
        args: Vec::new(),
    });
    for (dst, value) in [(2, 1), (3, 9), (4, 0)] {
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(dst),
            value: crate::mir::ConstValue::Integer(value),
        });
    }
    for (site_id, kind, index) in [
        (11, crate::mir::ArrayElementWriteKind::LiteralAppend, None),
        (12, crate::mir::ArrayElementWriteKind::Push, None),
        (
            13,
            crate::mir::ArrayElementWriteKind::Set,
            Some(ValueId::new(2)),
        ),
        (
            14,
            crate::mir::ArrayElementWriteKind::Insert,
            Some(ValueId::new(2)),
        ),
    ] {
        block.add_instruction(
            crate::mir::array_element_write::instruction(
                crate::mir::ArrayWriteSiteId::new(site_id),
                None,
                kind,
                crate::mir::ArrayWriteProducerKind::IndexAssignment,
                ValueId::new(1),
                index,
                ValueId::new(3),
            )
            .expect("valid array write"),
        );
    }
    block.add_instruction(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });

    let mut module = MirModule::new("array-write-object".to_owned());
    module.add_function(function);
    crate::mir::semantic_refresh::refresh_and_validate_for_boundary(
        &mut module,
        crate::mir::ContractRefreshBoundary::Verifier,
    )
    .expect("complete input contracts before publishing the borrowed view");
    let out = std::env::temp_dir().join(format!(
        "hakorune-array-write-object-{}.o",
        std::process::id()
    ));
    let result = crate::host_providers::llvm_codegen::try_compile_published_static_method_object(
        &module,
        out.to_str().expect("temporary path is valid UTF-8"),
    );
    assert_eq!(
        result,
        Ok(true),
        "typed array object compile failed: {result:?}"
    );
    assert!(out.exists(), "typed array object was not emitted");
    assert!(std::fs::metadata(&out).expect("object metadata").len() > 0);
    let _ = std::fs::remove_file(out);

    // The same published rows must also own the executable path.  The
    // release runtime/FFI archive is a build prerequisite for this optional
    // local smoke; object generation above remains the always-on focused gate.
    let runtime_archive = std::path::Path::new("target/release/libnyash_kernel.a");
    let ffi_library = std::path::Path::new("target/release/libhako_llvmc_ffi.so");
    if runtime_archive.exists() && ffi_library.exists() {
        let exe = std::env::temp_dir().join(format!(
            "hakorune-array-write-object-{}",
            std::process::id()
        ));
        let exe_result = crate::host_providers::llvm_codegen::emit_published_static_method_exe(
            &module,
            exe.to_str().expect("temporary path is valid UTF-8"),
            Some("target/release"),
            None,
        );
        assert_eq!(
            exe_result,
            Ok(true),
            "typed array exe compile failed: {exe_result:?}"
        );
        let status = std::process::Command::new(&exe)
            .status()
            .expect("run typed array executable");
        assert!(status.success(), "typed array executable failed: {status}");
        let _ = std::fs::remove_file(exe);
    }
}
