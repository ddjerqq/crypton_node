use chbs::{config::BasicConfig, prelude::*};
use chbs::probability::Probability;

pub fn passphrase(count: usize) -> String {
    let mut config = BasicConfig::default();
    config.words = count;

    config.capitalize_first = Probability::Never;
    config.capitalize_words = Probability::Never;

    config.to_scheme().generate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passphrase() {
        let p = passphrase(12);
        println!("{}", p);
    }
}