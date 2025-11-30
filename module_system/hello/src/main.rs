use hello::english;
use hello::spanish;
use hello::romanian;
use hello::romanian::alex;

fn main() {
    //hello::greet();
    english::greet();
    spanish::greet();
    romanian::greet();
    alex::greet();
}
