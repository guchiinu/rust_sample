fn main() {
    variable();
}

fn variable() {
    let mut a: i32;
    let b: i32;

    a = 1;
    b = 2;

    a = a + b;

    println!("{}", a);
}