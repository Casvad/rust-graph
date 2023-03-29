mod models;
mod schema;
mod repositories;

//#![deny(missing_docs)]
fn main() {
    println!("Startup graphql");
    validate_args()
}

fn validate_args(){
    let args = std::env::args();
    let mut skip_args = args.skip(1);
    let key = skip_args.next().unwrap(); // crash the program if no arg is send
    let value = skip_args.next().expect("The value was not set"); // the same as key but prettier
    println!("The key is {}", debug_option)
}