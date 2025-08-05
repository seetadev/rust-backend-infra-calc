use aws_sdk_ses::{Client, Error};
use aws_sdk_ses::types::{Body, Content, Destination, Message};
use crate::config::AppConfig;

pub struct EmailService {
    client: Client,
    from_email: String,
}

impl EmailService {
    pub fn new(config: &AppConfig) -> Self {
        let aws_config = aws_config::from_env()
            .region(aws_config::Region::new(config.aws_region.clone()));
        
        // Note: In a real implementation, you'd want to properly configure AWS credentials
        let client = Client::new(&aws_config::load_from_env().await);
        
        Self {
            client,
            from_email: config.ses_from_email.clone(),
        }
    }

    pub async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), Error> {
        let dest = Destination::builder().to_addresses(to).build();
        
        let subject_content = Content::builder().data(subject).charset("UTF-8").build();
        let body_content = Content::builder().data(body).charset("UTF-8").build();
        let body = Body::builder().text(body_content).build();

        let msg = Message::builder()
            .subject(subject_content)
            .body(body)
            .build();

        self.client
            .send_email()
            .source(&self.from_email)
            .destination(dest)
            .message(msg)
            .send()
            .await?;

        Ok(())
    }

    pub async fn send_html_email(&self, to: &str, subject: &str, html_body: &str) -> Result<(), Error> {
        let dest = Destination::builder().to_addresses(to).build();
        
        let subject_content = Content::builder().data(subject).charset("UTF-8").build();
        let html_content = Content::builder().data(html_body).charset("UTF-8").build();
        let body = Body::builder().html(html_content).build();

        let msg = Message::builder()
            .subject(subject_content)
            .body(body)
            .build();

        self.client
            .send_email()
            .source(&self.from_email)
            .destination(dest)
            .message(msg)
            .send()
            .await?;

        Ok(())
    }
}