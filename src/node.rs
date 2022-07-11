use std::collections::HashMap;

/// A node in a prefix tree
///
/// Its main task is to hold children and access them by a char key.
///
/// Recursively, it enables also storing sequences of chars. But beware
/// that if two inserted sequences are part of another, only the longer
/// sequence will be yielded by [`iter_suffixes`]. You might want to
/// introduce extra 'stop symbols' to prevent that.
///
/// This is the internal API for the trie.
#[derive(Default, Debug)]
pub(crate) struct TrieNode {
    pub children: HashMap<char, TrieNode>,
}

impl TrieNode {
    /// Insert a char sequence below this node
    pub fn insert(&mut self, mut content: impl Iterator<Item = char>) {
        if let Some(ch) = content.next() {
            self.children.entry(ch).or_default().insert(content)
        }
    }

    /// If `prefix` is contained in this node, return a reference to its last node
    ///
    /// If prefix is empty, self is returned. (recursion anchor)
    pub fn get_node(&self, mut prefix: impl Iterator<Item = char>) -> Option<&Self> {
        if let Some(ch) = prefix.next() {
            self.children.get(&ch)?.get_node(prefix)
        } else {
            Some(self)
        }
    }

    /// Iterate all content below this node
    pub fn iter_content(& self) -> ChildIter {
        ChildIter::new(self)
    }

    /// Remove the specified char sequence from below this node
    pub fn remove(&mut self, mut content: impl Iterator<Item = char>) -> Option<()> {
        if let Some(ch) = content.next() {
            let node = self.children.get_mut(&ch)?;
            let _ = node.remove(content)?;
            if node.children.is_empty() {
                self.children.remove(&ch);
            }
        }
        Some(())
    }
}

pub(crate) struct ChildIter<'a> {
    remaining: Vec<(&'a TrieNode, String)>,
}

impl<'a> ChildIter<'a> {
    fn new(node: &'a TrieNode) -> ChildIter<'a> {
        ChildIter {
            remaining: vec![(node, String::new())],
        }
    }
}

impl<'a> Iterator for ChildIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((node, s)) = self.remaining.pop() {
            for (&ch, child) in &node.children {
                let mut prefix = s.clone();
                prefix.push(ch);
                self.remaining.push((child, prefix));
            }
            Some(s)
        } else {
            None
        }
    }
}
