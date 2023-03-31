use crate::resources::{FlowStep, ObjectDescription};
use crate::ty::flow_step;
use crate::{resources, rest_api, Client};

pub struct Template {}

impl Template {
    pub async fn test(client: &Client) {
        let flow = resources::Flow {
            description: "description".to_string(),
            id: None,
            message_summary: None,
            statistics: None,
            steps: Box::new([FlowStep::template(ObjectDescription {
                description: None,
                id: "68489".to_string(),
            })]),
            template_id: "1".to_string(),
        };
        println!(
            "{}",
            crate::request::RequestBuilder::get(client, "flow_templates", &[])
                .expect("failed to create request builder")
                .execute()
                .await
                .expect("failed test 0")
                .into_inner()
                .text()
                .await
                .unwrap()
        );
        rest_api::flows::create(client, flow)
            .await
            .expect("failed test");
    }
}
