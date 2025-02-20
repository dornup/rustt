use itertools::Itertools;

fn fake_bin(s: &str) -> String {
    s.chars().map(|c|match c {
        '0'..='4'=>'0',
        _ => '1'
    }).collect()
}

fn spacify(s: &str) -> String {
    s.chars().join(" ")
}

fn main() {
    println!("{:?}", 2_i32.pow(10));
    let x = vec![1,2,3]
    .into_iter()
    .fold(1, |acc, i| {acc * i});
    // .collect::<Vec<i32>>()
    println!("{:?}", x)
}