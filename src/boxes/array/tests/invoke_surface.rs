use super::*;

#[test]
fn invoke_surface_routes_insert_remove_clear_contains_indexof_join_reverse_and_length_alias() {
    let array = ArrayBox::new();
    assert!(matches!(
        array
            .invoke_surface(
                ArrayMethodId::Push,
                vec![Box::new(IntegerBox::new(10)) as Box<dyn NyashBox>],
            )
            .unwrap(),
        ArraySurfaceInvokeResult::Void
    ));

    assert!(matches!(
        array
            .invoke_surface(
                ArrayMethodId::Set,
                vec![
                    Box::new(IntegerBox::new(0)) as Box<dyn NyashBox>,
                    Box::new(IntegerBox::new(11)) as Box<dyn NyashBox>,
                ],
            )
            .unwrap(),
        ArraySurfaceInvokeResult::Void
    ));

    let get = array
        .invoke_surface(
            ArrayMethodId::Get,
            vec![Box::new(IntegerBox::new(0)) as Box<dyn NyashBox>],
        )
        .unwrap();
    match get {
        ArraySurfaceInvokeResult::Value(value) => {
            assert_eq!(value.to_string_box().value, "11");
        }
        ArraySurfaceInvokeResult::Void => panic!("get must return a value"),
    }

    let insert_result = array
        .invoke_surface(
            ArrayMethodId::Insert,
            vec![
                Box::new(IntegerBox::new(1)) as Box<dyn NyashBox>,
                Box::new(StringBox::new("Alpha")) as Box<dyn NyashBox>,
            ],
        )
        .unwrap();
    assert!(matches!(insert_result, ArraySurfaceInvokeResult::Void));

    let length = array
        .invoke_surface(ArrayMethodId::from_name("size").unwrap(), vec![])
        .unwrap();
    match length {
        ArraySurfaceInvokeResult::Value(value) => {
            assert_eq!(value.to_string_box().value, "2");
        }
        ArraySurfaceInvokeResult::Void => panic!("length must return a value"),
    }

    let contains = array
        .invoke_surface(
            ArrayMethodId::Contains,
            vec![Box::new(StringBox::new("Alpha")) as Box<dyn NyashBox>],
        )
        .unwrap();
    match contains {
        ArraySurfaceInvokeResult::Value(value) => {
            assert_eq!(value.to_string_box().value, "true");
        }
        ArraySurfaceInvokeResult::Void => panic!("contains must return a value"),
    }

    let index = array
        .invoke_surface(
            ArrayMethodId::IndexOf,
            vec![Box::new(StringBox::new("Alpha")) as Box<dyn NyashBox>],
        )
        .unwrap();
    match index {
        ArraySurfaceInvokeResult::Value(value) => {
            assert_eq!(value.to_string_box().value, "1");
        }
        ArraySurfaceInvokeResult::Void => panic!("indexOf must return a value"),
    }

    let joined = array
        .invoke_surface(
            ArrayMethodId::Join,
            vec![Box::new(StringBox::new("|")) as Box<dyn NyashBox>],
        )
        .unwrap();
    match joined {
        ArraySurfaceInvokeResult::Value(value) => {
            assert_eq!(value.to_string_box().value, "11|Alpha");
        }
        ArraySurfaceInvokeResult::Void => panic!("join must return a value"),
    }

    let reversed = array
        .invoke_surface(ArrayMethodId::Reverse, vec![])
        .unwrap();
    match reversed {
        ArraySurfaceInvokeResult::Value(value) => {
            assert_eq!(value.to_string_box().value, "ok");
        }
        ArraySurfaceInvokeResult::Void => panic!("reverse must return a value"),
    }

    let first_after_reverse = array
        .invoke_surface(
            ArrayMethodId::Get,
            vec![Box::new(IntegerBox::new(0)) as Box<dyn NyashBox>],
        )
        .unwrap();
    match first_after_reverse {
        ArraySurfaceInvokeResult::Value(value) => {
            assert_eq!(value.to_string_box().value, "Alpha");
        }
        ArraySurfaceInvokeResult::Void => panic!("get after reverse must return a value"),
    }

    let sorted = array.invoke_surface(ArrayMethodId::Sort, vec![]).unwrap();
    match sorted {
        ArraySurfaceInvokeResult::Value(value) => {
            assert_eq!(value.to_string_box().value, "ok");
        }
        ArraySurfaceInvokeResult::Void => panic!("sort must return a value"),
    }

    let first_after_sort = array
        .invoke_surface(
            ArrayMethodId::Get,
            vec![Box::new(IntegerBox::new(0)) as Box<dyn NyashBox>],
        )
        .unwrap();
    match first_after_sort {
        ArraySurfaceInvokeResult::Value(value) => {
            assert_eq!(value.to_string_box().value, "11");
        }
        ArraySurfaceInvokeResult::Void => panic!("get after sort must return a value"),
    }

    let slice = array
        .invoke_surface(
            ArrayMethodId::Slice,
            vec![
                Box::new(IntegerBox::new(0)) as Box<dyn NyashBox>,
                Box::new(IntegerBox::new(1)) as Box<dyn NyashBox>,
            ],
        )
        .unwrap();
    match slice {
        ArraySurfaceInvokeResult::Value(value) => {
            assert_eq!(value.to_string_box().value, "[11]");
        }
        ArraySurfaceInvokeResult::Void => panic!("slice must return a value"),
    }

    let removed = array
        .invoke_surface(
            ArrayMethodId::Remove,
            vec![Box::new(IntegerBox::new(1)) as Box<dyn NyashBox>],
        )
        .unwrap();
    match removed {
        ArraySurfaceInvokeResult::Value(value) => {
            assert_eq!(value.to_string_box().value, "Alpha");
        }
        ArraySurfaceInvokeResult::Void => panic!("remove must return a value"),
    }

    let popped = array.invoke_surface(ArrayMethodId::Pop, vec![]).unwrap();
    match popped {
        ArraySurfaceInvokeResult::Value(value) => {
            assert_eq!(value.to_string_box().value, "11");
        }
        ArraySurfaceInvokeResult::Void => panic!("pop must return a value"),
    }

    let cleared = array.invoke_surface(ArrayMethodId::Clear, vec![]).unwrap();
    assert!(matches!(cleared, ArraySurfaceInvokeResult::Void));

    let length_after_clear = array.invoke_surface(ArrayMethodId::Length, vec![]).unwrap();
    match length_after_clear {
        ArraySurfaceInvokeResult::Value(value) => {
            assert_eq!(value.to_string_box().value, "0");
        }
        ArraySurfaceInvokeResult::Void => panic!("length after clear must return a value"),
    }
}
