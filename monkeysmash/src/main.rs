use rand::Rng;
use rand::seq::SliceRandom;

mod dictionary;
mod parser;


fn main() {
    dictionary::clear_dictionary();

    dictionary::create_dictionary();

    //fills database with words
    if let Err(e) = parser::parse_file(){
        eprintln!("{}", e);
    }

    //dictionary::clear_dictionary();

    let lines = rand::thread_rng().gen_range(3..7);
    let mut i = 0;

    while i <= lines{
        println!("{}", finite_state_machine());
        i += 1;
    }


    
}


fn finite_state_machine() -> String {
    let nouns = ["apple", "cow", "freedom", "glory", "hockey", "flower", "lo mein", "hamburger", "Canada", "France", "Pittsburgh"]; 
    let adjectives = ["Canadian", "purple", "green", "massive", "liberating", "topographical"];
    let pronouns = ["he", "she", "it", "they", "I", "we", "you"];
    let articles = ["a", "the"];
    let conjunctions = ["for", "and", "nor", "but", "or", "yet", "so"];
    let prepositions = ["in", "under", "of","towards", "before", "of", "for"];
    let future = ["will", "is going to", "is to", "shall"];
    let adverbs = ["quickly", "slowly", "carefully", "tonight", "yesterday", "here", "often"];
    let indicative_verbs = ["is", "goes", "flies", "does", "eats"];
    let transitive_verbs = ["needs", "has", "takes", "requests", "sees"];

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

                if case == 1{
                    line += adjectives.choose(&mut rand::thread_rng()).unwrap();
                    state = State::AdjecS;
                    
                } else if case == 2{
                    line += nouns.choose(&mut rand::thread_rng()).unwrap();
                    state = State::NounS;
                } 
            }
            State::PronounS => {
                let case = rand::thread_rng().gen_range(1..=3);

                if case == 1{
                    line += indicative_verbs.choose(&mut rand::thread_rng()).unwrap();
                    state = State::IndicS;
                    
                } else if case == 2{
                    line += transitive_verbs.choose(&mut rand::thread_rng()).unwrap();
                    state = State::TransS;

                } else if case == 3{
                    line += conjunctions.choose(&mut rand::thread_rng()).unwrap();
                    state = State::ConjS;

                } 
            }
            State::NounS => {
                let case = rand::thread_rng().gen_range(1..=6);

                if case == 1{
                    line += indicative_verbs.choose(&mut rand::thread_rng()).unwrap();
                    state = State::IndicS;
                    
                } else if case == 2{
                    line += transitive_verbs.choose(&mut rand::thread_rng()).unwrap();
                    state = State::TransS;

                } else if case == 3{
                    line += conjunctions.choose(&mut rand::thread_rng()).unwrap();
                    state = State::ConjS;
                    
                }
                else if case == 4{
                    line += adverbs.choose(&mut rand::thread_rng()).unwrap();
                    state = State::AdverbS;

                } else if case == 5{
                    line += future.choose(&mut rand::thread_rng()).unwrap();
                    state = State::FutureS;
                
                } 
                else if case == 6{
                    state = State::DoneS;
                
                }
            }
            State::PrepS => {
                let case = rand::thread_rng().gen_range(1..=3);

                if case == 1{
                    line += articles.choose(&mut rand::thread_rng()).unwrap();
                    state = State::ArticleS;
                    
                } else if case == 2{
                    line += pronouns.choose(&mut rand::thread_rng()).unwrap();
                    state = State::PronounS;

                } else if case == 3{
                    line += nouns.choose(&mut rand::thread_rng()).unwrap();
                    state = State::NounS;

                } 
            }
            State::AdjecS => {
                let case = rand::thread_rng().gen_range(1..=2);

                if case == 1{
                    line += adjectives.choose(&mut rand::thread_rng()).unwrap();
                    state = State::AdjecS;
                    
                } else if case == 2{
                    line += nouns.choose(&mut rand::thread_rng()).unwrap();
                    state = State::NounS;
                } 
            }
            State::IndicS => {
                let case = rand::thread_rng().gen_range(1..=3);

                if case == 1{
                    line += adverbs.choose(&mut rand::thread_rng()).unwrap();
                    state = State::AdverbS;
                    
                } else if case == 2{
                    line += conjunctions.choose(&mut rand::thread_rng()).unwrap();
                    state = State::ConjS;

                } else if case == 3{
                    state = State::DoneS;

                } 
            }
            State::AdverbS => {
                let case = rand::thread_rng().gen_range(1..=5);

                if case == 1{
                    line += conjunctions.choose(&mut rand::thread_rng()).unwrap();
                    state = State::ConjS;
                    
                } else if case == 2{
                    line += prepositions.choose(&mut rand::thread_rng()).unwrap();
                    state = State::PrepS;

                } else if case == 3{
                    line += transitive_verbs.choose(&mut rand::thread_rng()).unwrap();
                    state = State::TransS;
                    
                }
                else if case == 4{
                    line += indicative_verbs.choose(&mut rand::thread_rng()).unwrap();
                    state = State::IndicS;

                } else if case == 5{
                    state = State::DoneS;
                
                } 
            }
            State::FutureS => {
                let case = rand::thread_rng().gen_range(1..=2);

                if case == 1{
                    line += indicative_verbs.choose(&mut rand::thread_rng()).unwrap();
                    state = State::IndicS;
                    
                } else if case == 2{
                    line += transitive_verbs.choose(&mut rand::thread_rng()).unwrap();
                    state = State::TransS;
                } 
            }
            State::ConjS => {
                let case = rand::thread_rng().gen_range(1..=2);

                if case == 1{
                    line += nouns.choose(&mut rand::thread_rng()).unwrap();
                    state = State::NounS;
                    
                } else if case == 2{
                    line += pronouns.choose(&mut rand::thread_rng()).unwrap();
                    state = State::PronounS;
                } 
            }
            State::TransS => {
                let case = rand::thread_rng().gen_range(1..=3);

                if case == 1{
                    line += articles.choose(&mut rand::thread_rng()).unwrap();
                    state = State::ArticleS;
                    
                } else if case == 2{
                    line += pronouns.choose(&mut rand::thread_rng()).unwrap();
                    state = State::PronounS;
                } else if case == 3{
                    line += nouns.choose(&mut rand::thread_rng()).unwrap();
                    state = State::NounS;
                }
            }
            State::DoneS => {
                break;
            }

        }
    
        line += " ";
    }
    line

}