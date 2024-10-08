fn main() {
    println!("Rust has keywords, similar to other languages...");

    // appendix of rust keywords:
    // https://doc.rust-lang.org/stable/book/appendix-01-keywords.html

    let immutable = "immutable constant variable";

    println!("{immutable}");

    // concurrency - multiple computations happening at the same time
    // immutability is favorable for concurrency but you can't always use
    // a variable that is immutable - once it is bound to an identifier you cannot change the value

}
