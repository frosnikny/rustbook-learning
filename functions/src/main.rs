fn main() {
    println!("Hello, world!");

    another_function(10);
    let x = return_function(true);
    println!("{x}");
    let x = return_function(false);
    println!("{x}");
}

// Rust не важно, где вы определяете свои функции, главное, чтобы они были определены где-то в той области видимости
fn another_function(x: i32) {
    println!("Another function, x = {x}");
}

// Операторы - это инструкции, которые выполняют какое-либо действие и не возвращают значение. Выражения вычисляют результирующее значение.

fn return_function(a: bool) -> i32 {
    if a {
        return 0;
    }
    let _y = 2; // оператор, т.к. невозможно let x = (let y = 6);
    let y = {
        let x = 3;
        x + 1
    }; // Справа от = - выражение (не включает ; в конце!!)
    y // Функции неявно возвращают последнее выражение
}