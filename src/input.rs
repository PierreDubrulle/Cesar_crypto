pub fn process_input(message:String, cle:String) -> (Vec<char>, u8)

{

    let message_lower:String = message.to_lowercase();
    let char_vec: Vec<char> = message_lower.chars().collect();
    let cle_int: u8 = cle.trim().parse().expect("Il faut un entier");

    return (char_vec, cle_int)
}