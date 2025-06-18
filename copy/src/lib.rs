pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exp_func_value = (c as f64).exp();
    let nat_log = (c.abs() as f64).ln();
    (c, exp_func_value, nat_log)
}

pub fn str_function(a: String) -> (String, String) {
    let result: String = a.split_ascii_whitespace().into_iter().map(|c| {
        match c.parse::<f64>() {
            Ok(num) => {
                let exp_val = num.exp();
                exp_val.to_string()
            }
            Err(_) => {
                eprintln!("Warning: Could not parse '{}' as a number", c);
                "ERROR".to_string()
            }
        }
    })
    .collect::<Vec<String>>()
    .join(" ");

    (a, result)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let result: Vec<f64> = b.iter().map(|c| {
        let num = (c.abs() as f64).ln();
        num
    }).collect();
    return (b, result);
}