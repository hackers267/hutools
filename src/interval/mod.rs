#[derive(Eq, PartialEq, Debug)]
pub enum Relation {
    Left,
    LeftIntersection,
    Include,
    RightIntersection,
    Right,
    Included,
}
pub trait Interval {
    fn relation(&self, another: &Self) -> Relation;
}
