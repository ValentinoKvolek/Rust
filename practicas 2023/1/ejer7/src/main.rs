fn main() {
    const MUL: u8 = 2;
    let mut array = [1,2,3,4,5,6];

    for num in array.iter_mut() {
        *num *= MUL;
    }

    println!("{:?}", array);



}


