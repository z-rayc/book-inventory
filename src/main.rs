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

    // Find a reference to a book from its code
    fn find_book(&mut self, code: String) -> Option<&Book> {
        for item in self.books.iter() {
            if (item).code == code {
                return Some(&item);
            }
        }
        None
    }

fn main() {
    // Create objects
    let mut library = Library::new();
    let book = Book::new("Harry Potter".to_string(), 2002, "F34".to_string());

    // Add book to library
    library.add_book(book);

    // Remove book from library
    library.remove_book("F34".to_string());
    println!("{:?}", &library);
}
