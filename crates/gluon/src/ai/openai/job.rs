use crate::utils::bounded_float::{Bounded, Scale01, Scale100s, Scale22};
use anyhow::{anyhow, Result};
use std::collections::HashMap;

use super::model::OpenAIModels;

pub struct OpenAIJob {
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
    pub top_p: Option<Scale01>,
    /// The frequency penalty parameter helps reduce the repetition of words
    /// or sentences within the generated text. It is a float value ranging
    /// from -2.0 to 2.0, which is subtracted to the logarithmic probability of a
    /// token whenever it appears in the output. By increasing the
    /// frequency penalty value, the model becomes more cautious and less likely
    /// to use repeated tokens frequently.
    ///
    /// In the official documentation:
    /// https://platform.openai.com/docs/api-reference/chat/create#chat/create-frequency_penalty
    pub frequency_penalty: Option<Scale22>,
    /// The presence penalty parameter stears how the model penalizes new tokens
    /// based on whether they have appeared (hence presense) in the text so far.
    ///
    /// The key difference between this param and the frequency param is that the
    /// `presence` penalty is not really concern with the frequency itself.
    ///
    /// In the official documentation:
    /// https://platform.openai.com/docs/api-reference/chat/create#chat/create-presence_penalty
    pub presence_penalty: Option<Scale22>,
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
    pub logit_bias: HashMap<String, Scale100s>,
    /// A unique identifier representing your end-user, which can help OpenAI
    /// to monitor and detect abuse. You can read more at:
    /// https://platform.openai.com/docs/guides/safety-best-practices/end-user-ids
    pub user: Option<String>,
}

impl OpenAIJob {
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

    pub fn new(
        model: OpenAIModels,
        temperature: Option<f64>,
        max_tokens: Option<u64>,
        top_p: Option<Scale01>,
        frequency_penalty: Option<Scale22>,
        presence_penalty: Option<Scale22>,
        n: Option<u64>,
        stream: bool,
        logit_bias: HashMap<String, Scale100s>,
        user: Option<String>,
    ) -> Self {
        Self {
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
        }
    }

    // === Setter methods with chaining ===

    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn max_tokens(mut self, max_tokens: u64) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn top_p(mut self, top_p: f64) -> Result<Self> {
        self.top_p = Some(Scale01::new(top_p)?);
        Ok(self)
    }

    pub fn frequency_penalty(mut self, frequency_penalty: f64) -> Result<Self> {
        self.frequency_penalty = Some(Scale22::new(frequency_penalty)?);
        Ok(self)
    }

    pub fn presence_penalty(mut self, presence_penalty: f64) -> Result<Self> {
        self.presence_penalty = Some(Scale22::new(presence_penalty)?);
        Ok(self)
    }

    pub fn n(mut self, n: u64) -> Self {
        self.n = Some(n);
        self
    }

    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = stream;
        self
    }

    pub fn logit_bias(mut self, mut logit_bias: HashMap<String, f64>) -> Result<Self> {
        let logit_bias = logit_bias
            .drain()
            .map(|(key, val)| Ok((key, Scale100s::new(val)?)))
            .collect::<Result<HashMap<String, Scale100s>>>()?;

        self.logit_bias = logit_bias;
        Ok(self)
    }

    // TODO: Add validation
    pub fn user(mut self, user: String) -> Self {
        self.user = Some(user);
        self
    }
}

impl OpenAIModels {
    pub fn new(model: &str) -> Result<Self> {
        let model = match model {
            "gpt-4-32k" => OpenAIModels::Gpt432k,
            "gpt-4" => OpenAIModels::Gpt4,
            "gpt-3.5-turbo" => OpenAIModels::Gpt35Turbo,
            "gpt-3.5-turbo-16k" => OpenAIModels::Gpt35Turbo16k,
            _ => return Err(anyhow!(format!("Invalid model {}", model))),
        };

        Ok(model)
    }

    pub fn as_str(&self) -> &str {
        match self {
            OpenAIModels::Gpt432k => "gpt-4-32k",
            OpenAIModels::Gpt4 => "gpt-4",
            OpenAIModels::Gpt35Turbo => "gpt-3.5-turbo",
            OpenAIModels::Gpt35Turbo16k => "gpt-3.5-turbo-16k",
        }
    }
}

impl Default for OpenAIJob {
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
