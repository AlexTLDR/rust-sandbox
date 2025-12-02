trait Shoot {
    fn shoot(&self);
}

struct FootballPlayer {
    name: String,
}

struct BasketballPlayer {
    name: String,
}

struct Gun {
    model: String,
}

impl Shoot for FootballPlayer {
    fn shoot(&self) {
        println!("{} shoots a football!", self.name);
    }


}

impl Shoot for BasketballPlayer {
    fn shoot(&self) {
        println!("{} shoots a basketball!", self.name);
    }

}
impl Shoot for Gun {
    fn shoot(&self) {
        println!("{} fires a bullet!", self.model);
    }
}

fn main() {
    let footballer = FootballPlayer {
        name: String::from("Alice"),

    };
    let basketballer = BasketballPlayer {
        name: String::from("Bob"),
    };
    let gun = Gun {
        model: String::from("Glock 19"),
    };
fn shoot<T: Shoot>(item: &T) {
        item.shoot();
    }

    shoot(&footballer);
    shoot(&basketballer);
    shoot(&gun);

}