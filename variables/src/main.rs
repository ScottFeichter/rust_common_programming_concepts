fn main() {
    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // trying to run this program will cause an error
    // the compiler has found the error cannot assign twice to immutable variable "x"
    // compile-time errors are very helpful to avoid bugs
    // need to use mut

    let mut y = 5;
    println!("The value of y is: {y}");
    y = 6;
    println!("The value of y is: {y}");
}
