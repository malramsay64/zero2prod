use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();

        let is_too_long = s.graphemes(true).count() > 256;

        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid subscriber name.", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberName;

    use claim::{assert_err, assert_ok};

    #[test]
    // GIVEN: A 256 character name
    // WHEN: Pasing the name to a Subscriber name
    // THEN: We construct the SubscriberName
    fn parse_long_name() {
        let name = "a".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    // GIVEN: A 257 character name
    // WHEN: Pasing the name to a Subscriber name
    // THEN: We fail to construct the SubscriberName, getting an Err
    fn parse_too_long_name_rejection() {
        let name = "a".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    // GIVEN: A string containing only whitespace
    // WHEN: Pasing the name to a Subscriber name
    // THEN: We fail to construct the SubscriberName, getting an Err
    fn parse_whitespace_name_rejection() {
        let name = " ".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    // GIVEN: An empty string
    // WHEN: Pasing the name to a Subscriber name
    // THEN: We fail to construct the SubscriberName, getting an Err
    fn parse_empty_name_rejection() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }
}
