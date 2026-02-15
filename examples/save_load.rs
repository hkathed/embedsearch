use embedsearch::SearchEngine;

fn main() -> anyhow::Result<()> {
    let mut engine = SearchEngine::new();

    engine.add_document("1", "hello world");
    engine.save("idx.bin")?;

    let loaded = SearchEngine::load("idx.bin")?;

    println!("{:?}", loaded.search("hello"));

    Ok(())
}
