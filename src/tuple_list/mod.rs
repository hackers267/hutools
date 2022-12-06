#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TupleList<T>(Vec<T>, Vec<T>);

impl<T> TupleList<T>
where
    T: Eq + Clone,
{
    /// Get the difference of two arrays in the TupleList
    /// 取得TupleList中两个数组的差集
    pub fn diff(self) -> Vec<T> {
        let source = self.0;
        let target = self.1;
        source.into_iter().filter(|x| !target.contains(x)).collect()
    }
    /// Get the intersection of two arrays in the TupleList
    /// 取得TupleList中两个数组中交集
    pub fn intersect(self) -> Vec<T> {
        let source = self.0;
        let target = self.1;
        source.into_iter().filter(|x| target.contains(x)).collect()
    }
    /// Get the rev of two arrays in the TupleList
    /// 反转TupleList的两个数组的位置
    pub fn rev(self) -> TupleList<T> {
        let source = self.0;
        let target = self.1;
        TupleList(target, source)
    }
    /// Get the union of two arrays in the TupleList
    /// 取得TupleList中两个数组的合集
    pub fn union(self) -> Vec<T> {
        let other = self.clone().diff();
        let target = self.1;
        [other, target].concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn diff_test() {
        let tuple_list = TupleList(vec![1, 2, 3], vec![2, 3, 4]);
        let result = tuple_list.diff();
        assert_eq!(result, vec![1])
    }
    #[test]
    fn intersect_test() {
        let tuple_list = TupleList(vec![1, 2, 3], vec![2, 3, 4]);
        let result = tuple_list.intersect();
        assert_eq!(result, vec![2, 3])
    }
    #[test]
    fn union_test() {
        let tuple_list = TupleList(vec![1, 2, 3], vec![2, 3, 4]);
        let result = tuple_list.union();
        assert_eq!(result, vec![1, 2, 3, 4])
    }
    #[test]
    fn rev_test() {
        let tuple_list = TupleList(vec![1, 2], vec![2, 3]);
        let result = tuple_list.rev();
        assert_eq!(result, TupleList(vec![2, 3], vec![1, 2]))
    }
}
