/// A Sink is a value into which other values can be sent.
pub trait SinkExt: Send {
    type Item;

    /// on data for sink push.
    fn on_data(&self, item: Self::Item);
}

/// A sink trait type wrapper.
pub struct Sinker<T>
where
    T: Send + Sync + 'static,
{
    pub(crate) sink: Box<dyn SinkExt<Item = T> + Send + Sync + 'static>,
}

impl<T> Sinker<T>
where
    T: Send + Sync + 'static,
{
    /// create a sink trait wrapper from T.
    pub fn new<S: SinkExt<Item = T> + Send + Sync + 'static>(sink: S) -> Self {
        Self {
            sink: Box::new(sink),
        }
    }
}
