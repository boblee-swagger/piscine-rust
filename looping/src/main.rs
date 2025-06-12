use std::io;

fn main() -> io::Result<()>{

    let mut num_attempts : i8 = 0;

    let answer = String::from("The letter e");
    let question = String::from("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
    let mut input = String::new();

    loop {
        println!("{}", question);
        io::stdin().read_line(&mut input)?;
        num_attempts += 1;

        if input.trim() == answer {
            break;
        }
    }

    println!("{}", answer);
    Ok(())
}