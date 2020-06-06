pub fn verse(n: u32) -> String {
    let mut result = String::from("");

    if n > 0 {
        let current_bottles: String = format!(
            "{} of beer on the wall, {} of beer.",
            format_num_bottles(n),
            format_num_bottles(n)
        );

        if n == 1 {
            result = format!(
                "{}\nTake it down and pass it around, no more bottles of beer on the wall.\n",
                current_bottles
            )
        }

        if n >= 2 {
            result = format!(
                "{}\nTake one down and pass it around, {} of beer on the wall.\n",
                current_bottles,
                format_num_bottles(n - 1)
            );
        }
    } else {
        result = String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    }
    result
}

pub fn format_num_bottles(n: u32) -> String {
    let mut format_bottle = format!("{} bottle", n);
    if n > 1 {
        format_bottle = format!("{}{}", format_bottle, "s");
    }
    format_bottle
}

pub fn sing(start: u32, end: u32) -> String {
    let mut result = String::from("");
    for x in (end..=start).rev() {
        result += &(verse(x));
        if x > end {
            result += "\n";
        }
    }
    result
}
