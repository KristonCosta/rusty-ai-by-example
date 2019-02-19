use super::state;

use super::miner::Miner;
use super::map::Locations;

pub struct EnterMineAndDigForNugget {}
pub struct VisitBankAndDepositGold {}
pub struct GoHomeAndSleepTilRested {}
pub struct QuenchThirst{}

impl state::State for EnterMineAndDigForNugget {
    fn new() -> Self {
        EnterMineAndDigForNugget{}
    }

    fn enter(&mut self,  miner: &mut Miner) {
        if miner.location() != Locations::Goldmine {
            println!(">> {:?}: Walkin' to the goldmine", miner.name());
            miner.change_location(Locations::Goldmine)
        }
    }

    fn execute(&mut self, miner: &mut Miner) {
        miner.add_to_gold_carried(1);
        miner.increase_fatigue();
        println!(">> {:?}: Pickin' up a nugget", miner.name());
        if miner.pocket_is_full() {
            miner.change_state::<VisitBankAndDepositGold>()
        }
        if miner.thirsty() {
            miner.change_state::<QuenchThirst>()
        }
    }

    fn exit(&mut self, miner: &mut Miner) {
        println!(">> {:?}: Ah'm leavin' the goldmine with mah pockets full o' sweet gold", miner.name())
    }
}

impl state::State for VisitBankAndDepositGold {
    fn new() -> Self {
        VisitBankAndDepositGold{}
    }

    fn enter(&mut self, miner: &mut Miner) {
        if miner.location() != Locations::Bank {
            println!(">> {:?}: Goin' to the bank. Yes siree", miner.name());
            miner.change_location(Locations::Bank)
        }
    }

    fn execute(&mut self, miner: &mut Miner) {
        miner.add_to_wealth(miner.gold_carried());
        miner.set_gold_carried(0);
        println!(">> {:?}: Depositing gold. Total savings now: {:?}", miner.name(), miner.wealth());
        if miner.comfortable() {
            println!(">> {:?}: WooHoo! Rich enough for now. Back home I go", miner.name());
            miner.change_state::<GoHomeAndSleepTilRested>()
        } else {
            miner.change_state::<EnterMineAndDigForNugget>()
        }
    }

    fn exit(&mut self, miner: &mut Miner) {
        println!(">> {:?}: Leavin' the bank", miner.name())
    }
}

impl state::State for GoHomeAndSleepTilRested {
    fn new() -> Self {
        GoHomeAndSleepTilRested{}
    }

    fn enter(&mut self, miner: &mut Miner) {
        if miner.location() != Locations::Shack {
            println!(">> {:?}: Walkin' home", miner.name());
            miner.change_location(Locations::Shack)
        }
    }

    fn execute(&mut self, miner: &mut Miner) {
        if !miner.fatigued() {
            println!(">> {:?}: Wad a God darn fantastic nap! Time to find more gold", miner.name());
            miner.change_state::<EnterMineAndDigForNugget>()
        } else {
            miner.decrease_fatigue();
            println!(">> {:?}: Zzzzzz....", miner.name())
        }
    }

    fn exit(&mut self, miner: &mut Miner) {
        println!(">> {:?}: Leaving the house", miner.name())
    }
}

impl state::State for QuenchThirst {
    fn new() -> Self where Self: Sized {
        QuenchThirst{}
    }

    fn enter(&mut self, miner: &mut Miner) {
        if miner.location() != Locations::Saloon {
            println!(">> {:?}: Boy, ah sure is thursty! Walkin' to the saloon", miner.name());
            miner.change_location(Locations::Saloon);
        }
    }

    fn execute(&mut self, miner: &mut Miner) {
        if miner.thirsty() {
            miner.buy_drink_and_whiskey();
            println!(">> {:?}: That's a mighty fine sippin liquer", miner.name());
            miner.change_state::<EnterMineAndDigForNugget>()
        } else {
            panic!("WHY DID I GO TO QUENCH MY THIRST WHEN I WASN'T THIRSTY?!")
        }
    }

    fn exit(&mut self, miner: &mut Miner) {
        println!(">> {:?}: Leaving the saloon, feelin' good", miner.name());
    }
}