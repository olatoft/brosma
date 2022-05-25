use std::fmt::{self, Display, Formatter};
use std::io::{self, Write};

#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
enum Color {
    White,
    Black
}

#[derive(Clone)]
#[derive(Copy)]
enum Type {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let letter = match self {
            Type::Pawn => "",
            Type::Knight => "S",
            Type::Bishop => "L",
            Type::Rook => "T",
            Type::Queen => "D",
            Type::King => "K"
        };
        
        write!(f, "{}", letter)
    }
}

#[derive(Clone)]
#[derive(Copy)]
struct Piece {
    color: Color,
    r#type: Type
}

fn column_to_letter(column: u8) -> Result<String, String> {
    match column {
        0 => Ok("a".to_string()),
        1 => Ok("b".to_string()),
        2 => Ok("c".to_string()),
        3 => Ok("d".to_string()),
        4 => Ok("e".to_string()),
        5 => Ok("f".to_string()),
        6 => Ok("g".to_string()),
        7 => Ok("h".to_string()),
        _ => Err("Out of board".to_string())
    }
}

#[derive(Clone)]
#[derive(Copy)]
struct Position {
    row: u8,
    column: u8
}

#[derive(Clone)]
#[derive(Copy)]
struct Move {
    r#type: Type,
    new_position: Position
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.r#type, column_to_letter(self.new_position.column).unwrap(), self.new_position.row + 1)
    }
}

fn initialize_board() -> Vec<Vec<Option<Piece>>> {
    vec![
        vec![
            Some(Piece {color: Color::White, r#type: Type::Rook}),
            Some(Piece {color: Color::White, r#type: Type::Knight}),
            Some(Piece {color: Color::White, r#type: Type::Bishop}),
            Some(Piece {color: Color::White, r#type: Type::Queen}),
            Some(Piece {color: Color::White, r#type: Type::King}),
            Some(Piece {color: Color::White, r#type: Type::Bishop}),
            Some(Piece {color: Color::White, r#type: Type::Knight}),
            Some(Piece {color: Color::White, r#type: Type::Rook})
        ],
        vec![
            Some(Piece {color: Color::White, r#type: Type::Pawn}),
            Some(Piece {color: Color::White, r#type: Type::Pawn}),
            Some(Piece {color: Color::White, r#type: Type::Pawn}),
            Some(Piece {color: Color::White, r#type: Type::Pawn}),
            Some(Piece {color: Color::White, r#type: Type::Pawn}),
            Some(Piece {color: Color::White, r#type: Type::Pawn}),
            Some(Piece {color: Color::White, r#type: Type::Pawn}),
            Some(Piece {color: Color::White, r#type: Type::Pawn}),
        ],
        vec![None, None, None, None, None, None, None, None],
        vec![None, None, None, None, None, None, None, None],
        vec![None, None, None, None, None, None, None, None],
        vec![None, None, None, None, None, None, None, None],
        vec![
            Some(Piece {color: Color::Black, r#type: Type::Pawn}),
            Some(Piece {color: Color::Black, r#type: Type::Pawn}),
            Some(Piece {color: Color::Black, r#type: Type::Pawn}),
            Some(Piece {color: Color::Black, r#type: Type::Pawn}),
            Some(Piece {color: Color::Black, r#type: Type::Pawn}),
            Some(Piece {color: Color::Black, r#type: Type::Pawn}),
            Some(Piece {color: Color::Black, r#type: Type::Pawn}),
            Some(Piece {color: Color::Black, r#type: Type::Pawn})
        ],
        vec![
            Some(Piece {color: Color::Black, r#type: Type::Rook}),
            Some(Piece {color: Color::Black, r#type: Type::Knight}),
            Some(Piece {color: Color::Black, r#type: Type::Bishop}),
            Some(Piece {color: Color::Black, r#type: Type::Queen}),
            Some(Piece {color: Color::Black, r#type: Type::King}),
            Some(Piece {color: Color::Black, r#type: Type::Bishop}),
            Some(Piece {color: Color::Black, r#type: Type::Knight}),
            Some(Piece {color: Color::Black, r#type: Type::Rook})
        ]
    ]
}

fn get_legal_pawn_moves(board: Vec<Vec<Option<Piece>>>, color: Color, position: Position) -> Vec<Move> {
    vec![Move { r#type: Type::Pawn, new_position: Position { row: position.row + 1, column: position.column }}]
}

fn get_legal_moves_for_piece(board: Vec<Vec<Option<Piece>>>, piece: Piece, position: Position) -> Vec<Move> {
    match piece.r#type {
        Type::Pawn => get_legal_pawn_moves(board, piece.color, position),
        _ => vec![]
    }
}

fn get_legal_moves_for_all_pieces(board: &Vec<Vec<Option<Piece>>>, turn: Color) -> Vec<Move> {
    board // Vec<Vec<Option<Piece>>>
    .into_iter() // Vec<Vec<Option<Piece>>>
    .enumerate() // Vec<(row, Vec<Option<Piece>>)>
    .map(|(row, vec_option_piece)| // (row, Vec<Option<Piece>>)
        vec_option_piece // Vec<Option<Piece>>
        .into_iter() // Vec<Option<Piece>>
        .enumerate() // Vec<(column, Option<Piece>)>
        .filter(|(_, option_piece)| // (column, Option<Piece>)
            match option_piece {
                Some(piece) => piece.color == turn,
                None => false
            }
        ) // Vec<(column, Option<Piece>)>
        // At this point we have filtered out all the Nones, so we know it's safe to unwrap
        .map(|(column, option_piece)| // (column, Option<Piece>)
            get_legal_moves_for_piece(
                board.to_vec(), option_piece.unwrap(), Position { row: row as u8, column: column as u8}
            ) // Vec<Move>
        ) // Vec<Vec<Move>>
        .flatten() // Vec<Move>
        .collect::<Vec<_>>()
    ) // Vec<Vec<Move>>
    .flatten() // Vec<Move>
    .collect::<Vec<_>>()
}

fn select_move(moves: Vec<Move>) -> Option<Move> {
    if moves.is_empty() {
        None
    } else {
        Some(moves[0])
    }
}

fn get_user_input() -> String {
    print!("Skriv ditt trekk: ");
    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    user_input
}

fn user_input_to_type_and_rest_of_user_input(user_input: String) -> Result<(Type, String), String> {
    if user_input.is_empty() {
        Err("User input empty".to_string())
    } else {
        match user_input[0] {
            
        }
    }
} 

fn user_input_to_move(user_input: String) -> Move {

}

fn get_user_move() -> Move {
    let user_input = get_user_input();
    user_input_to_move(user_input)
}

fn main() {
    let board = initialize_board();
    let moves = get_legal_moves_for_all_pieces(&board, Color::White);

    println!("Lovlege trekk:");

    for r#move in &moves {
        println!("\t{}", r#move)
    }

    println!();
    let selected_move_option = select_move(moves);

    match selected_move_option {
        Some(selected_move) => println!("Valgt trekk: {}", selected_move),
        None => println!("No legal moves")
    }

    let user_input = get_user_input();
    println!("Ditt trekk: {}", user_input)
}
