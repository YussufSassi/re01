fn generate_password() -> String {
    use rand::Rng;
    let password_length = rand::thread_rng().gen_range(8..=10);
    let mut password = String::new();
    for _ in 0..password_length {
        let ascii_value = rand::thread_rng().gen_range(33..=126) as u8;
        password.push(ascii_value as char);
    }
    password
}

fn get_ascii_sum(pass: &str) -> u32 {
    let binding = pass.chars().collect::<Vec<char>>();
    let chars: &[char] = binding.as_slice();
    let mut val: u32 = 0;
    for c in chars {
        val += *c as u32
    }
    val
}

fn main() {
    let mut val: u32 = 0;

    while val < 1000 {
        let password = generate_password();

        val = get_ascii_sum(&password);

        if val >= 1000 {
            println!("Here's your password: {}", password);
        }
    }
}
