
pub fn name_initials(names : Vec<&str>) -> Vec<String> {
    let mut res : Vec<String> = Vec::new();
    for (_i, val) in names.iter().enumerate(){
        res.push(val.split_ascii_whitespace().enumerate().map(|(i, f)|  {
            let mut m = f.chars().next().unwrap().to_string() + ".";
            if i != val.split_ascii_whitespace().collect::<Vec<_>>().len()-1  {
                m.push_str(" ");
            }
            return m;
            
        }).collect());
    }

    return res;

}