#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(super) enum ArrayTextCell {
    Flat(String),
}

impl ArrayTextCell {
    #[inline(always)]
    pub(super) fn flat(value: String) -> Self {
        Self::Flat(value)
    }

    #[inline(always)]
    pub(super) fn as_str(&self) -> &str {
        match self {
            Self::Flat(value) => value.as_str(),
        }
    }

    #[inline(always)]
    pub(super) fn as_mut_string(&mut self) -> &mut String {
        match self {
            Self::Flat(value) => value,
        }
    }

    #[inline(always)]
    pub(super) fn into_string(self) -> String {
        match self {
            Self::Flat(value) => value,
        }
    }

    #[inline(always)]
    pub(super) fn len(&self) -> usize {
        self.as_str().len()
    }

    #[inline(always)]
    pub(super) fn insert_const_mid_lenhalf(&mut self, middle: &str) -> i64 {
        match self {
            Self::Flat(value) => Self::insert_const_mid_lenhalf_string(value, middle),
        }
    }

    #[inline(always)]
    pub(super) fn insert_const_mid_lenhalf_string(value: &mut String, middle: &str) -> i64 {
        let split = (value.len() / 2) as i64;
        insert_const_mid_flat(value, middle, split);
        value.len() as i64
    }
}

impl From<String> for ArrayTextCell {
    #[inline(always)]
    fn from(value: String) -> Self {
        Self::flat(value)
    }
}

#[inline(always)]
fn insert_const_mid_flat(value: &mut String, middle: &str, split: i64) {
    if value.is_empty() {
        value.push_str(middle);
        return;
    }
    if middle.is_empty() {
        return;
    }
    let split = split.clamp(0, value.len() as i64) as usize;
    if value.is_char_boundary(split) {
        value.insert_str(split, middle);
        return;
    }
    *value = materialize_insert_const_mid_flat(value.as_str(), middle, split as i64);
}

#[inline(always)]
fn materialize_insert_const_mid_flat(source: &str, middle: &str, split: i64) -> String {
    if source.is_empty() {
        return middle.to_owned();
    }
    if middle.is_empty() {
        return source.to_owned();
    }
    let split = split.clamp(0, source.len() as i64) as usize;
    let prefix = source.get(0..split).unwrap_or("");
    let suffix = source.get(split..).unwrap_or("");
    let total = prefix.len() + middle.len() + suffix.len();
    let mut out = String::with_capacity(total);
    out.push_str(prefix);
    out.push_str(middle);
    out.push_str(suffix);
    out
}
