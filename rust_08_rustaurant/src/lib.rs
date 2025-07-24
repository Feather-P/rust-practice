mod front_of_house;

fn eat_at_restaurant() {
    use front_of_house::hosting;
    hosting::add_to_waitlist();
}