use std::ops::{Deref, DerefMut};

use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone, PartialEq)]
pub struct Clusters<'a>(Vec<&'a str>);

impl<'a> Clusters<'a> {
    pub fn new<S>(s: &'a S, is_extended: bool) -> Clusters
    where
        S: UnicodeSegmentation + ?Sized,
    {
        let clusters: Vec<_> = s.graphemes(is_extended).collect();

        Clusters(clusters)
    }
}

impl<'a> From<Clusters<'a>> for String {
    /// Returns the inner string by aggregating the inner Strings of
    /// the `Clusters`.
    fn from(value: Clusters<'a>) -> String {
        String::from_iter(value)
    }
}
impl<'a> From<Clusters<'a>> for Box<str> {
    fn from(value: Clusters<'a>) -> Box<str> {
        Box::from_iter(value)
    }
}

impl<'a> Deref for Clusters<'a> {
    type Target = Vec<&'a str>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Clusters<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> IntoIterator for Clusters<'a> {
    type Item = &'a str;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
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
