fn main() {
    let number = "42".parse::<i32>();
    let x = match number {
        Ok(value) => value,
        Err(er) => {
            println!("Не получилось((( {}", er);
            return;
        },    
    };
}
