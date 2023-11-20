use std::io;

struct Position {
    x: char,
    y: u8,
}

struct Move {
    from: Position,
    to: Position,
}
// #[warn(dead_code)]
// struct ChessBoard {
//     board: [[char; 8]; 8],
// }

fn main() {
println!("Welcome to Rust Chess!");
let Move { from , to } = Move { from: read_move(true), to: read_move(false)};

println!("Your piece moves from {}{} to {}{}", from.x, from.y, to.x, to.y);
}

// fn print_board(board: ChessBoard) {
//     for i in 0..8 {
//         for j in 0..8 {
//             print!("{}", board.board[i][j]);
//         }
//         println!("");
//     }
// }

// fn move_validation(Move { from, to }: Move) -> bool
// {
//     //TODO check if move is valid
//     //TODO check if piece is on from position
//     //TODO check if piece is on to position
//     //TODO check if piece can move to to position
//     return true;
// }

fn read_move(fromorwhere: bool) -> Position {
    
    //0 = from, 1 = where
    if fromorwhere {
        println!("Print position of a piece you want to move: (e.g. a2)");
    } else {
        println!("Print position where you want to move: (e.g. a2)");
    } 
    

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    //input must be 2 chars long if not return read_move_from()
    if input.len() != 3 {
        println!("Invalid input wrong length");
        return read_move(fromorwhere);
    }
    //convert input to lowercase
    input = input.to_lowercase();
    
    //firt char must be a letter if not return read_move_from()
    if !input.chars().next().unwrap().is_alphabetic() {
        println!("Invalid input first char is not a letter");
        return read_move(fromorwhere);
    }
    
    //first char must be a letter between a and h if not return read_move_from()
    if input.chars().next().unwrap() < 'a' || input.chars().next().unwrap() > 'h' {
        println!("Invalid input first char is not a letter between a and h");
        return read_move(fromorwhere);
    }

    let first_char = input.chars().next().unwrap();
    
    //second char must be a number if not return read_move_from()
    if !input.chars().nth(1).unwrap().is_numeric() {
        println!("Invalid input second char is not a number");
        return read_move(fromorwhere);
    }
    
    //second char must be a number between 1 and 8 if not return read_move_from()
    if input.chars().nth(1).unwrap().to_digit(10).unwrap() < 1 || input.chars().nth(1).unwrap().to_digit(10).unwrap() > 8 {
        println!("Invalid input second char is not a number between 1 and 8");
        return read_move(fromorwhere);
    }

    let second_char = input.chars().nth(1).unwrap().to_digit(10).unwrap() as u8;
    

    return Position {
        x: first_char,
        y: second_char, 
    };
}
