//! Toast notification component
//!
//! Provides toast notifications with different levels (success, error, info, warning).

use std::time::{Duration, Instant};

pub mod constructors;
pub mod methods;

/// Toast notification level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastLevel {
    Success,
    Error,
    Info,
    Warning,
}

/// A single toast notification
#[derive(Debug, Clone)]
pub struct Toast {
    pub message: String,
    pub level: ToastLevel,
    pub created_at: Instant,
    pub duration: Duration,
}

/// Manages multiple toast notifications
#[derive(Debug, Default)]
pub struct ToastManager {
    toasts: Vec<Toast>,
    max_toasts: usize,
}
