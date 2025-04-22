use std::collections::HashMap;



fn main (){
    let mut a = HashMap::new();
    // a[&5] = 1
    println!("{:?}", a.insert("anton", 1));
    println!("{:?}", a.insert("bogdan", 2));
    // let s = String::from("123");
    println!("{:?}", a);
    let mut b = HashMap::from([
        (0, 9)
    ]);
    println!("{:?}", b.entry(0));
    println!("{:?}", b.entry(1));
    b.entry(0)
    .and_modify(|v|*v += 1)
    .or_insert(0);
    println!("{:?}", b)

}

// codewars

fn count_red_beads(n: u32) -> u32 {
    //     if n > 1 {
    //         (n-1) * 2
    //     } else {
    //         0
    //     }
        n.saturating_sub(1) * 2
    }

fn parse(code: &str) -> Vec<i32> {
    let mut ans = vec![];
    code.chars().fold(0, |acc, ch| match ch {
        'i' => acc + 1,
        'd' => acc - 1,
        's' => acc * acc,
        'o' => {
            ans.push(acc);
            acc
        },
        _ => acc
    });
    ans
}