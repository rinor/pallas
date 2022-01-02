use std::any::Any;

use pallas_chainsync::{Consumer, NoopObserver, BlockLike, State, Message};
use pallas_machines::{primitives::Point, EncodePayload, DecodePayload, Agent, MachineOutput};

#[derive(Debug)]
pub struct Content();

impl EncodePayload for Content {
    fn encode_payload(&self, e: &mut pallas_machines::PayloadEncoder) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}

impl DecodePayload for Content {
    fn decode_payload(d: &mut pallas_machines::PayloadDecoder) -> Result<Self, Box<dyn std::error::Error>> {
        todo!()
    }
}

impl BlockLike for Content {
    fn block_point(&self) -> Result<Point, Box<dyn std::error::Error>> {
        todo!()
    }
}

struct LastMessage(Box<dyn Any>);

impl MachineOutput for LastMessage {
    fn send_msg(&self, data: &impl EncodePayload) -> Result<(), Box<dyn std::error::Error>> {
        self.0 = Box::new(data);
        
        Ok(())
    }
}

#[test]
fn machine_is_done_when_intersect_not_found() {
    let known_points = vec![Point(0000000u64, hex::decode("abc123").unwrap())];

    let last_message = Box<Mesage>::new();

    let machine = Consumer::<Content, _>::initial(known_points, NoopObserver {});
    assert_eq!(machine.state, State::Idle);

    machine.send_next(tx)
}
