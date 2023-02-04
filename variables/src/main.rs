// Константы неизменяемы ВСЕГДА (независимо от кода)
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // let - новая переменная (по умолчанию неизменяемая)
    // mut - изменяемая переменная
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    // Затенение переменной (помогает избежать spaces_str и spaces_num)
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    // Кортеж - единый комбинированный элемент с неизменяемой длинной (для изменяемой существуют векторы)
    let tup = (500, 6.4, 1);
    // Операция деструктуризации (destructuring)
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");

    // Можем обращаться напрямую
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let _one = x.2;
    // unit - кортеж без значение: () - пустой возвращаемый тип

    // В массиве все элементы одного типа, фиксированная длинна
    let _a: [i32; 5] = [1, 2, 3, 4, 5]; // Простая инициализация (можно опустить [i32, 5])
    let _a = [3; 5]; // let a = [3, 3, 3, 3, 3];

}

