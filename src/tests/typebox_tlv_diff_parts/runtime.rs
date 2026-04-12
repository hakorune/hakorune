use super::helpers::*;
use std::env;
use std::fs;

#[test]
#[ignore = "MIR13 parity: TimeBox now tolerance under unified ops pending"]
fn timebox_now_tlv_vs_typebox_with_tolerance() {
    ensure_host();

    let _g = EnvGuard::set("NYASH_DISABLE_TYPEBOX", "1");
    let (bt1, id1, _hold1) = create_plugin_instance("TimeBox");
    let t_tlv = with_host(|h| {
        let v = inv_some(h, &bt1, "now", id1, &[]);
        v.to_string_box().value.parse::<i64>().unwrap_or(0)
    });

    let _g2 = EnvGuard::remove("NYASH_DISABLE_TYPEBOX");
    let (bt2, id2, _hold2) = create_plugin_instance("TimeBox");
    let t_tb = with_host(|h| {
        let v = inv_some(h, &bt2, "now", id2, &[]);
        v.to_string_box().value.parse::<i64>().unwrap_or(0)
    });

    let diff = (t_tb - t_tlv).abs();
    assert!(diff < 5_000, "TimeBox.now difference too large: {}ms", diff);
}

#[test]
#[ignore = "MIR13 parity: CounterBox singleton delta under unified ops pending"]
fn counterbox_singleton_delta_increments() {
    ensure_host();

    let _g = EnvGuard::set("NYASH_DISABLE_TYPEBOX", "1");
    let (bt1, id1, _hold1) = create_plugin_instance("CounterBox");
    let (a1, b1) = with_host(|h| {
        let a = inv_some(h, &bt1, "get", id1, &[]);
        inv_void(h, &bt1, "inc", id1, &[]);
        let b = inv_some(h, &bt1, "get", id1, &[]);
        (
            a.to_string_box().value.parse::<i64>().unwrap_or(0),
            b.to_string_box().value.parse::<i64>().unwrap_or(0),
        )
    });
    assert_eq!(b1 - a1, 1, "CounterBox TLV should increment by 1");

    let _g2 = EnvGuard::remove("NYASH_DISABLE_TYPEBOX");
    let (bt2, id2, _hold2) = create_plugin_instance("CounterBox");
    let (a2, b2) = with_host(|h| {
        let a = inv_some(h, &bt2, "get", id2, &[]);
        inv_void(h, &bt2, "inc", id2, &[]);
        let b = inv_some(h, &bt2, "get", id2, &[]);
        (
            a.to_string_box().value.parse::<i64>().unwrap_or(0),
            b.to_string_box().value.parse::<i64>().unwrap_or(0),
        )
    });
    assert_eq!(b2 - a2, 1, "CounterBox TypeBox should increment by 1");
}

#[test]
#[ignore = "MIR13 parity: FileBox RW/close under unified ops pending"]
fn filebox_rw_close_tmpdir_tlv_vs_typebox() {
    ensure_host();

    let mut p = std::env::temp_dir();
    p.push(format!(
        "nyash_test_{}_{}.txt",
        std::process::id(),
        rand_id()
    ));
    let path_str = p.to_string_lossy().to_string();

    env::set_var("NYASH_DISABLE_TYPEBOX", "1");
    let (bt1, id1, _hold1) = create_plugin_instance("FileBox");
    let out_tlv = with_host(|h| {
        let _ = h
            .invoke_instance_method(
                &bt1,
                "open",
                id1,
                &[
                    Box::new(crate::box_trait::StringBox::new(&path_str)),
                    Box::new(crate::box_trait::StringBox::new("w")),
                ],
            )
            .expect("open tlv");
        let _ = h
            .invoke_instance_method(
                &bt1,
                "write",
                id1,
                &[Box::new(crate::box_trait::StringBox::new("hello"))],
            )
            .expect("write tlv");
        let _ = h
            .invoke_instance_method(&bt1, "close", id1, &[])
            .expect("close tlv");
        let _ = h
            .invoke_instance_method(
                &bt1,
                "open",
                id1,
                &[
                    Box::new(crate::box_trait::StringBox::new(&path_str)),
                    Box::new(crate::box_trait::StringBox::new("r")),
                ],
            )
            .expect("open2 tlv");
        let rd = inv_some(h, &bt1, "read", id1, &[]);
        let _ = h
            .invoke_instance_method(&bt1, "close", id1, &[])
            .expect("close2 tlv");
        rd.to_string_box().value
    });

    let _g2 = EnvGuard::remove("NYASH_DISABLE_TYPEBOX");
    let (bt2, id2, _hold2) = create_plugin_instance("FileBox");
    let out_tb = with_host(|h| {
        let _ = h
            .invoke_instance_method(
                &bt2,
                "open",
                id2,
                &[
                    Box::new(crate::box_trait::StringBox::new(&path_str)),
                    Box::new(crate::box_trait::StringBox::new("w")),
                ],
            )
            .expect("open tb");
        let _ = h
            .invoke_instance_method(
                &bt2,
                "write",
                id2,
                &[Box::new(crate::box_trait::StringBox::new("hello"))],
            )
            .expect("write tb");
        let _ = h
            .invoke_instance_method(&bt2, "close", id2, &[])
            .expect("close tb");
        let _ = h
            .invoke_instance_method(
                &bt2,
                "open",
                id2,
                &[
                    Box::new(crate::box_trait::StringBox::new(&path_str)),
                    Box::new(crate::box_trait::StringBox::new("r")),
                ],
            )
            .expect("open2 tb");
        let rd = inv_some(h, &bt2, "read", id2, &[]);
        let _ = h
            .invoke_instance_method(&bt2, "close", id2, &[])
            .expect("close2 tb");
        rd.to_string_box().value
    });

    let _ = fs::remove_file(&path_str);

    assert_eq!(
        out_tlv, out_tb,
        "TLV vs TypeBox results should match (FileBox)"
    );
}
