use std::io;

//Stałe, tylko dla wygody

const INF: i8 = 2;
const NEGINF: i8 = -2;

const PERSON: i8 = -1;
const COMPUTER: i8 = 1;

//Zwraca listę z koordynatami pustych pól

fn empty_cells(board: &[Vec<i8>]) -> Vec<Vec<usize>>{
    let mut cells: Vec<Vec<usize>> = Vec::new();

    for (x, row) in board.iter().enumerate() {
        for (y, &cell) in row.iter().enumerate(){
            if cell == 0{
                cells.push(vec![x, y]);
            }
        }
    }
    cells
}

//Sprawdza, czy można wykonać ruch, dokładniej sprawdza czy koordynaty ruchu są puste

fn valid_move(x: usize, y: usize, board: &[Vec<i8>]) -> bool{
    empty_cells(board).contains(&vec![x, y])
}

//Funkcja wywoływana by przyjąć i zapisać ruch użytkownika

fn get_player_move(board: &mut Vec<Vec<i8>>){
    let mut res = String::new();
    io::stdin().read_line(&mut res).expect("Failed reading input.");
    let cords: Vec<usize> = res.split_whitespace().map(|x| {x.parse::<usize>().expect("Failed parsing at split.")-1}).collect();

    if valid_move(cords[1], cords[0], &board){
        board[cords[1]][cords[0]] = PERSON;
    } else {
        println!("You cannot make that move.");
        get_player_move(board);
    }
}   

//Rysowanie planszy

fn draw_board(board: &[Vec<i8>]){
    for n in 0..board.len(){
        for m in 0..board.len(){
            match board[n][m] {
                0 => print!(" - "),
                -1 => print!(" X "),
                _ => print!(" O ")
            }
        }
        println!();
    }
}

//Funkcja sprawdza czy podany gracz wygrał

fn wins(board: &[Vec<i8>], player: i8) -> bool{
    let len = board.len();

    fn win_line(line: &[i8], sign: i8, len: usize) -> bool{
        line[0..len].iter().all(|&x| x == sign)
    }

    fn win_diagonal(board: &[Vec<i8>], sign: i8, len: usize) -> bool{
        let mut diag = [0i8; 100];
        diag[..len].copy_from_slice(&board[0]);
        let mut transposed_diag = [0i8; 100];
        transposed_diag[..len].copy_from_slice(&board[0]);

        for n in 0..len{
            diag[n] = board[n][n];
            transposed_diag[n] = board[n][len-n-1];
        }

        win_line(&diag, sign, len) || win_line(&transposed_diag, sign, len)
    
    }

    let mut transposed_board = vec![vec![0i8; len]; len];

    for i in 0..len{
        for j in 0..len{
            transposed_board[i][j] = board[j][i];
        }
    }

    board.iter().any(|b| win_line(&b, player, len)) || transposed_board.iter().any(|b| win_line(&b, player, len)) || win_diagonal(&board, player, len) || win_diagonal(&transposed_board, player, len)

}

//Funkcja zwracająca gracza, który wygrał lub 0 jeśli nikt nie wygrał

fn evaluate(board: &[Vec<i8>]) -> i8 {
    if wins(&board, COMPUTER){
        1
    } else if wins(&board, PERSON){
       -1
    } else {
        0
    }
}

//Struktura ułatwiająca zapisanie funkcji, używana tylko jako typ zwrotny funkcji minimax

#[derive(Copy, Clone)]
struct Best (usize, usize, i8);

fn minimax(board: &mut Vec<Vec<i8>>, depth: u8, player: i8) -> Best{
    //Inicjalizacja wewnętrzych zmiennych (a i b na podstawie argumentów)
    let mut best: Best;
    //Dobranie wartości nieskończoności do gracza, komputer maksymalizuje więc otrzymuje -nieskończoność
    match player {
        COMPUTER => best = Best(0, 0, NEGINF),
        _ => best = Best(0, 0, INF)
    }

    //Sprawdzenie czy ktokolwiek wygrał i odpowiednia zmiana wartości Best
    if wins(board, COMPUTER) || wins(board, PERSON) || depth == 0{
        let score = evaluate(&board);
        return Best(0, 0, score);
    }

    for cell in empty_cells(&board).iter(){
        let (x, y) = (cell[0], cell[1]);
        board[x][y] = player;
        let mut score = minimax(board, depth-1, -player);
        board[x ][y] = 0;
        score.0 = x;
        score.1 = y;
        
        //Scenariusz dla komputera, maksymalizacja
        if (player == COMPUTER && score.2 > best.2) || (player == PERSON && score.2 < best.2){
                best = score;
                if best.2 == player{
                    return best;
                }
        }
    }

    best
}

fn main(){
    
    //Pętla główna
    loop {       
        //Zaczyna człowiek
        let mut current_player = PERSON;

        //Przyjęcie rozmiaru planszy
        let mut board_size = String::new();

        println!("Enter the size of the board.");

        io::stdin().read_line(&mut board_size).expect("Failed reading input");

        //Konwersja rozmiaru planszy na typ liczbowy z obsługą wyjątków
        let board_size: usize = match board_size.trim().parse(){
            Ok(n) => n,
            Err(err) =>{println!("Failed parsing the size of the board. {}", err); continue}
        };

        //Inicjalizacja planszy
        let mut board = vec![vec![0i8; board_size]; board_size];

        println!("You are X and go first.");

        let mut playing = true;

        //Pętla rozgrywki
        while playing{
            if current_player == PERSON{
                //Rysuje planszę po czym pobiera od użytkownika ruch              
                draw_board(&board);
                get_player_move(&mut board);
                
                //Ilość wolnych pól
                let depth = empty_cells(&board).len();

                //Sprawdzanie stanu gry
                if wins(&board, PERSON) {
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
                
            } else if current_player == COMPUTER {

                //Ilość wolnych pól
                let mut depth = empty_cells(&board).len();

                //Oblicza najlepszy ruch z pomocą minimax i umieszcza znacznik komputera na otrzymanych koordynatach
                let computer_move = minimax(&mut board, depth as u8, COMPUTER);
                board[computer_move.0 as usize][computer_move.1 as usize] = COMPUTER;

                //Dekrementacja na rzecz poprawnej obsługi remisu
                depth -= 1;

                //Sprawdzanie stanu gry
                if wins(&board, COMPUTER){
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

        //Obsługa ponownej rozgrywki
        println!("Do you want to play again?");

        let mut ans = String::new();

        //Jeśli odpowiedź nie zaczyna się na "Y" lub "y" program przerywa główną pętlę 
        io::stdin().read_line(&mut ans).expect("Failed to read in your answer.");
        if !ans.to_lowercase().starts_with('y'){
            break;
        }
    }
}