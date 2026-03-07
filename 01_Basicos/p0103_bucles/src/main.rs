fn main() {
    println!("Pruebas de bucles");

    // Ejemplo de bucle loop con valor devuelto
    // ----------------------------------------

    let mut contador = 1;
    // stop_loop se establece cuando en bucle para
    let stop_loop = loop {
        contador *= 2;
        if contador > 100 {
            // Parar el bucle. Devolver el valor de contador
            break contador;
        }
    };

    // Muestra cuando se ha salido del bucle
    println!("Ruptura del bucle en el contador = {}.", stop_loop);

    // Ejemplo de bucle while
    // ----------------------------------------
    while contador > 15 {
        println!("Vuelve el bucle...");
        contador = contador - 9;
    }

    println!("Se terminó el bucle cuando el contador vale: {}", contador);

    // Ejemplo de bucle for
    // ----------------------------------------
    let pajaros_grandes = ["avestruz", "pavo real", "cigüeña"];
    for pajaro in pajaros_grandes.iter() {
        println!("{} es un pájaro grande.", pajaro);
    }

    // Ejemplo de definición de intervalo
    // ----------------------------------------
    println!("Dobles de los primeros números:");
    for num in 0..6 {
        println!("{}", num * 2);
    }
}
