/*
    Prueba de diversos tipos de variables escalares
    Para las variables numéricas hay un programa aparte
*/
fn main() {
    // Tipos numéricos
    let ngrande: i64 = 3_670_500_200_000;
    let ejemhexa: i32 = 0xffa044;
    let ejemoctal: u32 = 0o457720;
    let ejembinario: u8 = 0b1010_1100;

    println!("Este es un número muy grande: {}", ngrande);
    println!("Este es un número dado en hexadecimal: {}", ejemhexa);
    println!("Este es un número dado en octal: {} (en octal 0o{:o}) ", ejemoctal, ejemoctal);
    println!("Este es un número dado en binario: {}", ejembinario);

    // Tipo booleano
    let enigma = true;

    println!("Este es un valor booleano: {}", enigma);

    // Tipo carácter
    let letra1: char = 'B';
    let letra2: char = 'j';
    let letra_espana: char = 'ñ';
    let gato_corazon: char = '😻';

    println!("La primera letra: {}", letra1);
    println!("La segunda letra: {}", letra2);
    println!("La letra eñe: {}", letra_espana);
    println!("La letra extraña: {}", gato_corazon);
    println!("");

    // Ejemplo de coma flotante
    let flotante1: f32 = 3.141592;
    let flotante2: f64 = 2.718281828459045;
    let flotante3 = -1.4142135623730950488016887242097; // f64 por defecto
    let flotante4 = 1.245e245;
    let flotante5: f32 = 0.42306e-16;

    println!("Ejemplos de números flotantes:");
    println!("Flotante 1 (f32): {}", flotante1);
    println!("Flotante 2 (f64): {}", flotante2);
    println!("Flotante 3 (f64): {}", flotante3);
    println!("Flotante 4 (f64): {}", flotante4);
    println!("Flotante 5 (f32): {}", flotante5);

}
