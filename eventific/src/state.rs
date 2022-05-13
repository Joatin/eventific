use crate::event::Event;

pub trait State<P>: Default {
    fn apply(&mut self, event: Event<P>);
}
