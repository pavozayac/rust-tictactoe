use std::io;

const INF: f32 = std::f32::INFINITY;
const NEGINF: f32 = std::f32::NEG_INFINITY; 

const PERSON: i8 = -1;
const COMPUTER: i8 = 1;

fn empty_cells(board: &Vec<Vec<i8>>) -> Vec<Vec<i8>>{
    let mut cells: Vec<Vec<i8>> = Vec::new();

    for (x, row) in board.iter().enumerate() {
        for (y, &cell) in row.iter().enumerate(){
            if cell == 0{
                cells.push(vec![x as i8, y as i8]);
            }
        }
    }
    cells
}

fn valid_move(x: &i8, y: &i8, board: &Vec<Vec<i8>>) -> bool{
    if empty_cells(board).contains(&vec![*x, *y]) {
        true
    } else {
        false
    }
}

fn get_player_move(board: &mut Vec<Vec<i8>>){
    let mut res = String::new();
    io::stdin().read_line(&mut res).expect("Failed reading input.");
    let cords: Vec<i8> = res.split_whitespace().map(|x| {x.parse::<i8>().expect("Failed parsing at split.")-1}).collect();

    if valid_move(&cords[1], &cords[0], &board){
        board[cords[1] as usize][cords[0] as usize] = PERSON;
    } else {
        println!("You cannot make that move.");
        get_player_move(board);
    }
}   

fn draw_board(board: &Vec<Vec<i8>>){
    for n in 0..board.len(){
        for m in 0..board.len(){
            match board[n][m] {
                0 => print!(" - "),
                -1 => print!(" X "),
                _ => print!(" O ")
            }
        }
        print!("\n");
    }
}

fn wins(board: &Vec<Vec<i8>>, player: &i8) -> bool{
    fn win_line(line: &Vec<i8>, sign: &i8) -> bool{
        let mut counter: usize = 0;
        for n in line{
            if n == sign{
                counter += 1;
            }
        }
        if counter == line.len(){
            true
        } else {
            false
        }
    }

    fn win_diagonal(board: &Vec<Vec<i8>>, sign: &i8) -> bool{
        let mut diag: Vec<i8> = Vec::new();
        let mut transposed_diag: Vec<i8> = Vec::new();

        for n in 0..board.len(){
            diag.push(board[n][n]);
            transposed_diag.push(board[n][board.len()-n-1]);
        }

        if win_line(&diag, &sign){
            true
        } else if win_line(&transposed_diag, &sign){
            true
        } else {
            false
        }
    }

    let mut transposed_board = vec![vec![0i8; board.len()]; board.len()];
    
    for i in 0..board.len(){
        for j in 0..board.len(){
            transposed_board[i][j] = board[j][i];
        }
    }

    if board.iter().any(|b| win_line(b, &player)){
        true
    } else if transposed_board.iter().any(|b| win_line(b, &player)){
        true
    } else if win_diagonal(&board, &player){
        true
    } else {
        false
    }
}

fn evaluate(board: &Vec<Vec<i8>>) -> i8 {
    if wins(&board, &COMPUTER){
        1
    } else if wins(&board, &PERSON){
       -1
    } else {
        0
    }
}

#[derive(Copy, Clone)]
struct Best (i32,i32,f32);

fn minimax(board: &mut Vec<Vec<i8>>, depth: u8, player: &i8, alpha: f32, beta: f32) -> Best{
    let mut best: Best;
    let mut a = alpha;
    let mut b = beta;
    match player {
        &COMPUTER => best = Best(-1, -1, NEGINF),
        _ => best = Best(-1, -1, INF)
    }

    if wins(&board, &COMPUTER) || wins(&board, &PERSON) || depth == 0{
        let score = evaluate(&board) as f32;
        return Best(-1, -1, score);
    }

    for cell in empty_cells(&board).iter(){
        let (x, y) = (cell[0], cell[1]);
        board[x as usize][y as usize] = *player;
        let mut score = minimax(board, depth-1, &-player, alpha, beta);
        board[x as usize][y as usize] = 0;
        score.0 = x as i32;
        score.1 = y as i32;

        if player == &COMPUTER{
            if score.2 > best.2{
                best = score;
            }
            a = a.max(score.2);
            if a >= b{
                break;
            }
        } else {
            if score.2 < best.2{
                best = score;
            }
            b = b.max(score.2);
            if a >= b{
                break;
            }
        }

    }

    best
}

fn main(){
    

    let checkboard: Vec<Vec<i8>> = vec![vec![-1,0,0], vec![0,-1,0], vec![0,0,-1]];

    if wins(&checkboard, &PERSON){
        println!("Check success");
    } else {
        println!("Check failed")
    }
    
    loop {       
        let mut current_player = PERSON;

        let mut board_size = String::new();

        println!("Enter the size of the board.");

        io::stdin().read_line(&mut board_size).expect("Failed reading input");

        let board_size: i32 = match board_size.trim().parse(){
            Ok(n) => n,
            Err(err) =>{println!("Failed parsing the size of the board. {}", err); continue}
        };

        let mut board = vec![vec![0i8; board_size as usize]; board_size as usize];

        println!("You are X and go first.");

        let mut playing = true;

        while playing{
            if current_player == -1{
                

                draw_board(&board);
                get_player_move(&mut board);

                let depth = empty_cells(&board).len();

                let does_win = wins(&board, &PERSON);

                if does_win{
                    draw_board(&board);
                    println!("You won!");
                    playing = false;

                } else if depth == 0 {
                    draw_board(&board);
                    println!("Tie!");
                    playing = false;
                } else {
                    current_player = 1;
                }
                
            } else if current_player == 1 {
                let mut depth = empty_cells(&board).len();

                let computer_move = minimax(&mut board, depth as u8, &COMPUTER, NEGINF, INF);
                board[computer_move.0 as usize][computer_move.1 as usize] = COMPUTER;
                
                depth = empty_cells(&board).len();

                if wins(&board, &COMPUTER){
                    draw_board(&board);
                    println!("The computer won!");
                    playing = false;
                } else if depth == 0{
                    draw_board(&board);
                    println!("Tie!");
                    playing = false;
                } else {
                    current_player = -1;
                }
            }
        }

        println!("Do you want to play again?");

        let mut ans = String::new();

        io::stdin().read_line(&mut ans).expect("Failed to read in your answer.");
        if !ans.to_lowercase().starts_with("y"){
            break;
        }
    }
}