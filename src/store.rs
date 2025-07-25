pub trait Store{
    fn store(&self);
}

pub fn store_all(stores: &[&dyn Store]) {
    for store in stores {
        store.store();
    }
}
