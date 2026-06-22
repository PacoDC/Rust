/*
    Pruebas de shadowing
*/
fn main() {

    println!("Pruebas de shadowing (ocultamiento de variables)");

    let x = 5;

    let x = x + 4;

    {
        let x = x * 2;
        println!("El valor de x en el scope interno es: {}", x);
    }

    println!("El valor de x en el scope externo es: {}", x);

    // Cambio de tipo

    println!(" --------------------------------");
    
    let espacios = "    ";

    let espacios = espacios.len();

    println!("El valor de espacios es: {}", espacios);

}

