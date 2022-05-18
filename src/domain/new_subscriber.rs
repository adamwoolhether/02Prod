use crate::domain::SubscriberEmail;
use crate::domain::SubscriberName;

pub struct NewSubscriber {
    // No longer using `String`
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
