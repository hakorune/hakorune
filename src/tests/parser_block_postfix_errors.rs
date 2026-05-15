use crate::parser::NyashParser;
use crate::tests::helpers::env::with_stage3_block_catch;

#[test]
fn top_level_catch_is_error() {
    with_stage3_block_catch(|| {
        let src = r#"catch (e) { print(e) }"#;
        assert!(NyashParser::parse_from_string(src).is_err());
    });
}

#[test]
fn top_level_cleanup_is_error() {
    with_stage3_block_catch(|| {
        let src = r#"cleanup { print("x") }"#;
        assert!(NyashParser::parse_from_string(src).is_err());
    });
}

#[test]
fn cleanup_then_catch_after_block_is_error() {
    with_stage3_block_catch(|| {
        let src = r#"
    { print("x") } cleanup { print("a") }
    catch (e) { print(e) }
    "#;
        assert!(NyashParser::parse_from_string(src).is_err());
    });
}

#[test]
fn cannot_attach_catch_to_loop_block() {
    with_stage3_block_catch(|| {
        let src = r#"
    loop { print("x") }
    catch (e) { print(e) }
    "#;
        assert!(NyashParser::parse_from_string(src).is_err());
    });
}
