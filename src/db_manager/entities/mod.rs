use std::path::Path;

use chrono::NaiveDate;

pub struct Document<'a> {
    id : i32,
    category : &'a Category<'a>,
    author : Author,
    series : Series,
    date : NaiveDate,
    path : Path
} 

pub struct Category<'a> {
    id : i32,
    name : &'a String,
    path : Path
}

pub struct Genre {
    id : i32,
    name : String
}

pub struct GenreDocument<'a> {
    document : &'a Document<'a>,
    genre : &'a Genre
}

pub struct Author {
    id : i32,
    name : String
}

pub struct Series {
    id : i32,
    name : String
}

pub struct Tag {
    id : i32,
    name : String
}

pub struct AuthorSeries {
    author : Author,
    series : Series
}

pub struct DocumentTag<'a> {
    document : &'a Document<'a>,
    tag : Tag
}

