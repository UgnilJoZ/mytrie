use crate::Trie;

    #[test]
    fn insert() {
        let trie = Trie::from(["Hallo", "Hallöchen!", "Tschüs!"]);
        let mut suffixes: Vec<String> = trie.content_with_prefix("").collect();
        suffixes.sort();
        assert_eq!(
            vec![
                String::from("Hallo"),
                String::from("Hallöchen!"),
                String::from("Tschüs!")
            ],
            suffixes
        );
    }

    #[test]
    fn prefix_query() {
        let trie = Trie::from(["Hallo", "Hallöchen!", "Tschüs!"]);
        let mut suffixes: Vec<String> = trie.content_with_prefix("Hall").collect();
        suffixes.sort();
        assert_eq!(
            vec![String::from("Hallo"), String::from("Hallöchen!")],
            suffixes
        );
    }

    #[test]
    fn remove_long() {
        let mut trie = Trie::from(["Hallo", "Hallöchen!", "Tschüs!"]);
        trie.remove("Hallöchen!");
        let mut suffixes: Vec<String> = trie.content_with_prefix("").collect();
        suffixes.sort();
        assert_eq!(
            vec![String::from("Hallo"), String::from("Tschüs!")],
            suffixes
        );
    }
