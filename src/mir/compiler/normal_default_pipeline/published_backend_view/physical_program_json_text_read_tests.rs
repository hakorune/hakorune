//! A `String` / `StringBox` / `String?` field is readable as a borrowed Text source in the
//! shapes the language reference lists; every other shape keeps its named refusal.
use super::tests::request;
use crate::mir::compiler::normal_default_pipeline::MirCompiler;

fn compile(field_type: &str, members: &str, main_body: &str) -> Result<(), String> {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = format!(
        "box Tag {{\n  name: {field_type}\n  n: i64\n  birth(name: {field_type}, n: i64) {{ me.name = name  me.n = n }}\n{members}\n}}\n\
         static box Main {{\n  main() {{\n    local t = new Tag(\"abc\", 3)\n    {main_body}\n  }}\n}}\n"
    );
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        MirCompiler::with_options(false)
            .compile_normal_with_published(request(&source), |view, _| {
                view.issue_lifecycle_physical_abi_input().map(|_| ())
            })
            .map(|_| ())
    })
}

#[test]
fn text_field_reads_compile_in_the_listed_shapes() {
    for field_type in ["String", "StringBox"] {
        for (members, body) in [
            ("", "print(t.name)  return 0"),
            ("", "print(t.name + \"x\")  return 0"),
            ("", "local s = t.name + \"x\"  print(s)  return 0"),
            ("", "local k = t.name.length()  return k"),
            ("  show(): i64 { print(me.name)  return 0 }", "return t.show()"),
            ("  show(): i64 { print(me.name + \"x\")  return 0 }", "return t.show()"),
            ("  len(): i64 { local k = me.name.length()  return k }", "return t.len()"),
            ("  get() { return me.name }", "print(t.get())  return 0"),
            // A Core call's receiver is its operand whatever the receiver is: a local here.
            ("  len(): i64 { local s = \"xyz\"; local k = s.length(); return k }", "return t.len()"),
            ("  len(): i64 { local s = \"xyz\"; local k = s.length() + 1; return k }", "return t.len()"),
            ("  len(): i64 { local s = \"xyz\"; local k = s.indexOf(\"z\"); return k }", "return t.len()"),
        ] {
            compile(field_type, members, body)
                .unwrap_or_else(|error| panic!("{field_type} / {members} / {body}: {error}"));
        }
    }
}

/// A type mismatch, a borrowed local alias, a terminal method call and a field that another
/// writer fills with a non-Text value are refused by name; none falls back to a copy or a borrow.
#[test]
fn text_field_reads_outside_the_listed_shapes_are_refused_by_name() {
    for (members, body, tag) in [
        ("", "print(t.n + \"x\")  return 0", "print-expression/source-unavailable"),
        ("  set(): i64 { me.name = 5  return 0 }", "print(t.name)  return 0", "print-expression/source-unavailable"),
        ("", "local s = t.name  print(s)  return 0", "artifact-source-unavailable"),
        ("", "return t.name.length()", "artifact-source-unavailable"),
    ] {
        let error = compile("String", members, body).expect_err(&format!("{members} / {body}"));
        assert!(error.contains(tag), "{members} / {body}: {error}");
    }
}

/// A nullable `String?` field has no connected read: the null arm is refused by name, not read as Text.
#[test]
fn nullable_text_field_read_is_refused_by_name() {
    let error = compile("String?", "", "print(t.name)  return 0").unwrap_err();
    // The same nullable Text contract may now refuse at source admission,
    // before a read projection or a physical Birth is constructed.
    let source_contract_refusal = error.contains("[mir/callable-semantic-package/issue]")
        && error.contains("[ordinary-new/argument-source-unavailable]")
        && error.contains("constructor Text formal contract is not connected");
    assert!(source_contract_refusal || error.contains("lexical-call-prior-home-unavailable")
        || error.contains("print-expression/source-unavailable"), "{error}");
}

/// A `take` birth whose source is spelled correctly but not connected is an unconnected form, not a
/// spelling mismatch; the two spelling errors keep `BirthStoreOwnershipMismatch`.
#[test]
fn take_birth_refusals_split_unconnected_from_spelling() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let compile_node = |node: &str| {
        let source = format!(
            "box Leaf {{\n  value: i64\n  birth(value) {{ me.value = value }}\n  get(): i64 {{ return me.value }}\n}}\n{node}\n\
             static box Main {{\n  main() {{\n    local leaf = new Leaf(1)\n    local n = new Node(move leaf)\n    return n.v\n  }}\n}}\n"
        );
        crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
            MirCompiler::with_options(false)
                .compile_normal_with_published(request(&source), |_, _| Ok::<(), String>(()))
                .map(|_| ())
        })
    };
    for node in [
        "box Node { next: Leaf  v: i64  birth(take next: Leaf) { me.v = next.value  me.next = move next } }",
        "box Node { v: i64  birth(take next: Leaf) { me.v = next.get() } }",
    ] {
        let error = compile_node(node).unwrap_err();
        assert!(error.contains("TakeBirthSourceUnconnected") && !error.contains("BirthStoreOwnershipMismatch"), "{node}: {error}");
    }
    for (node, issue) in [
        ("box Node { next: Leaf  v: i64  birth(take next: Leaf) { me.next = next  me.v = 1 } }", "TakeStoreRequiresMove"),
        ("box Node { next: Leaf  v: i64  birth(next: Leaf) { me.next = move next  me.v = 1 } }", "MoveRequiresTakeFormal"),
    ] {
        let error = compile_node(node).unwrap_err();
        assert!(error.contains("BirthStoreOwnershipMismatch") && error.contains(issue), "{node}: {error}");
    }
}

/// An unplanned `move` argument names the real reason its emission was not prepared.
#[test]
fn unplanned_move_names_the_unprepared_emission_cause() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = "box Leaf {\n  value: i64\n  birth(value) { me.value = value }\n}\n\
        box Holder { keep: Leaf  birth(take k: Leaf) { me.keep = move k } }\n\
        static box Main { main() { local x = new Leaf(1)  local h = new Holder(move x)  local y = new Leaf(2)  h.keep = move y  return 4 } }";
    let error = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        MirCompiler::with_options(false)
            .compile_normal_with_published(request(source), |_, _| Ok::<(), String>(()))
            .map(|_| ())
    })
    .unwrap_err();
    assert!(error.contains("move-argument-unplanned") && error.contains("cause=NewEmissionUnavailable(")
        && !error.contains("cause=NewEmissionUnavailable "), "{error}");
}
