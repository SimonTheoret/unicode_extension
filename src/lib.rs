use std::fmt::Display;
use std::ops::{Deref, DerefMut};

use unicode_segmentation::UnicodeSegmentation;

//TODO: Add documentation here. Describe the data structure, its cost
//and reasons and potential alternatives
#[derive(Debug, Clone, PartialEq)]
pub struct Clusters<'a>(Vec<&'a str>);

// ---------- INHERENT PUBLIC IMPLEMENTATIONS----------
impl<'a> Clusters<'a> {
    //TODO: Add documentation here
    pub fn new<S>(s: &'a S, is_extended: bool) -> Clusters
    where
        S: UnicodeSegmentation + ?Sized,
    {
        let clusters: Vec<_> = s.graphemes(is_extended).collect();

        Clusters(clusters)
    }
    // pub fn bytes(&self) -> &[u8] {
    //     let concated = self.as_slice().concat();
    // }
}

// ---------- INHERENT PRIVATE IMPLEMENTATIONS----------
impl<'a> Clusters<'a> {
    /// This method copies the caller and gives back a String.
    fn copy_to_string(&self) -> String {
        let cloned = self.clone();
        String::from(cloned)
    }
}
// ----------TRAIT IMPLEMENTATIONS----------

impl<'a> From<Clusters<'a>> for String {
    //TODO: Add documentation here
    /// Returns the inner string by aggregating the inner Strings of
    /// the `Clusters`.
    fn from(value: Clusters<'a>) -> String {
        String::from_iter(value)
    }
}
impl<'a> From<Clusters<'a>> for Box<str> {
    //TODO: Add documentation here
    fn from(value: Clusters<'a>) -> Box<str> {
        Box::from_iter(value)
    }
}

//TODO: Add documentation here. Specify implications of deref(mut)
impl<'a> Deref for Clusters<'a> {
    //TODO: Add documentation here
    type Target = Vec<&'a str>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//TODO: Add documentation here. Specify implications of deref(mut)
impl<'a> DerefMut for Clusters<'a> {
    //TODO: Add documentation here
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> IntoIterator for Clusters<'a> {
    //TODO: Add documentation here
    type Item = &'a str;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<'a> Display for Clusters<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.copy_to_string())
    }
}

// ----------TESTING----------

#[cfg(test)]
mod tests {
    use super::*;
    fn short_owned_sentence() -> String {
        String::from("Étiré")
    }

    #[test]
    fn test_new_cluster() {
        let sentences = short_owned_sentence().leak();
        let c = Clusters::new(sentences, true);
        let expected_clusters = Clusters(vec!["É", "t", "i", "r", "é"]);
        assert_eq!(c, expected_clusters);
    }
}
