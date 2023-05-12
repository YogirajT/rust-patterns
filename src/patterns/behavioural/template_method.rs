#![allow(dead_code)]
trait WomboCombo {
    fn go(&self) {
        self.move1();
        self.move2();
        self.move3();
    }

    fn move1(&self);
    fn move2(&self);
    fn move3(&self);
}

struct FalcoCombo;

impl WomboCombo for FalcoCombo {
    fn move1(&self) {
        println!("Falcon Punch");
    }

    fn move2(&self) {
        println!("Reverse Swing Kick");
    }

    fn move3(&self) {
        println!("K/O");
    }
}

struct FoxCombo;

impl WomboCombo for FoxCombo {
    fn move1(&self) {
        println!("High Kick");
    }

    fn move2(&self) {
        println!("Left Hook");
    }

    fn move3(&self) {
        println!("K/O");
    }
}

fn main() {
    let that_aint_falco = FalcoCombo;
    let whoa = FoxCombo;

    that_aint_falco.go();
    whoa.go();
}
