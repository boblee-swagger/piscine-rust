
pub fn delete_and_backspace(s: &mut String) {
    let mut chars: Vec<char> = Vec::new();
    
    let mut num_signs = 0;
    for c in s.chars(){

        if c == '-' {
            chars.pop();
        }else if c == '+'{
            num_signs += 1;
            continue;
        }else{
            if num_signs != 0 {
                num_signs -= 1;
                continue;
            }
            chars.push(c);
        }
    }
    *s = chars.iter().collect();
}

pub fn do_operations(v: &mut [String]) {
    let sums: Vec<String> = v.iter().map(|f|{
        if f.contains('+') {
            f.split('+').map(|s| s.trim().parse::<i8>().unwrap()).sum::<i8>().to_string()
        }else if f.contains('-'){
            let parts : Vec<i8> = f.split('-').map(|s| s.trim().parse::<i8>().unwrap()).collect();
            parts[0].saturating_sub(parts[1]).to_string()
        }else{
            f.clone()
        }
    }).collect();
    v.clone_from_slice(&sums);
    
}