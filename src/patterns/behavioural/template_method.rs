#![allow(dead_code)]

trait WomboCombo {
    fn go(&mut self) {
        self.move1();
        self.move2();
        self.move3();
    }

    fn move1(&mut self);
    fn move2(&mut self);
    fn move3(&mut self);
}

struct FalcoCombo {
    out: String
}

impl WomboCombo for FalcoCombo {


    fn move1(&mut self) {
        self.out.push_str("Falcon Punch ");
    }

    fn move2(&mut self) {
        self.out.push_str("Reverse Swing Kick ");
    }

    fn move3(&mut self) {
        self.out.push_str("K/O");
    }
}

struct FoxCombo {
    out: String
}

impl WomboCombo for FoxCombo {
    fn move1(&mut self) {
        self.out.push_str("High Kick ");
    }

    fn move2(&mut self) {
        self.out.push_str("Left Hook ");
    }

    fn move3(&mut self) {
        self.out.push_str("K/O");
    }
}


#[cfg(test)]
mod visitor_tests {
    use super::*;

    #[test]
    fn test_template_method() {
        let mut that_aint_falco = FalcoCombo{ out: String::new() };
        let mut that_up_smash: FoxCombo = FoxCombo{ out: String::new() };

        that_aint_falco.go();
        that_up_smash.go();

        let expected_output1 = "Falcon Punch Reverse Swing Kick K/O";
        let expected_output2 = "High Kick Left Hook K/O";
        assert_eq!(that_aint_falco.out, expected_output1);
        assert_eq!(that_up_smash.out, expected_output2);
    }

}
