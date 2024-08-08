use discord::Discord;

mod discord;
mod packet;
mod socket;

fn main() {
    let discord = Discord::start().unwrap();
}
