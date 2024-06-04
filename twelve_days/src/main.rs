fn main() {
    for day in 1..13 {
        println!(
            "On the {} day of Christmas my true love gave to me...",
            print_day(&day)
        );
        print_gift(&day);
        println!();
    }
}

fn print_day(day: &u32) -> String {
    match day {
        1 => "1st".to_string(),
        2 => "2nd".to_string(),
        3 => "3rd".to_string(),
        4..=12 => day.to_string() + "th",
        _ => "error!".to_string(),
    }
}

fn print_gift(day: &u32) {
    let mut day = *day;
    while day > 0 {
        match day {
            1 => println!("a partridge in a pear tree."),
            2 => {
                println!("two turtle doves,");
                print!("and ");
            },
            3 => println!("three french hens,"),
            4 => println!("four calling birds"),
            5 => println!("five golden rings"),
            6 => println!("six geese a-laying"),
            7 => println!("seven swans a-swimming"),
            8 => println!("eight maids a-milking"),
            9 => println!("nine ladies dancing"),
            10 => println!("ten lords a-leaping"),
            11 => println!("eleven pipers piping"),
            12 => println!("twelve drummers drumming"),
            _ => println!("error!"),
        }
        day -= 1;
    }
}
