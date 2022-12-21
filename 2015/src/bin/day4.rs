fn solve(secret_key: &str, len: usize) {
    let n = (1..)
        .map(|n| {
            (n, {
                let key = format!("{}{}", secret_key, n);
                let compute = md5::compute(key);
                compute
                    .iter()
                    .flat_map(|n| [n & 0xF0, n & 0xF])
                    .take(len)
                    .collect::<Vec<_>>()
            })
        })
        .find(|(_, computed)| computed.iter().all(|&e| e == 0))
        .map(|(n, _)| n)
        .unwrap_or_default();

    println!("Solution for {secret_key} and length {len} = {n}");
}

fn main() {
    solve("ckczppom", 5);
    solve("ckczppom", 6);
}
