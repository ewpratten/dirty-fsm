use std::{collections::HashMap, fmt::Debug, hash::Hash, marker::PhantomData};

use crate::Action;
use chrono::{DateTime, Utc};
use log::{debug, trace};

/// Defines a state machine
#[derive(Debug)]
pub struct StateMachine<S, Error, Context>
where
    S: Debug + Default + Hash + Eq + Clone,
{
    default_state: S,
    current_state: S,
    last_state: S,
    last_timestamp: DateTime<Utc>,
    action_map: HashMap<S, Box<dyn Action<S, Error, Context>>>,
    _phantom_error: PhantomData<Error>,
    _phantom_context: PhantomData<Context>,
}

impl<S, Error, Context> StateMachine<S, Error, Context>
where
    S: Debug + Default + Hash + Eq + Clone,
{
    /// Construct a new StateMachine
    pub fn new() -> Self {
        Self {
            default_state: S::default(),
            current_state: S::default(),
            last_state: S::default(),
            last_timestamp: Utc::now(),
            action_map: HashMap::new(),
            _phantom_context: PhantomData::default(),
            _phantom_error: PhantomData::default(),
        }
    }

    /// Add an action to the state machine
    pub fn add_action<A>(&mut self, state: S, mut action: A) -> Result<(), Error>
    where
        A: Action<S, Error, Context> + 'static,
    {
        debug!("Registering action for state: {:?}", state);
        #[cfg(feature = "puffin")]
        puffin::profile_function!();

        // Run the on_register callback
        action.on_register()?;

        // Actually register the action
        self.action_map.insert(state, Box::new(action));
        Ok(())
    }
    /// Remove an action from the state machine
    pub fn remove_action(&mut self, state: S) {
        #[cfg(feature = "puffin")]
        puffin::profile_function!();

        self.action_map.remove(&state);
    }

    /// Run a single iteration of the state machine
    pub fn run(&mut self, context: &Context) -> Result<(), Error> {
        trace!("Executing a single iteration of the state machine");
        #[cfg(feature = "puffin")]
        puffin::profile_function!();

        if self.action_map.contains_key(&self.current_state) {
            // Fetch the current action
            let action = self.action_map.get_mut(&self.current_state).unwrap();

            // Handle executing the first run action
            if self.current_state != self.last_state {
                debug!(
                    "Running on_first_run on state {:?} after a state switch",
                    self.current_state
                );
                #[cfg(feature = "puffin")]
                puffin::profile_scope!("first_run");

                // Execute the first run fn
                action.on_first_run(&context)?;
            }

            // Calculate the time delta
            let time_delta = Utc::now() - self.last_timestamp;

            // Execute the action
            trace!("Executing action for state {:?}", self.current_state);
            {
                #[cfg(feature = "puffin")]
                puffin::profile_scope!("execute");

                let control = action.execute(&time_delta, &context)?;

                // Handle the control flags
                self.last_state = self.current_state.clone();
                match control {
                    crate::action::ActionFlag::Continue => {
                        trace!("Action requested to continue executing");
                    }
                    crate::action::ActionFlag::Stop => {
                        #[cfg(feature = "puffin")]
                        puffin::profile_scope!("on_stop");

                        trace!("Action requested to stop executing");
                        self.current_state = S::default();
                        action.on_finish(false)?;
                    }
                    crate::action::ActionFlag::SwitchState(new_state) => {
                        #[cfg(feature = "puffin")]
                        puffin::profile_scope!("on_switch");

                        trace!("Action requested to switch to state {:?}", new_state);
                        self.current_state = new_state;
                        action.on_finish(false)?;
                    }
                }
            }

            // Update the last timestamp
            self.last_timestamp = Utc::now();
        } else {
            trace!(
                "No action is configured for state {:?}. Doing nothing",
                self.current_state
            );
        }

        Ok(())
    }
}
