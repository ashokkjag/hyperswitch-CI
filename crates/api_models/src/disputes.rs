use masking::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use utoipa::ToSchema;

use super::enums::{DisputeStage, DisputeStatus};

#[derive(Default, Clone, Debug, Serialize, ToSchema)]
pub struct DisputeResponse {
    /// The identifier for dispute
    pub dispute_id: String,
    /// The identifier for payment_intent
    pub payment_id: String,
    /// The identifier for payment_attempt
    pub attempt_id: String,
    /// The dispute amount
    pub amount: String,
    /// The three-letter ISO currency code
    pub currency: String,
    /// Stage of the dispute
    pub dispute_stage: DisputeStage,
    /// Status of the dispute
    pub dispute_status: DisputeStatus,
    /// connector to which dispute is associated with
    pub connector: String,
    /// Status of the dispute sent by connector
    pub connector_status: String,
    /// Dispute id sent by connector
    pub connector_dispute_id: String,
    /// Reason of dispute sent by connector
    pub connector_reason: Option<String>,
    /// Reason code of dispute sent by connector
    pub connector_reason_code: Option<String>,
    /// Evidence deadline of dispute sent by connector
    pub challenge_required_by: Option<String>,
    /// Dispute created time sent by connector
    pub created_at: Option<String>,
    /// Dispute updated time sent by connector
    pub updated_at: Option<String>,
    /// Time at which dispute is received
    pub received_at: String,
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct DisputeListConstraints {
    /// limit on the number of objects to return
    pub limit: Option<i64>,
    /// status of the dispute
    pub dispute_status: Option<DisputeStatus>,
    /// stage of the dispute
    pub dispute_stage: Option<DisputeStage>,
    /// reason for the dispute
    pub reason: Option<String>,
    /// connector linked to dispute
    pub connector: Option<String>,
    /// The time at which dispute is received
    #[schema(example = "2022-09-10T10:11:12Z")]
    pub received_time: Option<PrimitiveDateTime>,
    /// Time less than the dispute received time
    #[schema(example = "2022-09-10T10:11:12Z")]
    #[serde(rename = "received_time.lt")]
    pub received_time_lt: Option<PrimitiveDateTime>,
    /// Time greater than the dispute received time
    #[schema(example = "2022-09-10T10:11:12Z")]
    #[serde(rename = "received_time.gt")]
    pub received_time_gt: Option<PrimitiveDateTime>,
    /// Time less than or equals to the dispute received time
    #[schema(example = "2022-09-10T10:11:12Z")]
    #[serde(rename = "received_time.lte")]
    pub received_time_lte: Option<PrimitiveDateTime>,
    /// Time greater than or equals to the dispute received time
    #[schema(example = "2022-09-10T10:11:12Z")]
    #[serde(rename = "received_time.gte")]
    pub received_time_gte: Option<PrimitiveDateTime>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SubmitEvidenceRequest {
    ///Dispute Id
    pub dispute_id: String,
    /// Logs showing the usage of service by customer
    pub access_activity_log: Option<String>,
    /// Billing address of the customer
    pub billing_address: Option<String>,
    /// File Id of cancellation policy
    pub cancellation_policy: Option<String>,
    /// Details of showing cancellation policy to customer before purchase
    pub cancellation_policy_disclosure: Option<String>,
    /// Details telling why customer's subscription was not cancelled
    pub cancellation_rebuttal: Option<String>,
    /// File Id of customer communication
    pub customer_communication: Option<String>,
    /// Customer email address
    pub customer_email_address: Option<String>,
    /// Customer name
    pub customer_name: Option<String>,
    /// IP address of the customer
    pub customer_purchase_ip: Option<String>,
    /// Fild Id of customer signature
    pub customer_signature: Option<String>,
    /// Product Description
    pub product_description: Option<String>,
    /// File Id of receipt
    pub receipt: Option<String>,
    /// File Id of refund policy
    pub refund_policy: Option<String>,
    /// Details of showing refund policy to customer before purchase
    pub refund_policy_disclosure: Option<String>,
    /// Details why customer is not entitled to refund
    pub refund_refusal_explanation: Option<String>,
    /// Customer service date
    pub service_date: Option<String>,
    /// File Id service documentation
    pub service_documentation: Option<String>,
    /// Shipping address of the customer
    pub shipping_address: Option<String>,
    /// Delivery service that shipped the product
    pub shipping_carrier: Option<String>,
    /// Shipping date
    pub shipping_date: Option<String>,
    /// File Id shipping documentation
    pub shipping_documentation: Option<String>,
    /// Tracking number of shipped product
    pub shipping_tracking_number: Option<String>,
    /// Any additional supporting file
    pub uncategorized_file: Option<String>,
    /// Any additional evidence statements
    pub uncategorized_text: Option<String>,
}
