#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// implementation - реализация. Все функции в нем - ассоциированные. Может быть несколько таких блоков.
impl Rectangle {
    // метод
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // просто ассоциированная функция
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    
    // для вызова ассоциированной функции используем ::
    let sq = Rectangle::square(3);
    println!("sq={:#?}", sq);
}
