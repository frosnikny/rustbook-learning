
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
}
