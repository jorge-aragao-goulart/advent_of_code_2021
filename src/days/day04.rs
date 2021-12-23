use super::{Day04, DaySolution};
use std::error::Error;

impl DaySolution for Day04 {
    fn part_1(&self, input_file: &String) -> Result<String, Box<dyn Error>> {
        let (call_order, boards) = read_input(input_file);
        let strategy = FirstWinEndsGame::new();
        let mut game = BingoGame::new(call_order, boards, Box::new(strategy));

        game.play();

        if let Some(last_call) = game.last_call {
            if let Some(winner) = game.strategy.get_end_trigger_board() {
                return Ok((last_call * winner.sum_unmarked()).to_string());
            }
        }

        Err(Box::new(UnwinableBingoGame))
    }

    fn part_2(&self, input_file: &String) -> Result<String, Box<dyn Error>> {
        let (call_order, boards) = read_input(input_file);
        let strategy = LastWinEndsGame::new(&boards);
        let mut game = BingoGame::new(call_order, boards, Box::new(strategy));

        game.play();

        if let Some(last_call) = game.last_call {
            if let Some(winner) = game.strategy.get_end_trigger_board() {
                return Ok((last_call * winner.sum_unmarked()).to_string());
            }
        }

        Err(Box::new(UnwinableBingoGame))
    }
}

// Shared
#[derive(Clone, Copy, Debug)]
struct BingoBoardCell {
    number: i32,
    marked: bool,
}
impl BingoBoardCell {
    fn new() -> Self {
        BingoBoardCell {
            number: 0,
            marked: false,
        }
    }
}

const NUM_CELLS_IN_BINGO_BOARD: usize = 5 * 5;
#[derive(Clone, Debug)]
struct BingoBoard {
    cells: [BingoBoardCell; NUM_CELLS_IN_BINGO_BOARD],
    bingo: bool,
}
impl BingoBoard {
    fn new(numbers: &Vec<i32>) -> Self {
        let mut me = BingoBoard {
            cells: [BingoBoardCell::new(); NUM_CELLS_IN_BINGO_BOARD],
            bingo: false,
        };
        let mut cells_filled: usize = 0;
        while cells_filled < NUM_CELLS_IN_BINGO_BOARD && cells_filled < numbers.len() {
            me.cells[cells_filled].number = numbers[cells_filled];
            cells_filled += 1;
        }

        me
    }

    fn mark(&mut self, called_number: i32) {
        for cell in &mut self.cells {
            if cell.number == called_number {
                cell.marked = true;
            }
        }
        self.bingo = self.check_bingo();
    }

    fn check_bingo(&self) -> bool {
        let c = &self.cells;
        // check horizontals
        (c[0].marked && c[1].marked && c[2].marked && c[3].marked && c[4].marked) ||
        (c[5].marked && c[6].marked && c[7].marked && c[8].marked && c[9].marked) ||
        (c[10].marked && c[11].marked && c[12].marked && c[13].marked && c[14].marked) ||
        (c[15].marked && c[16].marked && c[17].marked && c[18].marked && c[19].marked) ||
        (c[20].marked && c[21].marked && c[22].marked && c[23].marked && c[24].marked) ||
        // check verticals
        (c[0].marked && c[5].marked && c[10].marked && c[15].marked && c[20].marked) ||
        (c[1].marked && c[6].marked && c[11].marked && c[16].marked && c[21].marked) ||
        (c[2].marked && c[7].marked && c[12].marked && c[17].marked && c[22].marked) ||
        (c[3].marked && c[8].marked && c[13].marked && c[18].marked && c[23].marked) ||
        (c[4].marked && c[9].marked && c[14].marked && c[19].marked && c[24].marked)
    }

    fn sum_unmarked(&self) -> i32 {
        let mut sum = 0;
        for cell in self.cells {
            if !cell.marked {
                sum += cell.number;
            }
        }
        sum
    }
}

trait EndGameStrategy {
    fn uptade(&mut self, just_bingoed: Vec<&BingoBoard>);
    fn has_game_ended(&self) -> bool;
    fn get_end_trigger_board(&self) -> Option<BingoBoard>;
}

struct BingoGame {
    call_order: Vec<i32>,
    last_call: Option<i32>,
    boards: Vec<BingoBoard>,
    strategy: Box<dyn EndGameStrategy>,
}
impl BingoGame {
    fn new(
        mut call_order: Vec<i32>,
        boards: Vec<BingoBoard>,
        strategy: Box<dyn EndGameStrategy>,
    ) -> Self {
        call_order.reverse();

        let me = BingoGame {
            call_order,
            last_call: None,
            boards,
            strategy,
        };

        me
    }

    fn has_game_ended(&self) -> bool {
        if self.call_order.is_empty() {
            return true;
        }

        self.strategy.has_game_ended()
    }

    fn step(&mut self) {
        if let Some(called_number) = self.call_order.pop() {
            self.last_call = Some(called_number);

            let mut just_bingoed: Vec<&BingoBoard> = Vec::new();
            for b in &mut self.boards {
                if b.bingo {
                    // no need to continue marking off numbers
                    continue;
                }

                b.mark(called_number);

                if b.bingo {
                    just_bingoed.push(b);
                }
            }

            self.strategy.uptade(just_bingoed);
        }
    }

    fn play(&mut self) {
        while !self.has_game_ended() {
            self.step();
        }
    }
}

use std::fmt;
#[derive(Debug)]
pub struct UnwinableBingoGame;
impl fmt::Display for UnwinableBingoGame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Bingo Game did not finish with a winner after all numbers were called"
        )
    }
}
impl Error for UnwinableBingoGame {}

use std::fs;
fn read_input(input_file: &String) -> (Vec<i32>, Vec<BingoBoard>) {
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");

    let mut call_order_str: Vec<&str> = contents.split("\n\n").collect();
    let boards_str: Vec<&str> = call_order_str.split_off(1);
    let call_order_str = call_order_str[0];

    let call_order: Vec<i32> = call_order_str
        .split(",")
        .map(|i_str| i_str.parse().unwrap())
        .collect();

    let mut boards = Vec::new();
    for b_str in boards_str {
        let board_nums: Vec<i32> = b_str
            .split_ascii_whitespace()
            .map(|i_str| i_str.parse().unwrap())
            .collect();
        boards.push(BingoBoard::new(&board_nums));
    }

    (call_order, boards)
}

// Part 1
struct FirstWinEndsGame {
    winner: Option<BingoBoard>,
}
impl FirstWinEndsGame {
    fn new() -> Self {
        FirstWinEndsGame { winner: None }
    }
}
impl EndGameStrategy for FirstWinEndsGame {
    fn uptade(&mut self, just_bingoed: Vec<&BingoBoard>) {
        if !just_bingoed.is_empty() {
            self.winner = Some(just_bingoed[0].clone());
        }
    }

    fn has_game_ended(&self) -> bool {
        match self.winner {
            Some(_) => true,
            _ => false,
        }
    }

    fn get_end_trigger_board(&self) -> Option<BingoBoard> {
        if let Some(winner) = &self.winner {
            Some(winner.clone())
        } else {
            None
        }
    }
}

// Part 2
struct LastWinEndsGame {
    num_boards: usize,
    num_boards_bingoed: usize,
    last_winner: Option<BingoBoard>,
}
impl LastWinEndsGame {
    fn new(boards: &Vec<BingoBoard>) -> Self {
        LastWinEndsGame {
            num_boards: boards.len(),
            num_boards_bingoed: 0,
            last_winner: None,
        }
    }
}
impl EndGameStrategy for LastWinEndsGame {
    fn uptade(&mut self, just_bingoed: Vec<&BingoBoard>) {
        self.num_boards_bingoed += just_bingoed.len();
        if self.num_boards_bingoed == self.num_boards {
            self.last_winner = Some(just_bingoed[just_bingoed.len() - 1].clone());
        }
    }

    fn has_game_ended(&self) -> bool {
        self.num_boards_bingoed == self.num_boards
    }

    fn get_end_trigger_board(&self) -> Option<BingoBoard> {
        if let Some(last_winner) = &self.last_winner {
            Some(last_winner.clone())
        } else {
            None
        }
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marks_and_checks_bingo() {
        let mut board = BingoBoard::new(&vec![
            14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3,
            7,
        ]);

        board.mark(14);
        assert!(board.cells[0].marked);
        board.mark(21);
        assert!(board.cells[1].marked);
        board.mark(17);
        assert!(board.cells[2].marked);
        board.mark(24);
        assert!(board.cells[3].marked);
        board.mark(4);
        assert!(board.cells[4].marked);

        assert!(board.check_bingo());
    }

    #[test]
    fn part_1() {
        let call_order = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let boards = vec![
            BingoBoard::new(&vec![
                22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20,
                15, 19,
            ]),
            BingoBoard::new(&vec![
                3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16,
                12, 6,
            ]),
            BingoBoard::new(&vec![
                14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0,
                12, 3, 7,
            ]),
        ];

        let strategy = FirstWinEndsGame::new();
        let mut game = BingoGame::new(call_order, boards, Box::new(strategy));
        game.play();
        assert_eq!(Some(24), game.last_call);
        assert_eq!(
            188,
            game.strategy
                .get_end_trigger_board()
                .unwrap()
                .sum_unmarked()
        );
    }

    #[test]
    fn part_2() {
        let call_order = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let boards = vec![
            BingoBoard::new(&vec![
                22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20,
                15, 19,
            ]),
            BingoBoard::new(&vec![
                3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16,
                12, 6,
            ]),
            BingoBoard::new(&vec![
                14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0,
                12, 3, 7,
            ]),
        ];

        let strategy = LastWinEndsGame::new(&boards);
        let mut game = BingoGame::new(call_order, boards, Box::new(strategy));
        game.play();
        assert_eq!(Some(13), game.last_call);
        assert_eq!(
            148,
            game.strategy
                .get_end_trigger_board()
                .unwrap()
                .sum_unmarked()
        );
    }
}
