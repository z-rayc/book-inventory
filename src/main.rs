#[derive(PartialEq, Debug)]
struct Book {
    title: String,
    year: u32,
    code: String,
}

impl Book {
    // Create a new book
    fn new(title: String, year: u32, code: String) -> Book {
        Book {
            title: title,
            year: year,
            code: code,
        }
    }
}
