use derive_new::new;

use super::document::Document;
use super::tag::Tag;

#[derive(new)]
pub struct DocumentTag<'a> {
    document : &'a Document<'a>,
    tag : Tag
}