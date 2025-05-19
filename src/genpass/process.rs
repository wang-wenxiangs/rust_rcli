use rand::prelude::{IndexedRandom, SliceRandom};

const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &[u8] = b"0123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*_+-=|?";
pub fn genpass_process(
    length: u8,
    uppercase: bool,
    lowercase: bool,
    numbers: bool,
    symbols: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::rng();
    let mut password: Vec<u8> = Vec::new();
    let mut chars = Vec::new();
    if uppercase {
        chars.extend_from_slice(UPPERCASE);
        password.push(
            *UPPERCASE
                .choose(&mut rng)
                .expect("UPPERCASE won't be empty"),
        );
    }
    if lowercase {
        chars.extend_from_slice(LOWERCASE);
        password.push(
            *LOWERCASE
                .choose(&mut rng)
                .expect("LOWERCASE won't be empty"),
        );
    }
    if numbers {
        chars.extend_from_slice(NUMBERS);
        password.push(*NUMBERS.choose(&mut rng).expect("NUMBERS won't be empty"));
    }
    if symbols {
        chars.extend_from_slice(SYMBOLS);
        password.push(*SYMBOLS.choose(&mut rng).expect("SYMBOLS won't be empty"));
    }

    for _ in 0..(length - password.len() as u8) {
        let c = chars.choose(&mut rng).expect("chars won't be empty");
        password.push(*c);
    }
    password.shuffle(&mut rng);

    let password = String::from_utf8(password)?;
    println!("password: {}", password);
    let result = zxcvbn::zxcvbn(&password, &[]);
    println!("Password level: {}", result.score());

    Ok(())
}
