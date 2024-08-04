use unicode_segmentation::UnicodeSegmentation;

use crate::{clusters::Clusters, UnicodeExtension};

#[derive(Debug, PartialEq, Clone)]
pub struct BorrowedClustersVec<'a> {
    clusters: Clusters<&'a str>,
    indices: Vec<usize>,
    is_extended: bool,
}
impl<'a> BorrowedClustersVec<'a> {
    pub fn new<S>(s: &'a S, is_extended: bool) -> BorrowedClustersVec<'a>
    where
        S: UnicodeSegmentation + ?Sized,
    {
        let f = |x: &'a str| -> &'a str { x }; // Identity function

        let (clusters, indices) = Clusters::new(s, f, is_extended);
        BorrowedClustersVec {
            clusters,
            indices,
            is_extended,
        }
    }
    pub fn extended(&self) -> bool {
        self.is_extended
    }
    pub fn indices(&self) -> &Vec<usize> {
        &self.indices
    }
}
impl<'a> UnicodeExtension<&'a str> for BorrowedClustersVec<'a> {
    fn clusters_indices(&self) -> (&Clusters<&'a str>, &Vec<crate::clusters::ByteOffset>) {
        (&self.clusters, &self.indices)
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
        let is_extended = true;
        let c = BorrowedClustersVec::new(sentences, is_extended);
        let expected_clusters = Clusters::new_from_raw(vec!["É", "t", "i", "r", "é"]);
        let expected_indices = vec![0, 2, 3, 4, 5];
        let expected_borrow_clusters_vec = BorrowedClustersVec {
            clusters: expected_clusters,
            indices: expected_indices,
            is_extended,
        };
        assert_eq!(c, expected_borrow_clusters_vec);
    }
}
