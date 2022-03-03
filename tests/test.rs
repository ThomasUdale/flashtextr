mod keyword_tests {
    use flashtextr::{FlashText};

    fn generate_processor(strings: Vec<&str>) -> FlashText {
        let mut x = FlashText::new(false);
        for word in strings {
            x.add_keyword(&word);
        }
        x
    }

    #[test]
    fn test_extract_one() {
        let x = generate_processor(vec!["isop","this is"]);
        let result = x.extract_keywords("isop.".to_string());
        assert_eq!(result, vec!["isop".to_string()])
    }

    #[test]
    fn test_extract_two() {
        let x = generate_processor(vec!["isop","this is"]);
        let result = x.extract_keywords("isop, this is.".to_string());
        assert_eq!(result, vec![("isop".to_string()), ("this is".to_string())])
    }

    #[test]
    fn test_extract_mixed() {
        let x = generate_processor(vec!["isop","this is"]);
        let result = x.extract_keywords("this isop".to_string());
        assert_eq!(result, vec!["isop".to_string()])
    }

    #[test]
    fn test_extract_contained() {
        let x = generate_processor(vec!["isop","this is"]);
        let result = x.extract_keywords("test bisop failure".to_string());
        assert_eq!(result, Vec::<String>::new())
    }

    #[test]
    fn test_extract_keyword_contained() {
        let x = generate_processor(vec!["yrdirhxe", "yrd" ]);
        let result = x.extract_keywords("yrd".to_string());
        assert_eq!(result, vec!["yrd".to_string()])
    }

    #[test]
    fn test_add_keyword() {
        let mut x = generate_processor(vec!["test"]);
        assert_eq!(x.has_keyword("word"), false);
        x.add_keyword("word");
        assert_eq!(x.has_keyword("word"), true);
    }

    #[test]
    fn test_initialise_flashtext() {
        let x = generate_processor(vec!["word"]);
        assert_eq!(x.has_keyword("word"), true);
    }
}
