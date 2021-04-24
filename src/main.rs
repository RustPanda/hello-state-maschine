
//ОбЪявим возможные сос state: () state: ()тояния StateMaschine
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Follower {
    a: i32,
}
impl Follower {
    fn new(a: i32) -> Follower {
        Follower { a }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Condidate {
    a: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Leader {
    a: i32,
}

///Машина состояний/ конечный автомат/ StateMashine
#[derive(Debug, PartialEq, Eq)]
struct StateMaschine<S> {
    state: S
}
impl StateMaschine<Follower> {
    fn new(state: Follower) -> Self { StateMaschine { state } }
}

//Реалезуем возможные переходы из одного состояния в другое для StateMashine
impl From< StateMaschine<Follower> > for StateMaschine<Condidate> {
    fn from(val: StateMaschine<Follower>) -> Self {
        Self { state: Condidate { a: val.state.a } } 
    }
}

impl From< StateMaschine<Condidate> > for StateMaschine<Follower> {
    fn from(val: StateMaschine<Condidate>) -> Self {
        Self { state: Follower { a: val.state.a } } 
    }
}
impl From< StateMaschine<Leader> > for StateMaschine<Follower> {
    fn from(val: StateMaschine<Leader>) -> Self {
        Self { state: Follower { a: val.state.a } } 
    }
}

impl From< StateMaschine<Condidate> > for StateMaschine<Leader> {
    fn from(val: StateMaschine<Condidate>) -> Self {
        Self { state: Leader { a: val.state.a } } 
    }
}


fn main() {
    println!("Hello StateMaschine!");

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nwe_state_mashine() {
        assert_eq!(StateMaschine::new(Follower::new(5)), StateMaschine { state: Follower {a: 5}});
    }

    #[test]
    fn condidate_from_follower() {
        let state_mashiene = StateMaschine::new(Follower::new(5) );
        assert_eq!(StateMaschine::<Condidate>::from(state_mashiene), StateMaschine { state: Condidate {a: 5} });
    }

    #[test]
    fn follower_from_condidate() {
        let state_mashiene = StateMaschine { state: Follower {a: 5} };

        assert_eq!(StateMaschine::<Follower>::from(state_mashiene), StateMaschine { state: Follower {a: 5} });
    }

    #[test]
    fn follower_from_leader() {
        let state_mashiene = StateMaschine { state: Leader {a: 5} };

        assert_eq!(StateMaschine::<Follower>::from(state_mashiene), StateMaschine { state: Follower {a: 5} });
    }

    #[test]
    fn leader_from_condidate() {
        let state_mashiene = StateMaschine { state: Condidate {a: 5} };

        assert_eq!(StateMaschine::<Leader>::from(state_mashiene), StateMaschine { state: Leader {a: 5} });
    }
    
}
