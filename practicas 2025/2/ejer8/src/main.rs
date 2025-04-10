/*

8- Definir la función llamada sumar_arreglos que recibe 2 arreglos del mismo tamaño de
números flotantes y retorna un nuevo arreglo que contiene la suma de los elementos de los
arreglos pasados por parámetro, correspondiéndose el resultado con cada posición de los
arreglos pasados por parámetro

*/

fn main() {

    let vector1 :  [f64;5] = [1.6,7.6,3.4,3.4,7.8];
    let vector2 : [f64;5] = [6.43,7.4,8.5,9.34,10.3];
    let v_resultado:[f64;5];
    v_resultado = sumar_arreglos(vector1, vector2);
    for i in v_resultado.iter() {
        println!("{}",i);
    }

}

fn sumar_arreglos(v1:  [f64;5], v2:  [f64;5]) -> [f64;5] {
    let mut nuevo_vector: [f64;5] = [0f64;5];

    for i in 0.. v1.iter().len(){
        nuevo_vector[i] = v1[i]+v2[i];
    }
    nuevo_vector
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sumar_arreglos_basicos() {
        let v1: [f64; 5] = [1.6, 7.6, 3.4, 3.4, 7.8];
        let v2: [f64; 5] = [6.43, 7.4, 8.5, 9.34, 10.3];
        assert_eq!(sumar_arreglos(v1, v2), [8.03, 15.0, 11.9, 12.74, 18.1]);
    }

    #[test]
    fn test_sumar_arreglos_ceros() {
        let v1: [f64; 5] = [0.0, 0.0, 0.0, 0.0, 0.0];
        let v2: [f64; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(sumar_arreglos(v1, v2), [1.0, 2.0, 3.0, 4.0, 5.0]);
    }

    #[test]
    fn test_sumar_arreglos_negativos() {
        let v1: [f64; 5] = [-1.5, -2.5, -3.5, -4.5, -5.5];
        let v2: [f64; 5] = [1.5, 2.5, 3.5, 4.5, 5.5];
        assert_eq!(sumar_arreglos(v1, v2), [0.0, 0.0, 0.0, 0.0, 0.0]);
    }

}