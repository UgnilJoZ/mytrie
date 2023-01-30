use crate::Trie;

#[test]
fn insert() {
    let trie = Trie::from(["Hallo", "Hallöchen!", "Tschüs!"]);
    let mut suffixes: Vec<String> = trie.iter_content("").collect();
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
    let mut suffixes: Vec<String> = trie.iter_content("Hall").collect();
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
    let mut suffixes: Vec<String> = trie.iter_content("").collect();
    suffixes.sort();
    assert_eq!(
        vec![String::from("Hallo"), String::from("Tschüs!")],
        suffixes
    );
}

#[test]
#[should_panic]
fn remove_nonexistent() {
    let mut trie = Trie::default();
    trie.remove("")
        .expect("content to remove is not present in trie");
}

#[test]
fn from_iter() {
    let _: Trie = ["aaa", "aab", "aac"].iter().collect();
}
