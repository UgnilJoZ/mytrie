use std::collections::HashMap;

#[derive(Default, Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn insert(&mut self, mut content: impl Iterator<Item=char>) {
        if let Some(ch) = content.next() {
            self.children.entry(ch).or_default().insert(content)
        }
    }

    fn get_node(&self, mut content: impl Iterator<Item = char>) -> Option<&Self> {
        if let Some(ch) = content.next() {
            self.children.get(&ch)?.get_node(content)
        } else {
            Some(&self)
        }
    }

    fn get_suffixes<'a>(&'a self) -> Box<dyn Iterator<Item=String> + 'a> {
        if self.children.is_empty() {
            Box::new(Some(String::new()).into_iter())
        } else {
            Box::new(
                self.children.iter().map(|(c, n)| {
                    n.get_suffixes().map(|s| c.to_string() + &s)
                }).flatten()
            )
        }
    }

    fn remove(&mut self, mut content: impl Iterator<Item=char>) {
        if let Some(ch) = content.next() {
            if let Some(node) = self.children.get_mut(&ch) {
                node.remove(content);
                if node.children.is_empty() {
                    self.children.remove(&ch);
                }
            }
        }
    }
}

struct SuffixIterator<'a> {
    _node: Option<&'a TrieNode>,
    iter: Box<dyn Iterator<Item=String> + 'a>,
}

impl Iterator for SuffixIterator<'_> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut result = self.iter.next()?;
        // Remove the stop symbol
        let _ = result.pop();
        Some(result)
    }
}

#[derive(Default, Debug)]
pub struct Trie(TrieNode);

impl Trie {
    pub fn from<'a>(content: impl IntoIterator<Item=&'a str>) -> Self {
        let mut trie = Trie::default();
        for s in content.into_iter() {
            trie.insert(s)
        }
        trie
    }

    pub fn insert(&mut self, content: &str) {
        // Insert char sequence with a stop symbol
        self.0.insert(content.chars().chain(['\u{0}']))
    }

    pub fn iter_suffixes<'a>(&'a self, prefix: &str) -> impl Iterator<Item=String> + 'a {
        let node = self.0.get_node(prefix.chars());
        SuffixIterator {
            _node: node,
            iter: match node {
                Some(node) => node.get_suffixes(),
                None => Box::new(None.into_iter()),
            }
        }
    }

    pub fn content_with_suffix<'a>(&'a self, prefix: &str) -> impl Iterator<Item=String> + 'a {
        let prefix = String::from(prefix);
        self.iter_suffixes(&prefix).map(move |suffix| prefix.clone() + &suffix)
    }

    pub fn remove(&mut self, content: &str) {
        self.0.remove(content.chars().chain(['\u{0}']));
    }
}


#[cfg(test)]
mod tests {
    use crate::Trie;

    #[test]
    fn insert() {
        let trie = Trie::from(["Hallo", "Hallöchen!", "Tschüs!"]);
        let mut suffixes: Vec<String> = trie.content_with_suffix("").collect();
        suffixes.sort();
        assert_eq!(vec![String::from("Hallo"), String::from("Hallöchen!"), String::from("Tschüs!")], suffixes);
    }

    #[test]
    fn prefix_query() {
        let trie = Trie::from(["Hallo", "Hallöchen!", "Tschüs!"]);
        let mut suffixes: Vec<String> = trie.content_with_suffix("Hall").collect();
        suffixes.sort();
        assert_eq!(vec![String::from("Hallo"), String::from("Hallöchen!")], suffixes);
    }

    #[test]
    fn remove_long() {
        let mut trie = Trie::from(["Hallo", "Hallöchen!", "Tschüs!"]);
        trie.remove("Hallöchen!");
        let mut suffixes: Vec<String> = trie.content_with_suffix("").collect();
        suffixes.sort();
        assert_eq!(vec![String::from("Hallo"), String::from("Tschüs!")], suffixes);
    }
}
