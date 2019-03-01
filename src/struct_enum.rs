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
    fn set_isbn(&mut self, isbn: &str){
        self.isbn = isbn.to_string();
    }
}

enum Months{
    January,
    February,
    March,
    April
}

fn gues_month(m: Months) {
  // Perform action depending on info
  match m {
    Months::January => println!("It's January"),
    Months::February => println!("It's February"),
    Months::March => println!("It's March"),
    Months::April => println!("It's April"),
  }
}

pub fn run_struct(){
    println!("########## Hi {} Struct here ##########", "Alex");
    let mut book = Book::new("Road home","2134532",1);
    book.set_isbn("23AD21A34");
    println!("My book {} {} {}", book.get_title(), book.isbn, book.id);

}

pub fn run_enum(){

    let month1 = Months::January;
    let month2 = Months::February;
    let month3 = Months::March;
    let month4 = Months::April;

    gues_month(month1);
    gues_month(month2);
    gues_month(month3);
    gues_month(month4);

}