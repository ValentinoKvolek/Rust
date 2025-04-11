/*

10-Definir la función llamada cantidad_de_cadenas_mayor_a que recibe como parámetros
un arreglo de String y un entero llamado límite. Esta función retorna la cantidad de Strings
del arreglo que son de longitud mayor al parámetro límite


*/


fn main() {
    let array_strig : [&str;5] = ["fermin","valen","yas","yoni","jp"];
    let limite = 4;
    let resueltado;

    resueltado =cantidad_de_cadenas_mayor_a(array_strig, limite);
    println!("{}", resueltado);
}

fn cantidad_de_cadenas_mayor_a(array : [&str;5], x : i32) -> u32 {
    let mut suma = 0;
    for i in array.iter() {
        if i.len() > x as usize {
            suma+=1;

        }
    }
    suma
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cantidad_de_cadenas_mayor_a_0() {
        let array_strig : [&str;5] = ["fermin","valen","yas","yoni","jp"];
        assert_eq!(cantidad_de_cadenas_mayor_a(array_strig, 6), 0);
    }
    #[test]
    fn test_cantidad_de_cadenas_mayor_a_5() {
        let array_strig : [&str;5] = ["fermin","valen","yas","yoni","jp"];
        assert_eq!(cantidad_de_cadenas_mayor_a(array_strig, 1), 5);
    }
}