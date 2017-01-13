fn main() {
    //types
    let x = true;
    let y:bool = false;

    if x {
        println!("hi");
    }

    if y {
        println!("bye");
    }
    
    //arrays
    let a = [1, 2, 3];
    //let mut m = [1, 2, 3];
    let mut n = [""; 20];

    let mut u = 0; 
    //while u < n.len() {
        //println!("{}", n[u]);
        //u += 1;
    //}
    //for x = 0; x < n.len(); x++ {
        //println!("{}", n[x]);
    //}
    
    
    //slices
    let p = [1, 2, 3, 4];
    let complete = &p[..];
    let middle = &p[1..4];

    //prints 2
    println!("{}", middle[0]);
    
    //functions
    fn foo(x:i32) -> i32 {x};
    let x: fn(i32) -> i32 = foo;
    println!("{}", x(5));
}
