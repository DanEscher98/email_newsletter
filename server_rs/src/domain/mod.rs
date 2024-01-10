mod subscriber_email;
mod subscriber_name;
use anyhow::Result;

pub use subscriber_email::ValidEmail;
use subscriber_name::SubscriberName;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub struct Subscriber {
    pub email: ValidEmail,
    pub name: SubscriberName,
}

impl Subscriber {
    pub fn parse(form: FormData) -> Result<Subscriber> {
        form.try_into()
    }
}

impl TryFrom<FormData> for Subscriber {
    type Error = anyhow::Error;

    fn try_from(form: FormData) -> Result<Subscriber> {
        let email = ValidEmail::parse(form.email)?;
        let name = SubscriberName::parse(form.name)?;

        Ok(Self { email, name })
    }
}
