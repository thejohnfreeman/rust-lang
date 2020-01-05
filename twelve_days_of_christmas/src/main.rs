const gifts: [&str; 12] = [
    "a partride in a pear tree",
    "two turtle doves",
    "three french hens",
    "four calling birds",
    "five golden rings",
    "six geese a-laying",
    "seven swans a-swimming",
    "eight maids a-milking",
    "nine ladies dancing",
    "ten lords a-leaping",
    "eleven pipers piping",
    "twelve drummers drumming",
];


fn main() {
    println!("Hello, world!");

    for day in 1..13 {
        println!("on the {} day of Christmas,", day);
        println!("my true love sent to me");
        for gift in (0..day).rev() {
            println!("{}", gifts[gift]);
        }
        println!();
    }
}
