use std::collections::HashMap;

use crate::config::{Client, Response};
use crate::ids::CustomerId;
use crate::resources::{
    CheckoutSession, CheckoutSessionLocale, CheckoutSessionMode, CheckoutSessionSubmitType,
    Currency,
};
use serde_derive::{Deserialize, Serialize};

/// The parameters for `CheckoutSession::create`
///
/// For more details see [https://stripe.com/docs/api/payment_methods/attach](https://stripe.com/docs/api/payment_methods/attach).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSession<'a> {
    /// The URL the customer will be directed to if they decide to cancel payment and return to your website.
    pub cancel_url: &'a str,

    /// A list of the types of payment methods (e.g. card) this Checkout Session is allowed to accept. The only supported values today are `card` and `ideal`.
    pub payment_method_types: Vec<&'a str>,

    /// The URL the customer will be directed to after the payment or subscription creation is successful.
    pub success_url: &'a str,

    /// A unique string to reference the Checkout Session.
    ///
    /// This can be a customer ID, a cart ID, or similar, and can be used to reconcile the session with your internal systems.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_reference_id: Option<&'a str>,

    /// The ID of the customer for this session.
    ///
    /// A new customer will be created unless an existing customer was provided in when the session was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// If provided, this value will be used when the Customer object is created.
    /// If not provided, customers will be asked to enter their email address.
    /// Use this parameter to prefill customer data if you already have an email
    /// on file.
    ///
    /// To access information about the customer once a session is complete, use the `customer` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<&'a str>,

    /// The value (`auto` or `required`) for whether Checkout should collect the customer's billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_collection: Option<&'a str>,

    /// The line items, plans, or SKUs purchased by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<CheckoutSessionLineItem<'a>>>,

    /// The IETF language tag of the locale Checkout is displayed in.
    ///
    /// If blank or `auto`, the browser's locale is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<CheckoutSessionLocale>,

    /// The mode of the Checkout Session, one of `payment`, `setup`, or `subscription`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<CheckoutSessionMode>,

    // A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in payment mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent_data: Option<CheckoutPaymentIntentData<'a>>,

    // A subset of parameters to be passed to SetupIntent creation for Checkout Sessions in setup mode.
    // TODO: setup_intent_data
    /// Describes the type of transaction being performed by Checkout in order
    /// to customize relevant text on the page, such as the submit button.
    /// `submit_type` can only be specified on Checkout Sessions using line
    /// items or a SKU, but not Checkout Sessions for subscriptions.
    ///
    /// Supported values are `auto`, `book`, `donate`, or `pay`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_type: Option<CheckoutSessionSubmitType>,
    // A subset of parameters to be passed to subscription creation for Checkout Sessions in subscription mode.
    // TODO: subscription_data
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CheckoutSessionLineItem<'a> {
    /// The amount to be collected per unit of the line item.
    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The name for the line item.
    pub name: &'a str,

    /// The quantity of the line item being purchased.
    pub quantity: u64,

    /// The description for the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// A list of images representing this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
    // TODO: remaining optional fields
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CheckoutPaymentIntentData<'a> {
    // The amount of the application fee (if any) that will be requested to be applied to the payment 
    // and transferred to the application owner’s Stripe account. The amount of the application fee 
    // collected will be capped at the total payment amount. For more information, see the 
    // PaymentIntents use case for connected accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<u64>,

    // An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    // Set of key-value pairs that you can attach to an object. This can be useful for storing 
    // additional information about the object in a structured format. Individual keys can be 
    // unset by posting an empty value to them. All keys can be unset by posting an empty value to metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,

    // The Stripe account ID for which these funds are intended. For details, see the PaymentIntents use 
    #[serde(skip_serializing_if = "Option::is_none")]
    // case for connected accounts.
    pub on_behalf_of: Option<&'a str>,

    // Email address that the receipt for the resulting payment will be sent to. If receipt_email is 
    // specified for a payment in live mode, a receipt will be sent regardless of your email settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<&'a str>,

    // TODO: setup_future_usage

    // Extra information about the payment. This will appear on your customer’s statement when this payment 
    // succeeds in creating a charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,

    // Provides information about the charge that customers see on their statements. Concatenated with the 
    // prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete 
    // statement descriptor. Maximum 22 characters for the concatenated descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<&'a str>,

    // The parameters used to automatically create a Transfer when the payment succeeds. 
    // For more information, see the PaymentIntents use case for connected accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CheckoutTransferData<'a>>,

    // A string that identifies the resulting payment as part of a group. See the PaymentIntents use case 
    // for connected accounts for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CheckoutTransferData<'a> {
    // If specified, successful charges will be attributed to the destination account for tax reporting, 
    // and the funds from charges will be transferred to the destination account. The ID of the resulting 
    // transfer will be returned on the successful charge’s transfer field.
    pub destination: &'a str,

    // The amount that will be transferred automatically when a charge succeeds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
}

impl CheckoutSession {
    /// Attach a payment method to a customer
    ///
    /// For more details see [https://stripe.com/docs/api/payment_methods/attach](https://stripe.com/docs/api/payment_methods/attach).
    pub fn create(client: &Client, params: CreateCheckoutSession) -> Response<CheckoutSession> {
        client.post_form("/checkout/sessions", params)
    }
}
