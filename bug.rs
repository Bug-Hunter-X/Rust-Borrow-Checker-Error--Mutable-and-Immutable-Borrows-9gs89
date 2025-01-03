fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x; // z is an immutable reference to x

    // This is okay
    *y += 10;
    println!("x = {}", x); // x = 15

    // This will cause a compiler error. Cannot borrow `x` as immutable because it is also borrowed as mutable.
    println!("x = {}", *z);
}