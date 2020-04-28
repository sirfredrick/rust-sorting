extern crate sorting_lib;

use std::io;
use std::io::prelude::*;
use sorting_lib::sorting::*;

/// Prints a menu of command options to the console.
fn print_menu() {
    println!("Sorting Algorithms:");
    println!("A: Add to a Vector");
    println!("C: Clear Vector");
    println!("S: Merge Sort");
    println!("O: Bubble Sort");
    println!("R: Gnome Sort");
    println!("T: Bogo Sort");
    println!("?: Print Menu");
    println!("Q: Quit");
    println!("Please enter a command:");
}

/// Creates a vector by asking for user input until a `Q` is entered.
/// Only values that are integers will be entered other values will be skipped.
/// Takes a mutable reference to a i32 Vector, and a mutable reference to a usize holding the length as parameters.
fn create_vector(vector: &mut Vec<i32>, length: &mut usize) {
    // Ask for input from the user
    println!("Please enter numbers and enter 'Q' to quit: ");
    // Go through the users input until `Q` is entered
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        // If the line is a String
        if !line.is_err() {
            // Unwrap the String
            let line2 = line.unwrap();
            // Create char variable to check for `Q` character
            let char = line2.chars().next().unwrap();
            // If `Q` is entered break from the for loop
            if char == 'Q' || char == 'q' {
                break;
            }
            // Otherwise, parse the String into an i32
            let val = line2.to_string().parse::<i32>();
            // If the parse was successful
            if !val.is_err() {
                // Unwrap and add the i32 to `vector`
                vector.push(val.unwrap());
            }
        }
    }
    *length = vector.len();
}

/// Empties `vector`
/// Takes a mutable reference to a i32 Vector, and a mutable reference to a usize holding the length as parameters.
fn clear_vector(vector: &mut Vec<i32>, length: &mut usize) {
    // Dereference `vector` and set it to a new empty vector
    *vector = vec![];
    // Dereference `length` and set it to 0
    *length = 0;
}

/// Main function that loops through the menu until a `Q` command is entered to the console.
fn main() {
    // Initialize `vector` as empty
    let mut vector: Vec<i32> = vec![];
    let mut length: usize = vector.len();

    // Loop through the menu until `Q` is entered
    loop {
        // Print the list of commands
        print_menu();
        // Create a new String to take in the console input
        let mut input = String::new();
        // Have the String hold the console input
        io::stdin().read_line(&mut input);
        // Setup a variable to help with exiting when `Q` is entered
        let mut should_exit = false;
        // Match the first character of the String and call a function based on the result
        // The sort methods will only be called if `Vector` is not empty
        match input.trim().chars().next().unwrap() {
            'A' | 'a' => create_vector(&mut vector, &mut length),
            'C' | 'c' => clear_vector(&mut vector, &mut length),
            'S' | 's' | _ if { !vector.is_empty() } => { merge_sort(&mut vector, 0, length - 1) },
            'O' | 'o' | _ if { !vector.is_empty() } => { bubble_sort(&mut vector) },
            'R' | 'r' | _ if { !vector.is_empty() } => { gnome_sort(&mut vector) },
            'T' | 't' | _ if { !vector.is_empty() } => { bogo_sort(&mut vector) },
            'Q' | 'q' => should_exit = true,
            _ => println!("Invalid input")
        }
        if !should_exit {
            // Print out `vector` after each iteration
            println!("Vector: {:?}", vector);
        } else {
            // If `Q` was pressed break out of the loop
            break;
        }
    }
}