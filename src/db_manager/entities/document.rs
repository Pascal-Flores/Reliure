use derive_new::new;
use chrono::NaiveDate;

use super::category::Category;
use super::author::Author;
use super::series::Series;
use std::path::Path;


#[derive(new)]
pub struct Document {
    id : i32,
    category : Category,
    author : Author,
    series : Series,
    date : NaiveDate,
    path : String
} 