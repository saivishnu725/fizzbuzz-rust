// fizz buzz

fn compare(current: i32, num: i32, output: &mut String, word: &str) {
    if current % num == 0 {
        output.push_str(word);
    }
}

fn main() {
    // 2, 3 and 5
    for i in 1..31 {
        let mut output = String::from("");
        compare(i, 2, &mut output, "Fudge");
        compare(i, 3, &mut output, "Fizz");
        compare(i, 5, &mut output, "Buzz");

        if output == "" {
            output.push_str(&i.to_string());
        }
        println!("{output}");
    }
}
