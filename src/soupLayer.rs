use crate::soup::Ingredient;

// essentially a binary tree based on ingredient size
// queried with a random walk
pub struct SoupLayer {
    ingredient: Ingredient,
    left: Option<Box<SoupLayer>>,
    right: Option<Box<SoupLayer>>
}