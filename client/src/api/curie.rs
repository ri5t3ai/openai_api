use serde::{Deserialize, Serialize};

use crate::models::CompletionResponse;

pub trait Curie {
    fn generate(&self, prompt: &str, max_tokens: usize) -> Result<CompletionResponse, String>;
}

impl Curie for crate::OpenAI {
    fn generate(&self, prompt: &str, max_tokens: usize) -> Result<CompletionResponse, String> {
        let model = "curie";
        self.generate::<CompletionResponse>(model, prompt, max_tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curie_generate() {
        let openai = crate::OpenAI::new("test_key".to_owned());
        let prompt = "Q: What is the capital of France?\nA:";
        let max_tokens = 5;

        let result = openai.generate::<CompletionResponse>("curie", prompt, max_tokens);
        assert!(result.is_ok());
    }
}