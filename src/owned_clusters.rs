use unicode_segmentation::UnicodeSegmentation;

use crate::{clusters::Clusters, ByteOffset, UnicodeExtension};

#[derive(Debug, PartialEq, Clone)]
pub struct OwnedClustersVec {
    clusters: Clusters<String>,
    indices: Vec<ByteOffset>,
    is_extended: bool,
}
impl OwnedClustersVec {
    pub fn new<S>(s: &S, is_extended: bool) -> OwnedClustersVec
    where
        S: UnicodeSegmentation + ?Sized,
    {
        let f = str::to_owned;

        let (clusters, indices) = Clusters::new(s, f, is_extended);
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

#[cfg(test)]
mod tests {
    use super::*;
    fn short_owned_sentence() -> String {
        String::from("Étiré")
    }

    fn owned_sentence() -> String {
        String::from("This is a test sentence")
    }
    #[test]
    fn test_new_cluster() {
        let sentences = short_owned_sentence().leak();
        let is_extended = true;
        let c = OwnedClustersVec::new(sentences, is_extended);
        let expected_clusters: Clusters<String> = Clusters::new_test(
            vec!["É", "t", "i", "r", "é"]
                .into_iter()
                .map(str::to_owned)
                .collect(),
        );
        let expected_indices = vec![0, 2, 3, 4, 5];
        let expected_owned_clusters_vec = OwnedClustersVec {
            clusters: expected_clusters,
            indices: expected_indices,
            is_extended,
        };
        assert_eq!(c, expected_owned_clusters_vec);
    }
}
