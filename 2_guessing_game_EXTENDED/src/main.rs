use rand::Rng;

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}
fn main() {
    let mut rng = rand::thread_rng();
    let mut options: Vec<String> = Vec::new();

    // Define size of the Vector for the possibilities
    println!("Set amount of words:");
    let size = get_input().trim().parse::<i32>().expect("Invalid value, please introduce a number.");

    // Fill the vector with the possibilities
    for i in 0..size {
        println!("Set possibility #{}:", i + 1);
        let option = get_input().trim().parse::<String>().unwrap();
        options.push(option);
    }

    // Get random position of the vector
    let option_index = rng.gen_range(0, size) as usize;
    let option = &options[option_index];

    // Get the users guess
    println!("Guess the word!");
    let guess = get_input().trim().parse::<String>().unwrap();
    
    // Check if the user guessed right
    if option == &guess {
        println!("you guessed RIGHT!: {}", guess);
    } else {
        println!("you guessed WRONG!: {}", option);
    }
}
