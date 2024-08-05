use std::ops::{Deref, DerefMut};

use unicode_segmentation::UnicodeSegmentation;

/// Byte offset from a grapheme cluster
pub type ByteOffset = usize;
/// GraphemeCluster are generated by the `graphemes()` method from the
/// `UnicodeSegmentation` trait.
pub type GraphemeCluster = str;
/// Grapheme offset from a grapheme cluster. Can be used as indexing
/// position in a cluster.
pub type GraphemeOffset = usize;

#[derive(Debug, Clone, PartialEq)]
pub struct Clusters<T>(Vec<T>);

impl<'a, T> Clusters<T> {
    /// Build a new cluster from `s`, which implements
    /// `UnicodeSegmentation`. Function `f`, which takes a reference
    /// to a GraphemeCluster, returns the generic type `T`. This
    /// function also returns the `Vec` of `ByteOffset` associated
    /// with the clusters.
    pub(crate) fn new<S, F>(s: &'a S, f: F, is_extended: bool) -> (Clusters<T>, Vec<ByteOffset>)
    where
        S: UnicodeSegmentation + ?Sized,
        F: FnMut(&'a GraphemeCluster) -> T,
        T: 'a,
    {
        let mut offsets: Vec<ByteOffset> = vec![];
        let clusters: Vec<T> = s
            .grapheme_indices(is_extended)
            .map(|(bo, gc)| {
                offsets.push(bo);
                gc
            })
            .map(f)
            .collect();

        (Clusters(clusters), offsets)
    }
    /// Function used only for testing purposes.
    #[allow(dead_code)]
    pub(crate) fn new_from_raw(content: Vec<T>) -> Clusters<T> {
        Clusters(content)
    }
}

impl From<Clusters<String>> for String {
    /// Returns the inner string by aggregating the inner Strings of
    /// the `Clusters`.
    fn from(value: Clusters<String>) -> String {
        value.into_iter().fold(String::default(), |mut acc, el| {
            acc.push_str(el.as_str());
            acc
        })
    }
}
impl From<Clusters<&str>> for String {
    fn from(value: Clusters<&str>) -> String {
        value.into_iter().fold(String::default(), |mut acc, el| {
            acc.push_str(el);
            acc
        })
    }
}

impl<T> Deref for Clusters<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Clusters<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> IntoIterator for Clusters<T> {
    type Item = T;
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
    fn test_new_cluster<'a>() {
        let sentences = short_owned_sentence().leak();
        let id = |x: &'a GraphemeCluster| -> &'a GraphemeCluster { x };
        let (c, i) = Clusters::new(sentences, id, true);
        let expected_clusters = Clusters(vec!["É", "t", "i", "r", "é"]);
        let expected_indices = vec![0, 2, 3, 4, 5];
        assert_eq!(c, expected_clusters);
        assert_eq!(i, expected_indices);
    }
}
