use crate::event::{Event};

/// Type used to represent a function that computes the state
///
/// The state builder should be a "pure" function. This means that it should have zero side effects and not depend on
/// any external resources, with the same input it should **always** produce the same output.
///
/// The state builder is where most of your business logic (not validation though) will reside. If you are going to test
/// anything in your app, this should be your highest priority
///
/// # Examples
///
/// ```
/// use eventific::StateBuilder;
/// # use eventific::EventData;
/// #
/// # struct MyState;
/// #
/// # #[derive(Debug, Clone, strum_macros::EnumIter)]
/// # enum MyEvent {}
/// #
/// # impl EventData for MyEvent {}
///
/// let state_builder: StateBuilder<MyState, MyEvent> = |(_state, _event)| {
///     // DO STUFF
/// };
/// ```
pub type StateBuilder<S, D, M = ()> = fn((&mut S, &Event<D, M>));
