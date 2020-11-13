use rand::Rng;
use std::iter;


enum BoardPiece{
    Empty,
    Star,
    Player,
}

pub struct Board{
    board:Vec<Vec<BoardPiece>>,
    x_size: usize,
    y_size: usize,
}


impl Board{
    pub fn new(x_size: usize, y_size: usize, min_stars: u32, max_stars: u32)->Board{
        let mut board:Vec<Vec<BoardPiece>>=Vec::new();
        let mut star_count: u32 = 0;

        for _ in 0..y_size{
            let mut row:Vec<BoardPiece>=Vec::new();
            for _ in 0..x_size{
                row.push(BoardPiece::Empty);
            }
            board.push(row);

        }

        let mut rng = rand::thread_rng();
        let num_stars = min_stars+rng.gen_range(0, max_stars-min_stars);
        while star_count < num_stars{
            let x:usize = rng.gen_range(0,x_size );
            let y:usize = rng.gen_range(0, y_size);
            println!("x: {} y: {}", x, y);
            match board[y][x]{
                BoardPiece::Empty => {
                    board[y][x]=BoardPiece::Star; 
                    star_count += 1;
                    println!("{}", star_count);
                },
                _ =>{},
            }

        }

        Board{
            board:board,
            x_size:x_size,
            y_size:y_size
        }

    }
    pub fn display(&mut self){
        let upper_letters:String= String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        let mut header:String =String::from("  ");
        let mut len_columns = 3;
        let mut space = String::from("  ");
        let y_digits = self.y_size.to_string().len();

        header.push_str(&iter::repeat(" ").take(y_digits+1).collect::<String>());
        if self.x_size>26{
            len_columns = 4;
            space.push(' ');
            for i in 0..self.x_size{
                let first_letter = upper_letters.chars().nth(i/26).unwrap();
                let second_letter = upper_letters.chars().nth(i%26).unwrap();

                header.push(first_letter);
                header.push(second_letter);
                header.push_str(&space);
            }
        } else {
            let len_columns = 3;
            for i in 0..self.x_size{
                header.push(upper_letters.chars().nth(i).unwrap());
                header.push_str(&space);
            }
        }
        println!("{}", header);

        for (i, board_row) in self.board.iter().enumerate(){
            let mut row:String = format!("{:<1$}", i.to_string(), y_digits+3);
            for val in board_row.iter(){

                match val{
                    BoardPiece::Empty=>row.push('.'),
                    BoardPiece::Star=>row.push('*'),
                    BoardPiece::Player=>row.push('P'),
                    }
                    row.push_str(&space); 
                }

            println!("{}", row)
        }
    }
}