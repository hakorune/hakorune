use super::*;

#[test]
fn self_ping_sets_last_fields() {
    let p = P2PBox::new("alice".to_string(), TransportKind::InProcess);
    let intent = IntentBox::new("ping".to_string(), serde_json::json!({}));
    let res = p.send(
        Box::new(StringBox::new("alice".to_string())),
        Box::new(intent),
    );
    if let Some(r) = res.as_any().downcast_ref::<ResultBox>() {
        assert!(matches!(r, ResultBox::Ok(_)));
    } else {
        panic!("send did not return ResultBox");
    }
    assert_eq!(p.get_last_from().to_string_box().value, "alice".to_string());
    assert_eq!(
        p.get_last_intent_name().to_string_box().value,
        "ping".to_string()
    );
}

impl P2PBox {
    #[allow(dead_code)]
    fn __debug_on_rust(&self, intent: &str, reply_intent: Option<&str>) {
        if let Ok(mut t) = self.transport.write() {
            let intent_name = intent.to_string();
            let last_from = Arc::clone(&self.last_from);
            let last_intent = Arc::clone(&self.last_intent_name);
            let transport_arc = Arc::clone(&self.transport);
            let reply_name = reply_intent.map(|s| s.to_string());
            t.register_intent_handler(
                &intent_name,
                Box::new(move |env| {
                    if let Ok(mut lf) = last_from.write() {
                        *lf = Some(env.from.clone());
                    }
                    if let Ok(mut li) = last_intent.write() {
                        *li = Some(env.intent.get_name().to_string_box().value);
                    }
                    if let Some(rn) = reply_name.clone() {
                        let to = env.from.clone();
                        let transport_arc = Arc::clone(&transport_arc);
                        let intent = IntentBox::new(rn, serde_json::json!({}));
                        Self::spawn_delayed_transport_send(
                            "P2PBox.debug_async_reply",
                            5,
                            transport_arc,
                            to,
                            intent,
                        );
                    }
                }),
            );
        }
    }
}

#[test]
#[ignore = "MIR13 migration: P2P async timing/initialization alignment pending"]
fn two_node_ping_pong() {
    let alice = P2PBox::new("alice".to_string(), TransportKind::InProcess);
    let bob = P2PBox::new("bob".to_string(), TransportKind::InProcess);
    bob.__debug_on_rust("ping", Some("pong"));
    alice.__debug_on_rust("pong", None);
    let ping = IntentBox::new("ping".to_string(), serde_json::json!({}));
    let _ = alice.send(Box::new(StringBox::new("bob")), Box::new(ping));
    assert_eq!(bob.get_last_intent_name().to_string_box().value, "ping");
    std::thread::sleep(std::time::Duration::from_millis(20));
    assert_eq!(alice.get_last_intent_name().to_string_box().value, "pong");
}

#[test]
fn on_once_disables_after_first_delivery() {
    let p = P2PBox::new("alice".to_string(), TransportKind::InProcess);
    let handler = crate::method_box::MethodBox::new(Box::new(p.clone()), "noop".to_string());
    let _ = p.on_once(Box::new(StringBox::new("hello")), Box::new(handler));
    let c0 = p.debug_active_handler_count(Box::new(StringBox::new("hello")));
    assert_eq!(c0.to_string_box().value, "1");
    let intent = IntentBox::new("hello".to_string(), serde_json::json!({}));
    let _ = p.send(Box::new(StringBox::new("alice")), Box::new(intent.clone()));
    let _ = p.send(Box::new(StringBox::new("alice")), Box::new(intent));
    let c1 = p.debug_active_handler_count(Box::new(StringBox::new("hello")));
    assert_eq!(c1.to_string_box().value, "0");
}

#[test]
fn off_clears_handlers() {
    let p = P2PBox::new("bob".to_string(), TransportKind::InProcess);
    let handler = crate::method_box::MethodBox::new(Box::new(p.clone()), "noop".to_string());
    let _ = p.on(Box::new(StringBox::new("bye")), Box::new(handler));
    let c0 = p.debug_active_handler_count(Box::new(StringBox::new("bye")));
    assert_eq!(c0.to_string_box().value, "1");
    let _ = p.off(Box::new(StringBox::new("bye")));
    let c1 = p.debug_active_handler_count(Box::new(StringBox::new("bye")));
    assert_eq!(c1.to_string_box().value, "0");
}

#[test]
#[ignore = "MIR13 migration: P2P ping success semantics pending"]
fn ping_success_between_two_nodes() {
    let alice = P2PBox::new("alice".to_string(), TransportKind::InProcess);
    let bob = P2PBox::new("bob".to_string(), TransportKind::InProcess);
    let ok = alice.ping(Box::new(StringBox::new("bob")));
    if let Some(b) = ok.as_any().downcast_ref::<BoolBox>() {
        assert!(b.value);
    } else {
        panic!("ping did not return BoolBox");
    }
}

#[test]
fn ping_timeout_on_missing_node() {
    let alice = P2PBox::new("alice".to_string(), TransportKind::InProcess);
    let ok = alice.ping_with_timeout(Box::new(StringBox::new("nobody")), 20);
    if let Some(b) = ok.as_any().downcast_ref::<BoolBox>() {
        assert!(!b.value);
    } else {
        panic!("ping_with_timeout did not return BoolBox");
    }
}
