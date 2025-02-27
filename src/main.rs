use itertools::Itertools;

fn main() {
    let mut m = [1, 2, 3];
    m = m.map(|elem| elem+1);
    println!("{:?}", m)

    let k = m.iter().fold(1, |acc, e| acc * e);
    println!("{:?}", k)
}

fn generate_range(start: usize, stop: usize, step: usize) -> Vec<usize> {
    (start..=stop).step_by(step).collect()
}

fn name_shuffler(s: &str) -> String {
    println!("{:?}", s.split_ascii_whitespace().rev().join(" "));
    // s.rsplit(' ').join(" ")
    String::new()
}

fn multi_table(n: u64) -> String {
    (1..=10).map(|c|format!("{} * {} = {}", c, n, c*n)).join("\n")
}

fn sort_numbers(arr: &Vec<i32>) -> Vec<i32> {
    let mut ans = arr.clone();
    ans.sort_unstable();
    ans
  }