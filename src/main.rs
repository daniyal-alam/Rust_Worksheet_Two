extern crate bookshelf;

use crate::bookshelf::{display_book, Book};

fn main() {
    let book1 = Book {
        title: String::from("Fourty rules of life "),
        author: String::from("Daniyal."),
        pages: 100,
    };

    display_book(&book1);
}
