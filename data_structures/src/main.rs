// Creating an Enum
enum user_active {
    ACTIVE,
    DISABLE,
}

// Creating a user Object Structure
struct User {
    username: String,
    email: String,
    status: user_active,
}

// Add functions to the user
impl User {
    // Function that build a user
    fn build_user(email: String, username: String) -> Self {
        Self {
            email: email,
            username: username,
            status: user_active::ACTIVE,
        }
    }
}


// Defining a structure that keeps the RGB Colors
// This is going to be used as a Tuple, without telling the namespaces
struct Color(i32,i32,i32);

// main Function
fn main() {
    // User variable
    // Creating an user
    let user1 = User::build_user("dricardosc15@gmail.com".to_string(), "rdani2005".to_string());
    // Print it's values
    let black = Color(0,0,0);
    let Red = Color(255,0,0);

    println!("email: {} username: {} Status: {:?}", user1.email, user1.username, user1.status);
    println!("{:?}", black);
    println!("{:?}", Red);
}

