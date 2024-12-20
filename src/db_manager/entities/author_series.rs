use derive_new::new;

use super::author::Author;
use super::series::Series;

#[derive(new)]
pub struct AuthorSeries {
    author : Author,
    series : Series
}