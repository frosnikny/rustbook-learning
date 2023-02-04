// Для вывода могут использоваться разные типажи. По стандарту для {} - Display
// Есть также Debug {:?}, для этого над структурой напишем:
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Функция создания юзера
fn _build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // Сюда просто перейдет параметр с таким же именем
        email,
        sign_in_count: 1,
    }
}

// Структура без именованных полей
struct Color(i32, i32, i32);

// Единично-подобная структура (подобно ()) - не имеет полей
struct AlwaysEqual; // Реализует типаж для элемента, но нет данных для хранения

fn main() {
    // Стоит отметить, что весь экземпляр структуры должен быть изменяемым (или неизменяемым);
    //Rust не позволяет помечать изменяемыми отдельные поля.
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("{}", user1.email);
    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1.email);

    // Всё берется у user1 кроме email
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // let user3 = User; - нельзя создать, не заполняя

    let _black = Color(0, 0, 0);

    let _subject = AlwaysEqual;

    // {:#?} - привед к более красивому выводу, также можно печатать в поток ошибок с помощью макроса dbg!
    println!("user2: {:?}", user2);
}
