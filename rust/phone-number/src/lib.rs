use std::borrow::Cow;

pub fn number(phone: &str) -> Option<String> {
    let mut pn = clean_number(phone);
    match pn.len() {
        10 => Some(pn),
        11 => if phone.starts_with('1') { pn.remove(0); Some(pn) }
              else { None },
        _ => None
    }
}

pub fn area_code(phone: &str) -> Option<String> {
    match number(phone) {
        Some(ref pn) => Some(area_code_part(pn).to_string()),
        None => None
    }
}

pub fn pretty_print(phone: &str) -> Cow<str> {
    match number(phone) {
        Some(ref pn) => pretty_number(pn).into(),
        None => "invalid".into()
    }
}

fn clean_number(phone: &str) -> String {
    phone.chars()
        .filter(|&c| c.is_digit(10) )
        .collect::<String>()
}

fn pretty_number(phone: &str) -> String {
    format!("({}) {}-{}",
        area_code_part(phone),
        local_exchange_part(phone),
        line_number_part(phone))
}

fn area_code_part(phone: &str) -> &str {
    &phone[0..3]
}

fn local_exchange_part(phone: &str) -> &str {
    &phone[3..6]
}

fn line_number_part(phone: &str) -> &str {
    &phone[6..10]
}
