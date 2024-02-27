
use candid::{self,CandidType, Deserialize };

#[derive(Clone, CandidType, Deserialize, Debug)]
pub struct State {
    pub counter: candid::Nat
}

impl Default for State {
    fn default() -> Self {
        // Create a new instance with default values
        Self {
            counter: Default::default()
        }
    }
}
impl State {
    pub fn get(&self) -> &candid::Nat { &self.counter}
    pub fn increment (&mut self) { self.counter+=1_u32;}
    pub fn set(&mut self, value:&candid::Nat){
        self.counter=value.clone();
    }
}

#[cfg(test)]

mod test {
    use super::*;
    #[test]
    fn test_name(){
        let mut state =State::default();
        state.set(&candid::Nat::from(5_u32));
        assert_eq!(state.counter, 5_u32);
        state.increment();
        assert_eq!(state.counter, 6_u32);
    }
}

