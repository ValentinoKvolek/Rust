/*

11-Definir la función llamada multiplicar_valores que recibe como parámetro un arreglo de
enteros y otro número entero llamado factor. Esta función multiplica los valores del arreglo
por el parámetro factor modificándolo


*/

fn main() {
    let mut array_int:[i32;4] = [1,2,3,4];
    let factor = 50;
    multiplicar_valores(&mut array_int , factor);
    for i in array_int.iter_mut() {
        println!("{}",i);
    }
}

fn multiplicar_valores( array: &mut [i32; 4], factor: i32){
    for i in array {
        *i *=  factor;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplicar_valores_test(){
        let mut array: [i32;4] = [1,2,3,4];
        multiplicar_valores(&mut array, 2);
        assert_eq!(array, [2,4,6,8]);

    }
}
