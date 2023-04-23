use rand::Rng;
use crate::dictionary::{pick_word};

//assembles the poem by returning a random sequence of lines 
pub fn assemble_poem() -> String {
    let lines = rand::thread_rng().gen_range(3..7);

    let mut output = String::new();
    //writes [lines] lines of poetry to output
    let mut i = 0;
    while i <= lines {
        //adds a line from finite_state_machine() to the end of output
        let line = finite_state_machine();
        output.push_str(&line);
        output.push_str("\n");
        i += 1;
    }

    return output;
}

//returns a single line of the poem
fn finite_state_machine() -> String {
    //creates a State object to build the finite state machine.
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

    //sets important variables for the finite state machine: word count, current and past state, and the line itself.
    let mut word_count = 0;
    let mut state = State::StartS;
    let mut prev_state = State::StartS;
    let mut line = String::from(" ");

    //starting at StartS, adds words based on the current state until Done_S is reached or the line has 10 words.
    loop {
        //At each state case, picks a random number.  Based off that number, current_state is set equal to one of a few possible states based on what could grammatically follow the current state.
        match state{
            State::StartS => {
                let case = rand::thread_rng().gen_range(1..=5);
                //depending on a random number, enters one of five possible states
                if case == 1 {
                    state = State::ArticleS;

                } 
                else if case == 2{
                    state = State::PronounS;

                }
                else if case == 3{
                    state = State::NounS;

                }
                else if case == 4{
                    state = State::AdjecS;

                }
                else if case == 5{
                    state = State::ExclaS;

                }

            }
            State::ArticleS => {
                let case = rand::thread_rng().gen_range(1..=2);
                line += &pick_word("articles");

                if case == 1{
                    state = State::AdjecS; 
                }
                else if case == 2{
                    state = State::NounS;
                } 

            }
            State::PronounS => {
                let case = rand::thread_rng().gen_range(1..=2);
                line += &pick_word("pronouns");

                if case == 1{
                    state = State::VerbS;

                }
                else if case == 2{
                    state = State::ConjS;

                }

            }
            State::NounS => {
                let case = rand::thread_rng().gen_range(1..=4);
                line += &pick_word("nouns");

                if case == 1{
                    state = State::VerbS;
        
                }
                else if case == 2{
                    state = State::ConjS;   

                }
                else if case == 3{
                    state = State::AdverbS;

                } 
                else if case == 4{
                    state = State::DoneS;
                        
                }

            }
            State::PrepS => {
                let case = rand::thread_rng().gen_range(1..=3);
                line += &pick_word("prepositions");

                if case == 1{
                    state = State::ArticleS;
                    
                } 
                else if case == 2{
                    state = State::PronounS;

                } 
                else if case == 3{
                    state = State::NounS;

                } 

            }
            State::AdjecS => {
                let case = rand::thread_rng().gen_range(1..=5);
                line += &pick_word("adjectives");

                    if case == 1{
                        state = State::AdjecS;
                            
                    }
                    else if case >= 2{
                        state = State::NounS;
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
                            
                        }
                        else if case == 2{
                            state = State::PrepS;
        
                        }
                        else if case == 3{
                            state = State::DoneS;
                        
                        } 
                    }
                    //if previous state is not a verb
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
                    
                }
                else if case == 2{
                    state = State::PronounS;

                }

            }
            State::VerbS => {
                let case = rand::thread_rng().gen_range(1..=4);
                line += &pick_word("verbs");

                if case == 1{
                    state = State::ArticleS;
                    
                } 
                else if case == 2{
                    state = State::PronounS;

                }
                else if case == 3{
                    state = State::NounS;

                }
                else if case == 4{
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
        //at the end of the loop, the previous state is set to the soon to be spent current state.
        prev_state = state;

    }
    line

}