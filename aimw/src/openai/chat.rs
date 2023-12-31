use std::collections::BTreeMap;

use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "role")]
pub enum Message {
    #[serde(rename = "system")]
    System {
        /// The contents of the system message.
        content: String,
        /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    #[serde(rename = "user")]
    User {
        /// The contents of the user message.
        content: String,
        /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    #[serde(rename = "assistant")]
    Assistant {
        /// The contents of the assistant message.
        content: String,
        /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        // TODO: tool_calls
    },
    // TODO: tool message
}

#[derive(Serialize)]
pub struct RequestBody {
    /// A list of messages comprising the conversation so far.
    messages: Vec<Message>,
    /// ID of the model to use. See the [model endpoint compatibility](https://platform.openai.com/docs/models/model-endpoint-compatibility) table for details on which models work with the Chat API.
    model: String,
    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_penalty: Option<f64>,
    /// Modify the likelihood of specified tokens appearing in the completion.
    /// Accepts a [`BTreeMap`] that maps tokens (specified by their token ID in the tokenizer) to an associated bias value from -100 to 100. Mathematically, the bias is added to the logits generated by the model prior to sampling. The exact effect will vary per model, but values between -1 and 1 should decrease or increase likelihood of selection; values like -100 or 100 should result in a ban or exclusive selection of the relevant token.
    #[serde(skip_serializing_if = "Option::is_none")]
    logit_bias: Option<BTreeMap<String, f64>>,
}

mod tests {
    use crate::openai::chat::Message;

    #[test]
    fn serialize_message() {
        let message = Message::System {
            content: "You are a helpful assistant".into(),
            name: None,
        };
        let json = serde_json::to_string(&message).unwrap();
        assert_eq!(
            json,
            r#"{"role":"system","content":"You are a helpful assistant"}"#
        );
    }
}
