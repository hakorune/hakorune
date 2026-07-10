use nyash_rust::parser::{GrammarProfile, NyashParser, ParserBuildConfig};
use nyash_rust::tokenizer::NyashTokenizer;

fn parse_with_profile(
    source: &str,
    grammar_profile: GrammarProfile,
) -> Result<nyash_rust::ast::ASTNode, nyash_rust::parser::ParseError> {
    NyashParser::parse_from_string_with_build_config(
        source,
        ParserBuildConfig {
            grammar_profile,
            ..ParserBuildConfig::default()
        },
    )
}

#[test]
fn tokenizer_retains_the_explicit_profile() {
    let tokenizer = NyashTokenizer::with_grammar_profile("try", GrammarProfile::Compat2025);
    assert_eq!(tokenizer.grammar_profile(), GrammarProfile::Compat2025);
}

#[test]
fn canonical_default_and_explicit_canonical_reject_statement_try() {
    let source = "try { local x = 1 } catch () { }";
    for result in [
        NyashParser::parse_from_string(source),
        parse_with_profile(source, GrammarProfile::Canonical),
    ] {
        let error = format!("{:?}", result.unwrap_err());
        assert!(error.contains("[parser/try_reserved]"), "{error}");
    }
}

#[test]
fn compat2025_accepts_only_the_closed_try_subset() {
    let accepted = "try { local x = 1 } catch () { } cleanup { local y = 2 }";
    assert!(parse_with_profile(accepted, GrammarProfile::Compat2025).is_ok());

    let rejected = "try { local x = 1 } catch (Error e) { }";
    let error = format!(
        "{:?}",
        parse_with_profile(rejected, GrammarProfile::Compat2025).unwrap_err()
    );
    assert!(error.contains("[parser/try_compat_not_normalizable]"));
}

#[test]
fn profile_plumbing_does_not_change_match_or_from_baselines() {
    let match_source = "local x = match 1 { 1 => 2, _ => 0 }";
    let from_source = "from Parent.method()";
    for profile in [GrammarProfile::Canonical, GrammarProfile::Compat2025] {
        assert!(parse_with_profile(match_source, profile).is_ok());
        assert!(parse_with_profile(from_source, profile).is_ok());
    }
}
