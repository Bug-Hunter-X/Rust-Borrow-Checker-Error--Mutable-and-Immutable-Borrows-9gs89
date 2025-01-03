fn main() {
    let mut x = 5;
    {
        let y = &mut x; // Mutable borrow
        *y += 10;
        println!("x = {}", x); // x = 15
    } // Mutable borrow ends here

    // Now we can borrow x immutably.
    let z = &x;
    println!("x = {}", *z); // x = 15

    //Alternative, using clone:
    let mut x = 5;
    let y = &mut x;
    *y += 10;
    let z = x.clone(); // Clone to avoid mutable borrow conflict
    println!("x = {}", x);
    println!("z = {}", z);
} 