pub struct Page<T> {
    pub total: u64,
    pub has_prev: bool,
    pub has_next: bool,
    pub items: Vec<T>,
}

impl<T> Page<T> {
    pub fn new(total: u64, has_prev: bool, has_next: bool, items: Vec<T>) -> Self {
        Self {
            total,
            has_prev,
            has_next,
            items,
        }
    }
}
