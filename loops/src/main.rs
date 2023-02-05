fn main() {
    // loop - выполняет до бесконечности или до остановки
    let mut i = 10;
    let result = loop {
        println!("again!");
        i += 1;
        if i > 13 {
            break i; // Цикл может возвращать значения
        }
    }; // ; - завершение конструкции result = ...
    println!("result = {result}");

    // Если необходимо выйти из внешнего цикла, а не из внутреннего - можно создать метку цикла
    let mut count = 0;
    // Метка цикла:
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // While 
    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    // Можно проходиться сразу по элементам
    for element in a {
        println!("the value is: {element}");
    }
    // Range (1..4) - метод .rev добавит обратный отсчёт
    for number in 1..4 {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
