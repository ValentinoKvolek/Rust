/*

2- Definir la función llamada es_primo que recibe un número entero positivo mayor a 1 y
retorna true si es primo, false caso contrario.

*/
use std::io;

fn main() {

    let mut aux = String :: new();

    io::stdin()
        .read_line(&mut aux)
        .expect("Error al leer la entrada");

    let input: i32 = match aux.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Operación inválida");
            return
        }
    };
    if es_primo(input){ println!("es primo") }else{ println!("no es primo"); }
}

fn es_primo(n: i32) -> bool{

    if n <= 1 {
        return false;
    }
    for i in 2 ..=((n as f64).sqrt() as i32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
