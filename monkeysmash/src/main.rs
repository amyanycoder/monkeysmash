use rand::Rng;
use std::env;
use crate::dictionary::{pick_word, build_database};

mod dictionary;
mod parser;


fn main() {
    let args: Vec<String> = env::args().collect();
    let flag = &args[1];

    
    match flag.as_str() {     
        "b" => build_database(),
        "r" => assemble_poem(),
        "h" | "" => println!("help"), 
        _ => println!("else")

    }
    
}

//assembles the poem by printing a random number of lines
fn assemble_poem() {
    let lines = rand::thread_rng().gen_range(3..7);

    //makes poems of [lines] number of lines
    let mut i = 0;
    while i <= lines{
        println!("{}", finite_state_machine());
        i += 1;
    }

    println!("{}", pick_word("nouns"));

}

//returns a single line of the poem
fn finite_state_machine() -> String {
    //creates an object State to build the finite state machine.
    enum State {
        StartS,
        ArticleS,
        PronounS,
        NounS,
        PrepS,
        AdjecS,
        IndicS,
        AdverbS,
        FutureS,
        TransS,
        ConjS,
        DoneS   
    }

    let mut state = State::StartS;

    let mut line = String::new();

    //starting at start_s, moves to a state based off of its current state, finishing at Done_S.
    loop {
        match state{
            State::StartS => {
                let case = rand::thread_rng().gen_range(1..=4);

                if case == 1{
                    state = State::ArticleS;
                } else if case == 2{
                    state = State::PronounS;
                } else if case == 3{
                    state = State::NounS;
                } else if case == 4{
                    state = State::AdjecS;
                }
            }
            State::ArticleS => {
                let case = rand::thread_rng().gen_range(1..=2);
                line += &pick_word("articles");

                if case == 1{
                    state = State::AdjecS; 
                } else if case == 2{
                    state = State::NounS;
                } 
            }
            State::PronounS => {
                let case = rand::thread_rng().gen_range(1..=3);
                line += &pick_word("pronouns");

                if case == 1{
                    state = State::IndicS;
                    
                } else if case == 2{
                    state = State::TransS;

                } else if case == 3{
                    state = State::ConjS;

                } 
            }
            State::NounS => {
                let case = rand::thread_rng().gen_range(1..=6);
                line += &pick_word("nouns");

                if case == 1{
                    state = State::IndicS;
                    
                } else if case == 2{
                    state = State::TransS;

                } else if case == 3{
                    state = State::ConjS;
                    
                }
                else if case == 4{
                    state = State::AdverbS;

                } else if case == 5{
                    state = State::FutureS;
                
                } 
                else if case == 6{
                    state = State::DoneS;
                
                }
            }
            State::PrepS => {
                let case = rand::thread_rng().gen_range(1..=3);
                line += &pick_word("prepositions");

                if case == 1{
                    state = State::ArticleS;
                    
                } else if case == 2{
                    state = State::PronounS;

                } else if case == 3{
                    state = State::NounS;

                } 
            }
            State::AdjecS => {
                let case = rand::thread_rng().gen_range(1..=2);
                line += &pick_word("adjectives");

                if case == 1{
                    state = State::AdjecS;
                    
                } else if case == 2{
                    state = State::NounS;
                } 
            }
            State::IndicS => {
                let case = rand::thread_rng().gen_range(1..=3);
                line += &pick_word("indicative_verbs");

                if case == 1{
                    state = State::AdverbS;
                    
                } else if case == 2{
                    state = State::ConjS;

                } else if case == 3{
                    state = State::DoneS;

                } 
            }
            State::AdverbS => {
                let case = rand::thread_rng().gen_range(1..=5);
                line += &pick_word("adverbs");

                if case == 1{
                    state = State::ConjS;
                    
                } else if case == 2{
                    state = State::PrepS;

                } else if case == 3{
                    state = State::TransS;
                    
                }
                else if case == 4{
                    state = State::IndicS;

                } else if case == 5{
                    state = State::DoneS;
                
                } 
            }
            State::FutureS => {
                let case = rand::thread_rng().gen_range(1..=2);
                line += &pick_word("future");

                if case == 1{
                    state = State::IndicS;
                    
                } else if case == 2{
                    state = State::TransS;
                } 
            }
            State::ConjS => {
                let case = rand::thread_rng().gen_range(1..=2);
                line += &pick_word("conjunctions");

                if case == 1{
                    state = State::NounS;
                    
                } else if case == 2{
                    state = State::PronounS;
                } 
            }
            State::TransS => {
                let case = rand::thread_rng().gen_range(1..=3);
                line += &pick_word("transitive_verbs");

                if case == 1{
                    state = State::ArticleS;
                    
                } else if case == 2{
                    state = State::PronounS;
                } else if case == 3{
                    state = State::NounS;
                }
            }
            State::DoneS => {
                break;
            }

        }
        //adds a space after the word
        line += " ";
    }
    line

}