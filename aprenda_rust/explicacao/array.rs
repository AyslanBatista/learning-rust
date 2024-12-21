fn main(){
    // Array, não permite que exista tipos diferentes 
    let numbers = [1.1, 2.0, 3.3]; // forma explicita  let numbers: [i32;3] = [1,2,3]

    println!("{:?}", numbers);
    println!("{:?}", numbers[0]); // imprimindo o primeiro objeto do array

    let mut numbers_mutavel = [1, 2, 3, 4]; // Array mutavel
    numbers_mutavel[0] = 9;
    println!("{:?}", numbers_mutavel);

    // Slice, fatiamento do array
    println!("{:?}", &numbers[1..3]);
    

}
