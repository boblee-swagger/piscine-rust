pub fn insert(vec : &mut Vec<String>, val : String) {
    vec.push(val);
}

pub fn at_index(slice : &[String], index : usize) -> &str {
    for (i, c) in slice.iter().enumerate(){
        if i == index {
            return c
        }
    }

    return "";
}