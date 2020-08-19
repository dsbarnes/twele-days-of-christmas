// 12 days of Christmas challenge from rust book
// https://doc.rust-lang.org/book/ch03-05-control-flow.html
// soln: https://github.com/BrooksPatton/learning-rust/blob/master/chapter3/twelve_days/src/main.rs
//

const GIFTS:[&str; 12] = [ 
    "A partridge in a pear tree",
    "Two turtle doves",
    "Three French hens",
    "Four calling birds",
    "Five gold rings, badam-pam-pam",
    "Six geese a laying",
    "Seven swans a swimming",
    "Eight maids a milking",
    "Nine ladies dancing",
    "Ten lords a leaping",
    "Eleven pipers piping",
    "12 drummers drumming",
];

const DAYS:[&str; 12] = [
    "first", 
    "second", 
    "third", 
    "forth", 
    "fif", 
    "sixth", 
    "seventh", 
    "eighth", 
    "ninth", 
    "tenth", 
    "eleventh", 
    "twelfth", 
];


fn main() {
    // If we wanted to reverse the const:
    // Yes, it needs to be a Vec
    // Cannot use .copy() nor .clone()
    // they are not methods on Trait std::iter::Iterator
    // no idea why
    let v:Vec<&str> = GIFTS.iter().cloned().rev().collect();
    print!("{:?}", v);


    for (index, day) in DAYS.iter().enumerate() {
        println!("On the {} day of Christmas\nMy true love gave to me", day);
        println!("{}", GIFTS[index]);
        // I never would have found this had it not been for the GitHub repo.
        // Is there a name for this??
        (0..index).rev().for_each(|gift| println!("{}", GIFTS[gift]));
        println!("");
    }
}
