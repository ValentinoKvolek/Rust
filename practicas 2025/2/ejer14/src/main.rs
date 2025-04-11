/*

14-Definir una función llamada incrementar que recibe como parámetro un número flotante
e incrementa en 1 su valor.

*/

fn main() {
    let mut n :f64 = 0.0;
    incrementar(&mut n);
    println!("{}", n);

}


fn incrementar (n : &mut f64) {

    *n += 1.0;
}
