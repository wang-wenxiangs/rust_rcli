use rand::prelude::{IndexedRandom, SliceRandom};

const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &[u8] = b"0123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*_+-=|?";

// 生成密码的函数
pub fn genpass_process(
    // 密码长度
    length: u8,
    // 是否包含大写字母
    uppercase: bool,
    // 是否包含小写字母
    lowercase: bool,
    // 是否包含数字
    numbers: bool,
    // 是否包含符号
    symbols: bool,
) -> anyhow::Result<()> {
    // 创建随机数生成器
    let mut rng = rand::rng();
    // 创建密码的向量
    let mut password: Vec<u8> = Vec::new();
    // 创建字符的向量
    let mut chars = Vec::new();
    // 如果包含大写字母，则将大写字母添加到字符向量中，并从大写字母中随机选择一个字符添加到密码向量中
    if uppercase {
        chars.extend_from_slice(UPPERCASE);
        password.push(
            *UPPERCASE
                .choose(&mut rng)
                .expect("UPPERCASE won't be empty"),
        );
    }
    // 如果包含小写字母，则将小写字母添加到字符向量中，并从小写字母中随机选择一个字符添加到密码向量中
    if lowercase {
        chars.extend_from_slice(LOWERCASE);
        password.push(
            *LOWERCASE
                .choose(&mut rng)
                .expect("LOWERCASE won't be empty"),
        );
    }
    // 如果包含数字，则将数字添加到字符向量中，并从数字中随机选择一个字符添加到密码向量中
    if numbers {
        chars.extend_from_slice(NUMBERS);
        password.push(*NUMBERS.choose(&mut rng).expect("NUMBERS won't be empty"));
    }
    // 如果包含符号，则将符号添加到字符向量中，并从符号中随机选择一个字符添加到密码向量中
    if symbols {
        chars.extend_from_slice(SYMBOLS);
        password.push(*SYMBOLS.choose(&mut rng).expect("SYMBOLS won't be empty"));
    }

    // 循环添加剩余的字符到密码向量中
    for _ in 0..(length - password.len() as u8) {
        let c = chars.choose(&mut rng).expect("chars won't be empty");
        password.push(*c);
    }
    // 随机打乱密码向量中的字符
    password.shuffle(&mut rng);

    // 将密码向量转换为字符串
    let password = String::from_utf8(password)?;
    // 打印密码
    println!("password: {}", password);
    // 使用zxcvbn库计算密码的强度
    let result = zxcvbn::zxcvbn(&password, &[]);
    // 打印密码强度
    println!("Password level: {}", result.score());
    // 返回结果

    Ok(())
}
