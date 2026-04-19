use std::{fmt, ops::Deref};

/// Runtime-private semantic read carrier for the current text corridor.
/// `BorrowedHandleBox` and `StringViewBox` may provide or replay this view,
/// but neither boundary adapter is the semantic carrier itself.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct TextRef<'a> {
    text: &'a str,
}

impl<'a> TextRef<'a> {
    #[inline(always)]
    pub(crate) fn new(text: &'a str) -> Self {
        Self { text }
    }

    #[inline(always)]
    pub(crate) fn as_str(self) -> &'a str {
        self.text
    }
}

impl fmt::Display for TextRef<'_> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.text)
    }
}

impl Deref for TextRef<'_> {
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.text
    }
}

/// Runtime-private semantic owned carrier for unpublished text.
/// `KernelTextSlot` remains the transport adapter/sink seed; future `TextCell`
/// work stays separate from this carrier step.
pub(crate) struct OwnedText(String);

impl OwnedText {
    #[inline(always)]
    pub(crate) fn from_string(value: String) -> Self {
        Self(value)
    }

    #[inline(always)]
    pub(crate) fn as_str(&self) -> &str {
        self.0.as_str()
    }

    #[inline(always)]
    pub(crate) fn into_string(self) -> String {
        self.0
    }
}
