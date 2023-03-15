use std::io;
use std::io::Write;

fn main() {
    println!("Выберите преобразование:");
    println!("1. Текст -> Шестнадцатеричный код");
    println!("2. Текст -> Двоичный код");
    println!("3. Текст -> Восьмеричный код");
    println!("4. Шестнадцатеричный код -> Текст");
    println!("5. Двоичный код -> Текст");
    println!("6. Восмеричный код -> Текст");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода");

    let choice = match input.trim().parse::<i64>() {
        Ok(n) => n,
        Err(_) => {
            println!("Неверный ввод");
            return;
        }
    };

    match choice {
        1 => {
            println!("Введите текст:");
            input.clear();
            io::stdin().read_line(&mut input).expect("Ошибка ввода");
            let text = input.trim();

            let hex = text
                .as_bytes()
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<String>>()
                .join(" ");
            println!("Шестнадцатеричный код: {}", hex);
        }
        2 => {
            println!("Введите текст:");
            input.clear();
            io::stdin().read_line(&mut input).expect("Ошибка ввода");
            let text = input.trim();

            let binary = text
                .as_bytes()
                .iter()
                .map(|b| format!("{:08b}", b))
                .collect::<Vec<String>>()
                .join(" ");
            println!("Двоичный код: {}", binary);
        }
        3 => {
            println!("Введите текст:");
            io::stdout().flush().unwrap(); // сбрасываем буфер вывода на экран
            input.clear();
            io::stdin().read_line(&mut input).expect("Ошибка ввода");
            let text = input.trim();

            let octal = text
                .chars()
                .map(|c| format!("{:o}", c as u32))
                .collect::<Vec<String>>()
                .join(" ");
            println!("Восьмеричный код: {}", octal);
        }
        4 => {
            println!("Введите шестнадцатеричный код:");
            input.clear();
            io::stdin().read_line(&mut input).expect("Ошибка ввода");
            let deleted_spaces: Vec<&str> = input.trim().split(" ").collect();

            let hex = deleted_spaces.join("");
            
            let text = hex
                .as_bytes()
                .chunks(2)
                .map(|b| u8::from_str_radix(std::str::from_utf8(b).unwrap(), 16).unwrap())
                .collect::<Vec<u8>>();

            println!("Текст: {}", std::str::from_utf8(&text).unwrap());
        }
        5 => {
            println!("Введите двоичный код:");
            input.clear();
            io::stdin().read_line(&mut input).expect("Ошибка ввода");

            let deleted_spaces: Vec<&str> = input.trim().split(" ").collect();

            let binary = deleted_spaces.join("");
            
            let text = binary
                .as_bytes()
                .chunks(8)
                .map(|b| u8::from_str_radix(std::str::from_utf8(b).unwrap(), 2).unwrap())
                .collect::<Vec<u8>>();

            println!("Текст: {}", std::str::from_utf8(&text).unwrap());
        }
        6 => {
            println!("Введите восьмеричный код:");
            io::stdout().flush().unwrap(); // сбрасываем буфер вывода на экран
            input.clear();
            io::stdin().read_line(&mut input).expect("Ошибка ввода");
            let octal = input.trim();

            let text = octal
                .split_whitespace()
                .map(|octal_digit| {
                    let decimal = u32::from_str_radix(octal_digit, 8).expect("Неверный 8й код.");
                    char::from_u32(decimal).unwrap()
                })
                .collect::<String>();
            println!("Текст: {}", text);
        }
        _ => {
            println!("Неверный выбор");
        }
    }

    println!("Нажмите на любую клавишу, чтобы завершить программу.");
    input.clear();
    io::stdin().read_line(&mut input).expect("Ошибка ввода");

}
