struct Pair(i64, i64);

struct KeywordNames {
    r#type: i64,
}

fn r#match(r#type: i64) -> i64 {
    let local: crate::model::Config = crate::model::Config;
    return crate::util::add1(r#type);
}
