use crate::parser::entry_sugar::parse_with_sugar_level;
use crate::syntax::sugar_config::SugarLevel;

#[test]
fn range_expression_rejects_even_with_sugar_enabled() {
    for level in [SugarLevel::Basic, SugarLevel::Full] {
        assert!(parse_with_sugar_level("r = 1 .. 5\n", level).is_err());
    }
}
