use md5::Digest;

fn match_5(digest: Digest) -> bool {
    (digest[0] == 0) && (digest[1] == 0) && ((digest[2] & 0xf0) == 0)
}

fn match_6(digest: Digest) -> bool {
    (digest[0] == 0) && (digest[1] == 0) && (digest[2] == 0)
}

fn advent_coin(key: &str, found: &dyn Fn(Digest) -> bool) -> i32 {
    let mut num = 0;
    loop {
        let s = format!("{}{}", key, num);
        let bytes = s.as_bytes();
        let digest = md5::compute(bytes);
        if found(digest) {
            break;
        }
        num = num + 1;
    }
    num
}

fn main() {
    const KEY: &str = "ckczppom";

    assert_eq!(advent_coin("abcdef", &match_5), 609043);
    assert_eq!(advent_coin("pqrstuv", &match_5), 1048970);
    assert_eq!(advent_coin(KEY, &match_5), 117946);
    assert_eq!(advent_coin(KEY, &match_6), 3938038);
}
