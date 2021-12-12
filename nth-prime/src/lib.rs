pub fn nth(n: u32) -> u32 {
    get_prime(n)
}

/*
pub fn sieve(s: u32) -> Vec<u32> {
    let mut current = 2;
    let mut p = 0;
    loop {
        let res = sieve_filter(s, p);

        if res.len() > s as usize {
            return res;
        } else {
            p += 1;
        }

        if p == 25 {
            return (2..15).map(|x| x as u32).collect::<Vec<u32>>()
        }
    }
}

pub fn sieve_filter(s: u32, p: i32) -> Vec<u32> {

    let arr = (2..s).map(|x| x as u32).collect::<Vec<u32>>();

    arr
        .filter(|x| x % arr[p as usize] as u32 != 0)
        .collect::<Vec<u32>>()
}
*/

pub fn get_prime(n: u32) -> u32 {

    if n == 0 {
        return 2 as u32;
    }
    let arr = (3..(4*n)).map(|x| x as u32).collect::<Vec<u32>>();

    (0..n).into_iter()
        .map(|prime| {arr.iter().filter(move |elem| *elem % arr[prime as usize] != 0)});


    return arr[n as usize] as u32;
}
