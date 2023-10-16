/* pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
} */

#[derive(Debug)]
pub enum Status {
    Active(String),
    Inactive(String),
    Suspended(String),
}

#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub pages: i32,
}

pub fn display_book(book: &Book) {
    println!("Book title: {}", book.title);
    println!("Book author: {}", book.author);
    println!("Book pages: {}", book.pages);
}
