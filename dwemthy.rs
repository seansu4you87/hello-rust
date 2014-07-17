trait Monster {
    fn attack(&self);
    fn new() -> Self;
}

// impl Monster for int {
//     fn attack(&self) {
//         println!("The int attacks for {:d}.", *self)
//     }
// }

struct Penguin {
    life: int,
    strength: int,
    charisma: int,
}

impl Monster for Penguin {
    fn attack(&self) {
        println!("The penguin attacks for {:d}.", self.strength)
    }

    fn new() -> Penguin {
        Penguin { life: 10, strength: 1, charisma: 100 }
    }
}

struct Sparrow {
    life: int,
    strength: int,
    charisma: int,
}

impl Monster for Sparrow {
    fn attack(&self) {
        println!("The sparrow attacks for {:d}.", self.strength)
    }

    fn new() -> Sparrow {
        Sparrow { life: 20, strength: 5, charisma: 20 }
    }
}

struct Wolf {
    life: int,
    strength: int,
    charisma: int,
}

impl Monster for Wolf {
    fn attack(&self) {
        println!("The wolf attacks for {:d}.", self.strength)
    }

    fn new() -> Wolf {
        Wolf { life: 25, strength: 25, charisma: 3 }
    }
}

struct Raccoon {
    life: int,
    strength: int,
    charisma: int,
}

impl Monster for Raccoon {
    fn attack(&self) {
        println!("The raccoon attacks for {:d}.", self.strength)
    }

    fn new() -> Raccoon {
        Raccoon { life: 10, strength: 15, charisma: 50 }
    }
}

fn monsters_attack(monsters: &[&Monster]) {
    for monster in monsters.iter() {
        monster.attack();
    }
}

fn main() {
    let penguin: &Penguin = &Monster::new();
    let sparrow: &Sparrow = &Monster::new();
    let wolf: &Wolf = &Monster::new();
    let raccoon: &Raccoon = &Monster::new();

    let dwemthys_vector: &[&Monster] = [
        penguin as &Monster,
        sparrow as &Monster,
        wolf as &Monster,
        raccoon as &Monster,
    ];

    monsters_attack(dwemthys_vector);
}
