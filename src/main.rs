use std::io;
mod input;
mod cesar;

fn main() 
{

    println!("Entrez la clé");
    let mut cle = String::new();
    io::stdin().read_line(&mut cle).expect("Problème dans la lecture de la clé");
    

    println!("Quel est le message");
    let mut message: String = String::new();
    io::stdin().read_line(&mut message).expect("Problème dans la lecture du message");
    let (mess_vec, cle_int) = input::process_input(message, cle);

    let final_message:String = cesar::process_crypto(mess_vec, cle_int);

    println!("Le message codé est : {}", final_message);
}
