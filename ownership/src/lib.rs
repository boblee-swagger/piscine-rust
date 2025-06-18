pub fn first_subword(mut s: String) -> String {


    let r =  s.chars().enumerate().find(|(i , f)| {
       if f.is_uppercase() && *i != 0 || *f == '_'{
            true
       }else{
            false
       }
    });

    let index = r.map( |i|  i.0);
    if index == None {
        return s;
    }
    return &s[0..index.unwrap()].to_string();
}