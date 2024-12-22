use diesel::{table, joinable, allow_tables_to_appear_in_same_query};

table! {
    author (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    series (id) {
        id -> Integer,
        author -> Integer,
        name -> Text,
    }
}

table! {
    tag (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    genre (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    category (id) {
        id -> Integer,
        name -> Text,
        path -> Text,
    }
}

table! {
    document (id) {
        id -> Integer,
        name -> Text,
        category -> Integer,
        author -> Integer,
        series -> Integer,
        date -> Text,
        path -> Text,
    }
}

table! {
    document_category (document, category) {
        document -> Integer,
        category -> Integer,
    }
}

table! {
    author_series (author, series) {
        author -> Integer,
        series -> Integer,
    }
}

table! {
    document_genre (document, genre) {
        document -> Integer,
        genre -> Integer,
    }
}

table! {
    document_tag (document, tag) {
        document -> Integer,
        tag -> Integer,
    }
}

joinable!(document -> author (author));
joinable!(document -> category (category));
joinable!(document -> series (series));
joinable!(document_category -> category (category));
joinable!(document_category -> document (document));
joinable!(author_series -> author (author));
joinable!(author_series -> series (series));
joinable!(document_genre -> document (document));
joinable!(document_genre -> genre (genre));
joinable!(document_tag -> document (document));
joinable!(document_tag -> tag (tag));

allow_tables_to_appear_in_same_query!(
    author,
    series,
    tag,
    genre,
    category,
    document,
    document_category,
    author_series,
    document_genre,
    document_tag,
);
