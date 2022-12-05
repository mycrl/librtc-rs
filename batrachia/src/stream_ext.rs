pub trait SinkExt {
    type Item;

    fn on_data(&mut self, item: Self::Item);
}

pub struct Sinker<T> {
    pub(crate) sink: Box<dyn SinkExt<Item = T>>,
}

impl<T> Sinker<T> {
    pub fn new<S: SinkExt<Item = T> + 'static>(sink: S) -> Self {
        Self {
            sink: Box::new(sink),
        }
    }
}
