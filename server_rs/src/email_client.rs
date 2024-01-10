use crate::domain::ValidEmail;
use reqwest::{Client, Url};

#[derive(Clone)]
pub struct EmailClient {
    http_client: Client,
    base_url: Url,
    sender: ValidEmail,
}

impl EmailClient {
    pub fn new(base_url: Url, sender: ValidEmail) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            sender,
        }
    }
    pub async fn send_email(
        &self,
        recipient: ValidEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> anyhow::Result<()> {
        let url = self.base_url.join("email")?;
        let request_body = SendEmailRequest {
            from: self.sender.as_ref().to_owned(),
            to: recipient.as_ref().to_owned(),
            subject: subject.to_owned(),
            html_body: html_content.to_owned(),
            text_body: text_content.to_owned(),
        };
        let builder = self.http_client.post(url).json(&request_body);
        Ok(())
    }
}

#[derive(serde::Serialize)]
struct SendEmailRequest {
    from: String,
    to: String,
    subject: String,
    html_body: String,
    text_body: String,
}

#[cfg(test)]
mod tests {
    use crate::{domain::ValidEmail, email_client::EmailClient};
    use fake::{
        faker::{
            internet::en::SafeEmail,
            lorem::en::{Paragraph, Sentence},
        },
        Fake, Faker,
    };
    use reqwest::Url;
    use wiremock::{matchers::any, Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn send_email_fires_request_to_base_url() {
        // Arrange
        let mock_server = MockServer::start().await;
        let sender = ValidEmail::parse(SafeEmail().fake()).unwrap();
        let uri = Url::parse(mock_server.uri().as_ref()).unwrap();
        let email_client = EmailClient::new(uri, sender);

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let subscriber_email = ValidEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();

        // Act
        let _ = email_client
            .send_email(subscriber_email, &subject, &content, &content)
            .await;

        // Assert
    }
}
