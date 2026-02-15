use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct InvertedIndex {
    pub postings: HashMap<String, HashMap<String, u32>>,
    pub doc_lengths: HashMap<String, usize>,
    pub total_docs: usize,
    pub total_tokens: usize,
}

impl InvertedIndex {
    pub fn add_document(&mut self, doc_id: &str, tokens: &[String]) {
        if tokens.is_empty() {
            return;
        }

        self.total_docs += 1;
        self.total_tokens += tokens.len();
        self.doc_lengths.insert(doc_id.to_string(), tokens.len());

        for token in tokens {
            self.postings
                .entry(token.clone())
                .or_default()
                .entry(doc_id.to_string())
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
    }
}
