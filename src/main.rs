mod greet;

fn main() {
    println!("{}", greet::shout(&greet::hello("wrold")));
    // manners
    println!("{}", greet::goodbye("everyone"));
}
