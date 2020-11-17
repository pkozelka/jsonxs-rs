//! Offers simple API to serialize existing data structure into JSON.
//! Instead of defining dedicated (serde-annotated) structures, and writing code that fills them prior save, you just save what you have, directly.
mod jsonxs;

pub use crate::jsonxs::JsonXsSerializer;
pub use crate::jsonxs::JsonXsValue;
