use crate::TrieNode;

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
/// trie.remove("Hello World!").unwrap();
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
        self.0.insert(content.chars().chain([STOP]))
    }

    /// Iterate all suffixes that follow this prefix
    ///
    /// The order of iteration is arbitrary.
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
            child_iter: node.map(|n| n.iter_content()),
        }
        .filter_map(|content| content.strip_suffix(STOP).map(str::to_string))
    }

    /// Iterate all strings in the trie with this prefix
    ///
    /// The order of iteration is arbitrary.
    ///
    /// Example:
    /// ```
    /// use mytrie::Trie;
    ///
    /// let trie = Trie::from(["Hallo", "Hallöchen", "Tschüs"]);
    /// let mut content: Vec<String> = trie.iter_content("Hall").collect();
    ///
    /// content.sort();
    /// assert_eq!(content, ["Hallo", "Hallöchen"]);
    /// ```
    pub fn iter_content<'a>(&'a self, prefix: &'a str) -> impl Iterator<Item = String> + 'a {
        self.iter_suffixes(prefix)
            .map(move |suffix| format!("{prefix}{suffix}"))
    }

    /// Remove a string from the trie
    ///
    /// On successful removal, `Some(())` is returned.
    /// If `content` was not present, `None` is returned.
    ///
    /// Example:
    /// ```
    /// use mytrie::Trie;
    ///
    /// let mut trie = Trie::from(["Hallo", "Hallöchen"]);
    /// trie.remove("Hallo").unwrap();
    /// ```
    pub fn remove(&mut self, content: &str) -> Option<()> {
        // We have to append the stop symbol again to make deletion successful
        self.0.remove(content.chars().chain([STOP]))
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
        self.0.get_node(prefix.chars()).is_some()
    }

    /// Checks if the specified string was inserted into the trie

    /// Example:
    /// ```
    /// use mytrie::Trie;
    ///
    /// let trie = Trie::from(["Hallo"]);
    /// assert!(!trie.contains("Hall"));
    /// assert!(trie.contains("Hallo"));
    /// ```
    pub fn contains(&self, content: &str) -> bool {
        self.0.get_node(content.chars().chain([STOP])).is_some()
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

    /// Remove everything that follows this suffix
    ///
    /// If a subtree was deleted, returns it. This will be another trie,
    /// containing all those removed strings minus the prefix.
    ///
    /// An example for clarification:
    /// ```
    /// use mytrie::Trie;
    ///
    /// let mut trie = Trie::from(["Hallo", "Hallöchen", "Tschüs"]);
    /// let removed: Trie = trie.remove_suffixes("Hal").unwrap();
    /// let mut removed: Vec<String> = removed.iter_content("").collect();
    /// removed.sort();
    ///
    /// assert_eq!(removed, vec!["lo", "löchen"]);
    /// ```
    pub fn remove_suffixes(&mut self, prefix: &str) -> Option<Self> {
        self.0.remove_suffixes(prefix.chars()).map(Trie)
    }
}

/// Iterate over all strings stored below the specified node
struct SuffixIterator<'a> {
    child_iter: Option<crate::node::ChildIter<'a>>,
}

impl<'a> Iterator for SuffixIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.child_iter.as_mut()?.next()
    }
}

/// The stop symbol on which no character shall follow
/// and which is appended while inserting
const STOP: char = '\u{0}';

impl<S> FromIterator<S> for Trie
where
    S: AsRef<str>,
{
    fn from_iter<T>(string_iter: T) -> Trie
    where
        T: IntoIterator<Item = S>,
    {
        Trie::from(string_iter)
    }
}
