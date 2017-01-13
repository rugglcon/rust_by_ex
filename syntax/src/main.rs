fn main() {
    //let x: i32;

    println!("Hello, world!");
    println!("{}", add_one(4));
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn diverges() -> ! {
    panic!("panic");
}
