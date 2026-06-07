
#[derive(Debug, Clone, PartialEq)]
pub enum BorrowState {
    Active,
    Moved,
}

#[derive(Debug, Clone)]
pub struct Lifetime {
    pub name: String,
    pub is_mut: bool,
    pub state: BorrowState,
}

impl Lifetime {
    pub fn new(name: String, is_mut: bool) -> Self {
        Self {
            name,
            is_mut,
            state: BorrowState::Active,
        }
    }
}
