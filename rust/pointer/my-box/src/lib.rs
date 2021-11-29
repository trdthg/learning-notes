pub(crate) use crate::libs;

use std::ops::Deref;

pub struct MySmartPointer<T>(T);

impl<T> MySmartPointer<T> {
    pub fn new(t: T) -> Self {
        MySmartPointer(t)
    }
}

impl<T> Deref for MySmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
