#![allow(unused_parens)]

mod board;
use board::Board;
use prompted::*;

fn main() {
    let mut board: Board = Board::new();

    // Print the initial board state
    println!("Let's play!");
    println!();
    board.print_board();

    // Set the first move to be the human player
    let mut human: bool = true;

    loop
    {
        if (true == human)
        {
            let row: String = input!("Row: ");

            let row: usize = match row.trim().parse()
            {
                Ok(number) => number,
                Err(_) => continue,
            };

            let column: String = input!("Column: ");

            let column: usize = match column.trim().parse()
            {
                Ok(number) => number,
                Err(_) => continue,
            };

            match board.chomp(row, column)
            {
                Ok(true) =>
                {
                    println!("You chomped {row}, {column}");
                    println!("You win!");
                    break;
                },
                Ok(false) => println!("You chomped {row}, {column}"),
                Err(message) =>
                {
                    println!("{message}");
                    continue;
                }
            }
        }
        else
        {
            match board.winning_move()
            {
                Some((row, column)) =>
                {
                    match board.chomp(row, column)
                    {
                        Ok(true) =>
                        {
                            println!("Computer chomped {row}, {column}");
                            println!("You lose :(");
                            break;
                        },
                        Ok(false) =>
                        {
                            println!("Computer chomped {row}, {column}");
                        }
                        Err(message) =>
                        {
                            println!("The computer did something wrong trying to chomp {row}, {column} ({message})");
                            break;
                        }
                    }

                },
                None =>
                {
                    println!("Computer can't find a winning move. You win!");
                    break;
                }
            }
        }

        board.print_board();
        human = !human;
    }
}
