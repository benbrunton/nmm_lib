use super::{Game, Player, GameMove, Location, Piece, GameStatus};

#[test]
fn new_games_have_18_pieces() {
    let game = Game::new();
    assert_eq!(game.get_pieces().len(), 18);
}

#[test]
fn new_games_have_had_zero_turns() {
    let game = Game::new();
    assert_eq!(game.get_turn(), 0);
}

#[test]
fn new_games_start_at_player_1() {
    let game = Game::new();
    assert_eq!(game.get_next_player(), Player::One);
}

#[test]
fn placing_onto_an_empty_table_is_a_valid_move() {
    let mut game = Game::new();
    let new_move = game_move!(One, Hand, A7);
    assert!(game.submit(new_move));
}

#[test]
fn playing_out_of_turn_is_an_invalid_move() {
    let mut game = Game::new();
    let new_move = game_move!(Two, Hand, A7);
    assert!(!game.submit(new_move));
}

#[test]
fn after_a_valid_move_the_player_rotates() {
    let mut game = Game::new();
    let new_move = game_move!(One, Hand, A7);
    game.submit(new_move);
    assert_eq!(game.get_next_player(), Player::Two);
}

#[test]
fn all_pieces_start_in_hand() {
    let game = Game::new();
    let pieces = game.get_pieces();
    for piece in pieces {
        assert_eq!(piece.get_location(), Location::Hand);
    }
}

#[test]
fn it_updates_the_piece_when_a_valid_move_is_taken() {
    let mut game = Game::new();
    let new_move = game_move!(One, Hand, A7);
    game.submit(new_move);
    let pieces = game.get_pieces();
    assert!(pieces.iter().any(|&x| x.get_location() == Location::A7));
}

#[test]
fn it_increments_the_turn_after_a_valid_move() {
    let mut game = Game::new();
    let new_move = game_move!(One, Hand, A7);
    game.submit(new_move);
    let turn = game.get_turn();
    assert_eq!(turn, 1);
}

#[test]
fn a_piece_cannot_be_placed_upon_an_existing_position() {
    let mut game = Game::new();
    let new_move = game_move!(One, Hand, A7);
    game.submit(new_move);
    let next_move = game_move!(Two, Hand, A7);
    assert_eq!(game.submit(next_move), false);
}

#[test]
fn a_player_can_only_move_its_own_piece() {
    let mut game = Game::new();
    let new_move = game_move!(One, Hand, A7);
    game.submit(new_move);
    let next_move = game_move!(Two, Hand, A4);
    game.submit(next_move);
    let pieces = game.get_pieces();
    assert_piece_exists(&pieces, Player::Two, Location::A4);
}

#[test]
fn it_can_load_from_json() {
    let game = Game::load(get_json()).unwrap();
    let turn = game.get_turn();
    assert_eq!(turn, 2);
    let pieces = game.get_pieces();
    assert_piece_exists(&pieces, Player::One, Location::A7);
}

#[test]
fn a_piece_cannot_be_moved_while_any_are_left_in_hand() {
    let mut game = Game::new();
    let new_move = game_move!(One, A7, A4);
    assert_eq!(game.submit(new_move), false);
}

#[test]
fn once_all_pieces_are_placed_a_piece_can_be_moved_one_space() {
    let mut game = Game::load(get_json()).unwrap();
    let new_move = game_move!(One, C5, C4);

    assert!(game.submit(new_move));
    let pieces = game.get_pieces();
    assert_piece_exists(&pieces, Player::One, Location::C4);
    assert_piece_doesnt_exist(&pieces, Player::One, Location::C5);
}

#[test]
fn pieces_cant_be_moved_more_than_one_space() {
    let mut game = Game::load(get_late_game()).unwrap();
    let new_move = game_move!(One, C5, E3);

    assert_eq!(game.submit(new_move), false);
}

#[test]
fn when_there_is_three_in_a_row_an_opponents_piece_can_be_removed() {
    let mut game = Game::load(get_json()).unwrap();
    let new_move = game_move!(One, F4, G4, D2);

    let successful_move = game.submit(new_move);
    assert!(successful_move);
    let pieces = game.get_pieces();
    assert_piece_doesnt_exist(&pieces, Player::Two, Location::D2);
}

#[test]
fn when_the_removal_results_in_the_opponent_having_less_than_3_pieces_the_game_ends() {
    let mut game = Game::load(get_completable_game()).unwrap();
    let new_move = game_move!(One, F4, G4, D7);
    assert!(game.submit(new_move));
    assert_eq!(game.get_status(), GameStatus::Win(Player::One));
}

#[test]
fn when_both_players_have_3_pieces_the_status_is_playing() {
    let game = Game::new();
    let status = game.get_status();
    assert_eq!(status, GameStatus::Playing);
}

#[test]
fn pieces_in_a_mill_cannot_be_removed_while_there_are_alternatives() {
    let mut game = Game::load(get_existing_mill()).unwrap();
    let new_move = game_move!(One, F4, G4, D3);
    assert_eq!(game.submit(new_move), false);
}

#[test]
fn when_the_game_has_been_won_no_more_moves_can_be_made() {
    let mut game = Game::load(get_completable_game()).unwrap();
    let new_move = game_move!(One, F4, G4, D7);
    game.submit(new_move);
    let invalid_move = game_move!(Two, F6, D6);
    assert_eq!(game.submit(invalid_move), false);
}

// ------------------------------------------------------------------------------------------------

fn assert_piece_exists(pieces: &Vec<Piece>, player: Player, location: Location) {
    assert!(pieces.iter().any(|&x| {
        x.get_location() == location && x.get_player() == player
    }), "piece doesn't seem to exist");
}

fn assert_piece_doesnt_exist(pieces: &Vec<Piece>, player: Player, location: Location) {
    assert_eq!(pieces.iter().any(|&x| {
        x.get_location() == location && x.get_player() == player
    }), false, "piece exists");
}

fn get_json() -> &'static str {
    r#"{
        "pieces": [
            { "player": 1, "location": "A7"},
            { "player": 1, "location": "A1"},
            { "player": 1, "location": "G1"},
            { "player": 1, "location": "G7"},
            { "player": 1, "location": "B6"},
            { "player": 1, "location": "E4"},
            { "player": 1, "location": "F4"},
            { "player": 1, "location": "F2"},
            { "player": 1, "location": "C5"},
            { "player": 2, "location": "F6"},
            { "player": 2, "location": "B4"},
            { "player": 2, "location": "D7"},
            { "player": 2, "location": "D1"},
            { "player": 2, "location": "A4"},
            { "player": 2, "location": "D5"},
            { "player": 2, "location": "D2"},
            { "player": 2, "location": "D6"},
            { "player": 2, "location": "E3"}
        ],
        "turn": 2,
        "player_turn": 1
    }"#
}

fn get_late_game() -> &'static str {
    r#"{
        "pieces": [
            { "player": 1, "location": "A7"},
            { "player": 1, "location": "A1"},
            { "player": 1, "location": "G1"},
            { "player": 1, "location": "G7"},
            { "player": 1, "location": "B6"},
            { "player": 1, "location": "E4"},
            { "player": 1, "location": "Captured"},
            { "player": 1, "location": "Captured"},
            { "player": 1, "location": "Captured"},
            { "player": 2, "location": "F6"},
            { "player": 2, "location": "B4"},
            { "player": 2, "location": "D7"},
            { "player": 2, "location": "D1"},
            { "player": 2, "location": "A4"},
            { "player": 2, "location": "Captured"},
            { "player": 2, "location": "Captured"},
            { "player": 2, "location": "Captured"},
            { "player": 2, "location": "Captured"}
        ],
        "turn": 2,
        "player_turn": 1
    }"#
}

fn get_completable_game() -> &'static str {
    r#"{
        "pieces": [
            { "player": 1, "location": "A7"},
            { "player": 1, "location": "A1"},
            { "player": 1, "location": "G1"},
            { "player": 1, "location": "G7"},
            { "player": 1, "location": "F4"},
            { "player": 1, "location": "E4"},
            { "player": 1, "location": "Captured"},
            { "player": 1, "location": "Captured"},
            { "player": 1, "location": "Captured"},
            { "player": 2, "location": "F6"},
            { "player": 2, "location": "Captured"},
            { "player": 2, "location": "D7"},
            { "player": 2, "location": "D1"},
            { "player": 2, "location": "Captured"},
            { "player": 2, "location": "Captured"},
            { "player": 2, "location": "Captured"},
            { "player": 2, "location": "Captured"},
            { "player": 2, "location": "Captured"}
        ],
        "turn": 2,
        "player_turn": 1
    }"#
}

fn get_existing_mill() -> &'static str {
    r#"{
        "pieces": [
            { "player": 1, "location": "A7"},
            { "player": 1, "location": "A1"},
            { "player": 1, "location": "G1"},
            { "player": 1, "location": "G7"},
            { "player": 1, "location": "F4"},
            { "player": 1, "location": "E4"},
            { "player": 1, "location": "Captured"},
            { "player": 1, "location": "Captured"},
            { "player": 1, "location": "Captured"},
            { "player": 2, "location": "F6"},
            { "player": 2, "location": "D3"},
            { "player": 2, "location": "D7"},
            { "player": 2, "location": "D1"},
            { "player": 2, "location": "D2"},
            { "player": 2, "location": "Captured"},
            { "player": 2, "location": "Captured"},
            { "player": 2, "location": "Captured"},
            { "player": 2, "location": "Captured"}
        ],
        "turn": 2,
        "player_turn": 1
    }"#
}
