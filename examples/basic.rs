use embedsearch::SearchEngine;

fn main() {
    let mut engine = SearchEngine::new();

    engine.add_document("1", "Rust is fast and memory safe");
    engine.add_document("2", "Search engines use inverted index");

    let results = engine.search("rust");

    for r in results {
        println!("{:?}", r);
    }
}
