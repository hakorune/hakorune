use super::{perf_observe, slot_ref, HandlePayload, SlotTable};

pub struct TextReadSession<'a> {
    table: &'a SlotTable,
}

impl<'a> TextReadSession<'a> {
    #[inline(always)]
    pub(super) fn new(table: &'a SlotTable) -> Self {
        Self { table }
    }

    #[inline(always)]
    pub fn str_handle<R>(&self, h: u64, f: impl FnOnce(&str) -> R) -> Option<R> {
        let text = slot_str_ref(self.table, h)?;
        perf_observe::text_read_handle(h);
        Some(f(text))
    }

    #[inline(always)]
    pub fn str_pair<R>(&self, a: u64, b: u64, f: impl FnOnce(&str, &str) -> R) -> Option<R> {
        perf_observe::text_read_pair(a, b);
        let a = slot_str_ref(self.table, a)?;
        let b = slot_str_ref(self.table, b)?;
        Some(f(a, b))
    }

    #[inline(always)]
    pub fn str3<R>(
        &self,
        a: u64,
        b: u64,
        c: u64,
        f: impl FnOnce(&str, &str, &str) -> R,
    ) -> Option<R> {
        perf_observe::text_read_triple(a, b, c);
        let a = slot_str_ref(self.table, a)?;
        let b = slot_str_ref(self.table, b)?;
        let c = slot_str_ref(self.table, c)?;
        Some(f(a, b, c))
    }
}

#[inline(always)]
fn slot_str_ref<'a>(table: &'a SlotTable, h: u64) -> Option<&'a str> {
    slot_ref(table, h).and_then(HandlePayload::as_str_fast)
}
