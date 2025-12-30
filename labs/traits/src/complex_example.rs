use std::fmt::Debug;

#[derive(Debug)]
struct Monster {
    health: i32,
}
#[derive(Debug)]
struct Wizard {
    health: i32,
}
#[derive(Debug)]
struct Elf {
    health: i32,
}

trait MonsterBehavior: Debug {
    fn take_damage(&mut self, damage: i32);
    fn display_self(&self) {
        println!("The monster is now: {self:?}")
    }
}

impl MonsterBehavior for Monster {
    fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }
}

trait DisplayHealth {
    fn health(&self) -> i32;
}
trait FightClose: Debug {
    fn attack_with_sword<T: MonsterBehavior>(&self, opponent: &mut T) {
        println!("You attack with your sword!");
        opponent.take_damage(10);
        opponent.display_self();
    }
    fn attack_with_hand<T: MonsterBehavior>(&self, opponent: &mut T) {
        println!("You attack with your hand!");
        opponent.take_damage(2);
        opponent.display_self();
    }
}

impl FightClose for Wizard {}
impl FightClose for Elf {}

trait FightFromDistance: Debug {
    fn attack_with_bow<T: MonsterBehavior>(&self, opponent: &mut T, distance: u32) {
        if distance < 10 {
            println!("You attack with your bow!");
            opponent.take_damage(10);
            opponent.display_self();
        }
    }
    fn attack_with_rock<T: MonsterBehavior>(&self, opponent: &mut T, distance: u32) {
        if distance < 3 {
            println!("You attack with your rock!");
            opponent.take_damage(5);
            opponent.display_self();
        }
    }
}
impl FightFromDistance for Elf {}

fn main() {
    let gandalf = Wizard { health: 60 };
    let legolas = Elf { health: 80 };

    let mut uruk_hai = Monster { health: 40 };
    gandalf.attack_with_sword(&mut uruk_hai);
    legolas.attack_with_bow(&mut uruk_hai, 5);
}
