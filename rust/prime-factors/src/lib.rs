pub fn factors(n: u64) -> Vec<u64> {
    let mut input = n;
    let mut output = vec![];
    while input != 1 {
        let factor = (2..=input).find(|x| input % x == 0).unwrap();
        output.push(factor);
        input = input / factor;
    }
    output
}
