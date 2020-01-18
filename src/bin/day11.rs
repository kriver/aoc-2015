use std::collections::HashSet;

fn is_forbidden(ch: u8) -> bool {
    match ch as char {
        'i' | 'o' | 'l' => true,
        _ => false
    }
}

fn is_valid(password: &[u8]) -> bool {
    // continuous straigth of 3
    let mut prev: u8 = 0;
    let mut straight = 0;
    // no i, o or l
    let mut has_forbidden = false;
    // two different non-overlapping pairs
    let mut pairs = HashSet::new();

    for c in password {
        if straight < 3 {
            if prev + 1 == *c {
                straight += 1;
            } else {
                straight = 1;
            }
        }
        has_forbidden |= is_forbidden(*c);
        if prev == *c {
            pairs.insert(*c);
        }
        prev = *c;
    }
    straight == 3 && !has_forbidden && pairs.len() >= 2
}

fn next_char(ch: u8) -> (u8, bool) {
    let mut next = ch;
    loop {
        let (n, w) = match next {
            b'z' => (b'a', true),
            _ => (next + 1, false)
        };
        // optimize, ski forbidden characters
        if !is_forbidden(n) {
            return (n, w)
        }
        next = n;
    }
}

fn next_word(word: &mut Vec<u8>) {
    let last = word.len() - 1;
    let (n, wrapped) = next_char(word[last]);
//    println!("{}: {} -> {}, {}", last, word[last], n, wrapped);
    word[last] = n;
    if wrapped {
        for i in (0..last).rev() {
            let (n, wrapped) = next_char(word[i]);
//            println!("{}: {} -> {}, {}", i, word[i], n, wrapped);
            word[i] = n;
            if !wrapped {
                break;
            }
        }
    }
}

fn next_password(password: &[u8]) -> Vec<u8> {
    let mut next = password.to_vec();
    loop {
        next_word(&mut next);
//        println!("{:?}", String::from_utf8_lossy(&next));
        if is_valid(&next) {
            break;
        }
    }
    next
}

fn main() {
    assert_eq!(next_char(b'a'), (b'b', false));
    assert_eq!(next_char(b'h'), (b'j', false));
    assert_eq!(next_char(b'z'), (b'a', true));

    assert!(!is_valid(b"hijklmmn"));
    assert!(!is_valid(b"abbceffg"));
    assert!(!is_valid(b"abbcegjk"));
    assert!(!is_valid(b"abcccc"));
    assert!(is_valid(b"abccbb"));

    assert_eq!(next_password(b"abcdefgh"), b"abcdffaa");
    assert_eq!(next_password(b"ghijklmn"), b"ghjaabcc");

    let next = next_password(b"vzbxkghb");
    assert_eq!(next, b"vzbxxyzz");
    let next = next_password(&next);
    assert_eq!(next, b"vzcaabcc");
}
