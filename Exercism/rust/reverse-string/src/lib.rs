pub fn reverse(input: &str) -> String {

    let rev: String =   input
                        .chars()
                        .rev()
                        .collect();


    return rev;
}
