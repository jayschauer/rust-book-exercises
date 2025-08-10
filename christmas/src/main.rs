fn main() {
    // Lyrics from genuis:
    // https://genius.com/Christmas-songs-the-twelve-days-of-christmas-lyrics

    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let ordinals = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for (day, &ordinal) in ordinals.iter().enumerate() {
        println!("On the {ordinal} day of Christmas my true love sent to me");

        // Print gifts in reverse order
        for gift in (0..=day).rev() {
            println!("{}", gifts[gift]);
        }
        println!(); // Blank line between days
    }
}
