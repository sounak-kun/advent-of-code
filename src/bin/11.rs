use std::io::{self, prelude::*};

fn increment_string(string: &mut str) {
    for byte in unsafe { string.as_bytes_mut() }.iter_mut().rev() {
        match byte {
            b'z' => *byte = b'a',
            b'a'..=b'y' => {
                *byte += 1;
                return;
            }
            _ => unreachable!(),
        }
    }
}

fn is_password_valid(password: &str) -> bool {
    if password
        .bytes()
        .any(|byte| byte == b'i' || byte == b'o' || byte == b'l')
    {
        return false;
    }

    let mut skip = false;
    if password
        .as_bytes()
        .windows(2)
        .filter(|w| match skip {
            false if w[0] == w[1] => {
                skip = true;
                true
            }
            _ => {
                skip = false;
                false
            }
        })
        .count()
        < 2
    {
        return false;
    }

    if password
        .as_bytes()
        .windows(3)
        .any(|w| w[1] == w[0] + 1 && w[2] == w[1] + 1)
    {
        return true;
    }

    false
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Parse input
    let input = input.trim();

    // Part 1
    let mut string = input.to_string();
    loop {
        increment_string(&mut string);
        if is_password_valid(&string) {
            break;
        }
    }
    println!("Part 1: {}", string);

    // Part 2
    loop {
        increment_string(&mut string);
        if is_password_valid(&string) {
            break;
        }
    }
    println!("Part 2: {}", string);
}
