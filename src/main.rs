use std::io;

fn main() {

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index: ");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("failed to read line");

    let index: usize = index 
        .trim()
        .parse()
        .expect("Index Entered was not a number");

    let element = a[index];

    println!("The value at the array[{index}] is: {element}", );

    // println!("Hello, world!");

    // let mut x = 5;

    // println!("The Value of x is: {x}");

    // x = 6;

    // println!("The Value of x is: {x}");

    // // Constant

    // const THREE_HOUR_IN_SECONDS: u32 = 60 * 60 * 3;

    // println!("The Value of constant is: {THREE_HOUR_IN_SECONDS}");

    // // Shadowing

    // let y = 9;

    // let y = y + 1;

    // {
    //     let y = y * 2;
    //     println!{"The Value of y is : {y}"};
    // }

    // println!{"The Value of y is : {y}"};

    // let space = "   ";
    // let space = space.len();

    // println!{"The Value of Space is : {space}"};

    // println!{"Data Types"};

    // let guess: u32 = "42".parse().expect("Not a number!");

    // println!{"The Value of Guess is : {guess}"};

    // println!("The Character Type");

    // let c = 'c';
    // let z: char = 'Z';
    // let heart_eye_cat = '😻';

    // println!{"The Value of characters are : {}, {}, {}", c, z, heart_eye_cat};

    // println!("Tuple Type");

    // let tup = (500, 6.4, 1);

    // let (p, q, r) = tup;

    // println!("The Value of q is: {q}");

    // println!("Array Type");

    // let a: [i32;5] = [1, 2, 3, 4, 5];

    // let month = ["January", "February", "March", "April", "May", "June", "July",
    //             "August", "September", "October", "November", "December"];

    // println!("The Data in Array a is: {}", a[1]);

    // println!("The Data in Array month is: {}", month[1]);

    // let b = [3;7];

    // println!("The Data in Array b is: {}", b[1]);

}
