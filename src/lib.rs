
pub fn acronym(input: &str) ->String{
    let mut acronym = String::new();
    let mut include_next = true;

    for c in input.chars(){
        if c.is_ascii_alphabetic(){
            if include_next{
                acronym.push(c.to_ascii_uppercase());
                include_next = false;
            }
        } else if c == '-' || c == ' '{
            include_next = true;
        }
    }

    //最终返回的值就是一个字符串
    acronym
}

