use std::io;
use std::str::FromStr;
use core::num;
use std::fs::{File};
use std::io::{prelude::*, BufWriter};
use std::io::{BufReader, Result, Write};


fn main() {
    loop {
        println!("\nElija una opción:");
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
                println!("Opción invalida, Saliendo del programa.");
                break;
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
            print!("\t FizzBuzz")
        } else if i % 5 == 0  {
            print!("\tBuzz")
        }else if i % 3 == 0  {
            print!("\t Fizz")
        }else {
            print!("\t{}", i);
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

fn formatter() -> Result<()> {

        let f: File = File::open("numbers-input.txt")?;
        let mut reader: BufReader<File> = BufReader::new(f);
    
        let mut buffer: String = String::new();
        let mut numbers_str: String = "".into();

        reader.read_line(&mut buffer)?;
        print!("\n\nImpresión de la serie\n");
        for (index, line) in reader.lines().enumerate() {
            if let Ok(num) = line?.parse::<i32>() {
                if num == 0 || num == 1{
                    println!("{} .", num);
                    numbers_str.push_str(num.to_string().as_str());
                    numbers_str.push_str(&format!("\t-Special Case\n"));
                    continue;
                }
                if is_even(num) && is_prime(num){
                    println!("{} Prime Even.", num);
                    numbers_str.push_str(num.to_string().as_str());
                    numbers_str.push_str(&format!("\t- Prime Even\n"));
                }else if is_even(num) {
                    println!("{} Even.", num);
                    numbers_str.push_str(num.to_string().as_str());
                    numbers_str.push_str(&format!("\t- Even\n"));
                }else if is_prime(num) {
                    println!("{} Prime.", num);
                    numbers_str.push_str(num.to_string().as_str());     
                    numbers_str.push_str(&format!("\t- Prime\n"));
                }else {
                    println!("{}", num);
                    numbers_str.push_str(num.to_string().as_str());     
                    numbers_str.push_str(&format!("\t- \n"));
                }
            }
        }
    
        fn is_even(num: i32) -> bool {
            num % 2 == 0
        }
        
        fn is_prime(num: i32) -> bool {
            if num <= 1 {
                return false;
            }
            for i in 2..=(num / 2) {
                if num % i == 0 {
                    return false;
                }
            }
            true
        }

        let mut archivo = File::create("numbers-output.txt")?;
        archivo.write_all(numbers_str.as_bytes())?;
        Ok(())

}

