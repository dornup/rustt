fn main() {
    let x = 123;
    println!("Hello, world!"); // выводит с переносом на следующую строку
    print!("{:f>10}", x); // вывод без переноса :>10 -выравнивание на 10 символов слева, 
    //:b, :x, :o - приведение в разные системы исчисления  
    let str = "гггг"; // string - много символов
    let ch = 'c'; // char - один символ!
    let arr = [1, 2, 3]; // array [] (массив) - длина не меняется, тип не меняется
    println!("{:?}", arr); // вывод в изначальном представлении (массив по другому не вывести, как иначе??)
    println!("{}", arr[2]); // выводтретьего элемента
    // arr[0] = 5; - нельзя, если immutable
    let mut arr1 = [1, 2, 3]; // mutable - изменяемая коллекция
    arr1[0] = 5; // так можно
    let tup = (1, " ", 3); // кортеж (можно менять отдельные элементы, но не тип и не длину)

    let v = vec![1, 2, 3]; // вектор - фиксированный тип, но длина изменяемая
    

}
