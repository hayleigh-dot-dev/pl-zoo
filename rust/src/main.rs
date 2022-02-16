use colored::*;
// I dont know how to choose the best crate from a bunch of options: saw this one
// in an example so I figured I'd use it too ^.^
//
// `Rng` is a trait. Apparently you need to pull traits into scope before you can
// use functions defined on that trait, which I guess makes sense, but if the
// compiler already knew I needed to pull in `Rng` to ue `.gen_range` why do I
// need to do this?
use rand::Rng;
use std::convert::TryInto;
use std::io;
use std::io::Write;

// For any rustaceans following along. You can play a game and see how long it
// takes before you get annoyed at how things are formatted!

// Whew this is already spookier than just using `String`. What does it mean?
//
// - &          -> OK so `&` is how we indicate something is a _reference_ to a
//                 value of whatever type proceeds this (in this case `str`).
// - 'static    -> This is a *lifetime annotation*. These are used to tell the
//                 compiler where we're borrowing a something _from_. `static`
//                 is a special annotation that means a reference can live for
//                 the entire duration of the program. All string literals have
//                 this lifetime. The compiler doesn't seem to care if I don't
//                 include this, but I figured I'd add it anyway?
// - str        -> `str` is an immutable sequence of utf8 bytes. This separates
//                 it from `String` which is, uh, not that?
// - 19         -> Arrays are fixed-size in Rust, so we need to encode that size
//                 in the type.
//
const WORDS: [&'static str; 19] = [
    "cover", "thick", "kayla", /* 🥰 */ "whack", "trunk", "power", "occur", "racer",
    "price", "smack", "crane", "prism", "pully", "lilac", /* 🥰 */
    "knoll", "goblin", "price", "thank", "tulip",
];

#[derive(Debug)]
enum Guess {
    Missing(String),
    Partial(String),
    Exact(String),
}

type History = Vec<(Guess, Guess, Guess, Guess, Guess)>;

// So it's common for rust modules/crates to alias `std::Result<T, E>` to use
// some specific error type by default. For io that's:
//
//     Result<T, std::io::Error>
//
// I'm not particularly fond of type aliases, but it's what everyone seems to do
// so... so be it.
//
// Incidentally, Rust docs list enums, structs, and type (aliases) all separately
// which feels a bit like leaking implementation details to me?
fn main() -> io::Result<()> {
    // Kayla tells me type annotations aren't really necessary in Rust. She's
    // right of course, a very useful property of HM-derived type systems is the
    // ability for principal type inference.
    //
    // I'm going to leave them in because my brain is smol and I barely hanging
    // on to what's going on as it is.
    let idx: usize = rand::thread_rng().gen_range(0..WORDS.len());
    let target: &str = WORDS[idx];

    let mut guesses: u8 = 0;
    let mut history: History = vec![];

    while guesses < 6 {
        let guess: String = prompt_guess();

        match check_guess(target, guess) {
            [a @ Guess::Exact(_), b @ Guess::Exact(_), c @ Guess::Exact(_), d @ Guess::Exact(_), e @ Guess::Exact(_)] =>
            // rust-fmt dropping this curly brace onto a new line might be the
            // last straw to be perfectly honest. smfh.
            {
                show_full_guess(&a, &b, &c, &d, &e);
                break;
            }

            [a, b, c, d, e] => history.push((a, b, c, d, e)),
        }

        for (a, b, c, d, e) in &history {
            show_full_guess(a, b, c, d, e)
        }

        guesses += 1;
    }

    Ok(()) // <-- Remember when I said semicolons can get fucked? Make sure you
           // remember *not* to put a semicolon after the last expression in a
           // function because if you do, it will mean something else.
           //
           // Rust implicitly returns the last expression in a function. If you
           // add a semicolon there, it'll helpfully assume you want to return
           // `()`.
}

fn prompt_guess() -> String {
    print!("> ");

    // This feels kind of weird, but we need to flush the output stream to ensure
    // the above message actually gets printed before we get user input. Without
    // this, writes are automatically buffered and flushed according to some
    // heuristic or criteria I am not privvy to.
    io::stdout().flush().ok().expect("");

    let mut input: String = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(6) => {
            // The buffer appended by `read_line` includes a newline character,
            // so lets trim that.
            //
            // So you remember that `input : String`. Well double-sike because
            // calling `trim_end()` returns `&str`. Why? Fuck you, that's why.
            input.trim_end().to_string()
        }

        _ => {
            print!("< Make sure your guess has exactly 5 letters!\n\n");
            prompt_guess()
        }
    }
}

fn check_guess(target: &str, input: String) -> [Guess; 5] {
    input
        .chars() // Split input into characters
        .enumerate() // Convert into an iterator that also contains the current index
        .map(|(i, c)| {
            let s: String = c.to_string();
            match target.chars().nth(i) {
                //
                Some(c_) if c == c_ => Guess::Exact(s),

                // So this is kind of a not very good implementation, we'll get
                // false `Partial`s if the user guesses a word like "loops" but
                // the actual word is "royal".
                //
                // There are two 'o's in the guess, and both will be marked as
                // a Partial match! I could do better, but do I want to...
                _ if target.contains(c) => Guess::Partial(s),

                _ => Guess::Missing(s),
            }
        })
        .collect::<Vec<Guess>>()
        // Ok *we* know that the Vec we just collected will always have exactly
        // 5 elements but the compiler, but the compiler can't guarantee that
        // statically. To help it along, we're going to try and convert it to an
        // array, and then unwrap the result because we know an Error is impossible.
        .try_into()
        .unwrap()
}

fn show_full_guess(a: &Guess, b: &Guess, c: &Guess, d: &Guess, e: &Guess) -> () {
    println!(
        "{}{}{}{}{}",
        to_coloured_string(a),
        to_coloured_string(b),
        to_coloured_string(c),
        to_coloured_string(d),
        to_coloured_string(e)
    )
}

fn to_coloured_string(guess: &Guess) -> String {
    match guess {
        Guess::Exact(s) => s.to_owned().green().to_string(),

        Guess::Partial(s) => s.to_owned().yellow().to_string(),

        Guess::Missing(s) => s.to_owned(),
    }
}
