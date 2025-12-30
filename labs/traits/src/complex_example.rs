use std::fmt::Debug;

#[derive(Debug)]
struct Wizard {
    health: i32,
}
#[derive(Debug)]
struct Elf {
    health: i32,
}

// #[derive(Debug)]
// struct Monster {
//     health: i32,
// }
//
// trait MonsterBehavior: Debug {
//     fn take_damage(&mut self, damage: i32);
//     fn display_self(&self) {
//         println!("The monster is now: {self:?}")
//     }
// }
//
// impl MonsterBehavior for Monster {
//     fn take_damage(&mut self, damage: i32) {
//         self.health -= damage;
//     }
// }
//
// trait DisplayHealth {
//     fn health(&self) -> i32;
// }
// trait FightClose: Debug {
//     fn attack_with_sword<T: MonsterBehavior>(&self, opponent: &mut T) {
//         println!("You attack with your sword!");
//         opponent.take_damage(10);
//         opponent.display_self();
//     }
//     fn attack_with_hand<T: MonsterBehavior>(&self, opponent: &mut T) {
//         println!("You attack with your hand!");
//         opponent.take_damage(2);
//         opponent.display_self();
//     }
// }
//
// trait FightFromDistance: Debug {
//     fn attack_with_bow<T: MonsterBehavior>(&self, opponent: &mut T, distance: u32) {
//         if distance < 10 {
//             println!("You attack with your bow!");
//             opponent.take_damage(10);
//             opponent.display_self();
//         }
//     }
//     fn attack_with_rock<T: MonsterBehavior>(&self, opponent: &mut T, distance: u32) {
//         if distance < 3 {
//             println!("You attack with your rock!");
//             opponent.take_damage(5);
//             opponent.display_self();
//         }
//     }
// }

// Changing here to traits as bounds

struct Monster {
    health: i32,
}

trait Magic {}
trait FightClose {}
trait FightFromDistance {}
impl FightClose for Wizard {}
impl FightClose for Elf {}
impl Magic for Wizard {}
impl FightFromDistance for Elf {}

fn attack_with_sword<T>(pc: &T, opponent: &mut Monster)
where
    T: FightClose + Debug,
{
    opponent.health -= 10;
    println!(
        "Sword attack! Opponent's health: {}. You are now at: {pc:?}",
        opponent.health
    );
}

fn attack_wirth_bow<T>(pc: &T, opponent: &mut Monster, distance: u32)
where
    T: FightFromDistance + Debug,
{
    if distance < 10 {
        opponent.health -= 10;
        println!(
            "Bow attack! Opponent's health: {}. You are now at: {pc:?}",
            opponent.health
        );
    }
}

fn fireball<T>(pc: T, opponent: &mut Monster)
where
    T: Magic + Debug,
{
    opponent.health -= 20;
    println!(
        "Fireball! Opponent's health: {}. You are now at: {pc:?}",
        opponent.health
    );
}

fn main() {
    let gandalf = Wizard { health: 60 };
    let legolas = Elf { health: 80 };

    let mut uruk_hai = Monster { health: 40 };
    // gandalf.attack_with_sword(&mut uruk_hai);
    // legolas.attack_with_bow(&mut uruk_hai, 5);

    attack_with_sword(&gandalf, &mut uruk_hai);
    attack_wirth_bow(&legolas, &mut uruk_hai, 5);
    fireball(gandalf, &mut uruk_hai);
}
