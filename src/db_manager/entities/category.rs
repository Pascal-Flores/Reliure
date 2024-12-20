use derive_new::new;
use std::path::Path;

#[derive(new)]
pub struct Category<'a> {
    id : i32,
    name : &'a String,
    path : &'a Path
}