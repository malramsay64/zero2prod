use validator::validate_email;

#[derive(Debug, Clone)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        match validate_email(&s) {
            true => Ok(Self(s)),
            false => Err(format! {"{} is not a valid subscriber email.", s}),
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberEmail;
    use claim::assert_err;
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary<G: quickcheck::Gen + rand::RngCore>(g: &mut G) -> Self {
            let email = SafeEmail().fake_with_rng(g);
            Self(email)
        }
    }

    // GIVEN: A randomly generated SafeEmail
    // WHEN: Constructing a SubscriberEmail through parsing
    // THEN: We should get an Ok
    #[quickcheck_macros::quickcheck]
    fn parse_valid_email(valid_email: ValidEmailFixture) -> bool {
        SubscriberEmail::parse(valid_email.0).is_ok()
    }

    #[test]
    // GIVEN: An emtpy string
    // WHEN: Parsing to a SubscriberEmail
    // THEN: We get an error
    fn parse_empty_string_rejected() {
        let email = "".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    // GIVEN: A string missing an @ symbol
    // WHEN: Parsing to a SubscriberEmail
    // THEN: We get an error
    fn parse_missing_at_string_rejected() {
        let email = "ursuladomain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    // GIVEN: A string starting with an @ symbol
    // WHEN: Parsing to a SubscriberEmail
    // THEN: We get an error
    fn parse_missing_subject_string_rejected() {
        let email = "@domain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }
}
