//! Utilities for testing a CouchDB application.

mod fake_server;

pub use self::fake_server::FakeServer;
pub use crate::document::DocumentBuilder;
pub use crate::view::{IsGrouped, IsReduced, IsUnreduced, ViewResponseBuilder};
