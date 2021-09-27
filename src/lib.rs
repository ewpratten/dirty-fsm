//! `dirty-fsm` is a "Quick and Dirty" implementation of a finite state machine.
//!
//! I mostly stole all this code from myself in older projects.

#![feature(derive_default_enum)]

pub mod action;
pub use action::{Action, ActionFlag};
pub mod statemachine;
pub use statemachine::StateMachine;

#[cfg(test)]
mod test;