/// Calculates the SDBM Hash of a given string.
pub fn sdbm_hash(s: &str) -> u32 {
    let mut hash: u32 = 0;

    for ch in s.encode_utf16() {
        hash = u32::from(ch)
            .wrapping_add(hash << 6)
            .wrapping_add(hash << 16)
            .wrapping_sub(hash);
    }

    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test from https://www.programmingalgorithms.com/algorithm/sdbm-hash?lang=C%2B%2B
    #[test]
    fn programming_algorithms() {
        assert_eq!(
            sdbm_hash("jdfgsdhfsdfsd 6445dsfsd7fg/*/+bfjsdgf%$^"),
            423809171
        );
    }

    // Tests from https://github.com/sindresorhus/sdbm/blob/0aded7015f7a4284383b3ca711f8e85e4a3134d6/test.js
    #[test]
    fn unicorn_rainbow() {
        assert_eq!(sdbm_hash("🦄🌈"), 4053542802);
    }

    #[test]
    fn nothing() {
        assert_eq!(sdbm_hash(""), 0);
    }

    #[test]
    fn h() {
        assert_eq!(sdbm_hash("h"), 104);
    }

    #[test]
    fn he() {
        assert_eq!(sdbm_hash("he"), 6822397);
    }

    #[test]
    fn hel() {
        assert_eq!(sdbm_hash("hel"), 865822127);
    }

    #[test]
    fn hell() {
        assert_eq!(sdbm_hash("hell"), 418186877);
    }

    #[test]
    fn hello() {
        assert_eq!(sdbm_hash("hello"), 684824882);
    }

    #[test]
    fn hello_() {
        assert_eq!(sdbm_hash("hello "), 2764485486);
    }

    #[test]
    fn hello_w() {
        assert_eq!(sdbm_hash("hello w"), 1079257225);
    }

    #[test]
    fn hello_wo() {
        assert_eq!(sdbm_hash("hello wo"), 4248762918);
    }

    #[test]
    fn hello_wor() {
        assert_eq!(sdbm_hash("hello wor"), 1285918668);
    }

    #[test]
    fn hello_worl() {
        assert_eq!(sdbm_hash("hello worl"), 1821008800);
    }

    #[test]
    fn hello_world() {
        assert_eq!(sdbm_hash("hello world"), 430867652);
    }

    #[test]
    fn lorem_ipsum() {
        assert_eq!(sdbm_hash("Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Donec quam felis, ultricies nec, pellentesque eu, pretium quis, sem. Nulla consequat massa quis enim. Donec pede justo, fringilla vel, aliquet nec, vulputate eget, arcu. In enim justo, rhoncus ut, imperdiet a, venenatis vitae, justo. Nullam dictum felis eu pede mollis pretium. Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Donec quam felis, ultricies nec, pellentesque eu, pretium quis, sem. Nulla consequat massa quis enim. Donec pede justo, fringilla vel, aliquet nec, vulputate eget, arcu. In enim justo, rhoncus ut, imperdiet a, venenatis vitae, justo. Nullam dictum felis eu pede mollis pretium. Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Donec quam felis, ultricies nec, pellentesque eu, pretium quis, sem. Nulla consequat massa quis enim. Donec pede justo, fringilla vel, aliquet nec, vulputate eget, arcu. In enim justo, rhoncus ut, imperdiet a, venenatis vitae, justo. Nullam dictum felis eu pede mollis pretium."), 81306442);
    }
}
