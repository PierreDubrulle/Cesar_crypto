pub fn process_crypto(message:Vec<char>, cle:u8) -> String
{
    let mut result_vec: Vec<char> = Vec::new();
    let alphabet= vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    let mess_vec = message;

    for l in mess_vec
    {
        for i in 0..alphabet.len()
        {
            if l == alphabet[i]
            {
                if i + usize::from(cle) > 25
                {
                    let indice = i + usize::from(cle);
                    let other_indice = usize::from(alphabet.len().abs_diff(indice));
                    result_vec.push(alphabet[other_indice]);

                }
                else 
                {
                    let indice: usize = i + usize::from(cle);
                    result_vec.push(alphabet[indice]);
                }
            }
        }
        if l.is_ascii_punctuation()
        {
            result_vec.push(l)
        }
    }
    let message_code:String = result_vec.iter().collect();
    return message_code;
}