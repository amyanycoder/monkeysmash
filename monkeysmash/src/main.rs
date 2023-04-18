use rand::Rng;
use std::env;
use crate::dictionary::{pick_word, pick_vowel_word, pick_consonant_word, build_database};

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
    #[derive(Copy, Clone)]
    enum State {
        StartS,
        ArticleS,
        PronounS,
        NounS,
        PrepS,
        AdjecS,
        VerbS,
        AdverbS,
        ModalS,
        ConjS,
        ExclaS,
        DoneS   
    }

    let mut word_count = 0;

    let mut state = State::StartS;
    let mut prev_state = State::StartS;

    let mut line = String::new();

    //starting at start_s, moves to a state based off of its current state, finishing at Done_S.
    loop {
        match state{
            State::StartS => {
                let case = rand::thread_rng().gen_range(1..=5);

                if case == 1 {
                    state = State::ArticleS;
                } else if case == 2{
                    state = State::PronounS;
                } else if case == 3{
                    state = State::NounS;
                } else if case == 4{
                    state = State::AdjecS;
                } else if case == 5{
                    state = State::ExclaS;
                } else if case == 6{
                    state = State::DoneS;
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
                let case = rand::thread_rng().gen_range(1..=2);
                line += &pick_word("pronouns");

                if case == 1{
                    state = State::VerbS;

                } else if case == 2{
                    state = State::ConjS;

                } 
            }
            State::NounS => {
                let case = rand::thread_rng().gen_range(1..=4);

                match prev_state {
                    State::ArticleS => {
                        if line.ends_with("an "){
                            println!("next is vowel");
                            line += &pick_vowel_word("nouns");
                        }
                        else if line.ends_with("a "){
                            println!("next is consonant");
                            line += &pick_consonant_word("nouns");
                        }
                        else{
                            println!("no vowel or consonant");
                            line += &pick_word("nouns");
                        }

                        state = State::VerbS;
                    }
                    _ => {
                        if case == 1{
                            state = State::VerbS;
                            line += &pick_word("nouns");

        
                        } else if case == 2{
                            state = State::ConjS;
                            line += &pick_word("nouns");

                            
                        }
                        else if case == 3{
                            state = State::AdverbS;
                            line += &pick_word("nouns");

        
                        } 
                        else if case == 4{
                            state = State::DoneS;
                            line += &pick_word("nouns");

                        
                        }

                    }

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
                let case = rand::thread_rng().gen_range(1..=5);

                match prev_state {
                    State::ArticleS => {
                        if line.ends_with("an "){
                            println!("next is vowel");
                            line += &pick_vowel_word("adjectives");
                        }
                        else if line.ends_with("a "){
                            println!("next is consonant");
                            line += &pick_consonant_word("adjectives");
                        }
                        else{
                            println!("no vowel or consonant");
                            line += &pick_word("adjectives");
                        }
                        state = State::NounS;
                    }
                    _ => {
                        if case == 1{
                            state = State::AdjecS;
                            
                        } else if case >= 2{
                            state = State::NounS;
                        } 

                        line += &pick_word("adjectives");

                    }
                }

            }
            State::AdverbS => {
                let case = rand::thread_rng().gen_range(1..=3);
                line += &pick_word("adverbs");

                //moves to the verb state if this adverb is not following a verb.  moves to another state otherwise
                match prev_state {
                    State::VerbS => {
                        if case == 1{
                            state = State::ConjS;
                            
                        } else if case == 2{
                            state = State::PrepS;
        
                        }
                        else if case == 3{
                            state = State::DoneS;
                        
                        } 
                    }
                    _ => {
                        state = State::VerbS;
                    }
                }
                
            }
            State::ModalS => {
                line += &pick_word("modal_verbs");

                state = State::VerbS;
                    

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
            State::VerbS => {
                let case = rand::thread_rng().gen_range(1..=4);
                line += &pick_word("verbs");

                if case == 1{
                    state = State::ArticleS;
                    
                } else if case == 2{
                    state = State::PronounS;
                } else if case == 3{
                    state = State::NounS;
                } else if case == 4{
                    state = State::ModalS;
                }
            }
            State::ExclaS => {
                let case = rand::thread_rng().gen_range(1..=2);
                line += &pick_word("exclamations");

                if case == 1{
                    state = State::ArticleS;
                }
                else if case == 2{
                    state = State::DoneS;
                }
            }
            State::DoneS => {
                break;
            }

        }
        
        word_count += 1;

        //ends the line if it has 10 or more words
        if word_count >= 10{
            match state {
                State::ArticleS => {
                    line += &pick_word("nouns");
                }
                State::ConjS => {
                    line += &pick_word("nouns");
                }
                State::ModalS => {
                    line += &pick_word("verbs");
                }
                State::PrepS => {
                    line += &pick_word("nouns");
                }
                State::AdjecS => {
                    line += &pick_word("nouns");
                }
                _ => {

                }               
            }
            
            break;
        }

        prev_state = state;

    }
    line

}