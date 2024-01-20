#[derive(Debug, Clone)]
pub struct Board
{
    grid: [[bool; Board::COLUMN_COUNT]; Board::ROW_COUNT],
}

impl Board
{
    //
    // Constants
    //

    const ROW_COUNT: usize = 4;
    const COLUMN_COUNT: usize = 5;


    //
    // Default Constructor
    //

    pub fn new() -> Self
    {
        return Self
        {
            grid: [[true; Board::COLUMN_COUNT]; Board::ROW_COUNT],
        }
    }



    //
    // The chomp() function makes a move on the board
    //

    pub fn chomp(&mut self, row: usize, column: usize) -> Result<bool, &'static str>
    {
        if (row > Board::ROW_COUNT || column > Board::COLUMN_COUNT)
        {
            return Err("Invalid row/column");
        }

        if (0 == row && 0 == column)
        {
            return Err("Can't choose 0,0");
        }

        if (false == self.grid[row][column])
        {
            return Err("Square already chomped");
        }

        for r in row..Board::ROW_COUNT
        {
            for c in column..Board::COLUMN_COUNT
            {
                self.grid[r][c] = false;
            }
        }

        return Ok(self.is_lost());
    }

    pub fn is_lost(&mut self) -> bool
    {
        for row in 0..Board::ROW_COUNT
        {
            for column in 0..Board::COLUMN_COUNT
            {
                if (0 == row && 0 == column)
                {
                    continue;
                }

                if (true == self.grid[row][column])
                {
                    return false;
                }
            }
        }

        return true;
    }

    //
    //
    //
    pub fn winning_move(&mut self) -> Option<(usize, usize)>
    {
        if (true == self.is_lost())
        {
            return None;
        }

        for row in 0..Board::ROW_COUNT
        {
            for column in 0..Board::COLUMN_COUNT
            {
                if (0 == row && 0 == column)
                {
                    continue;
                }

                if (true == self.grid[row][column])
                {
                    let mut board_copy: Board = self.clone();
                    board_copy.chomp(row, column);

                    // Find see if there is a winning move for the next player
                    let m: Option<(usize, usize)> = board_copy.winning_move();

                    match m
                    {
                        Some(_t) => {},
                        None => return Some((row, column)),
                    }
                }
            }
        }

        return None;
    }

    //
    // Prints the board
    //

    pub fn print_board(&self)
    {
        // Print the header
        print!(" ");

        for column in 0..Board::COLUMN_COUNT
        {
            print!("{column}");
        }

        println!();

        // Print the board
        for row in 0..Board::ROW_COUNT
        {
            print!("{row}");

            for column in 0..Board::COLUMN_COUNT
            {
                if (false == self.grid[row][column])
                {
                    print!(".");
                }
                else
                {
                    print!("#");
                }
            }

            println!();
        }
    }
}
