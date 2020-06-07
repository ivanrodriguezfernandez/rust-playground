pub fn build_proverb(list: &[&str]) -> String {
    let mut result = String::from("");
    if list.len() == 1 {
        result.push_str("And all for the want of a nail.");
    }
    println!("{:?}\n", list);
    println!("len-> {}\n", list.len());

    if list.len() == 2 {
        let aux2 = format!(
            "For want of a {} the {} was lost.\n",
            list[0].to_string(),
            list[1].to_string()
        );

        result.push_str(&aux2.to_string());
    }

    if list.len() > 2 {
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

    if list.len() > 1 {
        result.push_str("And all for the want of a nail.");
    }
    result.to_string()
}
