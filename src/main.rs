extern crate termcolor;

use std::io::Write;
use termcolor::{ Color, ColorChoice, ColorSpec, StandardStream, WriteColor };

fn welcome() {
    println!(
        "\nRust TicTacToe\n\
         --------------\n\
         Code is available at: https://github.com/lilyanB/rust-tictactoe"
    )
}

fn print_player(player: char) {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    match player {
        'X' => stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue))).unwrap(),
        'O' => stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap(),
        _ => (),
    }

    write!(&mut stdout, "{}", player).unwrap();
    stdout.reset().unwrap();
}

fn display_grid(state: &[char]) {
    for i in 0..3 {
        let offset = i * 3;

        print!("-------------\n| ");
        print_player(state[offset]);
        print!(" | ");
        print_player(state[offset + 1]);
        print!(" | ");
        print_player(state[offset + 2]);
        println!(" |");
    }

    println!("-------------");
}

fn user_plays(state: &mut [char], player: char) {
    loop {
        print!("Player '");
        print_player(player);
        println!("', enter a number: ");

        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            println!("Couldn't read line! Try again.");
            continue;
        }

        if let Ok(number) = input.trim().parse::<usize>() {
            if number < 1 || number > 9 {
                println!("The field number must be between 1 and 9.");
                continue;
            }

            let number = number - 1;

            if state[number] == 'X' || state[number] == 'O' {
                print!("This field is already taken by '");
                print_player(state[number]);
                println!("'.");
                continue;
            }

            state[number] = player;

            break;
        } else {
            println!("Only numbers are allowed.");
            continue;
        }
    }
}

fn check_lines(state: &[char], player: char) -> bool {
    for row in state.chunks(3) {
        if row.iter().all(|&v| v == player) {
            display_grid(&state);
            println!("One line is finish !");
            return true;
        }
    }
    false
}

fn check_columns(state: &[char], player: char) -> bool {
    for col in 0..3 {
        if (0..3).all(|row| state[row * 3 + col] == player) {
            display_grid(&state);
            println!("One column is finish !");
            return true;
        }
    }
    false
}

fn check_diagonals(state: &[char], player: char) -> bool {
    if (0..3).all(|i| state[i * 3 + i] == player) {
        display_grid(&state);
        println!("One diagonal is finish !");
        return true;
    }
    if (0..3).all(|i| state[i * 3 + (3 - 1 - i)] == player) {
        display_grid(&state);
        println!("One diagonal is finish !");
        return true;
    }
    false
}

fn has_won(state: &[char], player: char) -> bool {
    check_lines(state, player) || check_columns(state, player) || check_diagonals(state, player)
}

fn is_finish(state: &[char]) -> bool {
    state.iter().all(|&v| (v == 'X' || v == 'O'))
}

fn main() {
    let mut state = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut player = 'X';

    welcome();

    loop {
        // display_grid the field
        display_grid(&state);

        // Ask for user input
        user_plays(&mut state, player);

        // Check if a player won
        if has_won(&state, player) {
            print!("Player '");
            print_player(player);
            println!("' won! \\(^.^)/");
            break;
        }

        // Check if all fields are used
        if is_finish(&state) {
            display_grid(&state);
            println!("All fields are used. No one won.");
            break;
        }

        // Switch player
        player = if player == 'X' { 'O' } else { 'X' };
    }
}
