fn main() {
    use flashtext::KeywordProcessor;
    let mut keywordprocessor = KeywordProcessor::new(false);
    keywordprocessor.add_keyword("apple");
    println!("{:?}", keywordprocessor.find_keywords("An apple fell from the tree"));
}
