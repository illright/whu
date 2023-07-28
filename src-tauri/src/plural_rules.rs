fn en(amount: i32) -> &'static str {
    match amount {
        1 => "one",
        _ => "other",
    }
}

fn ru(amount: i32) -> &'static str {
    let mod_100 = amount % 100;
    match amount % 10 {
        1 if mod_100 <= 10 || mod_100 >= 20 => "one",
        2..=4 if mod_100 <= 10 || mod_100 >= 20 => "few",
        _ => "many",
    }
}

pub fn select(locale: &str, amount: i32) -> &'static str {
    match locale {
        "en" => en(amount),
        "ru" => ru(amount),
        _ => "other",
    }
}
