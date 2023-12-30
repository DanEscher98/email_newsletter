mod subscriber_email;
mod subscriber_name;
use anyhow::Result;

use subscriber_email::SubscriberEmail;
use subscriber_name::SubscriberName;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}

impl NewSubscriber {
    pub fn parse(form: FormData) -> Result<NewSubscriber> {
        form.try_into()
    }
}

impl TryFrom<FormData> for NewSubscriber {
    type Error = anyhow::Error;

    fn try_from(form: FormData) -> Result<NewSubscriber> {
        let email = SubscriberEmail::parse(form.email)?;
        let name = SubscriberName::parse(form.name)?;

        Ok(Self { email, name })
    }
}
