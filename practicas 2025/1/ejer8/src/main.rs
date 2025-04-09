use std::io;
fn main() {
    const CADENA: &str = "mongo";

    let mut input = String::new();
    let mut cont : u8 = 0;
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer");

    let input =  input.trim();


    if let Some(input_char)= input.chars().next(){

        for i in CADENA.chars() {
            if i == input_char {
                cont += 1;
            }
        }
        println!("El caracter '{}' aparece '{}' veces", input_char, cont);

    }else { println!("Error al leer"); }
    
}
