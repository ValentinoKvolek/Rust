/*

7-Definir la función llamada cantidad_de_mayores que recibe como parámetro un arreglo
de números enteros y un número entero llamado límite. Esta función retorna la cantidad de
números mayores al límite que tiene el arreglo.

*/

fn main() {
    let array_enteros : [i32;5] = [1,2,3,4,5];
    let limite: i32 = 2;
    println!("{:?}",cantidad_de_mayores(array_enteros, limite));

}
fn cantidad_de_mayores(vec :[i32;5], limite :i32) -> i32 {
    let mut cant =0;
    for i in vec.iter() {
        if i > &limite {
            cant+=1;
        }
    }
    cant
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mayores_que_dos() {
        let array = [1, 2, 3, 4, 5];
        assert_eq!(cantidad_de_mayores(array, 2), 3); // 3, 4, 5 son mayores que 2
    }

    #[test]
    fn test_mayores_que_cero() {
        let array = [1, 2, 3, 4, 5];
        assert_eq!(cantidad_de_mayores(array, 0), 5); // Todos son mayores que 0
    }

    #[test]
    fn test_mayores_que_cinco() {
        let array = [1, 2, 3, 4, 5];
        assert_eq!(cantidad_de_mayores(array, 5), 0); // Ninguno es mayor que 5
    }

    #[test]
    fn test_con_negativos() {
        let array = [-2, -1, 0, 1, 2];
        assert_eq!(cantidad_de_mayores(array, 0), 2); // 1 y 2 son mayores que 0
    }
}