#![allow(dead_code)]

fn r#fn() {
    println!("A function called `fn`... fun");
}

fn cel_to_far(cel: f32) -> f32 {
    cel * 1.8 + 32.0
}

fn far_to_cel(far: f32) -> f32 {
    (far - 32.0) / 1.8
}

fn fib(n: usize) -> usize {
    if n < 2 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
const DAYS: [&'static str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
const GIFTS: [&'static str; 12] = [
    "a partridge in a pear tree",
    "two turtle doves",
    "three french hens",
    "four calling birds",
    "five golden rings",
    "six geese a laying",
    "seven swans a swimming",
    "eight maids a milking",
    "nine ladies dancing",
    "ten lords a leaping",
    "eleven pipers piping",
    "twelve drummers drumming",
];

fn twelve_days() {
    for day in 0..12 {
        println!("On the {} day of Christmas my true love gave to me:", DAYS[day]);
        for gift in (1..day+1).rev() {
            println!("{}", GIFTS[gift]);
        }

        if day != 0 {
            print!("and ");
        }
        println!("{}", GIFTS[0]);
        println!("");
    }
}

fn main() {
    twelve_days();
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn temp_conversion_round_trips() {
        assert_eq!(32.0, cel_to_far(far_to_cel(32.0)));
    }

    #[test]
    fn fib_works() {
        assert_eq!(0, fib(0));
        assert_eq!(1, fib(1));
        assert_eq!(1, fib(2));
        assert_eq!(2, fib(3));
        assert_eq!(55, fib(10));
    }
}
