fn main() {
    // Создание строки

    let mut s = String::new();

    let data = "initial contents";

    // Преобразовать данный (в которые есть типаж Display) к строке
    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    // Можно через from
    let s = String::from("initial contents");

    // Добавление к строке

    let mut s = String::from("foo");
    s.push_str("bar");

    // Можно через другую строку
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    // Или через сложение строк
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // Более удобно будет через format!
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");

    // Срез строки
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{s}");

    // Методы перебора строк

    for c in "Зд".chars() {
        println!("{}", c);
    }

    for b in "Зд".bytes() {
        println!("{}", b);
    }
}
