use Player;
use Location;

#[macro_export]
macro_rules! game_move {
    ($player:ident, $from:ident, $to:ident) 
        => (GameMove::new(Player::$player, Location::$from, Location::$to, None));
    ($player:ident, $from:ident, $to:ident, $remove:ident)
        => (GameMove::new(Player::$player, Location::$from, Location::$to, Some(Location::$remove)));
}

#[derive(PartialEq, Debug, Clone)]
pub struct GameMove {
    player: Player,
    from: Location,
    to: Location,
    remove: Option<Location>
}

impl GameMove {
    pub fn new(player: Player, from: Location, to: Location, remove: Option<Location>) -> GameMove {
        GameMove {
            player,
            from,
            to,
            remove
        }
    }

    pub fn get_player(&self) -> Player {
        self.player
    }

    pub fn get_from(&self) -> Location {
        self.from
    }

    pub fn get_to(&self) -> Location {
        self.to
    }

    pub fn get_remove(&self) -> Option<Location> {
        self.remove
    }
}
