pub fn raindrops(n: u32) -> String {
    let mut response = "".to_string();

    if n % 3 == 0 {
        response.push_str("Pling")
    }

    if n % 5 == 0 {
        response.push_str("Plang")
    }
    if n % 7 == 0 {
        response.push_str("Plong")
    }

    if response.is_empty() {
        n.to_string()
    } else {
        response
    }
}
