// перечисление, для них также можно объявлять методы в impl
enum IpAddrKind {
    V4,
    V6,
}

fn route(_ip_kind: IpAddrKind) {}

// перечисление с соответсвующим значениями
enum IpAddr {
    // У каждого типа адреса могут быть свои значения, т.е. можно так: V4(u8, u8, u8, u8),
    V4(String),
    V6(String),
}

/*
Полезное перечисление Option выглядит так:
enum Option<T> {
    None,
    Some(T),
}
Оно уже включено в прелюдию
*/

// Для теста match
#[derive(Debug)]
enum UsState {
    Alabama,
    _Alaska,
    // --snip--
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    Nothing,
}

fn main() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let _home = IpAddr::V4(String::from("127.0.0.1"));
    let _loopback = IpAddr::V6(String::from("::1"));

    // Option (надо использовать там, где возможен null, и обрабатывать это)
    let _some_number = Some(5);
    let _some_char = Some('e');
    let _absent_number: Option<i32> = None;

    // Тест match
    let pen = Coin::Penny;
    let nic = Coin::Nickel;
    let dim = Coin::Dime;
    let qua = Coin::Quarter(UsState::Alabama);
    let noth = Coin::Nothing;
    dbg!(value_in_cents(pen));
    dbg!(value_in_cents(nic));
    dbg!(value_in_cents(dim));
    dbg!(value_in_cents(qua));
    dbg!(value_in_cents(noth));

    // if let
    /*
    можно было бы:
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    Но вместо этого:
    */
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    // Как _ в match:
    else {
        println!("Not available")
    }
}

// Так работает match, в отличие от if - возвращает любой тип
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Т.к. у Quarter есть поле, можем использовать его здесь:
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
        // Для всех остальных (other - это переменная, которую мы решили так назвать, если не используется - можно просто _)
        _other => 0,
    }
}
