use std::io;

struct Position {
    x: char,
    y: u8,
}

struct Move {
    from: Position,
    to: Position,
}

struct ChessBoard {
    board: [[char; 8]; 8],
}

struct Index {
    x: u8,
    y: u8,
}

fn main() {
println!("Welcome to Rust Chess!");

let mut board = ChessBoard {
    board: [
        ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
        ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
        ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
    ], 
};



loop {
print_board(&board);
let Move { from, to } = Move {
    from: read_move(true),
    to: read_move(false),
};

//restart loop if move is invalid
if !move_validation(&board , &from, &to ) {
    println!("Invalid move");
    continue;
}

println!("your piece moves from {}{} to {}{}", from.x, from.y, to.x, to.y);
println!("your piece moves from {}{} to {}{}", position_to_index(&from).x, position_to_index(&from).y, position_to_index(&to).x, position_to_index(&to).y);
board = move_piece(&board, from, to);
}

}

fn move_piece(board: &ChessBoard, from: Position, to: Position) -> ChessBoard{
    let Index { x, y } = position_to_index(&from);
    let (from_x, from_y) = (x as usize, y as usize);
    let Index { x , y }= position_to_index(&to);
    let (to_x, to_y) = (x as usize, y as usize);
    let mut new_board = board.board;
    new_board[to_y][to_x] = new_board[from_y][from_x];
    new_board[from_y][from_x] = ' ';
    return ChessBoard {
        board: new_board,
    };
}


fn print_board(board: &ChessBoard) {
    //TODO print board nicely with letters and numbers
    for i in 0..8 {
        for j in 0..8 {
            print!("{}", board.board[i][j]);
        }
        println!("");
    }
}

fn position_to_index(position: &Position) -> Index {
    let x = position.x as u8 - 97;
    let y = position.y - 1;
    return Index {
        x,
        y,
    };
}







fn move_validation(board: &ChessBoard, from: &Position,to: &Position) -> bool
{
//check if from and to are not the same
if from.x == to.x && from.y == to.y {
    return false;
}
//ckeck if from is not empty
if board.board[position_to_index(&from).y as usize][position_to_index(&from).x as usize] == ' ' {
    return false;
}



return true;
}

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
