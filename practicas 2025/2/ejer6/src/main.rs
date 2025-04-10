
/*
6-Definir la función llamada longitud_de_cadenas que recibe un arreglo de String y retorna
un arreglo con la longitud de las cadenas del parámetro, correspondiéndose en posición del
arreglo

*/

fn main() {
    let vector_string:[&str;3] =["pelado", "valen", "yoni"];
    let vector_resultado:[usize;3];
    vector_resultado = longitud_de_cadenas(vector_string);
    for i in vector_resultado.iter() {
        println!("{}", i);
    }
}

fn longitud_de_cadenas(vector_str:[&str;3]) -> [usize;3] {
    let mut new_vector:[usize;3] = [0;3];
    let mut pos:usize =0;

    for i in vector_str.iter() {
        new_vector[pos] = i.len();
        pos+=1;
    }
    new_vector
}