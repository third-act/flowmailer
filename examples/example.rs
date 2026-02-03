use flowmailer::{auth, Attachment, Client, MailAddress, MailBuilder};

#[tokio::main]
async fn main() {
    static CLIENT_ID: &str = "[ID]";
    static CLIENT_SECRET: &str = "[SECRET]";
    static ACCOUNT_ID: &str = "[ACCOUNT ID]";
    let auth = auth::Auth::new(
        auth::ClientSecret::new(CLIENT_SECRET.to_string()),
        auth::ClientId::new(CLIENT_ID.to_string()),
        auth::AccountId::new(ACCOUNT_ID.to_string()),
    );
    let flowmailer = Client::new(auth);

    // Example 1: Template email with attachment
    let pdf_bytes: Vec<u8> = vec![/* PDF file bytes would go here */];
    let mailbuilder = MailBuilder::new_template(
        MailAddress::new("sender@example.com"),
        MailAddress::new("receiver@example.com"),
        "[FLOW SELECTOR]",
    )
    // Set variables within the template.
    .set_data(serde_json::json!({
        "code": "123456"
    }))
    .expect("set data")
    // Set the subject of the mail.
    .set_subject("test_flowmailer_template_mail")
    // Add an attachment
    .add_attachment(
        Attachment::builder()
            .filename("report.pdf")
            .content_type("application/pdf")
            .content_bytes(&pdf_bytes)
            .build(),
    );
    // Sends the mail.
    mailbuilder.send(&flowmailer).await.expect("failed to send");

    // Example 2: Plain text email with multiple attachments
    let image_bytes: Vec<u8> = vec![/* Image bytes would go here */];
    let text_mail = MailBuilder::new_text(
        MailAddress::new("sender@example.com"),
        MailAddress::new("receiver@example.com"),
        "Hello! Please find the attached files.",
    )
    .set_subject("Files attached")
    .add_attachment(
        Attachment::builder()
            .filename("document.pdf")
            .content_type("application/pdf")
            .content_bytes(&pdf_bytes)
            .build(),
    )
    .add_attachment(
        Attachment::builder()
            .filename("photo.jpg")
            .content_type("image/jpeg")
            .content_bytes(&image_bytes)
            .build(),
    );
    text_mail.send(&flowmailer).await.expect("failed to send");

    // Example 3: HTML email with inline image
    let html_mail = MailBuilder::new_hmtl(
        MailAddress::new("sender@example.com"),
        MailAddress::new("receiver@example.com"),
        r#"<html><body><h1>Hello!</h1><img src="cid:logo123"/></body></html>"#,
    )
    .set_subject("HTML with inline image")
    .add_attachment(
        Attachment::builder()
            .filename("logo.png")
            .content_type("image/png")
            .content_bytes(&image_bytes)
            .content_id("<logo123>")
            .disposition_related() // Use "related" for inline images
            .build(),
    );
    html_mail.send(&flowmailer).await.expect("failed to send");
}
