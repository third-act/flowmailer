use flowmailer::{auth, Client, MailAddress, MailBuilder};

#[tokio::main]
async fn main() {
    static CLIENT_ID: &str = "[ID]";
    static CLIENT_SECRET: &str = "[SECRET]";
    static ACCOUNT_ID: &str = "[ACCOUNT ID]";
    let auth = auth::Auth::new(
        auth::ClientSecret::new(CLIENT_SECRET.to_string()),
        auth::ClientId::new(CLIENT_ID.to_string()),
        auth::AccountId::new(ACCOUNT_ID.to_string()),
    )
    .unwrap();
    let flowmailer = Client::new(auth);
    let mailbuilder = MailBuilder::new_template(
        MailAddress::new("sender.mail"),
        MailAddress::new("receiver.mail"),
        "[FLOW SELECTOR]",
    )
    // Set variables within the template.
    .set_data(serde_json::json!({
        "code": "123456"
    }))
    .expect("set data")
    // Set the subject of the mail.
    .set_subject("test_flowmailer_template_mail");
    // Sends the mail.
    mailbuilder.send(&flowmailer).await.expect("");
}
