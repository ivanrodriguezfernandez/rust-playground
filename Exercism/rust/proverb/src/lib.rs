pub fn build_proverb(list: &[&str]) -> String {
    let mut result = String::from("");

    println!("{:?}\n", list);
    println!("len-> {}\n", list.len());
    if list.len() > 1 {
        for x in 0..list.len() - 1 {
            print!("{} \n", list[x]);
            result.push_str(
                &format!(
                    "For want of a {} the {} was lost.\n",
                    list[x].to_string(),
                    list[x + 1].to_string()
                )
                .to_string(),
            );
        }
    }

    if list.len() > 0 {
        result.push_str("And all for the want of a nail.");
    }

    result.to_string()
}
