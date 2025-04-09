/*

4- Definir la función llamada cantidad_impares que recibe como parámetro un arreglo de
números enteros y retorna la cantidad de números impares.

*/

fn main() {

    let resultado :i32;
    let vec = [1,2,3,4,5];

    resultado = suma_impar(vec);

    println!("{:?}", resultado);

}

fn suma_impar(vec:[i32;5]) -> i32 {
    let mut suma = 0;
    for i in vec.iter(){
        if i % 2 != 0 {
            suma += i;
        }
    }
    return suma;
}
