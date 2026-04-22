use super::*;
use crate::backend::vm_types::VMError;
use crate::box_trait::StringBox;
use crate::boxes::array::ArrayBox;
use crate::boxes::MapBox;
use std::sync::Arc;

fn stringbox_receiver(text: &str) -> VMValue {
    VMValue::BoxRef(Arc::new(StringBox::new(text)))
}

fn mapbox_receiver() -> VMValue {
    VMValue::BoxRef(Arc::new(MapBox::new()))
}

fn arraybox_receiver(values: &[&str]) -> VMValue {
    let array = ArrayBox::new();
    for value in values {
        array.push(Box::new(StringBox::new(*value)));
    }
    VMValue::BoxRef(Arc::new(array))
}

#[test]
fn method_dispatch_stringbox_is_space_after_slot_miss() {
    let mut interp = MirInterpreter::new();
    let arg = ValueId(1);
    interp.regs.insert(arg, VMValue::String(" ".to_string()));

    let got = interp
        .execute_method_call(&stringbox_receiver("seed"), "is_space", &[arg])
        .expect("is_space should stay handled");

    assert_eq!(got, VMValue::Bool(true));
}

#[test]
fn method_dispatch_stringbox_is_alpha_after_slot_miss() {
    let mut interp = MirInterpreter::new();
    let arg = ValueId(1);
    interp.regs.insert(arg, VMValue::String("A".to_string()));

    let got = interp
        .execute_method_call(&stringbox_receiver("seed"), "is_alpha", &[arg])
        .expect("is_alpha should stay handled");

    assert_eq!(got, VMValue::Bool(true));
}

#[test]
fn method_dispatch_stringbox_unknown_method_still_fails_fast() {
    let mut interp = MirInterpreter::new();

    let err = interp
        .execute_method_call(&stringbox_receiver("seed"), "missing_method", &[])
        .expect_err("unknown StringBox method must fail fast");

    match err {
        VMError::InvalidInstruction(msg) => {
            assert_eq!(msg, "Unknown method 'missing_method' on StringBox");
        }
        other => panic!("unexpected error: {:?}", other),
    }
}

#[test]
fn method_callee_stringbox_length_strips_duplicate_receiver_arg() {
    let mut interp = MirInterpreter::new();
    let recv = ValueId(1);
    interp.regs.insert(recv, stringbox_receiver("seed"));

    let got = interp
        .execute_method_callee("StringBox", "length", &Some(recv), &[recv])
        .expect("StringBox.length should tolerate unified duplicate receiver arg");

    assert_eq!(got, VMValue::Integer(4));
}

#[test]
fn method_callee_raw_string_len_strips_duplicate_receiver_arg() {
    let mut interp = MirInterpreter::new();
    let recv = ValueId(1);
    interp
        .regs
        .insert(recv, VMValue::String("seed".to_string()));

    let got = interp
        .execute_method_callee("StringBox", "len", &Some(recv), &[recv])
        .expect("String.len should tolerate unified duplicate receiver arg");

    assert_eq!(got, VMValue::Integer(4));
}

#[test]
fn method_callee_mapbox_set_get_strips_duplicate_receiver_arg() {
    let mut interp = MirInterpreter::new();
    let recv = ValueId(1);
    let key = ValueId(2);
    let val = ValueId(3);
    let recv_alias = ValueId(4);
    interp.regs.insert(recv, mapbox_receiver());
    let alias_value = interp.regs.get(&recv).expect("receiver inserted").clone();
    interp.regs.insert(recv_alias, alias_value);
    interp.regs.insert(key, VMValue::String("a".to_string()));
    interp.regs.insert(val, VMValue::String("b".to_string()));

    interp
        .execute_method_callee("MapBox", "set", &Some(recv), &[recv_alias, key, val])
        .expect("MapBox.set should tolerate unified duplicate receiver arg");

    let got = interp
        .execute_method_callee("MapBox", "get", &Some(recv), &[recv_alias, key])
        .expect("MapBox.get should tolerate unified duplicate receiver arg");

    assert_eq!(got.to_string(), "b");
}

#[test]
fn method_callee_arraybox_get_strips_duplicate_receiver_alias_arg() {
    let mut interp = MirInterpreter::new();
    let recv = ValueId(1);
    let index = ValueId(2);
    let recv_alias = ValueId(3);
    interp.regs.insert(recv, arraybox_receiver(&["row"]));
    let alias_value = interp.regs.get(&recv).expect("receiver inserted").clone();
    interp.regs.insert(recv_alias, alias_value);
    interp.regs.insert(index, VMValue::Integer(0));

    let got = interp
        .execute_method_callee("ArrayBox", "get", &Some(recv), &[recv_alias, index])
        .expect("ArrayBox.get should tolerate unified duplicate receiver arg");

    assert_eq!(got.to_string(), "row");
}

#[test]
fn method_callee_arraybox_push_keeps_legitimate_self_argument() {
    let mut interp = MirInterpreter::new();
    let recv = ValueId(1);
    let recv_alias = ValueId(2);
    let index = ValueId(3);
    interp.regs.insert(recv, arraybox_receiver(&[]));
    let alias_value = interp.regs.get(&recv).expect("receiver inserted").clone();
    interp.regs.insert(recv_alias, alias_value);
    interp.regs.insert(index, VMValue::Integer(0));

    interp
        .execute_method_callee("ArrayBox", "push", &Some(recv), &[recv_alias])
        .expect("ArrayBox.push should keep a real self argument");

    let got = interp
        .execute_method_callee("ArrayBox", "get", &Some(recv), &[index])
        .expect("ArrayBox.get should read back pushed self argument");

    match got {
        VMValue::BoxRef(bx) => assert_eq!(bx.type_name(), "ArrayBox"),
        other => panic!("expected ArrayBox, got {:?}", other),
    }
}
