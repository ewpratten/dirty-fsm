# Dirty FSM
[![Crates.io](https://img.shields.io/crates/v/dirty-fsm)](https://crates.io/crates/dirty-fsm) 
[![Docs.rs](https://docs.rs/dirty-fsm/badge.svg)](https://docs.rs/dirty-fsm) 
[![Build](https://github.com/Ewpratten/dirty-fsm/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/dirty-fsm/actions/workflows/build.yml)
[![Clippy](https://github.com/Ewpratten/dirty-fsm/actions/workflows/clippy.yml/badge.svg)](https://github.com/Ewpratten/dirty-fsm/actions/workflows/clippy.yml)
[![Audit](https://github.com/Ewpratten/dirty-fsm/actions/workflows/audit.yml/badge.svg)](https://github.com/Ewpratten/dirty-fsm/actions/workflows/audit.yml)


`dirty-fsm` is a "Quick and Dirty" implementation of a finite state machine. Most of the concepts come from code I wrote as part of [`io.github.frc5024.lib5k.libkontrol`](https://github.com/frc5024/lib5k/tree/d1c53dcbda38824866e4117461315b26ba51905e/lib5k/src/main/java/io/github/frc5024/libkontrol/statemachines).

## Example

In the following example, I model a state machine that represents a claw and a button. When the button is pressed, the claw will toggle between open and closed.

We start by setting a feature flag and loading the library.

```rust ignore
// This feature is required to use the new `#[default]` macro on enum variants
#![feature(derive_default_enum)]

use dirty_fsm::*;
use thiserror::Error;
```

Next, we define the states of the machine.

```rust ignore
/// The possible states of the claw
#[derive(Debug, Default, PartialEq, Eq, Clone, Hash)]
enum ClawState {
    /// The claw is closed
    #[default]
    ClawClosed,

    /// The claw is open
    ClawOpen
}
```

Along with the states, we need to define some kind of error type (although if not needed *at all*, we can just use `()`).

```rust ignore
/// Defines errors that can occur while running actions
#[derive(Debug, Error)]
enum ClawError {
    /// An example error
    #[error("Example error")]
    ExampleError,
}
```

Next, we define the code to actually run during our first state (`ClawClosed`). This is a regular Rust struct, that implements the `Action` trait.

`Action` contains a few simple functions that are called at various points throughout the action's life:

- `on_register`: Called *once* when the action is registered with a state machine (via `StateMachine::add_action`)
- `on_first_run`: Called *once* right before the first `execute` call after this state has been started or switched to. This should be treated like an initializer function. Usually used to save information about the environment before performing an operation in `execute`.
- `execute`: Called multiple times during the action's life. This is where the action's code should go. It should be treated as the body of a `while true` loop, since it will be run over and over until it returns an `ActionFlag` that indicates the action is done.
- `on_finish`: Called *once* right after the last `execute` call once this action is finished.

```rust ignore
/// Action that actually handles the claw being closed
#[derive(Debug)]
struct ClawClosedAction;

impl Action<ClawState, ClawError, bool> for ClawClosedAction {
    fn on_register(&mut self) -> Result<(), ClawError> {
        println!("ClawClosedAction has been registered with the state machine");
        Ok(())
    }

    fn on_first_run(&mut self, context: &bool) -> Result<(), ClawError> {
        println!("Button has been pressed, claw is closing");
        Ok(())
    }

    fn execute(
        &mut self,
        delta: &chrono::Duration,
        context: &bool,
    ) -> Result<crate::action::ActionFlag<ClawState>, ClawError> {
        println!("Claw code is running now");

        // If the button is pressed, switch to the next claw state
        if context {
            Ok(ActionFlag::SwitchState(ClawState::ClawOpen))
        } else {
            Ok(ActionFlag::Continue)
        }
    }

    fn on_finish(&mut self, interrupted: bool) -> Result<(), ClawError> {
        println!("ClawClosedAction is done executing");
        Ok(())
    }
}
```

Since we have two states, this needs to be done again for the other state.

```rust ignore
/// Action that actually handles the claw being opened
#[derive(Debug)]
struct ClawOpenedAction;

impl Action<ClawState, ClawError, bool> for ClawOpenedAction {
    fn on_register(&mut self) -> Result<(), ()> {
        println!("ClawOpenedAction has been registered with the state machine");
        Ok(())
    }

    fn on_first_run(&mut self, context: &bool) -> Result<(), ClawError> {
        println!("Button has been pressed, claw is opening");
        Ok(())
    }

    fn execute(
        &mut self,
        delta: &chrono::Duration,
        context: &bool,
    ) -> Result<crate::action::ActionFlag<ClawState>, ClawError> {
        println!("Claw code is running now");

        // If the button is pressed, throw an error as an example
        if *context {
            Err(ClawError::ExampleError)
        } else {
            Ok(ActionFlag::Continue)
        }
    }

    fn on_finish(&mut self, interrupted: bool) -> Result<(), ClawError> {
        println!("ClawOpenedAction is done executing");
        Ok(())
    }
}
```

Finally, the code to start and run the state machine:

```rust ignore
fn main() {
    // Create the state machine
    let mut claw_machine = StateMachine::new();
    claw_machine.add_action(ClawState::ClawClosed, ClawClosedAction {}).unwrap();
    claw_machine.add_action(ClawState::ClawOpen, ClawOpenedAction {}).unwrap();

    // State. This example assumes some outside "force" is changing this value
    let mut button_pressed = false;

    // Run the state machine
    loop {
        claw_machine.run(&mut button_pressed).unwrap();
    }
}
```
