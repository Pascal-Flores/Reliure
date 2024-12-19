use std::path::Path;

use chrono::NaiveDate;
use derive_new::new;

#[derive(new)]
pub struct Document<'a> {
    id : i32,
    category : &'a Category<'a>,
    author : Author,
    series : Series,
    date : NaiveDate,
    path : &'a Path
} 

#[derive(new)]
pub struct Category<'a> {
    id : i32,
    name : &'a String,
    path : &'a Path
}

#[derive(new)]
pub struct Genre {
    id : i32,
    name : String
}

#[derive(new)]
pub struct GenreDocument<'a> {
    document : &'a Document<'a>,
    genre : &'a Genre
}

#[derive(new, PartialEq, Debug)]
pub struct Author {
    id : i32,
    name : String
}

#[derive(new)]
pub struct Series {
    id : i32,
    name : String
}

#[derive(new)]
pub struct Tag {
    id : i32,
    name : String
}

#[derive(new)]
pub struct AuthorSeries {
    author : Author,
    series : Series
}

#[derive(new)]
pub struct DocumentTag<'a> {
    document : &'a Document<'a>,
    tag : Tag
}

