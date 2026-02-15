[![crates.io](https://img.shields.io/crates/v/embedsearch.svg)](https://crates.io/crates/embedsearch)
[![docs.rs](https://docs.rs/embedsearch/badge.svg)](https://docs.rs/embedsearch)


# embedsearch
 Tiny, fast, embeddable full-text search engine written in pure Rust.


Embedsearch

    Tiny, fast, embeddable full-text search engine written in pure Rust. embedsearch lets you add Google-style search to any Rust app in minutes. No servers. No Elasticsearch. No external services. Just a small in-process library.

Perfect for:

    1. Docs search
    2. Desktop apps
    3. Internal tools
    4. Small websites
    5. CLIs
    6. Local knowledge bases

Features

    1. Inverted index
    2. BM25 ranking (same family used by major search engines)
    3. Parallel indexing (via Rayon)
    4. Snippet generation + highlighting
    5. Persistence: Save/Load index to disk
    6. Pure Rust: No runtime services or external dependencies
    7. Zero setup: Just add the crate and go

Install
    bash

    cargo add embedsearch


Quick Start

 
    use embedsearch::SearchEngine;

    fn main() {
        let mut engine = SearchEngine::new();

        // Add documents
        engine.add_document("1", "Rust is fast and memory safe");
        engine.add_document("2", "Search engines use inverted indexes");

        // Search
        let results = engine.search("rust");

        for r in results {
            println!("{:?}", r);
        }
    }


Output

    SearchResult { 
        doc_id: "1", 
        score: 1.38, 
        snippet: "**Rust** is fast and memory safe" 
    }



Advanced Usage
    1. Parallel Indexing.
    2. Fast for large datasets. Uses all CPU cores automatically.

    let docs = vec![
        ("1".to_string(), "Rust is fast".to_string()),
        ("2".to_string(), "BM25 ranking is great".to_string()),
    ];
    engine.add_documents_parallel(docs);


Save / Load Index : 
    Great for prebuilding search or shipping indexes with apps for instant startup.
    

    engine.save("index.bin")?;
    let engine = SearchEngine::load("index.bin")?;


Search API Variations


    // Top-K search
    let results = engine.search_top("rust", 3);

    // Pretty CLI output
    engine.search_pretty("rust", 3);

    // JSON results (API friendly)
    println!("{}", engine.search_json("rust", 3));


Examples
Run these locally to see embedsearch in action:

    cargo run --example basic — Basic search functionality.
    cargo run --example save_load — Persistence and disk I/O.
    cargo run --example parallel — Multi-threaded indexing performance.
