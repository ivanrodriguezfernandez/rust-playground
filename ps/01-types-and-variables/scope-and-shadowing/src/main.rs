fn scope_and_shadowing() {
    let a = 123;
    {
        let b = 456;
        println!("inside, b ={}", b);
        let a = 77;
        println!("inside, a={}", a);
    }
    println!("a={}", a);
    //println!("outside, b= {}", b);
}

fn main() {
    //println!("{}", a);
    scope_and_shadowing();
}
