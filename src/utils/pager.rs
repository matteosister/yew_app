pub struct Paginator {
    per_page: u8,
}

impl Paginator {
    pub fn new(per_page: u8) -> Self {
        Self { per_page }
    }

    pub fn get_page<'a, T>(&self, collection: &'a Vec<T>, num: usize) -> &'a [T] {
        let offset = (self.per_page as usize * num - self.per_page as usize) as usize;
        if collection.len() > offset * 10 {
            &collection[offset..=offset + 10]
        } else {
            &[]
        }
    }
}
