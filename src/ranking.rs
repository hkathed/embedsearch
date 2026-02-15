use crate::index::InvertedIndex;
use std::collections::HashMap;

pub fn bm25(query_tokens: &[String], index: &InvertedIndex) -> Vec<(String, f64)> {
    let mut scores: HashMap<&str, f64> = HashMap::new();

    let k1 = 1.2;
    let b = 0.75;

    let avg_doc_len = index.total_tokens as f64 / index.total_docs.max(1) as f64;

    for key in query_tokens {
        if let Some(postings) = index.postings.get(key) {
            let df = postings.len() as f64;

            let idf = ((index.total_docs as f64 - df + 0.5) / (df + 0.5) + 1.0).ln();

            for (doc, freq) in postings {
                let tf = *freq as f64;

                let doc_len = *index.doc_lengths.get(doc).unwrap() as f64;

                let denom = tf + k1 * (1.0 - b + b * (doc_len / avg_doc_len));

                let score = idf * (tf * (k1 + 1.0)) / denom;

                *scores.entry(doc.as_str()).or_insert(0.0) += score;
            }
        }
    }

    let mut vec: Vec<_> = scores
        .into_iter()
        .map(|(d, s)| (d.to_string(), s))
        .collect();

    vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    vec
}
