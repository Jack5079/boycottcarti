use mpris::{Event, PlayerFinder};

fn main() {
    let player_finder = PlayerFinder::new().expect("Could not connect to D-Bus");

    for player in player_finder.iter_players().unwrap() {
        let player = player.unwrap();
        if player
            .get_metadata()
            .unwrap()
            .artists()
            .unwrap_or_default()
            .contains(&"Playboi Carti")
        {
            player.next().unwrap();
        }
        for event in player.events().unwrap() {
            if let Event::TrackChanged(metadata) = event.unwrap() {
                if metadata
                    .artists()
                    .unwrap_or_default()
                    .contains(&"Playboi Carti")
                {
                    player.next().unwrap();
                }
            }
        }
    }
}
