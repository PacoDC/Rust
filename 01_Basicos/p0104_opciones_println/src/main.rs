//
// Demonstración de las posibilidades de println! en Rust
//

fn main() {

    println!("Demostración de las posibilidades de println! en Rust");
    println!("-----------------------------------------------");
    println!("1. Imprimir un número entero: {}", 42);
    println!("2. Imprimir un número flotante: {}", 3.14);
    println!("3. Imprimir una cadena de texto: {}", "Hola, mundo!");
    println!("4. Imprimir múltiples valores: {} y {}", "Rust", 2024);
    println!("5. Imprimir con formato: {:>80}", "alineado a la derecha");
    println!("6. Imprimir con formato: {:<80}", "alineado a la izquierda");
    println!("7. Imprimir con formato: {:^80}", "centrado");
    println!("8. Imprimir con precisión: {:.2}", 3.14159);
    println!("9. Imprimir con padding: {:0>5}", 42);
    println!("10. Imprimir con padding y formato: {:0>5.2}", 3.14);
    println!("11. Imprimir con padding y formato: {:0>5.2}", 3.14159);
    println!("12. Imprimir con ancho: {numero:x>ancho$}", numero=-365, ancho=10);

}
