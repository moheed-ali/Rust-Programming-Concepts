#[derive(Debug)] 

enum UsState {
    Washington,
    Alabama,
    New_York,
    San_Francisco,
    Las_Vagas,
    Alaska,
    // --snip--
}

enum Coin{
    penny, 
    Nickel,
    Dime,
    Quater(UsState),
}

fn value_in_cents(coin: Coin) -> u8{
    match coin{
        Coin::penny=> 1,
        Coin::Nickel=> 5,
        Coin::Dime=> 10,
        Coin::Quater(state)=> {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main () {
    println!("Enum Match");
    let money = value_in_cents(Coin::Quater(UsState::New_York));
    println!("The Money is : {money}");
}