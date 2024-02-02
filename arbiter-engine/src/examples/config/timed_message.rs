#[macro_use]
use serde::{Deserialize, Serialize};
use arbiter_macros::Behaviors;

use super::*;
use crate::{
    agent::Agent,
    examples::timed_message::TimedMessage,
    machine::{Behavior, CreateStateMachine, Engine, MachineInstruction, StateMachine},
    world::World,
};

#[derive(Serialize, Deserialize, Debug, Behaviors)]
#[serde(tag = "type")]
enum Behaviors {
    TimedMessage(TimedMessage),
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn config_test() {
    let mut world = World::new("world");
    world.build_with_config::<Behaviors>("src/examples/config/test_config.toml");

    world.run().await;
}
