use std::{io, vec};

fn main() 
{

    let mut result_vec=Vec::new();
    println!("Entrez la clé");
    let mut cle = String::new();
    io::stdin().read_line(&mut cle).expect("Problème dans la lecture de la clé");
    let cle_int: u8 = cle.trim().parse().expect("Il faut un entier");
    

    println!("Quel est le message");
    let mut message: String = String::new();
    io::stdin().read_line(&mut message).expect("Problème dans la lecture du message");
    let message = message.to_lowercase();

    let alphabet= vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];


    for l in message.chars()
    {
        for i in 0..alphabet.len()
        {
            if l == alphabet[i]
            {
                //let indice = i + usize::from(cle_int);
                if i + usize::from(cle_int) > 25 {
                    let indice = i + usize::from(cle_int);
                    let other_indice = usize::from(alphabet.len().abs_diff(indice));
                    result_vec.push(alphabet[other_indice]);
                }
                else {
                    let indice: usize = i + usize::from(cle_int);
                    result_vec.push(alphabet[indice]);
                }
                
            }
        }
    }

    let message_code:String =  result_vec.iter().collect();
    println!("{}", message_code);
}