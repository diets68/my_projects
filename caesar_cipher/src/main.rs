use std::io;

fn input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("failed to read");
        input.trim().to_string()
}

fn encrypt(message: &str, shift: u8) -> String {
    let mut result = String::new();
    println!("your message was: {}", message);
    println!("shifting message by: {}", shift);
     for c in message.chars(){
      let shifted = (c as u8 + shift) as char;
result.push(shifted);
    }
    result

}
fn main(){ 
    let message = input("Enter message:");
    let shift: u8 = input("Enter shift:").parse().expect("invalid number");
    let encrypted = encrypt(&message, shift);
    println!("Encrypted: {}", encrypted);
}