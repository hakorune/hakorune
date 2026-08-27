/* panic!("comment"); /* todo!("nested") */ */
#[allow(dead_code)]
fn sample() {
    let _ = r##"panic!(\"raw\") unwrap() expect(\"raw\")"##;
    let _ = Some(1).unwrap();
    let _ = Some(1).expect("value");
    panic!("direct");
    todo!("direct");
}

#[cfg_attr(feature = "future", allow(dead_code))]
mod nested {
    #![allow(dead_code)]

    pub fn child() {
        include!("generated.rs");
    }
}

#[cfg(test)]
fn test_only() {}
