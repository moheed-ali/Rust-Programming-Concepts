enum Coin{
    penny, 
    Nickel,
    Dime,
    Quater,
}

fn value_in_cents(coin: Coin) -> u8{
    match coin{
        Coin::penny=> {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel=> 5,
        Coin::Dime=> 10,
        Coin::Quater=> 25,
    }
}

fn main () {
    println!("Enum Match");
    let money = value_in_cents(Coin::penny);
    println!("The Money is : {money}");
}