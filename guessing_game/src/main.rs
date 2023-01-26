use std::io; // io - библиотека ввода/вывода, часть стандартной библиотеки std

use std::cmp::Ordering; // Перечисление, сравнивающее числа и выдающее Less/Greater/Equal

use rand::Rng;
// Типаж Rng определяет методы, реализующие генераторы случайных чисел, и этот типаж должен быть в области видимости, 
// чтобы эти методы можно было использовать.

// крейт — это набор файлов с исходным кодом на Rust
/* Проект, который мы создавали, представляет собой двоичный крейт, являющийся исполняемым файлом. 
Крейт rand — библиотечный крейт, содержащий код, предназначенный для использования в других программах. 
Он не может быть выполнен сам по себе. */
// Для его добавления меняем Cargo.toml
// rand = "0.8.5"
// 0.8.5 - версия. Cargo понимает семантическое версионирование (SemVer) 
// => 0.8.5 здесь = ^0.8.5, то есть любая версия не ниже 0.8.5, но <0.9.0
// Т.к. у 0.8... API точно будет таким, какой нам нужно, а в 0.9 уже может отличаться

// Crates.io — это место, где участники экосистемы Rust размещают свои проекты с открытым исходным кодом для использования другими.
// Именно оттуда Cargo берет версии

/*Когда вы создаёте проект в первый раз, Cargo определяет все версии зависимостей, которые соответствуют критериям, 
а затем записывает их в файл Cargo.lock. Когда вы будете собирать свой проект в будущем, Cargo увидит, 
что файл Cargo.lock существует, 
и будет использовать указанные там версии, а не выполнять всю работу по выяснению версий заново.  */
// Это необходимо, чтобы новые версии не ломали проект. 

/*Если вы захотите обновить пакет, Cargo предоставляет команду update, 
которая игнорирует файл Cargo.lock и определяет последние версии, соответствующие вашим спецификациям из файла Cargo.toml.  
Но это обновление произойдёт до версии <0.9.0 в нашем случае, чтобы обновиться до нее - надо менять .toml*/

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    /*мы вызываем функцию rand::thread_rng, дающую нам генератор случайных чисел, который мы собираемся использовать: 
    тот самый, который является локальным для текущего потока выполнения и запускается операционной системой. 
    Затем мы вызываем его метод gen_range. 
    Этот метод определяется Rng, который мы включили в область видимости с помощью оператора use rand::Rng */
    // Тип используемого выражения диапазона принимает форму start..=end и включает обе границы

    // println!("The secret number is: {secret_number}");

    // loop - бесконечный цикл
    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // Создаем переменную:
        // let apples = 5; - обычное создание переменной
        // mut - добавляем переменной изменяемость
        // String - расширяемый фрагмент текста
        /* Синтаксис :: в строке ::new указывает, что new является ассоциированной функцией типа String. 
        Ассоциированная функция — это функция, реализованная для типа, в данном случае String. Функция new создаёт новую, пустую строку. 
        Функцию new можно встретить во многих типах, это типичное название для функции, которая создаёт новое значение какого-либо типа. */

        /* Функция stdin возвращает экземпляр std::io::Stdin, 
        который является типом, представляющим дескриптор стандартного ввода для вашего терминала. */
        io::stdin()
            .read_line(&mut guess)
            /* Далее строка .read_line(&mut guess) вызывает метод read_line на дескрипторе стандартного ввода 
            для получения ввода от пользователя. Мы также передаём &mut guess в качестве аргумента read_line, 
            ообщая ему, в какой строке хранить пользовательский ввод.
            Строковый аргумент должен быть изменяемым, чтобы метод мог изменить содержимое строки. */
            /* Символ & указывает, что этот аргумент является ссылкой, которая предоставляет возможность нескольким частям вашего кода 
            получить доступ к одному фрагменту данных без необходимости копировать эти данные в память несколько раз. 
            Чтобы сделать её изменяемой, нужно написать &mut guess, а не &guess. В главе 4 ссылки будут описаны более подробно. */

            .expect("Failed to read line");
            // Мы могли бы написать этот код так: io::stdin().read_line(&mut guess).expect("Failed to read line");
            // Но удобнее разделить

            /* Result, возвращаемый read_line — это перечисление, часто называемое enum, 
            то есть тип, который может находиться в одном из нескольких возможных состояний. 
            Мы называем каждое такое состояние вариантом. 
            Варианты Result — это Ok и Err. Вариант Ok указывает на то, что операция прошла успешно, 
            а внутри Ok находится успешно сгенерированное значение. 
            Вариант Err означает, что операция не удалась, а Err содержит информацию о том, как и почему это произошло.*/
            /* У экземпляра Result есть метод expect, который можно вызвать. 
            Если этот экземпляр Result является значением Err, expect вызовет сбой программы и отобразит сообщение, 
            которое вы передали в качестве аргумента. Если метод read_line возвращает Err, 
            то это, скорее всего, результат ошибки базовой операционной системы. Если экземпляр Result является значением Ok, 
            expect возьмёт возвращаемое значение, которое удерживает Ok, и вернёт вам только это значение, 
            чтобы вы могли его использовать далее. 
            В данном случае это значение представляет собой количество байтов, введённых пользователем. */

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
            // Подчёркивание _ является всеохватывающим выражением
        };
        // Rust позволяет нам затенять предыдущее значение guess новым. Затенение позволяет нам повторно использовать имя переменной guess, 
        // чтобы избежать создания двух уникальных переменных, таких как guess_str и guess, например
        /* Метод trim на экземпляре String удалит любые пробельные символы в начале и конце строки для того, 
        чтобы мы могли сопоставить строку с u32, который содержит только числовые данные.  
        При вводе 5 будет символ конца строки, а в Win и возврат коретки. Метод trim убирает \n или \r\n, оставляя только 5. */
        // Метод parse для строк преобразует строку в другой тип. Здесь мы используем его для преобразования строки в число. 
        // Нужный тип сообщаем Расту через :
    
        println!("You guessed: {guess}"); // Обычный format

        // cmp сравнивает два значения и возвращает Ordering
        /*Выражение match состоит из веток (arms). 
        Ветка состоит из шаблона для сопоставления и кода, который будет запущен, если значение, 
        переданное в match, соответствует шаблону этой ветки */
        // Выражение match заканчивается после первого успешного совпадения
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!");
                break;
            }
        }
    }
}
