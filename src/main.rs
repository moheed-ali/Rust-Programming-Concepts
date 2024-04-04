// #[derive(Debug)] 

// enum UsState {
//     Washington,
//     Alabama,
//     New_York,
//     San_Francisco,
//     Las_Vagas,
//     Alaska,
//     // --snip--
// }

// enum Coin{
//     penny, 
//     Nickel,
//     Dime,
//     Quater(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8{
//     match coin{
//         Coin::penny=> 1,
//         Coin::Nickel=> 5,
//         Coin::Dime=> 10,
//         Coin::Quater(state)=> {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// }

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}


fn main () {
    println!("Enum Match");

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("The Value in Six is: {:?}", six);

    // let money = value_in_cents(Coin::Quater(UsState::New_York));
    // println!("The Money is : {money}");
}