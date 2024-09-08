/*
The 9 cells are annotated like so:
- - -     0 1 2
- - -     3 4 5
- - -     6 7 8

- = Unused cell
X = X's cell
O = O's cell
-------------------------------
For every cell we have a list of possible combos. When a player 
plays a cell, we iterate over the list of possible combos and see 
if any of those combos is satisfied. For example, if it's Player 1's 
turn, and they play cell 0, we check for the following combos:
X X X   X - -   X - -
- - -   - X -   X - -
- - -   - - X   X - -
If any of these combos is satisfied, Player 1 wins. Depending on 
whose turn it is, we'll either be looking for X's or O's.
*/
use std::io;

#[derive(Clone, Copy)]
enum CellState { Unused,X,O }

impl CellState {
    fn as_str(&self) -> &str {
        match self {
            CellState::Unused => return "-",
            CellState::X => return "X",
            CellState::O => return "O"
        }
    }
}

struct BoardData {
    cells: [CellData;9]
}

impl BoardData {
    // For debugging purposes
    fn Dbg_Print_Cell_States(&self) {
        let mut s = String::new();
        for i in self.cells.iter() {
            s.push_str(i.cell_state.as_str());
            s.push_str(",")
        }
        println!("{s}");
    }
}

#[derive(Clone)]
struct CellData {
    combos: Vec<[i32;3]>,
    cell_state: CellState
    
}

impl Default for BoardData {
    fn default() -> Self {
        BoardData {
            cells:[
                CellData {combos: vec![[0,1,2],[0,4,8],[0,3,6]], cell_state:CellState::Unused},
                CellData {combos: vec![[0,1,2],[0,4,7]], cell_state:CellState::Unused},
                CellData {combos: vec![[2,1,0],[2,4,6],[2,5,8]], cell_state:CellState::Unused},
                CellData {combos: vec![[0,3,6],[3,4,5]], cell_state:CellState::Unused},
                CellData {combos: vec![[1,4,7],[3,4,5]], cell_state:CellState::Unused},
                CellData {combos: vec![[2,5,8],[5,4,3]], cell_state:CellState::Unused},
                CellData {combos: vec![[6,3,0],[6,4,2],[6,7,8]], cell_state:CellState::Unused},
                CellData {combos: vec![[6,7,8],[7,4,1]], cell_state:CellState::Unused},
                CellData {combos: vec![[8,5,2],[8,4,0],[8,7,6]], cell_state:CellState::Unused}
            ]
        }
    }
}

struct GameState {
    board_data: BoardData,
    game_running: bool,
    current_turn: i32,   // 1 = Player 1 (X), 2 = Player 2 (O)
    player_choice: usize
}

pub fn game_loop() {

    // Initialize GameState.
    let mut state: GameState = GameState {
        board_data: BoardData {..Default::default()},
        game_running: true, current_turn: 1, player_choice: 0
    };

    init();

    // Main loop. First get user input, then calculate game logic,
    // then render.
    while state.game_running {
        user_input(&mut state);
        game_logic(&mut state);
        render(&mut state);
        // state.game_running = false;
    }
}

// Mainly to print some startup text.
fn init() {
    println!("Tic Tac Toe");
    println!("-----------------");
    println!("Board cells are laid out like so:");
    println!("- - -\t0 1 2");
    println!("- - -\t3 4 5");
    println!("- - -\t6 7 8");
    println!("Player enters cell number to play their turn.");
    println!("-----------------");
}

fn user_input(state: &mut GameState) {
    println!("Player {}'s turn:", state.current_turn);
    let mut choice = String::new();
    io::stdin().read_line(&mut choice);

    // Trim data the try to parse. If parse is successful,
    // set player_choice so game logic can use it.
    match str::parse::<usize>(choice.trim()) {
        Ok(a) => {
            if a >= 0 && a <= 8 { state.player_choice = a }
        },
        Err(a) => { println!("{a}") }
    }
}

fn game_logic(state: &mut GameState) {

    // Clone the cell so we're not fighting with the borrow checker.
    // This is the cell that the current player has chosen to play.
    let mut cell: CellData = 
        state.board_data.cells[state.player_choice as usize].clone();

    // Check if chosen cell is already occupied. If not, we proceed.
    // If occupied, we let user know they have to enter a new choice.
    match cell.cell_state {
        CellState::Unused => {

            // Set cell_state depending on which player has played.
            // Player 1 is X, Player 2 is O.
            cell.cell_state = if state.current_turn == 1 { 
                CellState::X } else { CellState::O };
            
            /*
            This is dumb but it works lol. We need to update the
            board data to include the new cell_state we set above
            so that when we're checking cells later on we're not 
            looking at old data.
             */ 
            state.board_data.cells[
                state.player_choice as usize] = cell;
            cell = state.board_data.cells[
                state.player_choice as usize].clone();
            
            // Flags to determine if a winning trio has been found.
            let mut x_trio_found = false;
            let mut o_trio_found = false;

            // Each cell has a list of possible combos. So we iterate
            // over the list of possible combos and check if any of 
            // them has been satisfied.
            for i in cell.combos.iter() {

                // These are to count the X's and O's in the cells
                // we're currently checking.
                let mut x_count = 0;
                let mut o_count = 0;

                // Each combo has 3 cell, so we check the state of 
                // each of the 3 cells in the combo we're iterating
                // over.
                match state.board_data.cells[i[0] as usize].cell_state {
                    CellState::X => {x_count+=1},
                    CellState::O => {o_count+=1},
                    _ => {}
                }
                match state.board_data.cells[i[1] as usize].cell_state {
                    CellState::X => {x_count+=1},
                    CellState::O => {o_count+=1},
                    _ => {}
                }
                match state.board_data.cells[i[2] as usize].cell_state {
                    CellState::X => {x_count+=1},
                    CellState::O => {o_count+=1},
                    _ => {}
                }

                // println!("X Count:{}, O Count:{}",x_count,o_count);

                // If X or O have a count of 3, we have a winner.
                if x_count == 3 { x_trio_found = true; }
                if o_count == 3 { o_trio_found = true; }
            }

            // If a player has won, we change game_running state 
            // and return so game ends.
            if x_trio_found {
                println!("X wins!");
                state.game_running = false; return;
            }
            else if o_trio_found {
                println!("O wins!");
                state.game_running = false; return;
            }

            // Set current_turn so next player can play.
            state.current_turn = if state.current_turn == 1 {
                2 } else { 1 };

            // Move previously cloned cell back into it's place, so
            // board_data now has updated data.
            state.board_data.cells[state.player_choice as usize] = cell;
            // state.board_data.Dbg_Print_Cell_States();
        }
        _ => {
            // If chosen cell is Unused, it's occupied, so player can't
            // choose that cell. We let the player know, so they can 
            // try again. We don't update current_turn because player
            // has to play their turn again.
            println!("Cell already used. Try another cell.");
        }
    }
    
}

fn render(state: &mut GameState) {

    // Render board
    println!("{} {} {}",
        state.board_data.cells[0].cell_state.as_str(),
        state.board_data.cells[1].cell_state.as_str(),
        state.board_data.cells[2].cell_state.as_str());
    println!("{} {} {}",
        state.board_data.cells[3].cell_state.as_str(),
        state.board_data.cells[4].cell_state.as_str(),
        state.board_data.cells[5].cell_state.as_str());
    println!("{} {} {}",
        state.board_data.cells[6].cell_state.as_str(),
        state.board_data.cells[7].cell_state.as_str(),
        state.board_data.cells[8].cell_state.as_str());
    
    println!("-----------------");
}
