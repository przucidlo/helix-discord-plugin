use discord::Discord;

mod discord;
mod message;
mod packet;
mod socket;

fn main() {
    let discord = Discord::start().unwrap();
}
