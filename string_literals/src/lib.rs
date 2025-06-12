pub fn is_empty(v: &str) -> bool {
    //v.is_empty()
    v.trim().len() == 0
}

pub fn is_ascii(v: &str) -> bool {
    let mut tmp = v.chars().next().into_iter();
    while let Some(ch) = tmp.next() {
        if !ch.is_ascii(){
            return false;
        }
    }
    return true;
}

pub fn contains(v: &str, pat: &str) -> bool {
    //v.contains(pat)
    let mut chars : Vec<char> = v.chars().collect();
    for (i, _t) in chars.iter_mut().enumerate(){
        let c  = &v[i..pat.len()];
        if  c == pat {
            return true;
        }
    }
    return false;
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    (&v[0..index], &v[index..v.len()])
}

pub fn find(v: &str, pat: char) -> usize {
    for (index, val) in v.chars().enumerate(){
        if val == pat {
            return index;
        }
    }
    return 0;
}