use itertools::Itertools;

fn main() {
    print_groups();
}

fn print_groups() {
    let mut groups = [const { vec![] }; 32];
    for n in 31..0x03FF {
        let bits = n & 31;
        if let Some(c) = char::from_u32(n as u32) {
            groups[bits].push(c);
        } else {
            // println!("{n}");
        }
    }
    for (idx, line) in groups.iter().enumerate() {
        println!(
            "{idx:02}: {}",
            line.iter().map(|x| format!("{x}")).join(" ")
        );
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    #[test]
    fn unicode_uppercase() {
        assert_eq!("SS", "ß".to_uppercase());
        assert_eq!("ÄÖÜ", "äöü".to_uppercase());
        assert_eq!("äöüAß", "äöüaß".to_ascii_uppercase());
    }
    #[test]
    fn unicode_letters() {
        assert_eq!(223, "ß".chars().next().unwrap() as u8);
        assert_eq!(228, "ä".chars().next().unwrap() as u8);
        assert_eq!(246, "ö".chars().next().unwrap() as u8);
        assert_eq!(252, "ü".chars().next().unwrap() as u8);
        assert_eq!(31, 31 & "ß".chars().next().unwrap() as u8);
        assert_eq!([0b_1100_0011, 0b_1001_1111], "ß".as_bytes());
        assert_eq!(vec!['ă'], "ă".chars().collect::<Vec<_>>());
        assert_eq!(vec![259], "ă".chars().map(|c| c as u16).collect_vec());
        assert_eq!(vec![1025], "Ё".chars().map(|c| c as u16).collect_vec());
        /*assert_eq!(
            "".to_string(),
            (200u8..255u8).map(|c: u8| c as char).collect::<String>()
        );*/
    }
    #[test]
    fn unicode_forms() {
        use unic_normal::StrNormalForm;
        assert_eq!(
            // cyrillic E and trema
            vec![0x415, 0x308],
            "Ё".nfd().map(|c: char| c as u32).collect_vec()
        );
    }
}
// ` : Gravis
// ´ : Akut
// ¨ : Trema
