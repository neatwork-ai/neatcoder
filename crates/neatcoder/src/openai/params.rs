use anyhow::Result;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::{collections::HashMap, fmt};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{
    openai::utils::{BoundedFloat, Range100s},
    utils::jsvalue_to_hmap,
    JsError,
};

use super::utils::{Bounded, Scale01, Scale100s, Scale22};

#[wasm_bindgen]
#[derive(Debug, Serialize, Clone, Copy)]
pub enum OpenAIModels {
    Gpt432k,
    Gpt4,
    Gpt35Turbo,
    Gpt35Turbo16k,
    Gpt35Turbo1106,
    Gpt41106Preview,
}

impl Default for OpenAIModels {
    fn default() -> Self {
        OpenAIModels::Gpt35Turbo16k
    }
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Clone)]
pub struct OpenAIParams {
    pub model: OpenAIModels,
    // TODO: THIS SHOULD BE Scale02
    /// Temperature is used to control the randomness or creativity
    /// of the model's output. Temperature is a parameter that affects
    /// the distribution of probabilities generated by the model.
    ///
    /// When the temperature is set to its mininmum, the sampling mechanism converges
    /// to greedy decoding, in other words the token stream will be deterministic
    pub temperature: Option<f64>,
    /// Limits the length of the generated output.
    /// If `None` it defaults to `Inf`
    pub max_tokens: Option<u64>,
    /// With top_p (probabilistic sampling), the model considers only the most
    /// likely words whose cumulative probability exceeds a specified threshold.
    /// This threshold is determined by the top_p parameter, which is typically
    /// set between 0 and 1.
    ///
    /// A lower value, such as 0.1 or 0.3, restricts the sampling to a
    /// narrower range of high-probability words. This leads to
    /// more focused and deterministic output, with fewer alternatives
    /// and reduced randomness.
    pub(crate) top_p: Option<Scale01>, // TODO: Add getter
    /// The frequency penalty parameter helps reduce the repetition of words
    /// or sentences within the generated text. It is a float value ranging
    /// from -2.0 to 2.0, which is subtracted to the logarithmic probability of a
    /// token whenever it appears in the output. By increasing the
    /// frequency penalty value, the model becomes more cautious and less likely
    /// to use repeated tokens frequently.
    ///
    /// In the official documentation:
    /// https://platform.openai.com/docs/api-reference/chat/create#chat/create-frequency_penalty
    pub(crate) frequency_penalty: Option<Scale22>, // TODO: Add getter
    /// The presence penalty parameter stears how the model penalizes new tokens
    /// based on whether they have appeared (hence presense) in the text so far.
    ///
    /// The key difference between this param and the frequency param is that the
    /// `presence` penalty is not really concern with the frequency itself.
    ///
    /// In the official documentation:
    /// https://platform.openai.com/docs/api-reference/chat/create#chat/create-presence_penalty
    pub(crate) presence_penalty: Option<Scale22>, // TODO: Add getter
    /// How many chat completion choices to generate for each input message.
    pub n: Option<u64>,
    /// Whether to stream back partial progress. If set, tokens will be sent as
    /// data-only server-sent events as they become available, with the stream
    /// terminated by a data: [DONE] message.
    pub stream: bool,
    /// Modify the likelihood of specified tokens appearing in the completion.
    ///
    /// Accepts a json object that maps tokens (specified by their token ID
    /// in the tokenizer) to an associated bias value from -100 to 100.
    /// Mathematically, the bias is added to the logits generated by the model
    /// prior to sampling. The exact effect will vary per model, but values
    /// between -1 and 1 should decrease or increase likelihood of selection;
    /// values like -100 or 100 should result in a ban or exclusive selection
    /// of the relevant token.
    pub(crate) logit_bias: HashMap<String, Scale100s>, // TODO: Add getter
    /// A unique identifier representing the end-user, which can help OpenAI
    /// to monitor and detect abuse. You can read more at:
    /// https://platform.openai.com/docs/guides/safety-best-practices/end-user-ids
    pub(crate) user: Option<String>,
}

#[wasm_bindgen]
impl OpenAIParams {
    #[wasm_bindgen(constructor)]
    pub fn new(
        model: OpenAIModels,
        temperature: Option<f64>,
        max_tokens: Option<u64>,
        top_p: Option<f64>,
        frequency_penalty: Option<f64>,
        presence_penalty: Option<f64>,
        n: Option<u64>,
        stream: bool,
        logit_bias: JsValue,
        user: Option<String>,
    ) -> Result<OpenAIParams, JsValue> {
        let top_p = match top_p {
            Some(top_p) => Some(
                BoundedFloat::new(top_p)
                    .map_err(|e| JsError::from_str(&e.to_string()))?,
            ),
            None => None,
        };

        let frequency_penalty = match frequency_penalty {
            Some(frequency_penalty) => Some(
                BoundedFloat::new(frequency_penalty)
                    .map_err(|e| JsError::from_str(&e.to_string()))?,
            ),
            None => None,
        };

        let presence_penalty = match presence_penalty {
            Some(presence_penalty) => Some(
                BoundedFloat::new(presence_penalty)
                    .map_err(|e| JsError::from_str(&e.to_string()))?,
            ),
            None => None,
        };

        let logit_bias =
            jsvalue_to_hmap::<String, BoundedFloat<Range100s>>(logit_bias)?;

        Ok(Self {
            model,
            temperature,
            max_tokens,
            top_p,
            frequency_penalty,
            presence_penalty,
            n,
            stream,
            logit_bias,
            user,
        })
    }

    pub fn empty(model: OpenAIModels) -> Self {
        Self {
            model,
            temperature: None,
            max_tokens: None,
            top_p: None,
            frequency_penalty: None,
            presence_penalty: None,
            n: None,
            stream: false,
            logit_bias: HashMap::new(),
            user: None,
        }
    }

    // === Setter methods with chaining ===

    #[wasm_bindgen(js_name = topP)]
    pub fn top_p(mut self, top_p: f64) -> Self {
        self.top_p = Some(Scale01::new(top_p).expect("Invalid top_p value"));
        self
    }

    #[wasm_bindgen(js_name = maxTokens)]
    pub fn max_tokens(mut self, max_tokens: u64) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    #[wasm_bindgen(js_name = frequencyPenalty)]
    pub fn frequency_penalty(mut self, frequency_penalty: f64) -> Self {
        self.frequency_penalty = Some(
            Scale22::new(frequency_penalty).expect("Invalid frequency penalty"),
        );
        self
    }

    #[wasm_bindgen(js_name = presencePenalty)]
    pub fn presence_penalty(mut self, presence_penalty: f64) -> Self {
        self.presence_penalty = Some(
            Scale22::new(presence_penalty).expect("Invalid presence penalty"),
        );
        self
    }

    #[wasm_bindgen(js_name = logicBias)]
    pub fn logit_bias(
        mut self,
        logit_bias: JsValue,
    ) -> Result<OpenAIParams, JsError> {
        let mut logit_bias = jsvalue_to_hmap::<String, f64>(logit_bias)?;

        let logit_bias = logit_bias
            .drain()
            .map(|(key, val)| Ok((key, Scale100s::new(val)?)))
            .collect::<Result<HashMap<String, Scale100s>>>()
            .map_err(|e| JsError::from_str(&e.to_string()))?;

        self.logit_bias = logit_bias;
        Ok(self)
    }

    pub fn user(mut self, user: String) -> Self {
        self.user = Some(user);
        self
    }
}

impl OpenAIModels {
    pub fn new(model: String) -> Self {
        let model = match model.as_str() {
            "gpt-4-32k" => OpenAIModels::Gpt432k,
            "gpt-4" => OpenAIModels::Gpt4,
            "gpt-3.5-turbo" => OpenAIModels::Gpt35Turbo,
            "gpt-3.5-turbo-16k" => OpenAIModels::Gpt35Turbo16k,
            "gpt-3.5-turbo-1106" => OpenAIModels::Gpt35Turbo1106,
            "gpt-4-1106-preview" => OpenAIModels::Gpt41106Preview,
            _ => panic!("Invalid model {}", model),
        };

        model
    }

    pub fn as_string(&self) -> String {
        match self {
            OpenAIModels::Gpt432k => String::from("gpt-4-32k"),
            OpenAIModels::Gpt4 => String::from("gpt-4"),
            OpenAIModels::Gpt35Turbo => String::from("gpt-3.5-turbo"),
            OpenAIModels::Gpt35Turbo16k => String::from("gpt-3.5-turbo-16k"),
            OpenAIModels::Gpt35Turbo1106 => String::from("gpt-3.5-turbo-1106"),
            OpenAIModels::Gpt41106Preview => String::from("gpt-4-1106-preview"),
        }
    }
}

impl<'de> Deserialize<'de> for OpenAIModels {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OpenAIModelsVisitor;

        impl<'de> Visitor<'de> for OpenAIModelsVisitor {
            type Value = OpenAIModels;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing an OpenAI model")
            }

            fn visit_str<E>(self, value: &str) -> Result<OpenAIModels, E>
            where
                E: de::Error,
            {
                match value {
                    "gpt-4-32k" => Ok(OpenAIModels::Gpt432k),
                    "gpt-4" => Ok(OpenAIModels::Gpt4),
                    "gpt-3.5-turbo" => Ok(OpenAIModels::Gpt35Turbo),
                    "gpt-3.5-turbo-16k" => Ok(OpenAIModels::Gpt35Turbo16k),
                    "gpt-3.5-turbo-1106" => Ok(OpenAIModels::Gpt35Turbo1106),
                    "gpt-4-1106-preview" => Ok(OpenAIModels::Gpt41106Preview),
                    _ => Err(E::custom(format!(
                        "unexpected OpenAI model: {}",
                        value
                    ))),
                }
            }
        }

        deserializer.deserialize_str(OpenAIModelsVisitor)
    }
}

impl Default for OpenAIParams {
    fn default() -> Self {
        Self {
            model: OpenAIModels::Gpt35Turbo,
            temperature: None,
            max_tokens: None,
            top_p: None,
            frequency_penalty: None,
            presence_penalty: None,
            n: None,
            stream: false,
            logit_bias: HashMap::new(),
            user: None,
        }
    }
}
