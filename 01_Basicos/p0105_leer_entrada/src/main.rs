/*
    Prueba de lectura de entrada desde la consola
 */
use std::io;

fn main() {

    println!("Introduce un texto cualquiera y pulsa enter:");
    
    let mut texto = String::new();
    
    io::stdin()
        .read_line(&mut texto)
        .expect("Error al leer la línea");
    
    println!("El texto introducido es: \"{}\"", texto);
}
