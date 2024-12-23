use std::path::Path;

pub(crate) mod document;
pub(crate) mod category;
pub(crate) mod genre;
pub(crate) mod genre_document;
pub(crate) mod author;
pub(crate) mod series;
pub(crate) mod tag;
pub(crate) mod document_tag;
pub(crate) mod user;
pub(crate) mod schema;

pub use schema::*;
pub use document::*;
pub use category::*;
pub use genre::*;
pub use genre_document::*;
pub use author::*;
pub use series::*;
pub use tag::*;
pub use document_tag::*;

