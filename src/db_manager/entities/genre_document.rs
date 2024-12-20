use derive_new::new;

use super::document::Document;
use super::genre::Genre;

#[derive(new)]
pub struct GenreDocument<'a> {
    document : &'a Document<'a>,
    genre : &'a Genre
}