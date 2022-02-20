
#[derive(PartialEq)]
#[derive(Clone,Copy)]
enum BoardStatus {
    Empty = 0,
    Player1 = 1,
    Player2 = 2,
    Cat = 3
}
impl BoardStatus {
    fn as_str(&self) -> &str {
        match self {
            &BoardStatus::Empty => " ",
            &BoardStatus::Player1 => "X",
            &BoardStatus::Player2 => "O",
            &BoardStatus::Cat => "NotSupposedToBeHERE"
        }
    }
}

struct Game {
    board: [BoardStatus;9],
    current_player: BoardStatus,
    game_status: BoardStatus
}
impl Game {
    fn print_board(&self){
        println!("
   1  2  3
  -------
A |{}|{}|{}|
  -------
B |{}|{}|{}|
  -------
C |{}|{}|{}|
  -------",
                  self.board[0].as_str(),
                  self.board[1].as_str(),
                  self.board[2].as_str(),
                  self.board[3].as_str(),
                  self.board[4].as_str(),
                  self.board[5].as_str(), 
                  self.board[6].as_str(),
                  self.board[7].as_str(),
                  self.board[8].as_str());
    }
    fn get_user_input(& self) -> i16{
        loop {
            let mut input = String::new();
            println!("Enter an input");
            std::io::stdin().read_line(&mut input).unwrap();
            input = input.as_str().trim().to_string();
            if input.len() != 2 {
                println!("bad input {} {}", input, input.len());
                continue;
            }
            let first = input.chars().nth(0).unwrap();
            let second = input.chars().nth(1).unwrap();
            let num: i16 = if first.is_numeric() {first as i16 - 49}
            else {second as i16 - 49};
            let ch: i16 = if first.is_numeric() {second as i16 - 97} else {first as i16 - 97};
            if ch > 2 || num > 2 {
                println!("bad input, out of range");
                continue;
            }
            return (ch * 3) + num;
        }
    }
    fn handle_user_input(&mut self, place: i16) {
        if self.board[place as usize]  != BoardStatus::Empty  { println!("Place taken"); return}
        self.board[place as usize] = self.current_player;
        self.current_player = if self.current_player  == BoardStatus::Player1  {BoardStatus::Player2} else {BoardStatus::Player1}
    }
    fn check_completed_game(&self) -> BoardStatus {
        for i in 0..2{
            if self.board[(i * 3)]  == self.board[(i * 3) + 1]  &&
                self.board[(i * 3)]  == self.board[(i * 3) + 2]  &&
                self.board[(i * 3)]  != BoardStatus::Empty  {
                    return if self.board[(i * 3)]  == BoardStatus::Player1  {BoardStatus::Player1} else {BoardStatus::Player2};
            }
            if self.board[i]  == self.board[i + 3]  &&
                self.board[i]  == self.board[i + 6]  &&
                self.board[i]  != BoardStatus::Empty  {
                    return if self.board[i]  == BoardStatus::Player1  {BoardStatus::Player1} else {BoardStatus::Player2};
            }
        }
        if (self.board[0] == self.board[4]  &&
            self.board[0]  == self.board[8]  &&
            self.board[0] != BoardStatus::Empty ) ||
            (self.board[6] == self.board[4]  &&
            self.board[6]  == self.board[2]  &&
            self.board[6] != BoardStatus::Empty ) {
                return if self.board[4] == BoardStatus::Player1 {BoardStatus::Player1} else {BoardStatus::Player2};
        }
        if !self.board.contains(&BoardStatus::Empty) {
            return BoardStatus::Cat;
        }
        return BoardStatus::Empty;
    }
}


fn main() {
    let mut game = Game{ board: [BoardStatus::Empty;9],
        current_player : BoardStatus::Player1,
        game_status: BoardStatus::Empty };
    while game.game_status  == BoardStatus::Empty {
        game.print_board();
        let u_input = game.get_user_input();
        game.handle_user_input(u_input);
        game.game_status = game.check_completed_game();
    }
    game.print_board();
    println!("Game over {} Won!",game.game_status.as_str())
}
