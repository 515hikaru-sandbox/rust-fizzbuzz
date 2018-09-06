fn main() {
    for n in 1..100 {
        let s = fizzbuzz(n);
        println!("{}", s);
    }
}

fn fizzbuzz(n: i64) -> String {
    if n % 15 == 0 {
        return String::from("FizzBuzz");
    };
    if n % 5 == 0 {
        return String::from("Buzz");
    };
    if n % 3 == 0 {
        return String::from("Fizz");
    };
    n.to_string()
}
