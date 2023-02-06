fn main() {
    // Вектор
    let _v: Vec<i32> = Vec::new();
    // С помощью макроса
    let mut v = vec![1, 2, 3];
    // Добавить в вектор
    v.push(5);

    // Получаем ссылку на элемент
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // Получаем Option<&i32>
    let third = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // В for изменяемый
    for i in &mut v {
        *i += 1;
    }

    // В for
    for i in &v {
        println!("{i}");
    }
}
