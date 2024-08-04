mod borrowed_clusters;
mod clusters;
mod owned_clusters;

use clusters::{ByteOffset, Clusters, GraphemeOffset};

pub use self::borrowed_clusters::BorrowedClustersVec;
pub use self::owned_clusters::OwnedClustersVec;

/// Full result of the diff.
type DiffResult<'a, T> = ((usize, (&'a T, &'a usize)), (&'a T, &'a usize));

pub trait UnicodeExtension<T> {
    /// Returns the `Clusters` and the vector of byte offset
    /// associated with the `Clusters`. Both the Clusters and the Vec
    /// should have the same length.
    fn clusters_indices(&self) -> (&Clusters<T>, &Vec<ByteOffset>);
    // TODO: modif this part of the documentation
    /// This function can be used to find the first index at which two
    /// GraphemeClusters start to differ.
    /// Returns the first pair of different elements of `Clusters` of
    /// `self` and the `Clusters` of `other`. Element potentially
    /// different are filtered with closure `find`. This closure
    /// returns a bool, specifying if the element should be considered
    /// or not. This method clones the elements of type `T`.
    fn find_difference<F>(&self, other: &Self, find: F) -> Option<ClustersDiff<T>>
    where
        F: Fn(&((GraphemeOffset, (&T, &usize)), (&T, &usize))) -> bool,
        T: Eq + Clone,
    {
        let closure = find;
        let (self_clusters, self_bindices) = self.clusters_indices();
        let (other_clusters, other_bindices) = other.clusters_indices();
        let diff = self_clusters // iterator format: ((Indice,(GraphemeOffset, cluster)),(GraphemeOffset, cluster))
            .iter()
            .zip(self_bindices)
            .enumerate()
            .zip(other_clusters.iter().zip(other_bindices))
            .find(|((idx, (sc, si)), (oc, oi))| {
                sc != oc && closure(&((*idx, (*sc, *si)), (*oc, *oi)))
            });
        ClustersDiff::try_new(diff)
    }
    /// Returns the first difference, according to find and the equality operator.
    fn find_difference_index<F>(&self, other: &Self, find: F) -> Option<ClustersOffsetDiff>
    where
        F: Fn(&((GraphemeOffset, (&T, &usize)), (&T, &usize))) -> bool,
        T: Eq,
    {
        let closure = find;
        let (self_clusters, self_bindices) = self.clusters_indices();
        let (other_clusters, other_bindices) = other.clusters_indices();
        let diff = self_clusters // iterator format: ((Indice,(GraphemeOffset, cluster)),(GraphemeOffset, cluster))
            .iter()
            .zip(self_bindices)
            .enumerate()
            .zip(other_clusters.iter().zip(other_bindices))
            .find(|((idx, (sc, si)), (oc, oi))| {
                sc != oc && closure(&((*idx, (*sc, *si)), (*oc, *oi)))
            });
        ClustersOffsetDiff::try_new(diff)
    }
}

#[derive(Debug, Clone)]
pub struct ClustersDiff<T>
where
    T: Clone,
{
    /// The grapheme offset off the first different element. Can be used in indexing: cluster[grapheme_offset].
    pub grapheme_offset: usize,
    /// The first different element of the first `Clusters`.
    pub first_element: T,
    /// The byte offset associated with the element contained in `first_ele`.
    pub first_byte_offset: usize,
    /// The first different element of the second `Clusters`.
    pub second_element: T,
    /// The byte offset associated with the element contained in `second_ele`.
    pub second_byte_offset: usize,
}

impl<T> ClustersDiff<T>
where
    T: Clone,
{
    fn try_new(diff_result: Option<DiffResult<T>>) -> Option<ClustersDiff<T>> {
        diff_result.map(
            |((idx, (first_element, first_byte_offset)), (second_element, second_byte_offset))| {
                ClustersDiff {
                    grapheme_offset: idx,
                    first_element: first_element.clone(),
                    first_byte_offset: *first_byte_offset,
                    second_element: second_element.clone(),
                    second_byte_offset: *second_byte_offset,
                }
            },
        )
    }
}

#[derive(Debug)]
pub struct ClustersOffsetDiff {
    pub grapheme_offset: usize,
    pub first_byte_offset: usize,
    pub second_byte_offset: usize,
}

impl ClustersOffsetDiff {
    fn try_new<T>(diff_result: Option<DiffResult<T>>) -> Option<ClustersOffsetDiff> {
        diff_result.map(|((idx, (_, first_byte_offset)), (_, second_byte_offset))| {
            ClustersOffsetDiff {
                grapheme_offset: idx,
                first_byte_offset: *first_byte_offset,
                second_byte_offset: *second_byte_offset,
            }
        })
    }
}
