fn main() {
    // lesson 1
    // let x = 123;
    // println!("Hello, world!"); // выводит с переносом на следующую строку
    // print!("{:f>10}", x); // вывод без переноса :>10 -выравнивание на 10 символов слева, 
    // //:b, :x, :o - приведение в разные системы исчисления  
    // let str = "гггг"; // string - много символов
    // let ch = 'c'; // char - один символ!
    // let arr = [1, 2, 3]; // array [] (массив) - длина не меняется, тип не меняется
    // println!("{:?}", arr); // вывод в изначальном представлении (массив по другому не вывести, как иначе??)
    // println!("{}", arr[2]); // вывод третьего элемента
    // // arr[0] = 5; - нельзя, если immutable
    // let mut arr1 = [1, 2, 3]; // mutable - изменяемая коллекция
    // arr1[0] = 5; // так можно
    // let tup = (1, " ", 3); // кортеж (можно менять отдельные элементы, но не тип и не длину)
    // let v = vec![1, 2, 3]; // вектор - фиксированный тип, но длина изменяемая
    

    // lesson 2
    // println!("{:?}", abc(5, 1).unwrap_or(0) + 2);
    let x: Result<u8, u8> = def(vec![1, 2, 3], 2);
    match x {
        Ok(number) => println!("Получено значение {}", number),
        Err(_) => println!("Ошибка"), // неиспользуемая переменная
    };
    // println!("{:?}", usize::MAX);
    // println!("{:?}", u64::MAX);

}

/*
fn abc(x: u8, y: u8) -> Option<u8>{ // lesson 2
    if x < y {
        return None;
    };
    return Some(x - y);
}
*/


fn def(v: Vec<u8>, i: u8) -> Result<u8, u8>{  // lesson 2
    if v.len() as u8 >= i + 1 {
        return Result::Ok(v[i as usize]); // подставится в 1 значение Result
    };
    return Result::Err(0); // Подставится во 2 значение Result
}

// aaaaaa
