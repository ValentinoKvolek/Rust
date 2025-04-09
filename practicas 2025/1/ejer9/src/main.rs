/*

    9- Escribir un programa que defina un arreglo de 5 números enteros, y luego imprima la
    suma de los valores del arreglo.

 */


fn main() {

    let array = [1, 2, 3, 4 , 5 ];
    let mut suma : i32 = 0;
    for i in array.iter() {
        suma += i;
    }
    println!("suma: {}", suma);
}
