use std::fmt::Debug;

use chrono::Duration;

/// Defines possible control flags an action can send to control its owning state machine.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActionFlag<S> {
    /// Continue executing this action
    Continue,
    /// Stop executing this action and execute the default action
    Stop,
    /// Switch to a new action
    SwitchState(S),
}

/// Defines an executable action.
pub trait Action<S, E, Context>: Debug {
    /// Called once when the action is registered with the state machine.
    fn on_register(&mut self) -> Result<(), E>;

    /// Called once each time this action is switched to from another state
    fn on_first_run(&mut self, context: &mut Context) -> Result<(), E>;

    /// Called on every state machine iteration while this action's state is active
    fn execute(&mut self, delta: &Duration, context: &mut Context) -> Result<ActionFlag<S>, E>;

    /// Called on the last iteration of this action's state
    fn on_finish(&mut self, interrupted: bool) -> Result<(), E>;
}
