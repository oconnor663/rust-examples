// Rust won't even compile this!
fn bad_ptr<'a>(x: i32) -> &'a i32 {
    let y = &x;
    y
}

fn main() {
    // It doesn't matter what's in main(). The bad_ptr() function by itself
    // already causes a compile-time error.
}
