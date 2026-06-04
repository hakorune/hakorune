use std::{fmt, ops::Deref};

/// Runtime-private read-only text view returned by borrowed-handle and
/// string-view paths. `BorrowedHandleBox` and `StringViewBox` may provide or
/// replay this view, but neither path owns the text itself.
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

/// Runtime-private owned text buffer waiting to be published through
/// `KernelTextSlot`. Future `TextCell` work stays separate from this buffered
/// text state.
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
