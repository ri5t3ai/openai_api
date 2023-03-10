use serde::{Deserialize, Serialize};

use crate::models::CompletionResponse;

pub trait Babbage {
    fn generate(&self, prompt: &str, max_tokens: usize) -> Result<CompletionResponse, String>;
}

impl Babbage for crate::OpenAI {
    fn generate(&self, prompt: &str, max_tokens: usize) -> Result<CompletionResponse, String> {
        let model = "babbage";
        self.generate::<CompletionResponse>(model, prompt, max_tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_babbage_generate() {
        let openai = crate::OpenAI::new("test_key".to_owned());
        let prompt = "Q: What is the capital of France?\nA:";
        let max_tokens = 5;

        let result = openai.generate::<CompletionResponse>("babbage", prompt, max_tokens);
        assert!(result.is_ok());
    }
}
