/*

3- Definir la función llamada suma_pares que recibe como parámetro un arreglo de
números enteros y retorna la suma de los números pares.

*/

fn main() {

    let vec = [1, 2, 3, 4, 5];

    let resultado :i32;

    resultado  = suma_pares(vec);

    println!("resultado : {}", resultado);
}

fn suma_pares(vec:[i32; 5]) -> i32{

    let mut sum = 0;

    for i in 0..vec.len() {

        if vec[i] % 2 == 0 {sum += vec[i];}
    }

    return sum;

}