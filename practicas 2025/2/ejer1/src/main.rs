/*

1-Definir la función llamada es_par que recibe como parámetro un número entero y retorna
true si el número es par, false caso contrario

*/

use std::io;

fn main() {

    let mut aux = String :: new(); //valor aux;

    println!("ingrese un numero");

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

   if es_par(input){
       println!("es par");
   }else { println!("no es par"); }

}


fn es_par( n: i32) -> bool {
    if n%2 == 0 {return true} else {return false};
}
