pub fn to_url(s : &str) -> String {
   let mut res = String::new();
   for (_i, v) in s.chars().enumerate(){
    if v.is_whitespace() {
        res.push_str("%20");
    }else{
        res.push(v);
    }
   }

   return res;
}