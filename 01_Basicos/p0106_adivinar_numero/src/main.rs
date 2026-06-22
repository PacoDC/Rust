/* 
    Adivinar número
 */
use std::io;
use std::cmp::Ordering;

use rand::Rng;


fn main() {

    println!("¡Adivina el número!");

    // Genera un número al azar
    let num_secreto = rand::thread_rng().gen_range(1..=100);

    // println!("El número secreto es: {}", num_secreto);

    loop {
    
        println!("Introduce un número:");

        let mut guess = String::new();

        // Pide un número por la consola
        io::stdin()
            .read_line(&mut guess)
            .expect("Error al leer la línea");

        println!("Tu número es: {}", guess);

        // Convierte la cadena a número
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, introduce un número válido");
                continue; // Vuelve al inicio del bucle
            }
        };

        // Analiza la comparación
        match guess.cmp(&num_secreto) {
            Ordering::Less => println!("Demasiado pequeño"),
            Ordering::Greater => println!("Demasiado grande"),
            Ordering::Equal => {
                println!("¡Has adivinado el número!");
                break; // Sale del bucle
            }
        } // match

    } // loop

}
