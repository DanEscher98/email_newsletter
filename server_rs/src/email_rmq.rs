use crate::domain::ValidEmail;
use deadpool_lapin::{
    lapin::{options, types, ExchangeKind},
    Config, Manager, Pool, PoolConfig, PoolError, Runtime,
};
use reqwest::{Client, Url};

pub type RmqPool = Pool;
type RmqConnection = deadpool::managed::Object<deadpool_lapin::Manager>;

#[derive(Clone)]
pub struct EmailRmq {
    pool: RmqPool,
    sender: ValidEmail,
    exchange: String,
}

pub enum EmailQueue {
    Validate,
    Publish,
}

impl ToString for EmailQueue {
    fn to_string(&self) -> String {
        match self {
            Self::Validate => String::from("validate"),
            Self::Publish => String::from("publish"),
        }
    }
}

impl EmailRmq {
    fn pool(url: String) -> anyhow::Result<RmqPool> {
        Ok(Config {
            url: Some(url),
            ..Default::default()
        }
        .builder(Some(Runtime::Tokio1))
        .config(PoolConfig::new(100))
        .build()?)
    }
    async fn init_exchange(pool: RmqPool, exchange: &str) -> anyhow::Result<()> {
        let channel = pool.get().await?.create_channel().await?;
        channel
            .exchange_declare(
                exchange,
                ExchangeKind::Direct,
                options::ExchangeDeclareOptions::default(),
                types::FieldTable::default(),
            )
            .await?;
        for queue in [EmailQueue::Publish, EmailQueue::Validate] {
            channel
                .queue_declare(
                    queue.to_string().as_str(),
                    options::QueueDeclareOptions::default(),
                    types::FieldTable::default(),
                )
                .await?;
        }
        Ok(())
    }
    #[tracing::instrument(name = "Connecting to RabbitMQ", skip(sender))]
    pub async fn new(url: String, sender: ValidEmail, exchange: String) -> anyhow::Result<Self> {
        let pool = Self::pool(url)?;
        Self::init_exchange(pool.clone(), exchange.as_str()).await?;
        Ok(Self {
            pool,
            sender,
            exchange,
        })
    }
    pub async fn send_email(
        &self,
        recipient: ValidEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
        queue: EmailQueue,
    ) -> anyhow::Result<()> {
        let email = SendEmail {
            from: self.sender.as_ref().to_owned(),
            to: recipient.as_ref().to_owned(),
            subject: subject.to_owned(),
            html_body: html_content.to_owned(),
            text_body: text_content.to_owned(),
        };

        let payload = serde_json::to_vec(&email)?;
        Ok(())
    }
}

#[derive(serde::Serialize)]
struct SendEmail {
    from: String,
    to: String,
    subject: String,
    html_body: String,
    text_body: String,
}
