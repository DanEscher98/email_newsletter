use anyhow::{anyhow, Result};
use validator::validate_email;

#[derive(Debug, Clone)]
pub struct ValidEmail(String);

impl ValidEmail {
    pub fn parse(s: String) -> Result<ValidEmail> {
        if validate_email(&s) {
            return Ok(Self(s));
        }
        Err(anyhow!(format!("{} is not a valid subscriber email.", s)))
    }
}
impl AsRef<str> for ValidEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::ValidEmail;
    use claim::assert_err;
    use fake::{faker::internet::en::SafeEmail, Fake};

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let email = SafeEmail().fake_with_rng(g);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valdi_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        ValidEmail::parse(valid_email.0).is_ok()
    }

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_err!(ValidEmail::parse(email));
    }
    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert_err!(ValidEmail::parse(email));
    }
    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert_err!(ValidEmail::parse(email));
    }
}
