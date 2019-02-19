use crate::miner;

pub trait State {
    fn new() -> Self where Self: Sized;
    fn enter(&mut self, miner: &mut miner::Miner);
    fn execute(&mut self, miner: &mut miner::Miner);
    fn exit(&mut self, miner: &mut miner::Miner);
}