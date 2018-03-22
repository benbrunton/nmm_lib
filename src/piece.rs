use Location;
use Player;

#[macro_export]
macro_rules! piece {
   ($player:ident, $location:ident) => (Piece::new(Player::$player, Location::$location));
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Piece{
    player: Player,
    location: Location    
}

impl Piece {
    pub fn new(player: Player, location: Location) -> Piece {
        Piece{
            player,
            location
        }
    }

    pub fn get_location(&self) -> Location {
        self.location
    }

    pub fn get_player(&self) -> Player {
        self.player
    }
}
