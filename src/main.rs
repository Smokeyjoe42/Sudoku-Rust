use rand::prelude::SliceRandom;

fn main() {
    let mut sudoku: [[i8; 9]; 9] = [[-1;9];9];
    build_board_rec(&mut sudoku, 0);
    build_puzzle(&mut sudoku, 20);

    //build_board(&mut sudoku);
    print_board(sudoku);
}

//Should create a unique sudoku puzzle
fn build_puzzle(bd: &mut [[i8; 9]; 9], num_of_blanks: i32){
    let mut local_bd = *bd;

    //make a random list of potential indicies to remove
    let mut indexes: Vec<usize> = (0..81).collect();
    indexes.shuffle(&mut rand::thread_rng());

    //controls the loop
    let mut counter: usize = 2;
    local_bd[(indexes[0]/9) as usize][(indexes[0]%9) as usize] = 0;
    local_bd[(indexes[1]/9) as usize][(indexes[1]%9) as usize] = 0;

    while (counter as i32) < num_of_blanks {
        let i = indexes[counter];
        let og_number = local_bd[i/9][i%9];
        local_bd[i/9][i%9] = 0;

        //check_uniqueness
        //if true then we can take out more 
        if check_uniqueness(&mut local_bd){
            counter += 1;
        }
        else{
            local_bd[i/9][i%9]= og_number;
        }
    }
    *bd = local_bd;
}

fn check_uniqueness(bd: &mut [[i8; 9]; 9]) -> bool {

    let mut local_bd = *bd;

    let mut x = 0;
    let mut y = 0;
    
    //first find an expression to test
    'outer: for row in 0..9{
        for col in 0..9{
            if bd[row][col] == 0{
                x = row;
                y = col;
                break 'outer;
            }

            if row == 8 && col == 8{
                return true;
            }
        }
    }

    //println!("test {}, {}", x, y);
    //print_board(local_bd);

    let tried_numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut count = 0;

    for i in tried_numbers{
        if check_box(local_bd, i, x as i8, y as i8) &&
        check_rows_cols(local_bd, i, x as i8, y as i8){
            local_bd[x][y] = i;
            if check_uniqueness(&mut local_bd){
                count += 1;
            }
        }
    }
    if count > 1 || count == 0{
        return false;
    }
    return true;
}



//Backtracking
//Builds a valid sudoku board recursivley
fn build_board_rec(bd: &mut [[i8; 9]; 9], start_index: i8) -> bool{

    if start_index == 81 {
    // If we reached the end of the puzzle, we have filled it completely.
    return true;
    }

    let mut tried_numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    tried_numbers.shuffle(&mut rand::thread_rng());

    //loop through all potential numbers
    if start_index < 81{
        for &i in &tried_numbers{

            if check_box(*bd, i, start_index/9, start_index%9)
            && check_rows_cols(*bd, i, start_index/9, start_index%9){

                bd[(start_index/9) as usize][(start_index%9) as usize] = i;
                if build_board_rec(bd, start_index + 1) == true{
                    return true
                }
            }
        }   
    }

    bd[(start_index/9) as usize][(start_index%9) as usize] = -1;
    return false;
}

fn check_rows_cols(bd: [[i8; 9]; 9], new_int: i8, x: i8, y: i8) -> bool{
    for col in 0..9{
        if bd[x as usize][col] == new_int{
            return false;
        }
    }
    for row in 0..9{
        if bd[row][y as usize] == new_int{
            return false;
        }
    }
    return true;
}

//returns true if the placement is valid within a box
fn check_box(bd: [[i8; 9]; 9], new_int: i8, x: i8, y: i8) -> bool{
    //loops through all elements in the box and checks for uniqueness
    //TODO I can do this inplace
    let start_x = (x/3) * 3;
    let start_y = (y/3) * 3;
    for row in start_x..=(start_x + 2){
        for col in start_y..=(start_y + 2){
            if bd[row as usize][col as usize] == new_int {
                return false;
            }
        }
    }
    return true;
}

fn print_board(bd: [[i8; 9];9]){
    for row in 0..9{
        if row % 3 == 0 && row != 0 && row != 9{
            print!(" ");
            for _i in 0..25 {print!("-");}
            print!("\n");
        }
        for col in 0..9{

            if col % 3 == 0 && col != 9{
                print!(" |");
            }

            print!(" {}", bd[row][col]);
        }
        print!(" |");
        print!("\n");
    }
}