pub fn raindrops(n: u32) -> String {
    if (n % 105) == 0 {
        String::from("PlingPlangPlong")
    } else if (n % 35) == 0 {
        String::from("PlangPlong")
    } else if (n % 21) == 0 {
        String::from("PlingPlong")
    } else if (n % 15) == 0 {
        String::from("PlingPlang")
    } else if (n % 7) == 0 {
        String::from("Plong")
    } else if (n % 5) == 0 {
        String::from("Plang")
    } else if (n % 3) == 0 {
        String::from("Pling")
    } else {
        n.to_string()
    }
}
