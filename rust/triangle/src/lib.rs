use std::collections::BTreeSet;
use std::ops::Add;

pub struct Triangle<T>([T; 3]);

impl<T: Add<Output = T> + Ord + Clone> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if sides[0].clone() < sides[1].clone() + sides[2].clone()
            && sides[1].clone() < sides[0].clone() + sides[2].clone()
            && sides[2].clone() < sides[0].clone() + sides[1].clone()
        {
            Some(Triangle(sides))
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.0.iter().collect::<BTreeSet<_>>().len() == 1
    }

    pub fn is_scalene(&self) -> bool {
        self.0.iter().collect::<BTreeSet<_>>().len() == 3
    }

    pub fn is_isosceles(&self) -> bool {
        self.0.iter().collect::<BTreeSet<_>>().len() == 2
    }
}
