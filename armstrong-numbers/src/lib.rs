pub fn is_armstrong_number(num: u32) -> bool {
    let b = 10;
    let k = f32::log10(num as f32).floor() as u32 + 1;

    let c = (0..k)
        .map(|i| {
            u32::pow(
                (num % u32::pow(b, i + 1) - num % u32::pow(b, i))
                    / u32::pow(b, i),
                k,
            )
        })
        .sum();

    num == c
}
