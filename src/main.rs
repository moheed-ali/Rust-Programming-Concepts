use std::io;

fn main () {

    // Temperature Converter

    let mut temp = String::new();

    println!("Enter the Temperature in Celsius: ");

    io::stdin()
        .read_line(&mut temp)
        .expect("Enter a number");
    
    let temp: f32 = temp
        .trim()
        .parse()
        .expect("It is not a number");
    
    println!("Celsius Temperature: {temp}°C");

    let mut celsius_to_fahrenheit = temp * 1.8;
    celsius_to_fahrenheit = celsius_to_fahrenheit + 32.0; 
    println!("Temperature in Fahrenheit: {}°F", celsius_to_fahrenheit);
    let celsius_to_kelvin = temp + 273.15;
    println!("Temperature in Kelvin: {}K", celsius_to_kelvin);


    // let mut num = 3;

    // while num != 0{
    //     println!("{num}!");
    //     num -= 1;
    // }
    // println!("Lift OFF!");

    // let a = [1, 2, 3, 4, 5];

    // let mut index = 0;

    // while index < 5 {
    //     println!("Value of Array[{index}]: {}", a[index]);
    //     index += 1;
    // }

    // for element in a {
    //     println!("The Value is: {element}");
    // }

    // for number in (1..4).rev(){
    //     println!("{number}");
    // }
    // println!("Lift OFF!");


    // let mut count = 0 ;

    // 'counting_up: loop{
    //     println!("Count = {count}");

    //     let mut remaining = 10;

    //     loop{
    //         println!("Remaining = {remaining}");

    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }

    // println!("End count = {count}");

    // let mut counter = 0; 

    // let result = loop {
    //     counter += 1;

    //     if counter == 10{
    //         break counter * 2;
    //     }
    // };

    // println!("The Value of Counter is: {result}");

    // let num = 6;

    // if num < 5 {
    //     println!("Condition was true");
    // } else{
    //     println!("Condition was false");
    // }

    // if num % 4 == 0 {
    //     println!("Number is divisible by 4");
    // } else if num % 3 == 0{
    //     println!("Number is divisible by 3");
    // } else if num % 2 == 0{
    //     println!("Number is divisible by 2");
    // } else {
    //     println!("Number {num} is not divisible by 4, 3 and 2");
    // }

    // let condition = true;
    // let number = if condition {5} else {6};

    // println!("The value of number is: {number}");
}

