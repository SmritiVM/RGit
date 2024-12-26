pub fn handle_message<T: ToString>(message: T) {
    println!("{}", message.to_string());
}