mod tests {
    #[test]
    #[ignore = "requires the original local audio fixture folder"]
    fn create_index() {
        crate::create_index_json(
            std::path::Path::new("C:\\Users\\arami\\Desktop\\zh"),
            "forvo_zh",
            None,
            1,
        )
        .unwrap();
    }

    #[test]
    #[ignore = "regenerates the bundled entries database from local source files"]
    fn entries() {
        crate::update_entries();
    }
}
