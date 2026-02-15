use embedsearch::SearchEngine;

fn main() {
    let mut engine = SearchEngine::new();

    let docs = vec![
        ("1".into(), "rust search engine".into()),
        ("2".into(), "parallel indexing with rayon".into()),
        ("3".into(), "bm25 ranking".into()),
    ];

    engine.add_documents_parallel(docs);

    println!("{:?}", engine.search("rust"));
}
