use super::*;
use crate::backend::vm_types::VMError;
use crate::box_trait::StringBox;
use crate::boxes::MapBox;
use std::sync::Arc;

fn stringbox_receiver(text: &str) -> VMValue {
    VMValue::BoxRef(Arc::new(StringBox::new(text)))
}

fn mapbox_receiver() -> VMValue {
    VMValue::BoxRef(Arc::new(MapBox::new()))
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
