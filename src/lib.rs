use rand::Rng;
/// Die Object
/// 
/// This object is created using the builder pattern. The user has the option of setting the number of sides and the RNG
/// used by the Die.
pub struct Die {
    sides: u8,
    rng: Box<dyn DieRng>,
    #[cfg(feature = "history")]
    history: Vec<u8>
}

impl Die {
    /// Retrieve a new instance of the Builder class for the Die.
    pub fn builder() -> DieBuilder {
        DieBuilder::new()
    }

    /// Rolls the Die using it's internal RNG
    pub fn roll(&mut self) -> u8 {
        let ret = self.rng.random_int(1, self.sides);
        #[cfg(feature = "history")]
        {
            self.history.push(ret);
        }
        ret
    }

    #[cfg(feature = "history")]
    pub fn get_history(&self) -> Vec<u8> {
        self.history.clone()
    }
}

/// Die Builder
/// 
/// This class is used to build a new die. The user has the option of setting the sides and RNG the die will use.
pub struct DieBuilder {
    sides: u8,
    rng: Box<dyn DieRng>
}

impl DieBuilder {
    /// Creates a new DieBuilder, which defaults to a 6 sided die using a standard RNG.
    pub fn new() -> DieBuilder {
        Self { 
            sides: 6,
            rng: Box::new(DieStdRng{})
        }
    }

    /// Set the desired number of sides for the Die. Default value is used if 0 is passed.
    pub fn sides(mut self, sides: u8) -> DieBuilder {
        if sides > 1 {
            self.sides = sides;
        }
        self
    }

    /// Set the desired RNG for the Die.
    pub fn rng(mut self, rng: Box<dyn DieRng>) -> DieBuilder {
        self.rng = rng;
        self
    }

    /// Build the Die object with the current Builder parameters.
    pub fn build(self) -> Die {
        Die {
            sides: self.sides,
            rng: self.rng,
            #[cfg(feature = "history")]
            history: Vec::new()
        }
    }
}

/// RNG trait defines an interface for a Random Number Generater. A user can implement their own RNG and pass it to
/// the DieBuilder method. This interface is UNSAFE, one utilizing the interface could potentially pass bad parameters
/// i.e. l >= h. The Die will NOT exhibit this behavior.
pub trait DieRng {
    fn random_int(&self, l: u8, h: u8) -> u8;
}

/// An RNG implementation using rand crate functions
struct DieStdRng {}

impl DieRng for DieStdRng {
    fn random_int(&self, l: u8, h: u8) -> u8 {
        rand::thread_rng().gen_range(l..h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct _DieTerribleRng {}
    impl DieRng for _DieTerribleRng {
        fn random_int(&self, l: u8, _h: u8) -> u8 {
            l
        }
    }

    #[test]
    fn build_and_roll_six_sided_die() {
        let mut die = DieBuilder::new().sides(6).build();
        let result = die.roll();
        let mut valid_range: std::ops::Range<u8> = 1..6;
        assert_eq!(valid_range.any(|i| i == result), true);
    }

    #[test]
    fn use_custom_rng() {
        let mut die = DieBuilder::new().sides(6).rng(Box::new(_DieTerribleRng{})).build();
        assert_eq!(die.roll(), 1);
    }

    #[cfg(feature = "history")]
    #[test]
    fn get_history() {
        let mut die = DieBuilder::new().sides(6).rng(Box::new(_DieTerribleRng{})).build();
        for _i in 0..2 {
            die.roll();
        }
        let history = die.get_history();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0], 1);
    }
}
