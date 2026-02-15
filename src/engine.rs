use crate::index::InvertedIndex;
use crate::ranking::bm25;
use crate::tokenizer::tokenize;
use crate::types::SearchResult;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
pub struct SearchEngine {
    index: InvertedIndex,
    documents: HashMap<String, String>,
}

impl SearchEngine {
    pub fn new() -> Self {
        Self {
            index: InvertedIndex::default(),
            documents: HashMap::new(),
        }
    }

    pub fn add_document(&mut self, doc_id: &str, text: &str) {
        let tokens = tokenize(text);

        self.documents.insert(doc_id.to_string(), text.to_string()); // NEW
        self.index.add_document(doc_id, &tokens);
    }

    pub fn add_documents_parallel(&mut self, docs: Vec<(String, String)>) {
        // tokenize in parallel first
        let tokenized: Vec<(String, Vec<String>)> = docs
            .into_par_iter()
            .map(|(id, text)| {
                let tokens = tokenize(&text);
                (id, tokens)
            })
            .collect();

        // then add sequentially (safe, simple)
        for (id, tokens) in tokenized {
            self.index.add_document(&id, &tokens);
        }
    }

    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        let tokens = tokenize(query);

        let raw = bm25(&tokens, &self.index);

        raw.into_iter()
            .map(|(doc, score)| {
                let text = self.documents.get(&doc).unwrap();

                SearchResult {
                    doc_id: doc,
                    score,
                    snippet: Self::build_snippet(text, &tokens),
                }
            })
            .collect()
    }

    pub fn doc_count(&self) -> usize {
        self.index.total_docs
    }

    fn build_snippet(text: &str, query_tokens: &[String]) -> String {
        let lower = text.to_lowercase();

        for token in query_tokens {
            if let Some(pos) = lower.find(token) {
                let start = pos.saturating_sub(60);
                let end = (pos + 60).min(text.len());

                let snippet = &text[start..end];

                return snippet.replace(token, &format!("**{}**", token));
            }
        }

        text.chars().take(120).collect()
    }

    pub fn search_top(&self, query: &str, k: usize) -> Vec<SearchResult> {
        let mut results = self.search(query);
        results.truncate(k);
        results
    }

    pub fn search_pretty(&self, query: &str, k: usize) {
        for (i, r) in self.search_top(query, k).iter().enumerate() {
            println!("{}. {}  ({:.3})", i + 1, r.doc_id, r.score);
        }
    }

    pub fn search_json(&self, query: &str, k: usize) -> String {
        serde_json::to_string_pretty(&self.search_top(query, k)).unwrap()
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        bincode::serialize_into(writer, &(&self.index, &self.documents))
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }

    pub fn load<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let (index, documents): (InvertedIndex, HashMap<String, String>) =
            bincode::deserialize_from(reader)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        Ok(Self { index, documents })
    }
}
