#[cfg(test)]
mod tests {
    use crate::engine::SearchEngine;

    #[test]
    fn basic_search() {
        let mut engine = SearchEngine::new();
        engine.add_document("1", "hello world");

        let r = engine.search("hello");

        assert_eq!(r[0].doc_id, "1");
    }
}
