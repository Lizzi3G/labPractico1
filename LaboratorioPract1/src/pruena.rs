use std::io;
use std::str::FromStr;

fn main() {
    loop {
        println!("Elija una opción:");
        println!("1. Fizz Buzz");
        println!("2. Counter");
        println!("3. Formatter");
        println!("0. Exit the menu");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Error al leer la entrada");

        let option = input.trim().to_string();

        if option == "0" {
            println!("Saliendo del programa.");
            break;
        } else {
            if !menu(&option) {
                println!("Opción inválida.");
            }
        }
    }
}

fn menu(option: &String) -> bool {
    match option.as_str() {
        "1" => {
            fizz_buzz();
            true
        }
        "2" => {
            counter();
            true
        }
        "3" => {
            formatter();
            true
        }
        _ => false,
    }
}

fn fizz_buzz() {
    // implementación de Fizz Buzz
    println!("Digita un número para jugar: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Error al leer de teclado");
    let number: u32 = u32::from_str(&input.trim()).unwrap();

    for i in 1..=number {

        if i % 3 == 0 && i % 5 == 0{
            println!("FizzBuzz")
        } else if i % 5 == 0  {
            println!("Buzz")
        }else if i % 3 == 0  {
            println!("Fizz")
        }else {
            println!("{}", i);
        }
    } 
}

fn counter() {
    //implementación del Controlador
    let mut input = String::new();

    println!("Introduce un texto:");
     io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    
    let symbol_count = count_symbols(&input);
    let mut vowels_count = 0;
    let mut  consonants_count: i32 = 0;
    let mut number_count: i32 = 0;

    for c in input.chars() {
        if "aeiouAEIOU".contains(c) {
            vowels_count += 1;
        } else if "bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ".contains(c) {
            consonants_count += 1;
        } else if c.is_digit(10) {
            number_count += 1;
        }
    }

    fn count_symbols(text: &str) -> usize {
        text.chars()
            .filter(|c| !c.is_alphanumeric() && !c.is_whitespace())
            .count()
    }
 
    println!("Digit: {}", number_count);
    println!("Vowels {}", vowels_count);
    println!("Consonants: {}", consonants_count);
    println!("Symbols: {}", symbol_count);
}

fn formatter() {
    //implementación del Formateador
}
