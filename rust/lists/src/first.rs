
#[derive(Debug)]
pub enum List<T> {
    Empty,
    Elem(T, Box<List<T>>),
}

