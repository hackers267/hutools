#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn left_reverse_test() {
        let relation = Relation::Left.reverse();
        assert_eq!(relation, Relation::Right);
    }
    #[test]
    fn right_reverse_test() {
        let relation = Relation::Right.reverse();
        assert_eq!(relation, Relation::Left);
    }
    #[test]
    fn include_reverse_test() {
        let relation = Relation::Include.reverse();
        assert_eq!(relation, Relation::Included);
    }
    #[test]
    fn included_reverse_test() {
        let relation = Relation::Included.reverse();
        assert_eq!(relation, Relation::Include);
    }
    #[test]
    fn left_intersection_reverse() {
        let relation = Relation::LeftIntersection.reverse();
        assert_eq!(relation, Relation::RightIntersection);
    }
    #[test]
    fn right_intersection_reverse() {
        let relation = Relation::RightIntersection.reverse();
        assert_eq!(relation, Relation::LeftIntersection);
    }
}
/**
区间之间的关系类型：

- Left 另一个区间在此区间的左侧
- LeftIntersection 另一个区间和此区间在左侧有交集
- Include 这个区间包含另一个区间
- RightIntersection 另一个区间和此区间在右侧有交集
- Right 另一个区间在此区间的右侧
- Included 这个区间被另一个区间所包含
*/
#[derive(Eq, PartialEq, Debug)]
pub enum Relation {
    /// Left 另一个区间在此区间的左侧
    Left,
    /// LeftIntersection 另一个区间和此区间在左侧有交集
    LeftIntersection,
    /// Include 这个区间包含另一个区间
    Include,
    /// RightIntersection 另一个区间和此区间在右侧有交集
    RightIntersection,
    /// Right 另一个区间在此区间的右侧
    Right,
    /// Included 这个区间被另一个区间所包含
    Included,
}
impl Relation {
    /**
     * 获取区间关系的取反
     *
     * - `Relation::Include`<-> `Relation::Included` 两个区间互相是包含和被包含的关系
     * - `Relation::Left` <-> `Relation::Right` 两个区间互相是左右关系
     * - `Relation::LeftIntersection` <-> `Relation::RightIntersection` 两个区间互相是左右部分交集的关系
     */
    pub fn reverse(&self) -> Relation {
        match self {
            Relation::Left => Relation::Right,
            Relation::Right => Relation::Left,
            Relation::Included => Relation::Include,
            Relation::Include => Relation::Included,
            Relation::LeftIntersection => Relation::RightIntersection,
            Relation::RightIntersection => Relation::LeftIntersection,
        }
    }
}
pub trait Interval {
    /**
     * 数据两个区间之间的关系：`包含`，`被包含`，`左侧`，`右侧`，`左侧部分交集`和`右侧部分交集`.
     */
    fn relation(&self, another: &Self) -> Relation;
}
