use clap::Parser;
use rand::Rng;
use rand::seq::SliceRandom;

/// Генератор безопасных паролей
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Длина пароля (по умолчанию: 12)
    #[arg(short = 'l', long, default_value_t = 12)]
    length: usize,

    /// Включить цифры (0-9)
    #[arg(short = 'd', long)]
    digits: bool,

    /// Включить специальные символы (!@#$% и т.д.)
    #[arg(short = 's', long)]
    special: bool,

    /// Исключить похожие символы (o, O, 0, l, I, 1)
    #[arg(short = 'e', long)]
    exclude_similar: bool,

    /// Количество генерируемых паролей
    #[arg(short = 'c', long, default_value_t = 1)]
    count: usize,
}

fn main() {
    let args = Cli::parse();
    
    println!("{} Генератор паролей", "🔐");
    println!("Длина: {} символов", args.length);
    println!("Цифры: {}", if args.digits { "включены" } else { "выключены" });
    println!("Спецсимволы: {}", if args.special { "включены" } else { "выключены" });
    println!("Исключить похожие: {}", if args.exclude_similar { "да" } else { "нет" });
    println!();

    for i in 0..args.count {
        match generate_strong_password(
            args.length,
            args.digits,
            args.special,
            args.exclude_similar,
        ) {
            Ok(password) => {
                if args.count > 1 {
                    println!("Пароль {}: {}", i + 1, password);
                } else {
                    println!("Сгенерированный пароль: {}", password);
                    print_password_strength(&password);
                }
            }
            Err(e) => {
                eprintln!("Ошибка: {}", e);
            }
        }
    }
}

fn generate_strong_password(
    length: usize, 
    use_digits: bool, 
    use_special: bool, 
    exclude_similar: bool
) -> Result<String, String> {
    if length < 4 {
        return Err("Пароль должен быть длиной хотя бы 4 символа".to_string());
    }

    let mut rng = rand::thread_rng();
    
    // Определяем наборы символов
    let uppercase = if exclude_similar { 
        "ABCDEFGHJKLMNPQRSTUVWXYZ" 
    } else { 
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ" 
    };
    
    let lowercase = if exclude_similar { 
        "abcdefghijkmnpqrstuvwxyz" 
    } else { 
        "abcdefghijklmnopqrstuvwxyz" 
    };
    
    let digits = if exclude_similar { 
        "23456789" 
    } else { 
        "0123456789" 
    };
    
    let special = "!@#$%^&*()_+-=[]{}|;:,.<>?";

    let mut all_chars = String::new();
    let mut password_chars = Vec::new();
    
    // Обязательно добавляем хотя бы один символ каждого типа
    password_chars.push(uppercase.chars().nth(rng.gen_range(0..uppercase.len())).unwrap());
    password_chars.push(lowercase.chars().nth(rng.gen_range(0..lowercase.len())).unwrap());
    
    all_chars.push_str(uppercase);
    all_chars.push_str(lowercase);
    
    if use_digits {
        password_chars.push(digits.chars().nth(rng.gen_range(0..digits.len())).unwrap());
        all_chars.push_str(digits);
    }
    
    if use_special {
        password_chars.push(special.chars().nth(rng.gen_range(0..special.len())).unwrap());
        all_chars.push_str(special);
    }
    
    // Добавляем оставшиеся символы
    while password_chars.len() < length {
        let idx = rng.gen_range(0..all_chars.len());
        password_chars.push(all_chars.chars().nth(idx).unwrap());
    }
    
    // Перемешиваем символы
    password_chars.shuffle(&mut rng);
    
    Ok(password_chars.into_iter().collect())
}

fn print_password_strength(password: &str) {
    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_digit(10));
    let has_special = password.chars().any(|c| !c.is_alphanumeric());
    
    let mut score = 0;
    if has_upper { score += 1; }
    if has_lower { score += 1; }
    if has_digit { score += 1; }
    if has_special { score += 1; }
    if password.len() >= 12 { score += 1; }
    
    let (strength, emoji) = match score {
        0..=2 => ("Слабый", "🔴"),
        3..=4 => ("Средний", "🟡"),
        _ => ("Сильный", "🟢"),
    };
    
    println!("{} Сложность пароля: {}", emoji, strength);
}