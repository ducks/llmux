//! Apply and verify module for llmux
//!
//! This module handles:
//! - Parsing edit formats from LLM output (unified diff, old/new pairs, full file)
//! - Applying edits to files with fuzzy matching
//! - Creating backups before modifications
//! - Running verification commands
//! - Rollback on verification failure
//! - Retry loop with error context
//!
//! # Example
//!
//! ```ignore
//! use llmux::apply_and_verify::{apply_and_verify, ApplyVerifyConfig};
//!
//! let config = ApplyVerifyConfig {
//!     verify_command: Some("cargo test".into()),
//!     verify_retries: 2,
//!     ..Default::default()
//! };
//!
//! let result = apply_and_verify(llm_output, &config, working_dir).await?;
//!
//! if result.success {
//!     println!("Edits applied and verified!");
//! }
//! ```

mod diff_applier;
mod edit_parser;
mod retry_loop;
mod rollback;
mod verification;

pub use diff_applier::{ApplyError, ApplyResult, DiffApplier, ModifiedFile};
pub use edit_parser::{
    DiffHunk, DiffLine, EditOperation, EditParseError, normalize_whitespace, parse_edits,
};
pub use retry_loop::{
    ApplyVerifyConfig, ApplyVerifyError, ApplyVerifyResult, AttemptResult, apply_and_verify,
    apply_only,
};
pub use rollback::{RollbackError, RollbackResult, RollbackStrategy, cleanup_backups, rollback};
pub use verification::{VerifyError, VerifyResult, run_verify};
