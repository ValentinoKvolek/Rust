/*

    10- Escribir un programa que defina dos arreglos de 5 números enteros cada uno, y luego
    cree un tercer arreglo que contenga la suma de los elementos de los dos arreglos originales

*/

fn main() {
    let array1 = [1,2,3,4,5];
    let array2 = [1,2,3,4,5];

    let resultado: Vec<i32> = array1.iter()
        .zip(array2.iter())
        .map(|(a, b)| a + b)
        .collect();

    println!("{:?}", resultado);
}
