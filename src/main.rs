fn main () {

    let num = 6;

    // if num < 5 {
    //     println!("Condition was true");
    // } else{
    //     println!("Condition was false");
    // }

    if num % 4 == 0 {
        println!("Number is divisible by 4");
    } else if num % 3 == 0{
        println!("Number is divisible by 3");
    } else if num % 2 == 0{
        println!("Number is divisible by 2");
    } else {
        println!("Number {num} is not divisible by 4, 3 and 2");
    }
}

