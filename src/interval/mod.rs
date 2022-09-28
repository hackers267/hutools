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
pub trait Interval {
    fn relation(&self, another: &Self) -> Relation;
}
