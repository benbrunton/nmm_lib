#[macro_use]
extern crate serde_json;

#[macro_use]
mod piece;
#[macro_use]
mod game_move;
mod location;
mod player;

#[cfg(test)]
mod test;

use serde_json::{Value, Error};
pub use piece::Piece;
pub use game_move::GameMove;
pub use location::Location;
pub use player::Player;

#[derive(PartialEq, Debug)]
pub enum GameStatus {
    Playing,
    Win(Player),
    Draw
}

#[derive(PartialEq, Debug)]
pub struct Game{
    pieces: Vec<Piece>,
    turn: u8,
    next_player: Player
}

impl Game {
    pub fn new() -> Game {
        Game{
            pieces: Self::get_new_pieces(),
            turn: 0,
            next_player: Player::One
        }
    }

    pub fn load(serialised_game: &str) -> Result<Game, Error> {
        let v: Value = serde_json::from_str(serialised_game)?;
        let pieces = Self::unwrap_pieces(v["pieces"].clone());
        Ok(Game{
            pieces: pieces,
            turn: v["turn"].as_u64().unwrap() as u8,
            next_player: Self::unwrap_player(v["player_turn"].as_u64().unwrap())
        })
    }

    pub fn get_json(&self) -> String {
        let pieces = self.wrap_pieces();
        let json = json!({
            "pieces": pieces,
            "turn": self.turn,
            "player_turn": Self::wrap_player(self.next_player)
        });
        
        String::from(json.to_string())
    }

    pub fn get_pieces(&self) -> Vec<Piece> {
        return self.pieces.clone();
    }

    pub fn get_turn(&self) -> u8 {
        return self.turn;
    }

    pub fn get_next_player(&self) -> Player {
        return self.next_player.clone();
    }

    pub fn get_status(&self) -> GameStatus {
        if self.get_player_captured_count(Player::One) > 6 {
            return GameStatus::Win(Player::Two);
        }

        if self.get_player_captured_count(Player::Two) > 6 {
            return GameStatus::Win(Player::One);
        }

        GameStatus::Playing
    }

    pub fn submit(&mut self, game_move: GameMove) -> bool {

        if self.get_status() != GameStatus::Playing {
            return false;
        }

        let old_location = game_move.get_from();
        let new_location = game_move.get_to();
        let player = game_move.get_player();
        let remove = game_move.get_remove();

        if !self.is_valid_move(player, old_location, new_location) {
            return false;
        }

        if !self.is_valid_removal(player, remove) {
            return false;
        }

        self.pieces = self.get_updated_pieces(player, old_location, new_location);

        if self.is_three_in_a_row(player, new_location) {
            self.pieces = self.get_updated_with_removed(player, remove);
        }

        self.turn += 1;
        self.next_player = Self::switch_player(self.next_player);

        true
    }

    fn is_valid_move(
        &self, 
        player: Player, 
        old_location: Location, 
        new_location: Location
    ) -> bool {
        if player != self.next_player {
            return false;
        }

        if old_location != Location::Hand && self.is_in_placement_phase() {
            return false;
        }

        if self.is_location_occupied(new_location) {
            return false;
        }

        if !self.is_in_placement_phase() && !self.is_next_door(old_location, new_location) {
            return false;
        }

        true
    }

    fn is_valid_removal(&self, player: Player, removal: Option<Location>) -> bool {
        match removal {
            None => true,
            Some(location) => {
                let other_player = Self::switch_player(player);
                !self.is_three_in_a_row(other_player, location) 
                    || !self.does_player_have_non_mill_pieces(other_player)
            }
        }
    }

    fn is_in_placement_phase(&self) -> bool {
        self.pieces.iter().any(|&piece| {
            piece.get_location() == Location::Hand
        })
    }

    fn is_location_occupied(&self, new_location: Location) -> bool {
        self.pieces.iter().any(|&piece| {
            piece.get_location() == new_location
        })
    }

    fn is_next_door(&self, new_location: Location, old_location: Location) -> bool {
        let rows = Location::get_rows(old_location).unwrap();
        for (a, b) in rows {
            if a == new_location || b == new_location {
                return true;
            }
        }

        false
    }

    fn does_player_have_non_mill_pieces(&self, player: Player) -> bool {
        self.pieces.iter().filter(|&piece| {
            piece.get_player() == player 
                && piece.get_location() != Location::Captured
                && piece.get_location() != Location::Hand
                && !self.is_three_in_a_row(player, piece.get_location())
        }).count() > 0
    }

    fn get_updated_pieces(
        &self, 
        player: Player,
        old_location: Location,
        new_location: Location
    ) -> Vec<Piece> {
        self.pieces.iter().map(|&piece| {
            if piece.get_location() == old_location && piece.get_player() == player {
                Piece::new(player, new_location)
            } else {
                piece.clone()
            }
        }).collect()
    }

    fn get_updated_with_removed(&self, player: Player, remove: Option<Location>) -> Vec<Piece> {
        match remove {
            Some(location) => {
                self.pieces.iter().map(|&piece| {
                    if piece.get_location() == location 
                        && piece.get_player() != player {
                        Piece::new(piece.get_player(), Location::Captured)
                    } else {
                        piece.clone()
                    }
               }).collect()
            },
            _ => self.pieces.clone()
        }
    }


    fn is_three_in_a_row(&self, player: Player, new_location: Location) -> bool {
        let new_location_output = format!("{:?}", new_location);
        let rows = Location::get_rows(new_location).expect(&new_location_output);

        for (a, b) in rows {
            if self.does_piece_exist(a, player) && self.does_piece_exist(b, player) {
                return true;
            }
        }

        false
    }

    fn does_piece_exist(&self, location: Location, player: Player) -> bool {
        self.pieces.iter().any(|&piece| {
            piece.get_location() == location && piece.get_player() == player
        })
    }

    fn get_player_captured_count(&self, player: Player) -> u8 {
        self.pieces.iter().filter(|&piece| {
            piece.get_location() == Location::Captured
                && piece.get_player() == player
        }).count() as u8
    }

    fn wrap_pieces(&self) -> Vec<Value> {
        self.pieces.iter().map(|&piece| {
            json!({
                "player": Self::wrap_player(piece.get_player()),
                "location": piece.get_location().to_str()
            })
        }).collect()
    }

    fn get_new_pieces() -> Vec<Piece> {
        vec!(
            piece!(One, Hand),
            piece!(One, Hand),
            piece!(One, Hand),
            piece!(One, Hand),
            piece!(One, Hand),
            piece!(One, Hand),
            piece!(One, Hand),
            piece!(One, Hand),
            piece!(One, Hand),
            piece!(Two, Hand),
            piece!(Two, Hand),
            piece!(Two, Hand),
            piece!(Two, Hand),
            piece!(Two, Hand),
            piece!(Two, Hand),
            piece!(Two, Hand),
            piece!(Two, Hand),
            piece!(Two, Hand)
        )
    }

    fn unwrap_pieces(v: Value) -> Vec<Piece> {
        v.as_array().unwrap().iter().map(|ref x| {
            Piece::new(
                Self::unwrap_player(x["player"].as_u64().unwrap()), 
                Location::from_str(x["location"].as_str().unwrap())
            )
        }).collect()
    }

    fn unwrap_player(player: u64) -> Player {
        match player {
            1 => Player::One,
            2 => Player::Two,
            _ => Player::One
        }
    }

    fn wrap_player(player: Player) -> u64 {
        match player {
            Player::Two => 2,
            _           => 1
        }
    }

    fn switch_player(player: Player) -> Player {
        match player {
            Player::One => Player::Two,
            _           => Player::One
        }
    }
}


