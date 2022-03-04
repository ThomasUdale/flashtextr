use std::collections::HashMap;
use pyo3::prelude::*;

#[derive(Debug, PartialEq)]
struct TrieNode {
    word_end_index: Option<usize>,
    children: HashMap<char, TrieNode>
}

#[pyclass]
#[derive(PartialEq)]
pub struct FlashText {
    pub case_sensitive: bool,
    word_dict: HashMap<usize, String>,
    word_index: usize,
    tree: TrieNode,
}

#[pymethods]
impl FlashText {
    #[new]
    /// Generate a new flashtext processor.
    pub fn new(case_sensitive: bool) -> FlashText {
        FlashText{
            case_sensitive,
            word_dict: HashMap::new(),
            word_index: 0,
            tree: TrieNode::new()
        }
    }

    pub fn add_keyword(&mut self,keyword: &str) {
        if !self.case_sensitive {
            self.tree.add_keyword(&keyword.to_lowercase(), self.word_index);
        } else {
            self.tree.add_keyword(&keyword, self.word_index);
        }
        self.word_dict.insert(self.word_index, keyword.to_string());
        self.word_index += 1;
    }

    pub fn has_keyword(&self, keyword: &str) -> bool {
        self.word_dict.values().any(|x| x == keyword)
    }

    pub fn print_tree(&self) {
        self.tree.print_tree(0);
    }

    pub fn extract_keywords(&self, sentence: String ) -> Vec<String> {
        let sentence = if self.case_sensitive {
            sentence
        } else {
            sentence.to_lowercase()
        };
        let mut extract_keywords: Vec<String> = vec![];
        let mut current_keyword_index = None;
        let mut current_level = &self.tree;
        let mut skip_word = false;
        for (i,char) in sentence.chars().enumerate() {
            if !char.is_alphanumeric() {
                skip_word = false;
                match current_level.word_end_index {
                    None => {},
                    Some(x) => {
                        current_keyword_index = Some(x)
                    },
                }
                if let Some(node) = current_level.children.get(&char) {
                    let mut curent_inner_level = node;
                    for inner_char in sentence[(i+1)..].chars() {
                        if !inner_char.is_alphanumeric() {
                            match curent_inner_level.word_end_index {
                                None => {},
                                Some(y) => current_keyword_index = Some(y)
                            }
                        }
                        if let Some(inner_node) = curent_inner_level.children.get(&inner_char) {
                            curent_inner_level = inner_node;
                        } else {
                            curent_inner_level = &self.tree;
                            break;
                        }
                    }
                    match curent_inner_level.word_end_index {
                        None => {},
                        Some(y) => current_keyword_index = Some(y)
                    }
                }
                match current_keyword_index {
                    None => {},
                    Some(x) => extract_keywords.push(self.word_dict.get(&x).unwrap().to_string())
                }
                current_keyword_index = None;
                current_level = &self.tree;
            } else {
                if skip_word {
                    continue
                }
                if let Some(node) = current_level.children.get(&char) {
                    current_level = node;
                } else {
                    current_keyword_index = None;
                    current_level = &self.tree;
                    skip_word = true;
                }
            }
        }
        match current_level.word_end_index {
            None => {},
            Some(x) => {
                current_keyword_index = Some(x)
            },
        }
        match current_keyword_index {
            None => {},
            Some(x) => extract_keywords.push(self.word_dict.get(&x).unwrap().to_string())
        }
        extract_keywords
    }

}

impl TrieNode {
    fn new() -> TrieNode {
        TrieNode {
            word_end_index: None,
            children: HashMap::new()
        }
    }

    fn add_keyword(&mut self, keyword: &str, word_index: usize) {
        if &keyword.chars().count() == &0 {
            return
        }

        let new_element = keyword.chars().nth(0).unwrap();

        if let Some(node) = self.children.get_mut(&new_element) {
            if &keyword.chars().count() == &1 {
                node.word_end_index = Some(word_index);
            }
            node.add_keyword(&keyword[1..], word_index);
        } else {
            let mut new_node = TrieNode::new();
            if &keyword.chars().count() == &1 {
                new_node.word_end_index = Some(word_index);
            } else {
                new_node.add_keyword(&keyword[1..], word_index);
            }
            self.children.insert(new_element, new_node);
        }
    }

    fn print_tree(&self, level: usize) {
        for (key, value) in self.children.iter() {
            println!{"{}{:?}"," ".repeat(level),key};
            value.print_tree(level+1);
        }
    }
}

#[pymodule]
fn flashtextr(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<FlashText>();
    Ok(())
}
