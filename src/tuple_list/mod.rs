#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TupleList<T>(pub Vec<T>, pub Vec<T>);

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
    /// Get the difference of two arrays in the TupleList by the predicate
    /// 通过指定的方法取得TupleList中两个数组的差集
    pub fn diff_by<F>(self, predicate: F) -> Vec<T>
    where
        F: Fn(&T, &T) -> bool,
    {
        let source = self.0;
        let target = self.1;
        source
            .into_iter()
            .filter(|x| return !target.iter().any(|v| predicate(x, v)))
            .collect()
    }
    /// Get the intersection of two arrays in the TupleList
    /// 取得TupleList中两个数组中交集
    pub fn intersect(self) -> Vec<T> {
        let source = self.0;
        let target = self.1;
        source.into_iter().filter(|x| target.contains(x)).collect()
    }
    /// Get the intersection of two arrays in the TupleList by the predicate
    /// 通过指定的方法取得TupleList中两个数组的交集
    pub fn intersect_by<F>(self, predicate: F) -> Vec<T>
    where
        F: Fn(&T, &T) -> bool,
    {
        let source = self.0;
        let target = self.1;
        source
            .into_iter()
            .filter(|x| return target.iter().any(|v| predicate(x, v)))
            .collect()
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
    /// Get the union of two arrays in the TupleList by the predicate
    /// 通过指定的方法取得TupleList中两个数组的合集
    pub fn union_by<F>(self, predicate: F) -> Vec<T>
    where
        F: Fn(&T, &T) -> bool,
    {
        let other = self.clone().diff_by(predicate);
        let target = self.1;
        [other, target].concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Eq, PartialEq, Clone)]
    struct User {
        name: String,
        age: u8,
    }

    #[test]
    fn diff_test() {
        let tuple_list = TupleList(vec![1, 2, 3], vec![2, 3, 4]);
        let result = tuple_list.diff();
        assert_eq!(result, vec![1])
    }

    #[test]
    fn diff_by_test() {
        let tuple_list = get_user_tuple_list();
        let result = tuple_list.diff_by(|x, v| x.name == v.name);
        assert_eq!(
            result,
            vec![User {
                name: "mdbook".to_string(),
                age: 12
            }]
        )
    }

    #[test]
    fn intersect_test() {
        let tuple_list = TupleList(vec![1, 2, 3], vec![2, 3, 4]);
        let result = tuple_list.intersect();
        assert_eq!(result, vec![2, 3])
    }

    fn get_user_tuple_list() -> TupleList<User> {
        TupleList(
            vec![
                User {
                    name: "rust".to_string(),
                    age: 16,
                },
                User {
                    name: "mdbook".to_string(),
                    age: 12,
                },
            ],
            vec![
                User {
                    name: "rust".to_string(),
                    age: 16,
                },
                User {
                    name: "cargo".to_string(),
                    age: 18,
                },
            ],
        )
    }

    #[test]
    fn intersect_by_test() {
        let tuple_list = get_user_tuple_list();
        let result = tuple_list.intersect_by(|x, v| x.name == v.name);
        assert_eq!(
            result,
            vec![User {
                name: "rust".to_string(),
                age: 16
            }]
        )
    }
    #[test]
    fn union_test() {
        let tuple_list = TupleList(vec![1, 2, 3], vec![2, 3, 4]);
        let result = tuple_list.union();
        assert_eq!(result, vec![1, 2, 3, 4])
    }
    #[test]
    fn union_by_test() {
        let tuple_list = get_user_tuple_list();
        let result = tuple_list.union_by(|x, v| x.name == v.name);
        assert_eq!(
            result,
            vec![
                User {
                    name: "mdbook".to_string(),
                    age: 12
                },
                User {
                    name: "rust".to_string(),
                    age: 16
                },
                User {
                    name: "cargo".to_string(),
                    age: 18
                }
            ]
        )
    }
    #[test]
    fn rev_test() {
        let tuple_list = TupleList(vec![1, 2], vec![2, 3]);
        let result = tuple_list.rev();
        assert_eq!(result, TupleList(vec![2, 3], vec![1, 2]))
    }
}
