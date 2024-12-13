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
    println!("Este es un número dado en octal: {}", ejemoctal);
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
}
