// fizz buzz

fn main() {
    // 3 and 5
    for i in 1..31 {
        let mut output = String::from("");
        if i % 3 == 0 {
            output.push_str("Fizz");
        }
        if i % 5 == 0 {
            output.push_str("Buzz");
        }
        if output == "" {
            output.push_str(&i.to_string());
        }
        println!("{output}");
    }
}
