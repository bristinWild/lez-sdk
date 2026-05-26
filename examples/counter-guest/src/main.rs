//! Counter program guest binary.

use nssa_core::program::{read_nssa_inputs, ProgramInput, ProgramOutput};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum CounterInstruction {
    Increment { amount: u64 },
    Reset,
}

fn main() {
    let (
        ProgramInput {
            self_program_id,
            caller_program_id,
            pre_states,
            instruction,
        },
        instruction_words,
    ) = read_nssa_inputs::<CounterInstruction>();

    let pre_states_clone = pre_states.clone();

    // Build router and dispatch
    let router = counter::router();

    let (post_states, chained_calls) = match instruction {
        CounterInstruction::Increment { amount } => {
            let [counter_account] = pre_states
                .try_into()
                .expect("Increment requires exactly one account");
            let args = counter::IncrementArgs { amount };
            let data = borsh::to_vec(&args).expect("serialize args");
            let result = router
                .dispatch(0, vec![counter_account], &data)
                .expect("increment failed");
            (result.post_states, result.chained_calls)
        }
        CounterInstruction::Reset => {
            let [counter_account] = pre_states
                .try_into()
                .expect("Reset requires exactly one account");
            let result = router
                .dispatch(1, vec![counter_account], &[])
                .expect("reset failed");
            (result.post_states, result.chained_calls)
        }
    };

    ProgramOutput::new(
        self_program_id,
        caller_program_id,
        instruction_words,
        pre_states_clone,
        post_states,
    )
    .with_chained_calls(chained_calls)
    .write();
}
