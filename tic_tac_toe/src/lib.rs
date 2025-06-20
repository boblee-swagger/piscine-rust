
pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if diagonals('O', table) || horizontal('O', table) || vertical('O', table){
        "player O won".to_string()
    }else if diagonals('X', table) || horizontal('X', table) || vertical('X', table){
        "player X won".to_string()
    }else{
        "tie".to_string()
    }
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    let main_diagonal_win = (0..3).all(|i| table[i][i] == player);
    let anti_diagonal_win = (0..3).all(|i| table[i][2-i] == player);
    main_diagonal_win || anti_diagonal_win
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    table.iter().any(|row| row.iter().all(|cell| *cell == player))
}



pub fn vertical(player : char, table: [[char; 3]; 3]) ->  bool {
    (0..3).any(|i| (0..3).all(|j| table[j][i] == player))
}

