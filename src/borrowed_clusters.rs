use unicode_segmentation::UnicodeSegmentation;

use crate::{clusters::Clusters, UnicodeExtension};

pub struct BorrowedClustersVec<'a> {
    clusters: Clusters<&'a str>,
    indices: Vec<usize>,
    is_extended: bool,
}
impl<'a> BorrowedClustersVec<'a> {
    pub fn new<S>(s: &'a S, is_extended: bool) -> BorrowedClustersVec<'a>
    where
        S: UnicodeSegmentation,
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
