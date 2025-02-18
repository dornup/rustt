fn main() {
    let x = 50;

    /*
    match x {
        10 => println!("{:?}", 10),
        0..=50 => println!("{:?}", 5), // .. - невключительно ..= включительно
        _ => println!("{:?}", 0), // _ - все оставшиеся случаи
    }
    */

    if x > 20 {
        println!("{:?}", 20)
    } else if x > 10{
        println!("{:?}", 0)
    } else {
        println!("{:?}", 1)
    }

    for i in (0..5).rev().step_by(2) {
        println!("{:?}", i)
    }

    let s = "abc";
    println!("{:?}", s.to_uppercase())
}
