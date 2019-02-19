use crate::lib::common::fsm::state::State;
use super::miner_wife::MinerWife;
use super::map::Locations;
use crate::lib::common::fsm::state_transition::StateTransition;
use rand::Rng;

pub struct WifesGlobalState;
pub struct DoHouseWork;
pub struct VisitBathroom;


pub enum MinerWifeStates{
    None,
}

impl State for WifesGlobalState {
    type Entity = MinerWife;
    type Enum = MinerWifeStates;

    fn new() -> Box<Self> {
        Box::new(WifesGlobalState)
    }

    fn enter(&mut self,  wife: &mut MinerWife) {}

    fn execute(&mut self, wife: &mut MinerWife) -> StateTransition<Self::Entity, Self::Enum> {
        if rand::thread_rng().gen_range(0,10) == 0 {
            return StateTransition::Push(VisitBathroom::new())
        }
        StateTransition::None
    }

    fn exit(&mut self, wife: &mut MinerWife) {}
}

impl State for DoHouseWork {
    type Entity = MinerWife;
    type Enum = MinerWifeStates;

    fn new() -> Box<Self> {
        Box::new(DoHouseWork)
    }

    fn enter(&mut self, wife: &mut MinerWife) {}

    fn execute(&mut self, wife: &mut MinerWife) -> StateTransition<Self::Entity, Self::Enum> {
        match rand::thread_rng().gen_range(0,3) {
            0 => {
                println!(">> {:?}: Moppin' the floor", wife.name());
            }
            1 => {
                println!(">> {:?}: Washin' the dishes", wife.name());
            },
            2 => {
                println!(">> {:?}: Makin' the bed", wife.name());
            },
            _ => {}
        }
        StateTransition::None
    }

    fn exit(&mut self, wife: &mut MinerWife) {}
}

impl State for VisitBathroom {
    type Entity = MinerWife;
    type Enum = MinerWifeStates;

    fn new() -> Box<Self> {
        Box::new(VisitBathroom)
    }

    fn enter(&mut self, wife: &mut MinerWife) {
        println!(">> {:?}: Walkin' to the can. Need to powda mah pretty li'lle nose", wife.name());
    }

    fn execute(&mut self, wife: &mut MinerWife) -> StateTransition<Self::Entity, Self::Enum> {
        println!(">> {:?}: Ahhhhhh! Sweet relief!", wife.name());
        StateTransition::Pop()
    }

    fn exit(&mut self, wife: &mut MinerWife) {
        println!(">> {:?}: Leavin' the Jon", wife.name())
    }
}