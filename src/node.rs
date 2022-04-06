use std::collections::HashMap;

#[derive(Default, Debug)]
pub(crate) struct TrieNode {
    pub children: HashMap<char, TrieNode>,
}

impl TrieNode {
    pub fn insert(&mut self, mut content: impl Iterator<Item = char>) {
        if let Some(ch) = content.next() {
            self.children.entry(ch).or_default().insert(content)
        }
    }

    pub fn get_node(&self, mut content: impl Iterator<Item = char>) -> Option<&Self> {
        if let Some(ch) = content.next() {
            self.children.get(&ch)?.get_node(content)
        } else {
            Some(self)
        }
    }

    pub fn get_suffixes<'a>(&'a self) -> Box<dyn Iterator<Item = String> + 'a> {
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
