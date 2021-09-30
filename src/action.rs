use std::fmt::Debug;

use chrono::Duration;

/// Defines possible control flags an action can send to control its owning state machine.
/// These flags are to be returned by [`Action::execute`](trait.Action.html#tymethod.execute)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActionFlag<S> {
    /// Continue executing the action that returns this flag
    Continue,
    /// Stop executing he action that returns this flag and execute the default action instead
    Stop,
    /// Switch to a new action (effective on the next call to [`StateMachine::run`](struct.StateMachine.html#method.run))
    SwitchState(S),
}

/// Defines an executable action.
/// This trait is to be implemented by anything that shall be run by a [`StateMachine`](struct.StateMachine.html).
pub trait Action<S, E, Context>: Debug {
    /// Called once when the action is registered with its parent [`StateMachine`](struct.StateMachine.html).
    fn on_register(&mut self) -> Result<(), E>;

    /// Called once each time this action is switched to from another state
    fn on_first_run(&mut self, context: &Context) -> Result<(), E>;

    /// Called on every state machine iteration while this action's state is active
    fn execute(&mut self, delta: &Duration, context: &Context) -> Result<ActionFlag<S>, E>;

    /// Called on the last iteration of this action's state
    fn on_finish(&mut self, interrupted: bool) -> Result<(), E>;
}
