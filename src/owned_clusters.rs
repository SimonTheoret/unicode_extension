use unicode_segmentation::UnicodeSegmentation;

use crate::{clusters::Clusters, ByteOffset, UnicodeExtension};

pub struct OwnedClustersVec {
    clusters: Clusters<String>,
    indices: Vec<ByteOffset>,
    is_extended: bool,
}
impl OwnedClustersVec {
    pub fn new<S>(s: S, is_extended: bool) -> OwnedClustersVec
    where
        S: UnicodeSegmentation,
    {
        let f = str::to_owned;

        let (clusters, indices) = Clusters::new(&s, f, is_extended);
        OwnedClustersVec {
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
impl UnicodeExtension<String> for OwnedClustersVec {
    fn clusters_indices<'a>(&self) -> (&Clusters<String>, &Vec<ByteOffset>) {
        (&self.clusters, &self.indices)
    }
}
