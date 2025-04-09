
/*

5-Defina la función llamada duplicar_valores que recibe un arreglo de números flotantes y
retorna un arreglo nuevo con los valores duplicados del parámetro


*/

fn main() {

    let vec = [10.3,20.5,30.3,40.2,50.3];
    let vecDuplicado : [f64;5];

    vecDuplicado= duplicar_valores(vec);

    for i in vecDuplicado.iter() {
        println!("{}", i);
    }
}

fn duplicar_valores(vec:[f64;5]) -> [f64;5] {

    let mut nuevo_vector: [f64;5] = [0.0;5];

    for i in 0..vec.len() {
        nuevo_vector[i] = vec[i] * 2.0;
    }

    return nuevo_vector;

}
