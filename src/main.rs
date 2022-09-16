use trix::trix;

fn main() {
    let trix: trix::Trix = trix::Trix::new();
    let trix_size = std::mem::size_of_val(&trix);

    println!("The initial Trix instance size is: {:#?}\n", trix_size);
    println!("{:#?}", trix);
}
