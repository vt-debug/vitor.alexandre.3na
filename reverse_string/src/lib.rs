pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}


fn main() {
    println!("{}", reverse("Olá"));
}
