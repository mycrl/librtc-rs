/// A Sink is a value into which other values can be sent.
pub trait SinkExt {
    type Item;

    /// on data for sink push.
    fn on_data(&mut self, item: Self::Item);
}

/// A sink trait type wrapper.
pub struct Sinker<T> {
    pub(crate) sink: Box<dyn SinkExt<Item = T>>,
}

impl<T> Sinker<T> {
    /// create a sink trait wrapper from T.
    pub fn new<S: SinkExt<Item = T> + 'static>(sink: S) -> Self {
        Self {
            sink: Box::new(sink),
        }
    }
}
