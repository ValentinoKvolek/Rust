/*

12-Definir una función llamada reemplazar_pares que recibe un arreglo de enteros y
reemplaza todos los números pares por -1

*/

fn main() {

    let mut array: [i32; 4] = [1,2,3,4];

    reemplazar_pares(&mut array);

    for i in array.iter() {
        print!("{:?} ", i);
    }
}
fn reemplazar_pares(v:&mut [i32;4]) {

    for i in v {

        if *i % 2== 0 {
            *i = -1;
        }
    }

}