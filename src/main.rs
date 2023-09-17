use std::io::{stdin, stdout, Read, Result, Write};

fn main() -> Result<()> {
    print!("Enter the cipher text: ");
    stdout().flush().unwrap();
    let mut cipher_text = String::new();
    let mut result_text: String = String::new();
    stdin().read_line(&mut cipher_text).unwrap();
    cipher_text = cipher_text.to_lowercase();
    for i in 1..26 {
        if search_rot(i, &cipher_text, &mut result_text) {
            break;
        }
    }
    println!("{}", result_text);
    pause();
    Ok(())
}
fn search_rot(rot: i32, cipher_text: &str, result_text: &mut String) -> bool {
    let keywords = vec![
        "what ", "is ", "the ", " of ", " by ", " to ", "did ", " in ", "when ", " how ",
    ];
    for c in cipher_text.chars() {
        if c != '\n' && c != ' ' {
            println!(
                "{}:{}, {}:{}",
                c,
                c as u32,
                char::from_u32((((c as i32 - 97) + rot) % (122 - 97 + 1) + 97) as u32).unwrap(),
                (((c as i32 - 97) + rot) % (122 - 97 + 1) + 97) as u32
            );
            // C_R = (((c-min)+rot)%(max-min+1)+min
            result_text.push(
                char::from_u32((((c as i32 - 97) + rot) % (122 - 97 + 1) + 97) as u32).unwrap(),
            );
            continue;
        }
        result_text.push(c);
    }
    for word in keywords {
        if result_text.contains(word) {
            println!(
                "{}: Candidate (ROT{}), keyword: ({})",
                result_text, rot, word
            );
            return true;
        }
    }
    println!("{}", result_text);
    result_text.clear();
    return false;
}
fn pause() {
    stdout().write(b"Press Enter to continue...").unwrap();
    stdout().flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
