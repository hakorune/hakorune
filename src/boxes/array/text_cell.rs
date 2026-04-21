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
}

impl From<String> for ArrayTextCell {
    #[inline(always)]
    fn from(value: String) -> Self {
        Self::flat(value)
    }
}
