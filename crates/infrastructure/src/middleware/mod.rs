// crates/infrastructure/src/middleware/mod.rs
// Módulo de middleware RBAC
// Vinculado con ADR-0006-rbac-sessions-audit.md

pub mod audit_middleware;
pub mod rbac;
pub mod rate_limit;

pub use audit_middleware::{audit_middleware, AuditService, extract_request_info, RequestInfo};
pub use rbac::AuthenticatedUser;
pub use rate_limit::{auth_rate_limit, refresh_rate_limit, api_rate_limit};
