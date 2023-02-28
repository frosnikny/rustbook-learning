// Тип HashMap<K, V> хранит ключи типа K на значения типа V.
// Данная структура организует и хранит данные с помощью функции хеширования.
// Хеш-карты полезны, когда нужно искать данные с помощью ключа, который может быть любого типа.

fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // Возвращает Option<&V>, затем вызывает copied для Option<V>, затем распаковывает значение или 0
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // Перезапись старых значений
    scores.insert(String::from("Blue"), 25);

    // Вставка значения в том случае, если значения нет
    scores.entry(String::from("Red")).or_insert(50);
    // Сам по себе or_insert - возвращает изменяемую ссылку на значение ключа
    scores.entry(String::from("Blue")).or_insert(50);
    // Тогда можно делать так:
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    /* По умолчанию HashMap использует функцию хеширования SipHash
    Это не самый быстрый из возможных алгоритмов хеширования, в данном случае производительность
    идёт на компромисс с обеспечением лучшей безопасности. Если после профилирования вашего кода окажется,
    что хеш-функция, используемая по умолчанию, очень медленная, вы можете заменить её используя другой hasher.
    Hasher - это тип, реализующий трейт BuildHasher.
    crates.io имеет достаточное количество библиотек, предоставляющих разные реализации
    hasher с множеством общих алгоритмов хеширования.*/
}
