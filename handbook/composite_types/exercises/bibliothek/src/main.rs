#[derive(Debug)]
struct Book {
    id: u32,
    title: String,
    author: String,
}
#[derive(Debug, PartialEq)]
enum LoanStatus {
    Available,
    Borrowed,
    Returned,
    Overdue,
}
#[derive(Debug)]
struct Loan {
    loan_id: u32,
    book: Book,
    borrower_name: String,
    status: LoanStatus,
    due_day: u32,
}

impl Loan {
    fn new(loan_id: u32, book: Book, borrower_name: String, due_day: u32) -> Self {
        Self {
            loan_id,
            book,
            borrower_name,
            status: LoanStatus::Borrowed,
            due_day,
        }
    }
    fn update_status(&mut self, new_status: LoanStatus) {
        self.status = new_status;
    }
    fn is_overdue(&self, current_day: u32) -> bool {
        self.status == LoanStatus::Borrowed && current_day > self.due_day
    }
    fn summary(&self) -> String {
        format!(
            "Loan {}: '{}' borrowed by {}, due on day {}, status: {:?}",
            self.loan_id, self.book.title, self.borrower_name, self.due_day, self.status
        )
    }
    fn mark_returned(&mut self) {
        self.status = LoanStatus::Returned
    }
}

fn main() {
    let rust_book = Book {
        id: 2,
        title: String::from("The Rust Programming Handbook"),
        author: String::from("Francesco Ciulla"),
    };
    let go_book = Book {
        id: 1,
        title: String::from("Ultimate Go Notebook"),
        author: String::from("William Kennedy"),
    };
    let mut loan_rust = Loan::new(3, rust_book, String::from("Alex"), 30);
    let mut loan_go = Loan::new(1, go_book, String::from("Alex"), 15);
    println!(
        "{} has loaned {} written by {} and the loan status in our system is {:?}",
        loan_rust.borrower_name, loan_rust.book.title, loan_rust.book.author, loan_rust.status
    );
    loan_rust.update_status(LoanStatus::Returned);
    println!(
        "{} has returned {} written by {} and the loan status in our system is {:?}",
        loan_rust.borrower_name, loan_rust.book.title, loan_rust.book.author, loan_rust.status
    );
    println!("{}", loan_rust.summary());
    loan_go.is_overdue(17);
    println!("{}", loan_go.summary());
    loan_go.mark_returned();
    println!("{}", loan_go.summary());
}
