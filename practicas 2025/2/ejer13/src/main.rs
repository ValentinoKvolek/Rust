/*


13-Definir una función llamada ordenar_nombres que recibe un arreglo de String y los
ordena en orden alfabético.


*/


fn main() {
    let mut array: [&str; 5] = ["fermin","valen","yas","yoni","jp"];
    ordenar_nombres(&mut array);
    for i in array.iter() {
        print!("{:?} ", i);
    }
}

fn ordenar_nombres(array: &mut [&str;5]) {
    let len = array.len();

    for _ in 0..len {
        for j in 0..len - 1 {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
}
