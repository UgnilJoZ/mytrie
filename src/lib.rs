use std::collections::HashMap;

#[derive(Default, Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn insert(&mut self, mut content: impl Iterator<Item = char>) {
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

    fn get_suffixes<'a>(&'a self) -> Box<dyn Iterator<Item = String> + 'a> {
        if self.children.is_empty() {
            // Empty node → empty iterator
            Box::new(Some(String::new()).into_iter())
        } else {
            // Recursively iterate through the children if they contain something
            Box::new(
                self.children
                    .iter()
                    .flat_map(|(c, n)| n.get_suffixes().map(|s| c.to_string() + &s)),
            )
        }
    }

    fn remove(&mut self, mut content: impl Iterator<Item = char>) {
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
    iter: Box<dyn Iterator<Item = String> + 'a>,
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

/// A prefix tree
///
/// A trie is able to store char sequences and iterate over
/// those with a common prefix.
///
/// It saves common prefixes only once, enabling fast iteration
/// over all suffixes belonging to a prefix.
///
/// Create an empty trie and insert something:
/// ```
/// use mytrie::Trie;
///
/// let mut trie = Trie::default();
/// trie.insert("Hello World!");
/// trie.remove("Hello World!");
/// ```
#[derive(Default, Debug)]
pub struct Trie(TrieNode);

impl Trie {
    /// Initialize a trie from a set of strings.
    ///
    /// Example:
    /// ```
    /// use mytrie::Trie;
    ///
    /// let trie = Trie::from(["Hello", "world"]);
    /// ```
    pub fn from(content: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        let mut trie = Trie::default();
        for s in content.into_iter() {
            trie.insert(s.as_ref())
        }
        trie
    }

    /// Adds a string to the trie
    ///
    /// Example:
    /// ```
    /// use mytrie::Trie;
    ///
    /// let mut trie = Trie::default();
    /// trie.insert("…");
    /// ```
    pub fn insert(&mut self, content: &str) {
        // Insert char sequence with a stop symbol
        self.0.insert(content.chars().chain(['\u{0}']))
    }

    /// Get all suffixes that follow this prefix
    ///
    /// Example:
    /// ```
    /// use mytrie::Trie;
    ///
    /// let trie = Trie::from(["Hallo", "Hallöchen"]);
    /// let mut suffixes: Vec<String> = trie.iter_suffixes("Hall").collect();
    ///
    /// suffixes.sort();
    /// assert_eq!(suffixes, ["o", "öchen"]);
    /// ```
    pub fn iter_suffixes<'a>(&'a self, prefix: &str) -> impl Iterator<Item = String> + 'a {
        let node = self.0.get_node(prefix.chars());
        SuffixIterator {
            _node: node,
            iter: match node {
                Some(node) => node.get_suffixes(),
                None => Box::new(None.into_iter()),
            },
        }
    }

    /// Get all strings in the trie with this prefix
    ///
    /// Example:
    /// ```
    /// use mytrie::Trie;
    ///
    /// let trie = Trie::from(["Hallo", "Hallöchen"]);
    /// let mut suffixes: Vec<String> = trie.content_with_prefix("Hall").collect();
    ///
    /// suffixes.sort();
    /// assert_eq!(suffixes, ["Hallo", "Hallöchen"]);
    /// ```
    pub fn content_with_prefix<'a>(&'a self, prefix: &str) -> impl Iterator<Item = String> + 'a {
        let prefix = String::from(prefix);
        self.iter_suffixes(&prefix)
            .map(move |suffix| prefix.clone() + &suffix)
    }

    /// Remove a string from the trie
    ///
    /// Example:
    /// ```
    /// use mytrie::Trie;
    ///
    /// let mut trie = Trie::from(["Hallo", "Hallöchen"]);
    /// trie.remove("Hallo");
    /// ```
    pub fn remove(&mut self, content: &str) {
        // We have to append the stop symbol again to make deletion successful
        self.0.remove(content.chars().chain(['\u{0}']));
    }

    /// Checks if something with this prefix is in the trie
    /// 
    /// Example:
    /// ```
    /// use mytrie::Trie;
    /// 
    /// let trie = Trie::from(["Hallo", "Hallöchen", "Tschüs", "Hallo Welt"]);
    /// assert!(trie.contains_prefix("Hall"));
    /// assert!(trie.contains_prefix("Hallo"));
    /// assert!(!trie.contains_prefix("ABC"));
    /// assert!(trie.contains_prefix(""));
    /// ```
    pub fn contains_prefix(&self, prefix: &str) -> bool {
        if let Some(_) = self.0.get_node(prefix.chars()) {
            true
        } else {
            false
        }
    }

    /// Returns true if the trie contains no content
    /// 
    /// Example:
    /// ```
    /// use mytrie::Trie;
    /// 
    /// let mut trie = Trie::default();
    /// assert!(trie.is_empty());
    /// trie.insert("");
    /// assert!(!trie.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.children.is_empty()
    }
}

#[cfg(test)]
mod tests;