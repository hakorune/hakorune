//! JoinIR integration API (SSOT entry points)
//!
//! Purpose: provide a small, stable surface for route lowerers and merge code.
//! This reduces "where should I call this from?" drift and avoids re-implementing
//! contract logic (SSOT, fail-fast checks) in each route helper.
//!
//! Policy:
//! - Prefer SSOT helpers over ad-hoc logic in route helpers.
//! - Avoid guessing (order/layout/name) in callers; callers pass explicit intent.
//! - Keep this module thin: mostly wrappers/re-exports with clear naming.
