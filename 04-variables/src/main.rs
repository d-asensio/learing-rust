fn mutable_variable () {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn shadowed_variable () {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x in the outer scope is: {}", x);
}

fn main() {
    println!("-- Mutable variable --");
    mutable_variable();

    println!("-- Shadowed variable --");
    shadowed_variable();
}
