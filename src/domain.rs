use unicode_segmentation::UnicodeSegmentation;

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}

// SubscriberName is a tuple struct with a single, unnamed field of type String.
pub struct SubscriberName(String);

// Ensures all instances of SubscriberName satisfy validation constrains.
impl SubscriberName {
    // Returns an instance of `SubscriberName` if the input satisfies
    // all validation constraints on subscriber names, or an error message otherwise.
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        // `.trim()` returns a view over input `s` without
        // trailing whitespace-like characters.
        // `.is_empty()` checks if the view contains any character.
        let is_empty_or_whitespace = s.trim().is_empty();

        // A graphmeme is defined by the Unicode standard as a "user-perceived" char.
        // `å` is a single grapheme, but it is composed of two characters (`a` and `̊`).
        //
        // `graphmemes` returns an iterator over the graphememes in input `s`.
        // `true` specifies that we want to use the extended graphmeme definition set. (recommended)
        let is_too_long = s.graphemes(true).count() > 256;

        // Iterate over all chars in the input `s` to check if any
        // match a char in the forbidden array.
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            panic!("{} is not a valid subscriber name.", s)
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
