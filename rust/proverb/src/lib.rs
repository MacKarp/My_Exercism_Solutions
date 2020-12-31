pub fn build_proverb(list: &[&str]) -> String {
    let mut output = String::new();
    if list.len() != 0 {
        for i in 0..list.len() - 1 {
            output += format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]).as_str();
        }
        output += format!("And all for the want of a {}.", list[0]).as_str();
    }
    output
}
