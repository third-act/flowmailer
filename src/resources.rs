use crate::ty;
use serde::{Deserialize, Serialize};

pub type Object = serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct Address {
    /// Email address or phone number.
    pub address: ty::String,
    /// Display name, only used for email messages.
    pub name: ty::String,
}

#[derive(Serialize, Deserialize)]
pub struct Attachment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<ty::Base64>,
    /// Content-ID header (required for disposition related).
    /// Example: <part1.DE1D8F7E.E51807FF@flowmailer.com>.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_id: Option<ty::String>,
    /// Examples: `application/pdf`, `image/jpeg`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<ty::String>,
    /// Content-Disposition header for the attachment.
    /// Supported values include: `attachment`, `inline` and `related`.
    /// Special value related should be used for images referenced in the HTML.
    pub disposition: ty::String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<ty::String>,
}

/// Undeliverable message.
#[derive(Serialize, Deserialize)]
pub struct BouncedMessage {
    /// The time flowmailer was done processing this message.
    pub backend_done: ty::Date,
    /// The time flowmailer started processing this message.
    pub backend_start: ty::Date,
    /// Date on which the message was bounced.
    pub bounce_received: ty::Date,
    pub bounce_snippet: ty::String,
    pub bounce_sub_type: ty::String,
    pub bounce_type: ty::String,
    /// Message events.
    /// Ordered by received, new events first.
    pub events: Box<[MessageEvent]>,
    /// Flow this message was processed in.
    pub flow: ObjectDescription,
    /// The email address in From email header.
    pub from: ty::String,
    /// The address in From email header.
    pub from_address: Address,
    /// E-Mail headers of the submitted email message.
    /// Only applicable when `messageType` = `EMAIL` and `addheaders` parameter is `true`.
    pub headers_in: Box<[Header]>,
    /// Headers of the final e-mail.
    /// Only applicable when `messageType` = `EMAIL` and `addheaders` parameter is `true`.
    pub headers_out: Box<[Header]>,
    /// Message id.
    pub id: ty::String,
    /// Link for the message details page. With resend button.
    pub message_details_link: ty::String,
    /// Content of the `Message-ID` email header.
    pub message_id_header: ty::String,
    /// Message type: EMAIL, SMS or LETTER.
    pub message_type: ty::String,
    /// Last online link.
    pub online_link: ty::String,
    /// Recipient address.
    pub recipient_adr: ty::String,
    /// Sender address.
    pub sender_address: ty::String,
    /// Source system that submitted this message.
    pub source: ObjectDescription,
    /// Current message status.
    pub status: ty::String,
    /// Message subject.
    /// Only applicable when `messageType` = `EMAIL`.
    pub subject: ty::String,
    /// The time this message was submitted to flowmailer.
    pub submitted: ty::String,
    /// Message tags, only available for api calls with `addtags` = `true`.
    pub tags: Box<[ty::String]>,
    /// The recipients in the To email header.
    pub to_address_list: Box<[Address]>,
    /// The SMTP transaction id, returned with the SMTP `250` response.
    pub transaction_id: ty::String,
}

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    /// Allowed IP addresses that can connect to Flowmailer using these credentials.
    pub allowed_addresses: Box<[ty::String]>,
    pub allowed_senders: Box<[ty::String]>,
    /// Contact info for these credentials.
    pub contact_info: ty::String,
    /// Description.
    pub description: ty::String,
    /// Source credentials ID.
    pub id: ty::String,
    pub inbound_domain: ty::String,
    pub inbound_recipients: Box<[InboundRecipient]>,
    /// Password.
    pub password: ty::String,
    /// Protocol: `SMTP`, `SMTP_RCPT`, `SMTP_HEADER`, `SMTP_IPONLY` or `SMPP`.
    pub protocol: ty::String,
    /// Source ID.
    pub source_id: ty::String,
    /// Username.
    pub username: ty::String,
}

#[derive(Serialize, Deserialize)]
pub struct DataExpression {
    /// Expression.
    pub expression: ty::String,
    /// Value which must match the result of the expression.
    pub value: ty::String,
}

/// Data set with statistics.
#[derive(Serialize, Deserialize)]
pub struct DataSet {
    /// Data set name.
    pub name: ty::String,
    /// List of samples in this dataset.
    pub samples: Box<[Sample]>,
}

/// Multiple data sets.
#[derive(Serialize, Deserialize)]
pub struct DataSets {
    /// List of data sets.
    pub list: Box<[DataSet]>,
}

#[derive(Serialize, Deserialize)]
pub struct DkimKey {
    pub cname_target: ty::String,
    pub domain: ty::String,
    pub public_key: ty::String,
    pub selector: ty::String,
}

/// DNS record that should be configured.
#[derive(Serialize, Deserialize)]
pub struct DnsRecord {
    /// Error messages for this DNS record.
    /// Only filled when DNS records are validated.
    pub error_messages: Box<[ty::String]>,
    /// Record name.
    pub name: ty::String,
    /// Current record status.
    /// Only filled when DNS records are validated.
    /// Possible values:
    /// `UNKNOWN: We had an error while validating this DNS record`,
    /// `ERROR: DNS record is not correct`,
    /// `WARNING: DNS record is functional but could be improved` or
    /// `OK: DNS record is ok`.
    pub status: ty::String,
    /// Record type.
    pub ty: ty::String,
    /// Record value description in HTML.
    pub value: ty::String,
    /// Warning messages for this DNS record.
    /// Only filled when DNS records are validated.
    pub warning_messages: Box<[ty::String]>,
    pub warnings: ty::Empty,
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub arguments: Box<[Object]>,
    pub code: ty::String,
    pub default_message: ty::String,
    pub field: ty::String,
    pub object_name: ty::String,
    pub rejected_value: Object,
}

#[derive(Serialize, Deserialize)]
pub struct Errors {
    /// List of errors.
    pub all_errors: Box<[Error]>,
}

#[derive(Serialize, Deserialize)]
pub struct EventFlow {
    /// Flow description.
    pub description: ty::String,
    /// Flow ID.
    pub id: ty::String,
    /// Id of the flow parent.
    pub parent_id: ty::String,
    /// Flow steps that each message in this flow will be processed by.
    pub steps: Box<[FlowStep]>,
}

/// Conditions which must be true for an event to use a flow.
#[derive(Serialize, Deserialize)]
pub struct EventFlowRuleSimple {
    pub data_expression: DataExpression,
    pub data_expression_value: ty::String,
    /// Data expressions which must be present in the message.
    pub data_expressions: Box<[DataExpression]>,
    /// Flow ID of the flow the events will use if the specified conditions are true.
    pub event_flow_id: ty::String,
    /// Match type of the event.
    pub event_type: ty::String,
    pub header: Header,
    pub header_value: ty::String,
    /// Email headers which must be present in the message.
    pub headers: Box<[Header]>,
    /// Name of the link that was clicked for CLICK events.
    pub link_name: ty::String,
    /// Url of the link that was clicked for CLICK events.
    pub link_target: ty::String,
    /// Message Flow ID which handled the original message.
    pub message_flow_id: ty::String,
}

#[derive(Serialize, Deserialize)]
pub struct Exception {
    pub message: ty::String,
    pub ty: ty::String,
}

/// Filtered recipient address.
#[derive(Serialize, Deserialize)]
pub struct Filter {
    /// Account ID.
    pub account_id: ty::String,
    /// Filtered recipient address.
    pub address: ty::String,
    /// Date on which this filter was added.
    pub date: ty::Date,
    /// Date on which this filter expires.
    pub expires_on: ty::Date,
    /// Filter ID.
    pub id: ty::String,
    /// Message event that was the reason for creating this filter.
    pub message_return: MessageReturn,
    /// This filter is for message type: `EMAIL` or `SMS`.
    pub message_type: ty::String,
    /// Filter reason.
    pub reason: ty::String,
}

/// Message flow.
#[derive(Serialize, Deserialize)]
pub struct Flow {
    /// Flow description.
    pub description: ty::String,
    /// Flow ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ty::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "messageSummary")]
    pub message_summary: Option<MessageSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<Box<[Sample]>>,
    /// Flow steps that each message in this flow will be processed by.
    pub steps: Box<[FlowStep]>,
    /// Id of the flow template.
    #[serde(rename = "templateId")]
    pub template_id: ty::String,
}

/// Conditions which must be true for a message to use a flow.
#[derive(Serialize, Deserialize)]
pub struct FlowRuleSimple {
    /// Data expressions which must be present in the message.
    pub data_expressions: Box<[DataExpression]>,
    /// Flow ID of the flow the messages will use if the specified conditions are true.
    pub flow_id: ty::String,
    /// Flow selector.
    pub flow_selector: ty::String,
    /// Email headers which must be present in the message.
    pub headers: Box<[Header]>,
    /// Sender email address or phone number.
    pub sender: ty::String,
    /// Source ID which must have submitted the message.
    pub source_id: ty::String,
}

/// A processing step in a flow.
#[derive(Serialize, Deserialize)]
pub struct FlowStep {
    /// Only applicable and required when `type` = `addAttachment`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "addAttachment")]
    pub add_attachment: Option<FlowStepAddAttachment>,
    /// Only applicable when `type` = `aggregate`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<FlowStepAggregate>,
    /// Comma separated list of link domains to modify for analytics parameters.
    /// Only applicable when `type` = `analytics`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "applyToLinkDomains")]
    pub apply_to_link_domains: Option<ty::String>,
    /// Only applicable and required when `type` = `archive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive: Option<FlowStepArchive>,
    /// Only applicable when `type` = `qamail`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub divisor: Option<ty::Int>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "enabledExpression")]
    pub enabled_expression: Option<ty::String>,
    /// Indicates whether the contact is required or not.
    /// Only applicable when `type` = `mailPlusContact`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "errorOnNotFound")]
    pub error_on_not_found: Option<ty::Bool>,
    /// Only applicable and required when `type` = `externalContent`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "externalContent")]
    pub external_content: Option<FlowStepExternalContent>,
    /// Only applicable and required when `type` = `externalData`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "externalData")]
    pub external_data: Option<FlowStepExternalData>,
    /// Only applicable and required when `type` = `extractdata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "extractData")]
    pub extract_data: Option<FlowStepExtractData>,
    /// Flow step ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ty::String>,
    /// Credentials to use for retrieving contacts from MailPlus.
    /// Only applicable when `type` = `mailPlusContact`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mailPlusApiCredentials")]
    pub mail_plus_api_credentials: Option<MailPlusAPICredentials>,
    /// Overwrite existing URL Parameters in links.
    /// Only applicable when `type` = `analytics`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "overwriteUrlParameters")]
    pub overwrite_url_parameters: Option<ty::Bool>,
    /// Only applicable when `type` = `resubmitMessage`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resubmitMessage")]
    pub resubmit_message: Option<FlowStepResubmitMessage>,
    /// Only applicable and required when `type` = `rewriteRecipient`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rewriteRecipient")]
    pub rewrite_recipient: Option<FlowStepRewriteRecipient>,
    /// Only applicable when `type` = `schedule`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<FlowStepSchedule>,
    /// Only applicable and required when `type` = `addHeader`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "setHeader")]
    pub set_header: Option<FlowStepSetHeader>,
    /// Only applicable and required when `type` = `setSender`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "setSender")]
    pub set_sender: Option<FlowStepSetSender>,
    /// Template for the new subject. Template variables can be used in this field.
    /// Only applicable when `type` = `subject`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "subjectTemplate")]
    pub subject_template: Option<ty::String>,
    /// Only applicable when `type` = `template`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<ObjectDescription>,
    // Flow step type.
    #[serde(rename = "type")]
    pub ty: ty::flow_step::Type,
    /// Email address the BCC mail will be sent to.
    /// Only applicable and required when `type` = `qamail`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<ty::String>,
    /// URL Parameters to add to all links. Template variables can be used in this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "urlParametersTemplate")]
    pub url_parameters_template: Option<ty::String>,
}

impl FlowStep {
    pub fn template(template: ObjectDescription) -> Self {
        Self {
            template: Some(template),
            ..Self::fill_none(ty::flow_step::Type::Template)
        }
    }

    fn fill_none(ty: ty::flow_step::Type) -> Self {
        Self {
            ty,
            add_attachment: None,
            aggregate: None,
            apply_to_link_domains: None,
            archive: None,
            divisor: None,
            enabled_expression: None,
            error_on_not_found: None,
            external_content: None,
            external_data: None,
            extract_data: None,
            id: None,
            mail_plus_api_credentials: None,
            overwrite_url_parameters: None,
            resubmit_message: None,
            rewrite_recipient: None,
            schedule: None,
            set_header: None,
            set_sender: None,
            subject_template: None,
            template: None,
            to: None,
            url_parameters_template: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct FlowStepAddAttachment {
    /// URL to load the external content from.
    /// Template variables can be used in this field.
    pub url_template: ty::String,
}

#[derive(Serialize, Deserialize)]
pub struct FlowStepAggregate {
    pub always_send_first: ty::Bool,
    pub max_time_seconds: ty::Int,
    pub quiet_time_seconds: ty::Int,
}

#[derive(Serialize, Deserialize)]
pub struct FlowStepArchive {
    /// Indicates whether this archive is available for online version link.
    pub online_link: ty::Bool,
    /// ISO8601 period notation indicating a shorter retention time (than account settings) for message archives created by this flow step.
    /// The following values are valid: `P1M`, `P3M`, `P6M` or `P1Y`.
    /// Empty means that the account retention time will be applied.
    pub retention: ty::String,
}

#[derive(Serialize, Deserialize)]
pub struct FlowStepExternalContent {
    /// Variable to store the external content in.
    pub result_variable: ty::String,
    /// URL to load the external content from.
    /// Template variables can be used in this field.
    pub url_template: ty::String,
}

#[derive(Serialize, Deserialize)]
pub struct FlowStepExternalData {
    /// When true the result variable will be filled with a structure that also contains the response headers.
    /// When false the result variable will be filled with just the response body.
    pub full_response_in_variable: ty::Bool,
    /// Template text for the request body.
    /// Only useful for the following request methods: `POST`, `PUT` and `PATCH`.
    pub request_body_template: ty::String,
    /// Request headers for external data HTTP request.
    pub request_headers: Box<[Header]>,
    /// HTTP request method.
    /// Valid values: `GET`, `POST`, `PUT`, `PATCH` and `DELETE`.
    pub request_method: ty::flow_step_external_data::RequestMethod,
    /// Format of the external data.
    /// Valid values: `json`.
    pub result_format: ty::String,
    /// Variable to store the external content in.
    pub result_variable: ty::String,
    /// URL to load the external content from.
    /// Template variables can be used in this field.
    pub url_template: ty::String,
}

#[derive(Serialize, Deserialize)]
pub struct FlowStepExtractData {
    pub data_type: ty::String,
    pub filename: ty::String,
    pub html_decode_text: ty::Bool,
    pub mime_type: ty::String,
    pub remove_mime_part: ty::Bool,
    pub selector: ty::String,
}

#[derive(Serialize, Deserialize)]
pub struct FlowStepResubmitMessage {
    /// Account to submit the message to.
    pub account: ObjectDescription,
    pub duplicate_message: ty::Bool,
    /// Selector of the flow to submit the message to.
    pub flow_selector: ty::String,
    /// Name of the header to add to the email.
    pub header_name: ty::String,
    /// Value to set in the header.
    pub header_value: ty::String,
    /// Source to submit the message to.
    pub source: ObjectDescription,
}

#[derive(Serialize, Deserialize)]
pub struct FlowStepRewriteRecipient {
    pub message_type: ty::String,
    pub recipient_name_template: ty::String,
    pub recipient_template: ty::String,
}

#[derive(Serialize, Deserialize)]
pub struct FlowStepSchedule {
    pub max_messages_per_hour: ty::Int,
    pub offset_type: ty::String,
    pub offset_value: ty::Int,
    pub scheduled_time_template: ty::String,
    pub time_range_day0: ty::String,
    pub time_range_day1: ty::String,
    pub time_range_day2: ty::String,
    pub time_range_day3: ty::String,
    pub time_range_day4: ty::String,
    pub time_range_day5: ty::String,
    pub time_range_day6: ty::String,
    pub time_zone: ty::String,
}

#[derive(Serialize, Deserialize)]
pub struct FlowStepSetHeader {
    /// Name of the header to add to the email.
    pub header_name: ty::String,
    /// Value to set in the header.
    pub header_value: ty::String,
}

#[derive(Serialize, Deserialize)]
pub struct FlowStepSetSender {
    pub sender_name_template: ty::String,
    pub sender_set_name: ty::Bool,
    pub sender_template: ty::String,
}

/// Message flow template.
#[derive(Serialize, Deserialize)]
pub struct FlowTemplate {
    /// Flow description.
    pub description: ty::String,
    pub editable: ty::Bool,
    /// Flow template ID.
    pub id: ty::String,
    /// Flow steps that each message in this flow will be processed by.
    pub steps: Box<[FlowStep]>,
    /// Id of the parent flow.
    pub template_id: ty::String,
}

#[derive(Serialize, Deserialize)]
pub struct Header {
    /// Header name.
    pub name: ty::String,
    /// Header value.
    pub value: ty::String,
}

#[derive(Serialize, Deserialize)]
pub struct InboundRecipient {
    pub destination_receipient: ty::String,
    pub inbound_address: ty::String,
}

/// Spotler API credentials.
#[derive(Serialize, Deserialize)]
pub struct MailPlusAPICredentials {
    /// Consumer key.
    pub consumer_key: ty::String,
    /// Consumer secret.
    pub consumer_secret: ty::String,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    /// The time flowmailer was done processing this message.
    pub backend_done: ty::Date,
    /// The time flowmailer started processing this message.
    pub backend_start: ty::Date,
    /// Message events.
    /// Ordered by received, new events first.
    pub events: Box<[MessageEvent]>,
    /// Flow this message was processed in.
    pub flow: ObjectDescription,
    /// The email address in `From` email header.
    pub from: ty::String,
    /// The address in `From` email header
    pub from_address: Address,
    /// E-Mail headers of the submitted email message.
    /// Only applicable when `messageType` = `EMAIL` and `addheaders` parameter is `true`.
    pub headers_in: Box<[Header]>,
    /// Headers of the final e-mail.
    /// Only applicable when `messageType` = `EMAIL` and `addheaders` parameter is `true`.
    pub headers_out: Box<[Header]>,
    /// Message id.
    pub id: ty::String,
    /// Link for the message details page. With resend button.
    pub message_details_link: ty::String,
    /// Content of the `Message-ID` email header.
    pub message_id_header: ty::String,
    pub message_type: ty::MessageType,
    /// Last online link.
    pub online_link: ty::String,
    /// Recipient address.
    pub recipient_address: ty::String,
    /// Sender address.
    pub sender_address: ty::String,
    /// Source system that submitted this message.
    pub source: ObjectDescription,
    /// Current message status.
    pub status: ty::String,
    /// Message subject.
    /// Only applicable when `messageType` = `EMAIL`.
    pub subject: ty::String,
    /// The time this message was submitted to flowmailer.
    pub submitted: ty::Date,
    /// Message tags, only available for api calls with `addtags` = `true`.
    pub tags: Box<[ty::String]>,
    /// The recipients in the `To` email header.
    pub to_address_list: Box<[Address]>,
    /// The SMTP transaction id, returned with the SMTP `250` response.
    pub transaction_id: ty::String,
}

/// Archived message text and/or HTML.
#[derive(Serialize, Deserialize)]
pub struct MessageArchive {
    /// Attachments, without the content.
    /// Only applicable when `messageType` = `EMAIL`.
    pub attachment: Box<[Attachment]>,
    pub data: Object,
    /// The archive flow step that created this archived message.
    pub flow_step_id: ty::String,
    /// Archived message HTML.
    pub html: ty::String,
    /// Link for the message details page with a resend button.
    /// Only applicable when `messageType` = `EMAIL`.
    pub message_details_link: ty::String,
    pub message_type: ty::MessageType,
    /// Online link.
    /// Only applicable when `messageType` = `EMAIL`.
    pub online_link: ty::String,
    /// Indicates whether this archive is available for online version link.
    pub online_version: ty::Bool,
    /// Archived message subject.
    pub subject: ty::String,
    /// Archived message text.
    pub text: ty::String,
}

/// Message event.
#[derive(Serialize, Deserialize)]
pub struct MessageEvent {
    /// Event data.
    pub data: ty::Base64,
    pub device_category: ty::String,
    /// Event data.
    pub extra_data: ty::Empty,
    /// Message event ID.
    pub id: ty::String,
    /// Database insert date.
    pub inserted: ty::Empty,
    pub link_name: ty::String,
    pub link_target: ty::String,
    /// Message ID.
    pub message_id: ty::String,
    /// Message tags.
    /// Only filled for the `GET /{account_id}/message_events` api call when the parameter `addmessagetags` is `true`.
    pub message_tags: Box<[ty::String]>,
    /// MTA that reported this event.
    pub mta: ty::String,
    pub operating_system: ty::String,
    pub operating_system_version: ty::String,
    pub received: ty::Date,
    pub referer: ty::String,
    pub remote_address: ty::String,
    /// Bounce snippet or SMTP conversation snippet.
    pub snippet: ty::String,
    /// Bounce sub type.
    pub sub_type: ty::Empty,
    /// Custom event type.
    pub tag: ty::Empty,
    pub ty: ty::TyCustom,
    pub user_agent: ty::String,
    pub user_agent_display_name: ty::String,
    pub user_agent_string: ty::String,
    pub user_agent_type: ty::String,
    pub user_agent_version: ty::String,
}

/// Messages that could not be processed.
#[derive(Serialize, Deserialize)]
pub struct MessageHold {
    /// The time flowmailer was done processing this message.
    pub backend_done: ty::Date,
    /// MIME message data or text for SMS messages.
    pub data: ty::Base64,
    /// Only for SMS messages.
    pub data_coding: ty::Byte,
    /// Message error text.
    pub error_text: ty::String,
    pub extra_data: Object,
    /// The selected flow.
    pub flow: ObjectDescription,
    /// Message ID.
    pub message_id: ty::String,
    pub message_ty: ty::MessageType,
    /// Message processing failure reason.
    pub reason: ty::String,
    /// Message recipient address.
    pub recipient: ty::String,
    /// Message sender address.
    pub sender: ty::String,
    /// Source system that submitted this message.
    pub source: ObjectDescription,
    /// Message status.
    pub status: ty::String,
    /// Message submit date.
    pub submitted: ty::Date,
    /// Transaction ID.
    pub transaction_id: ty::String,
}

/// Message event.
#[derive(Serialize, Deserialize)]
pub struct MessageReturn {
    /// Event data.
    pub data: ty::Base64,
    pub device_category: ty::String,
    /// Event data.
    pub extra_data: ty::Empty,
    /// Message event ID.
    pub id: ty::String,
    /// Database insert date.
    pub inserted: ty::Empty,
    pub link_name: ty::String,
    pub link_target: ty::String,
    /// Message ID.
    pub message_id: ty::String,
    /// Message tags.
    /// Only filled for the `GET /{account_id}/message_events` api call when the parameter `addmessagetags` is `true`.
    pub message_tags: Box<[ty::String]>,
    /// MTA that reported this event.
    pub mta: ty::String,
    pub operating_system: ty::String,
    pub operating_system_version: ty::String,
    /// Event date.
    pub received: ty::Date,
    pub referer: ty::String,
    pub remote_address: ty::String,
    /// Bounce snippet or SMTP conversation snippet.
    pub snippet: ty::String,
    pub source_mta: ty::Empty,
    /// Bounce sub type.
    pub sub_type: ty::Empty,
    /// Custom event type.
    pub tag: ty::Empty,
    pub ty: ty::TyCustom,
    pub user_agent: ty::String,
    pub user_agent_display_name: ty::String,
    pub user_agent_string: ty::String,
    pub user_agent_type: ty::String,
    pub user_agent_version: ty::String,
}

/// Message statistics summary.
#[derive(Serialize, Deserialize)]
pub struct MessageSummary {
    /// Average delivery time in milliseconds.
    pub avarage_deliver_time_millis: ty::Int,
    /// Number of times a link has been clicked.
    pub clicked: ty::Int,
    /// Number of messages delivered.
    pub delivered: ty::Int,
    /// Number of times a message has been opened.
    pub opened: ty::Int,
    /// Number of messages processed.
    pub processed: ty::Int,
    /// Number of messages sent.
    pub sent: ty::Int,
    /// Number of messages in which a link has been clicked.
    pub unique_clicked: ty::Int,
    /// Number of messages that have been opened.
    pub unique_opened: ty::Int,
}

#[derive(Serialize, Deserialize)]
pub struct NamedSample {
    pub name: ty::String,
    pub other: ty::Bool,
    pub value: ty::Int,
}

/// Generic resource model with an ID and description.
#[derive(Serialize, Deserialize)]
pub struct ObjectDescription {
    /// Resource description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<ty::String>,
    /// Resource ID.
    pub id: ty::String,
}

/// Statistics for a single recipient.
#[derive(Serialize, Deserialize)]
pub struct Recipient {
    /// Recipient email address or phone number.
    pub address: ty::String,
    /// One or more filters for this recipient.
    pub filters: Box<[Filter]>,
    /// Message statistics for this recipient.
    pub message_summary: MessageSummary,
}

#[derive(Serialize, Deserialize)]
pub struct ResendMessage {
    /// Override recipient email address or phone number.
    pub recipient_address: ty::String,
}

#[derive(Serialize, Deserialize)]
pub struct Sample {
    pub timestamp: ty::Date,
    pub value: ty::Int,
}

#[derive(Serialize, Deserialize)]
pub struct SampleMessage {
    pub created: ty::Date,
    pub description: ty::String,
    pub extra_data: Object,
    pub from_address: ty::String,
    pub from_name: ty::String,
    pub id: ty::String,
    pub message_type: ty::String,
    pub mimedata: ty::Base64,
    pub sender: ty::String,
    pub source: ObjectDescription,
}

/// A SenderDomain configures which return-path and online tracking domain is used to send messages.
/// DKIM keys are created for each configured SenderDomain.
///
/// DNS records are filled for the following api calls:
/// 1. `POST /{account_id}/sender_domains/validate`
/// 2. `GET /{account_id}/sender_domains/{domain_id}`
/// 3. `GET /{account_id}/sender_domains/by_domain/{domain}`
///
/// DNS records are validated for the following api calls:
/// 1. `POST /{account_id}/sender_domains/validate`
/// 2. `GET /{account_id}/sender_domains/{domain_id}` when `validate` parameter is `true`
/// 3. `GET /{account_id}/sender_domains/by_domain/{domain}` when `validate` parameter is `true`
#[derive(Serialize, Deserialize)]
pub struct SenderDomain {
    /// List of DNS records that should exist.
    pub dns_records: Box<[DnsRecord]>,
    /// ID of this SenderDomain.
    pub id: ty::String,
    /// Domain used for bounce receiving, usually a subdomain of the `senderDomain`.
    pub return_path_domain: ty::String,
    /// Domain used to select this SenderDomain for emails with a matching `From` header.
    pub sender_domain: ty::String,
    /// Only filled when DNS records are validated.
    pub warnings: Box<[Error]>,
    /// Domain used for online tracking, usually a subdomain of the `senderDomain`.
    pub web_domain: ty::String,
}

/// Sender identities are used to rewrite sender information.
/// Called 'sender rewrites' in Flowmailer dashboard.
#[derive(Serialize, Deserialize)]
pub struct SenderIdentity {
    pub account_fallback: ty::Bool,
    pub description: ty::String,
    pub dkim_keys: Box<[DkimKey]>,
    pub id: ty::String,
    pub return_path_domain: ty::String,
    pub selection_type: ty::String,
    pub selection_value: ty::String,
    pub sender_email: ty::String,
    pub sender_name: ty::String,
    pub web_domain: ty::String,
}

#[derive(Serialize, Deserialize)]
pub struct SimulateMessage {
    /// Attachments. Only applicable when `messageType` = `EMAIL`.
    pub attachments: Box<[Attachment]>,
    /// Extra data that will be available in templates.
    pub data: Object,
    pub delivery_notification_type: ty::DeliveryNotificationType,
    /// Freely configurable value that can be used to select a flow or one of its variants.
    /// Examples: invoice, previsit, ticket.
    pub flow_selector: ty::String,
    /// From header address.
    /// Only applicable when `messageType` = `EMAIL`.
    pub header_from_address: ty::String,
    /// From header name.
    /// Only applicable when `messageType` = `EMAIL`.
    pub header_from_name: ty::String,
    /// To header address.
    /// Only applicable when `messageType` = `EMAIL`.
    pub header_to_address: ty::String,
    /// To header name.
    /// Only applicable when `messageType` = `EMAIL`.
    pub header_to_name: ty::String,
    /// Email headers.
    pub headers: Box<[Header]>,
    /// Email HTML content.
    /// Only applicable when `messageType` = `EMAIL`.
    pub html: ty::String,
    /// Valid values: `EMAIL`, `SMS` and `LETTER`.
    pub message_type: ty::MessageType,
    /// Complete email MIME message with headers.
    /// Only applicable when `messageType` = `EMAIL`.
    pub mimedata: ty::Base64,
    /// Recipient email address or phone number.
    /// For email messages this cannot contain a display name.
    pub recipient_address: ty::String,
    pub schedule_at: ty::Date,
    /// Sender email address or phone number.
    /// For email messages this cannot contain a display name.
    pub sender_address: ty::String,
    pub source_id: ty::String,
    /// Email subject.
    pub subject: ty::String,
    /// Tags.
    pub tags: Box<[ty::String]>,
    /// Text content.
    pub text: ty::String,
}

#[derive(Serialize, Deserialize)]
pub struct SimulateMessageResult {
    /// Attachments, without the content.
    /// Only applicable when `messageType` = `EMAIL`.
    pub attachments: Box<[Attachment]>,
    pub data: Object,
    pub flow: ObjectDescription,
    /// Archived message HTML.
    pub html: String,
    pub message_type: ty::MessageType,
    /// Archived message subject.
    pub subject: ty::String,
    /// Archived message text.
    pub text: ty::String,
}

/// Information about a source system.
/// A source system can submit messages to flowmailer.
#[derive(Serialize, Deserialize)]
pub struct Source {
    /// Source description.
    pub description: ty::String,
    /// Email DSN messages will be sent to this address.
    pub dsn_address: ty::String,
    /// Disable sending DSN messages for this source.
    pub dsn_disable: ty::String,
    /// Email feedback loop messages will be sent to this address.
    pub feedback_loop_address: ty::String,
    /// Human readable notifications for undelivered messages will be sent to this address.
    pub human_readalbe_dsn_address: ty::String,
    /// Enable sending human readable notifications for undelivered messages for this source.
    pub human_readable_dsn_enable: ty::Bool,
    /// Source ID.
    pub id: ty::String,
    /// Date this source was last active.
    pub last_active: ty::Date,
    /// Maximum message size in bytes.
    pub max_message_size: ty::Int,
    /// Message statistics summary for this source.
    pub message_summary: MessageSummary,
    /// Message statistics for this source.
    pub statistics: Box<[Sample]>,
    pub tis_required: ty::Bool,
    pub ty: ty::source::Type,
}

/// An email or sms message that can be submitted to Flowmailer.
#[derive(Serialize, Deserialize)]
pub struct SubmitMessage {
    /// Attachments.
    /// Only applicable when `messageType` = `EMAIL`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Box<[Attachment]>>,
    /// Extra data that will be available in templates.
    pub data: Object,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "deliveryNotificationType")]
    pub delivery_notification_type: Option<ty::DeliveryNotificationType>,
    /// Freely configurable value that can be used to select a flow or one of its variants.
    /// Examples: invoice, previsit, ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "flowSelector")]
    pub flow_selector: Option<ty::String>,
    /// From header address.
    /// Only applicable when `messageType` = `EMAIL`.
    #[serde(rename = "headerFromAddress")]
    pub header_from_address: ty::String,
    /// From header name.
    /// Only applicable when `messageType` = `EMAIL`.
    #[serde(rename = "headerFromName")]
    pub header_from_name: ty::String,
    /// To header address.
    /// Only applicable when `messageType` = `EMAIL`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "headerToAddress")]
    pub header_to_address: Option<ty::String>,
    /// To header name.
    /// Only applicable when `messageType` = `EMAIL`.
    #[serde(rename = "headerToName")]
    pub header_to_name: ty::String,
    /// Email headers.
    pub headers: Box<[Header]>,
    /// Email HTML content.
    /// Only applicable when `messageType` = `EMAIL`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<ty::String>,
    #[serde(rename = "messageType")]
    pub message_type: ty::MessageType,
    /// Complete email MIME message with headers.
    /// Only applicable when `messageType` = `EMAIL`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimedata: Option<ty::Base64>,
    /// Recipient email address or phone number.
    /// For email messages this cannot contain a display name.
    #[serde(rename = "recipientAddress")]
    pub recipient_address: ty::String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scheduleAt")]
    pub schedule_at: Option<ty::Date>,
    /// Sender email address or phone number.
    /// For email messages this cannot contain a display name.
    #[serde(rename = "senderAddress")]
    pub sender_address: ty::String,
    /// Email subject.
    pub subject: ty::String,
    /// Tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Box<[ty::String]>>,
    /// Text content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<ty::String>,
}

/// A flowmailer content template.
#[derive(Serialize, Deserialize)]
pub struct Template {
    /// Content-ID header (required for disposition related).
    /// Example: `<part1.DE1D8F7E.E51807FF@flowmailer.com>`.
    /// Only supported for custom content-types.
    pub content_id: ty::String,
    /// Template content.
    pub data: ty::String,
    /// Decode Base64.
    /// Only supported for custom content-types.
    pub decode_base64: ty::Bool,
    /// Template description.
    pub description: ty::String,
    /// Content-Disposition header for the attachment.
    /// Supported values include: `attachment`, `inline` and `related`.
    /// Special value `related` should be used for images referenced in the HTML.
    /// Only supported for custom content-types.
    pub disposition: ty::template::Disposition,
    /// Content filename.
    /// Only supported for custom content-types and `application/vnd.flowmailer.itext+pdf`.
    pub filename: ty::String,
    /// Template ID.
    pub id: ty::String,
    /// Supported mime types: `text/plain`, `text/html` and `application/vnd.flowmailer.itext+pdf`.
    pub mime_type: ty::template::MimeType,
    /// The only supported template engine is `freemarker-2.3.20`.
    pub template_engine: ty::template::TemplateEngine,
}
