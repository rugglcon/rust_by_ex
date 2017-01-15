fn main() {
    let mut x = 5;

    //if
    if x == 5 {
        println!("x is five");
    }

    //assigning a value with if/else
    let mut y = if x == 5 {10} else {15};
    println!("{}", y);

    //for
    for x in 0..10 {
        println!("{}", x);
    }

    //enumerations
    for (index, value) in (5..10).enumerate() {
        println!("index = {}, value = {}", index, value);
    }

    //iterations
    let lines = "hello\nworld".lines();

    for (linenumber, line) in lines.enumerate() {
        println!("{}: {}", linenumber, line);
    }

    //breaking iterations
    let mut m = 5;
    let mut done = false;
    loop {
        m += m - 3;
        println!("{}", m);
        if m % 5 == 0 {break;}
    }

    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 {continue 'outer;}
            if y % 2 == 0 {continue 'inner;}
            println!("x: {}, y: {}", x, y);
        }
    }
}
