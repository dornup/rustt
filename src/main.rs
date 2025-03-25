fn add(a:i32, b:i32) -> i32 { // обычная функция, а хочеца анонимную(
    a + b // tail expression
}

fn main() {
    let add2 = |a, b| a + b  // ее анонимное + можно сделать {}
    add(2, 3);
    add2(2, 3);

    let b = 3;
    let add3 = |a| a + b; // берет значения извне, в отличие от обычной fn
    add3(2);

    let x = 3;
    println!("{:05}", x) // 0>5 - выравнивание слева нули
    println!("{:.2}", x) // цифры после запятой
    println!("{}", (3_f64.sqrt().round() as u32).pow(2)) // - квадраты и степени

}

// задачки с codewars

// Convert a Number to a String!
fn number_to_string(i: i32) -> String {
    // i.to_string() - работает
    format!("{}", i)
}

// Number of Decimal Digits
fn digits(n: u64) -> usize {
    n
    .to_string()
    .len()
    // n.to_string().chars().count() // - то же самое
  }

// Sum of angles
fn angle(n: u32) -> u32 {
    (n - 2) * 180
  }

// Substituting Variables Into Strings: Padded Numbers
fn solution(n: u32) -> String {
    format!("Value is {:05}", n)
}

// Dollars and Cents
fn format_money(amount: f64) -> String {
    format!("${:.2}", amount)
}

// Find Nearest square number
fn nearest_sq(n: u32) -> u32 {
    (n as f64)
    .sqrt()
    .round()
    .powi(2) as _
}

//Ones and Zeros
fn binary_slice_to_number(slice: &[u32]) -> u32 {
    //slice.iter().rev().enumerate().map(|(i,e)| e*2_u32.pow(i as u32)).sum::<u32>() // - map + sum = fold
    slice
    .iter()
    .rev()
    .enumerate()
    .fold(0, |acc, (i,e)|acc + e*2_u32.pow(i as u32))
}