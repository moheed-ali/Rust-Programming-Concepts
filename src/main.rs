struct User {
    active: bool,
    username: String,
    email:String,
    sign_in_count: u64, 
}

fn main () {
    println!("Sturcture in Rust");

    let user1 = User {
        active: true,
        username: String::from("admin"),
        email: String::from("user@example.com"),
        sign_in_count: 1,
    };

    // user1.email = String::from("anotheremail@example.com");

    println!("User Name: {}, Active: {}, Email: {}, Sign in count: {}", user1.username, user1.active, user1.email, user1.sign_in_count);

}