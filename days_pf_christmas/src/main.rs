fn sing() {
    let lyric = [
        "partridge in a pear tree",
        "turtle doves",
        "french hens",
        "calling birds",
        "gold rings",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming"
    ];

    for index in 0..lyric.len() {
        println!("\nOn the {}th day of Christmas my true love sent to me", index+1);

        if index == 0 {
            println!("A {}", lyric[index]);
        } else {
            println!("{} {}", index+1, lyric[index]);
        }

        for down_index in (0..index).rev() {
            if down_index == 0 {
                println!("And... a {}!", lyric[down_index]);
            } else {
                println!("{} {}", down_index+1, lyric[down_index]);
            }
        }
    }
}


fn main() {
    println!("Ladies and gentlemen, please hold your applause till the end...");
    sing();
}
