//! Caller-owned Fault storage. Recording never allocates, formats or runs hooks.
//! C must not copy or mutate these records; message provenance stays Rust-private.

use std::ptr;

#[path = "fault_checked_object.rs"]
mod checked_object;

const ABI_VERSION: u32 = 1;
const SUPPRESSED_CAPACITY: usize = 8;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Status {
    Normal = 0,
    Fault = 1,
    InvalidContract = 2,
}

#[repr(C)]
pub(crate) struct Diagnostic {
    reason: u32,
    reserved: u32,
    site: u64,
    details: [i64; 2],
    message: *mut u8,
    message_len: usize,
}

impl Diagnostic {
    pub(crate) fn new(reason: u32, site: u64, details: [i64; 2]) -> Self {
        Self { reason, reserved: 0, site, details, message: ptr::null_mut(), message_len: 0 }
    }

    // The caller prepares this residence before Fault capture. No C pointer,
    // borrowed string, allocator conversion or registry lookup is accepted.
    pub(crate) fn with_message(self, bytes: Box<[u8]>) -> Self {
        let message_len = bytes.len();
        Self {
            reason: self.reason, reserved: self.reserved, site: self.site, details: self.details,
            message: Box::into_raw(bytes) as *mut u8, message_len,
        }
        // The consumed old Diagnostic drops here, including a replaced message.
    }

    pub(crate) fn message(&self) -> Option<&[u8]> {
        if self.message.is_null() { return None; }
        // SAFETY: only with_message installs this allocation; Drop owns release.
        Some(unsafe { std::slice::from_raw_parts(self.message, self.message_len) })
    }
}

impl Drop for Diagnostic {
    fn drop(&mut self) {
        if !self.message.is_null() {
            let bytes = ptr::slice_from_raw_parts_mut(self.message, self.message_len);
            self.message = ptr::null_mut();
            self.message_len = 0;
            // SAFETY: this is the original Box slice, transferred exactly once.
            unsafe { drop(Box::from_raw(bytes)); }
        }
    }
}

#[repr(C)]
pub(crate) struct FaultFrame {
    abi_version: u32,
    primary_present: u32,
    suppressed_len: u32,
    omitted: u32,
    primary: Diagnostic,
    suppressed: [Diagnostic; SUPPRESSED_CAPACITY],
}

impl FaultFrame {
    pub(crate) fn new() -> Self {
        Self {
            abi_version: ABI_VERSION, primary_present: 0, suppressed_len: 0, omitted: 0,
            primary: Diagnostic::new(0, 0, [0; 2]),
            suppressed: std::array::from_fn(|_| Diagnostic::new(0, 0, [0; 2])),
        }
    }

    fn valid(&self) -> bool {
        self.abi_version == ABI_VERSION && self.primary_present <= 1 && self.omitted <= 1
            && self.suppressed_len as usize <= SUPPRESSED_CAPACITY
            && (self.primary_present == 1 || (self.suppressed_len == 0 && self.omitted == 0))
            && (self.omitted == 0 || self.suppressed_len as usize == SUPPRESSED_CAPACITY)
    }

    /// Failure returns ownership unchanged. Success consumes even an omitted
    /// diagnostic; its inert bytes are dropped without allocation or user code.
    pub(crate) fn record(&mut self, diagnostic: Diagnostic) -> Result<Status, Diagnostic> {
        if !self.valid() || diagnostic.reason == 0 || diagnostic.reserved != 0 {
            return Err(diagnostic);
        }
        if self.primary_present == 0 {
            self.primary = diagnostic;
            self.primary_present = 1;
        } else if (self.suppressed_len as usize) < SUPPRESSED_CAPACITY {
            self.suppressed[self.suppressed_len as usize] = diagnostic;
            self.suppressed_len += 1;
        } else {
            self.omitted = 1;
            drop(diagnostic);
        }
        Ok(Status::Fault)
    }

    /// Reporting borrows until frame disposal. Propagation calls neither this
    /// method nor record; a successful operation returns Normal independently.
    pub(crate) fn diagnostics(&self) -> Result<(Option<&Diagnostic>, &[Diagnostic], bool), Status> {
        if !self.valid() { return Err(Status::InvalidContract); }
        Ok((
            (self.primary_present != 0).then_some(&self.primary),
            &self.suppressed[..self.suppressed_len as usize], self.omitted != 0,
        ))
    }

    fn dispose(&mut self) -> Status {
        if !self.valid() { return Status::InvalidContract; }
        self.abi_version = 0;
        self.primary = Diagnostic::new(0, 0, [0; 2]);
        for diagnostic in &mut self.suppressed {
            *diagnostic = Diagnostic::new(0, 0, [0; 2]);
        }
        self.primary_present = 0;
        self.suppressed_len = 0;
        self.omitted = 0;
        Status::Normal
    }

    /// Only the final entry chooses the output sink, after all cleanup. This
    /// borrows payloads; neither propagation nor recording invokes reporting.
    pub(crate) fn report(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        let (primary, suppressed, omitted) = self.diagnostics()
            .map_err(|_| std::io::Error::from(std::io::ErrorKind::InvalidInput))?;
        for (role, diagnostic) in primary.into_iter().map(|d| ("primary", d))
            .chain(suppressed.iter().map(|d| ("suppressed", d)))
        {
            write!(output, "[fault:{role}] reason={} site={} details={},{}",
                diagnostic.reason, diagnostic.site, diagnostic.details[0], diagnostic.details[1])?;
            if let Some(message) = diagnostic.message() {
                output.write_all(b" message=")?;
                output.write_all(message)?;
            }
            output.write_all(b"\n")?;
        }
        if omitted { output.write_all(b"[fault] additional diagnostics omitted\n")?; }
        Ok(())
    }
}

/// Placement initialization only. `storage` must denote fresh, uniquely owned,
/// aligned storage of size FaultFrame. It must not contain a live frame.
#[export_name = "nyash.fault.frame_init_v1"]
pub unsafe extern "C" fn frame_init(storage: *mut std::ffi::c_void) -> u32 {
    if storage.is_null() { return Status::InvalidContract as u32; }
    // SAFETY: alignment, writable size and freshness are the caller contract.
    unsafe { storage.cast::<FaultFrame>().write(FaultFrame::new()); }
    Status::Normal as u32
}

/// `storage` must be a live, exclusively borrowed frame initialized above.
/// Null/header rejection does not validate arbitrary foreign pointers.
#[export_name = "nyash.fault.record_static_v1"]
pub unsafe extern "C" fn record_static(
    storage: *mut std::ffi::c_void, reason: u32, site: u64, detail0: i64, detail1: i64,
) -> u32 {
    if storage.is_null() { return Status::InvalidContract as u32; }
    // SAFETY: initialized frame and exclusive synchronous access are required.
    let frame = unsafe { &mut *storage.cast::<FaultFrame>() };
    match frame.record(Diagnostic::new(reason, site, [detail0, detail1])) {
        Ok(status) => status as u32,
        Err(_) => Status::InvalidContract as u32,
    }
}

/// Final-entry owner only, after reporting. Payload disposal delegates to the
/// Diagnostic destructor; the invalidated empty frame cannot be reused.
#[export_name = "nyash.fault.frame_dispose_v1"]
pub unsafe extern "C" fn frame_dispose(storage: *mut std::ffi::c_void) -> u32 {
    if storage.is_null() { return Status::InvalidContract as u32; }
    // SAFETY: same initialized/exclusive storage contract as record_static.
    unsafe { (&mut *storage.cast::<FaultFrame>()).dispose() as u32 }
}

/// Explicit final-entry reporting only, never an operation or propagation hook.
/// Result is reporting status (0 success, -1 invalid frame, -2 sink failure),
/// not Normal/Fault. Even a failed report leaves disposal to the final owner.
#[export_name = "nyash.fault.report_final_v1"]
pub unsafe extern "C" fn report_final(storage: *const std::ffi::c_void) -> i32 {
    if storage.is_null() { return -1; }
    // SAFETY: caller provides a live aligned shared borrow until report returns.
    let frame = unsafe { &*storage.cast::<FaultFrame>() };
    if !frame.valid() { return -1; }
    match frame.report(&mut std::io::stderr().lock()) {
        Ok(()) => 0,
        Err(_) => -2,
    }
}

#[cfg(test)]
#[path = "fault_tests.rs"]
mod tests;
