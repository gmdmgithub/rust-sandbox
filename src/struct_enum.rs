struct Book{
    title: String,
    isbn: String,
    id: i32
}

impl Book{

    // Construct Book
    fn new(tit: &str, isb: &str, i: i32) -> Book {
        Book {
            title: tit.to_string(),
            isbn: isb.to_string(),
            id: i
        }
    }
    //use self to obtain own (struct) variables
    fn get_title(&self) -> String{
        format!("My book: {}",self.title)
    }
}

pub fn run_struct(){
    println!("########## Hi {} Struct here ##########", "Alex");
    let book = Book::new("Road home","2134532",1);
    println!("My book {} {} {}", book.get_title(), book.isbn, book.id);

}