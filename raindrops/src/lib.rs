pub fn raindrops(n: u32) -> String {

    let contains_factor = |i| n % i == 0;

    let mut res = "".to_string();

    if contains_factor(3) {
        res += "Pling";
    }

    if contains_factor(5) {
        res += "Plang";
    }

    if contains_factor(7) {
        res += "Plong";
    }

    if res == "" {
        return n.to_string()
    }

    res
}
