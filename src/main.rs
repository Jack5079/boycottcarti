use std::thread;

use mpris::{Event, PlayerFinder};

fn main() {
    let player_finder = PlayerFinder::new().expect("Could not connect to D-Bus");

    thread::scope(|scope| {
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

            let id = player.unique_name().to_owned();
            scope.spawn(move || {
                let plr = PlayerFinder::new()
                    .expect("Could not connect to D-Bus")
                    .iter_players()
                    .unwrap()
                    .flat_map(|opt| opt)
                    .find(|p| p.unique_name() == id)
                    .unwrap();
                for event in plr.events().unwrap().flat_map(|opt| opt) {
                    if let Event::TrackChanged(metadata) = event {
                        if metadata
                            .artists()
                            .unwrap_or_default()
                            .contains(&"Playboi Carti")
                        {
                            plr.next().unwrap();
                        }
                    }
                }
            });
        }
    })
}
