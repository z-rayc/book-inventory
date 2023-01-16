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

#[derive(Debug)]
struct Library {
    books: Vec<Book>,
}

impl Library {
    // Create a new library
    fn new() -> Library {
        Library {
            books: Vec::<Book>::new(),
        }
    }

    // Add a book to the library
    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }
}
