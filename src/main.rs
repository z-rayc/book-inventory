use std::collections::HashMap;
use std::io;
use std::io::Write; // For flush

type BookId = u16;

#[derive(PartialEq, Debug)]
struct Book {
    book_id: BookId,
    title: String,
    author: String,
    year: u16,
    available: bool,
}

const HR_DIV: &str = "-------------------------";

fn print_line() {
    println!("{}", HR_DIV.to_string());
}

impl Book {
    // Create a new book
    fn new(book_id: BookId, title: String, author: String, year: u16) -> Book {
        Book {
            book_id: book_id,
            title: title,
            author: author,
            year: year,
            available: true,
        }
    }

    fn print_info(&self) {
        println!(
            "Title: {}, author: {}, year: {}, ID: {}, available: {}",
            self.title, self.author, self.year, self.book_id, self.available
        );
    }

    fn borrow_book(&mut self) {
        self.available = false;
    }

    fn return_book(&mut self) {
        self.available = true;
    }
}

#[derive(Debug)]
struct Library {
    id_counter: u16,
    books: HashMap<BookId, Book>,
}

impl Library {
    // Create a new library
    fn new() -> Library {
        Library {
            id_counter: 0,
            books: HashMap::new(),
        }
    }

    // Add a book to the library
    fn add_book(&mut self, title: String, author: String, year: u16) {
        let id: BookId = self.id_counter;

        let book: Book = Book::new(id, title, author, year);
        self.books.insert(id, book);

        self.id_counter += 1;
    }

    // Returns a reference to the book with the given ID
    fn get_book_by_id(&mut self, id: BookId) -> Option<&mut Book> {
        self.books.get_mut(&id)
    }

    fn get_books_by_title(&mut self, title: String) -> Vec<&Book> {
        let mut books_by_title = Vec::<&Book>::new();
        for (_, item) in self.books.iter() {
            if item.title == title {
                books_by_title.push(item);
            }
        }
        books_by_title
    }

    fn get_books_by_author(&mut self, author: String) -> Vec<&Book> {
        let mut books_by_author = Vec::<&Book>::new();
        for (_, item) in self.books.iter() {
            if item.author == author {
                books_by_author.push(item);
            }
        }
        books_by_author
    }

    // Remove the book with the given ID from the library
    fn remove_book(&mut self, id: BookId) -> bool {
        let found = self.get_book_by_id(id);
        match found {
            Some(_) => {
                self.books.remove(&id);
                true
            }
            None => false,
        }
    }

    fn print_books(&self) {
        println!("Books in the library: ");
        print_line();

        for i in 0..(self.books.len() as u16) {
            let item = self.books.get(&i);
            match item {
                Some(book) => book.print_info(),
                None => continue,
            }
        }
        print_line();
    }

    // Fill the library with some books
    fn fill_with_books(&mut self) {
        self.add_book("Snømannen".to_string(), "Jo Nesbø".to_string(), 2007);
        self.add_book("Panserhjerte".to_string(), "Jo Nesbø".to_string(), 2009);
        self.add_book("Gjenferd".to_string(), "Jo Nesbø".to_string(), 2011);
        self.add_book("Politi".to_string(), "Jo Nesbø".to_string(), 2013);
        self.add_book("Tørst".to_string(), "Jo Nesbø".to_string(), 2017);
        self.add_book("Kniv".to_string(), "Jo Nesbø".to_string(), 2019);
        self.add_book("Blodmåne".to_string(), "Jo Nesbø".to_string(), 2022);
    }
}

fn print_book_list(list: Vec<&Book>) {
    for book in list {
        book.print_info();
    }
}

fn get_user_number() -> u16 {
    loop {
        let mut input = String::new();

        // Get user input
        io::stdin()
            .read_line(&mut input)
            .expect("Number not recognised.");

        // Parse input to integer
        let input: u16 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        return input;
    }
}

fn user_add_book(library: &mut Library) {
    print!("Enter the title: ");
    io::stdout().flush().unwrap();
    let title = get_user_string();

    print!("Enter the author: ");
    io::stdout().flush().unwrap();
    let author = get_user_string();

    print!("Enter the year: ");
    io::stdout().flush().unwrap();
    let year = get_user_number();

    library.add_book(title, author, year);
    println!("Book added to library.")
}

fn user_remove_book(library: &mut Library) {
    print!("Enter the ID: ");
    io::stdout().flush().unwrap();

    let book_id = get_user_number();
    let removed = library.remove_book(book_id);

    match removed {
        true => println!("The book with ID {} was successfully removed.", book_id),
        false => println!("The book was not found."),
    }
}

fn get_user_string() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Word not recognised.");

    // Remove trailing \r\n or \n
    s.as_str()
        .strip_suffix("\r\n")
        .or(s.strip_suffix("\n"))
        .unwrap_or(&s);

    s.to_string()
}

fn user_get_books_by_string(library: &mut Library, x: String) -> Vec<&Book> {
    print!("Enter the {}: ", x);
    io::stdout().flush().unwrap();

    let s = get_user_string();

    /* let title = "title";
    let author = "author"; */

    if x == "title".to_string() {
        library.get_books_by_title(s)
    } else if x == "author".to_string() {
        library.get_books_by_author(s)
    } else {
        Vec::new()
    }
}

fn user_get_books_by_title(library: &mut Library) -> Vec<&Book> {
    return user_get_books_by_string(library, "title".to_string());
}

fn user_get_books_by_author(library: &mut Library) -> Vec<&Book> {
    return user_get_books_by_string(library, "author".to_string());
}

fn user_get_book_by_id(library: &mut Library) -> Option<&mut Book> {
    print_line();
    print!("Enter the ID: ");
    io::stdout().flush().unwrap();

    let id = get_user_number();
    library.get_book_by_id(id)
}

fn show_menu() {
    println!("\n### Menu ###");
    print_line();
    println!("0. Quit");
    println!("1. Add a book");
    println!("2. Remove a book");
    println!("3. Find books by title");
    println!("4. Find books by author");
    println!("5. Find book by ID");
    println!("6. Borrow a book");
    println!("7. Return a borrowed book");
    println!("8. List all books");

    println!("\nPlease enter a number between 0 and 8.\n");
}

fn show_text_interface_loop(library: &mut Library) {
    let mut running: bool = true;
    while running {
        show_menu();

        let user_choice = get_user_number();

        match user_choice {
            0 => {
                println!("Exiting the application. Goodbye.");
                running = false;
            }
            1 => {
                println!("\n### Add a book ###");
                print_line();
                user_add_book(library)
            }
            2 => {
                println!("\n### Remove a book ###");
                print_line();
                user_remove_book(library)
            }
            3 => {
                println!("\n### Find books by title ###");
                print_line();
                print_book_list(user_get_books_by_title(library))
            }
            4 => {
                println!("\n### Find books by author ###");
                print_line();
                print_book_list(user_get_books_by_author(library))
            }
            5 => {
                println!("\n### Find book by ID ###");
                let book = user_get_book_by_id(library);
                match book {
                    Some(b) => b.print_info(),
                    None => println!("Could not find the book with the given ID."),
                }
            }
            6 => {
                println!("\n### Borrow book by ID ###");
                let book = user_get_book_by_id(library);
                match book {
                    Some(b) => {
                        if b.available == true {
                            b.borrow_book();
                            println!("The book was successfully borrowed.")
                        } else {
                            println!("The book could not be borrowed, because it is unavailable.")
                        }
                    }
                    None => println!("Could not find the book to borrow."),
                }
            }
            7 => {
                println!("\n### Return book by ID ###");
                let book = user_get_book_by_id(library);
                match book {
                    Some(b) => {
                        if b.available == false {
                            b.return_book();
                            println!("The book was successfully returned.")
                        } else {
                            println!("The book could not be returned, because it was not borrowed.")
                        }
                    }
                    None => println!("Could not find the book to return."),
                }
            }
            8 => library.print_books(),
            _ => println!("Input not recognised. Try again.\n"),
        }
    }
}

fn main() {
    // Initialise library
    let mut library: Library = Library::new();
    library.fill_with_books();

    show_text_interface_loop(&mut library);
}
