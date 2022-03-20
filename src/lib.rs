//! We support GET and POST HTTP methods. The response contains a JSON object, which always has a
//! Boolean field ok. If ok equals true, the request was successful, and the result of the query
//! can be found in the result field. In case of an unsuccessful request, ok equals false, and the
//! error is explained in the error field (e.g. SHORT_NAME_REQUIRED). All queries must be made
//! using UTF-8.
//!
pub mod entity;
pub mod methods;
#[cfg(test)]
pub(crate) mod entity_test;
#[cfg(test)]
pub(crate) mod methods_test;

pub use entity::*;
pub use methods::*;
