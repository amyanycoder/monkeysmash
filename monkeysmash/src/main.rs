use rand::Rng;
use rand::seq::SliceRandom;

fn main() {


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
        Start_S,
        Article_S,
        Pronoun_S,
        Noun_S,
        Prep_S,
        Adjec_S,
        Indic_S,
        Adverb_S,
        Future_S,
        Trans_S,
        Conj_S,
        Done_S   
    }

    let mut state = State::Start_S;

    let mut line = String::new();

    loop {
        match state{
            State::Start_S => {
                let case = rand::thread_rng().gen_range(1..=4);

                if case == 1{
                    state = State::Article_S;
                } else if case == 2{
                    state = State::Pronoun_S;
                } else if case == 3{
                    state = State::Noun_S;
                } else if case == 4{
                    state = State::Adjec_S;
                }
            }
            State::Article_S => {
                let case = rand::thread_rng().gen_range(1..=2);

                if case == 1{
                    line += adjectives.choose(&mut rand::thread_rng()).unwrap();
                    state = State::Adjec_S;
                    
                } else if case == 2{
                    line += nouns.choose(&mut rand::thread_rng()).unwrap();
                    state = State::Noun_S;
                } 
            }
            State::Pronoun_S => {
                let case = rand::thread_rng().gen_range(1..=3);

                if case == 1{
                    line += indicative_verbs.choose(&mut rand::thread_rng()).unwrap();
                    state = State::Indic_S;
                    
                } else if case == 2{
                    line += transitive_verbs.choose(&mut rand::thread_rng()).unwrap();
                    state = State::Trans_S;

                } else if case == 3{
                    line += conjunctions.choose(&mut rand::thread_rng()).unwrap();
                    state = State::Conj_S;

                } 
            }
            State::Noun_S => {
                let case = rand::thread_rng().gen_range(1..=6);

                if case == 1{
                    line += indicative_verbs.choose(&mut rand::thread_rng()).unwrap();
                    state = State::Indic_S;
                    
                } else if case == 2{
                    line += transitive_verbs.choose(&mut rand::thread_rng()).unwrap();
                    state = State::Trans_S;

                } else if case == 3{
                    line += conjunctions.choose(&mut rand::thread_rng()).unwrap();
                    state = State::Conj_S;
                    
                }
                else if case == 4{
                    line += adverbs.choose(&mut rand::thread_rng()).unwrap();
                    state = State::Adverb_S;

                } else if case == 5{
                    line += future.choose(&mut rand::thread_rng()).unwrap();
                    state = State::Future_S;
                
                } 
                else if case == 6{
                    state = State::Done_S;
                
                }
            }
            State::Prep_S => {
                let case = rand::thread_rng().gen_range(1..=3);

                if case == 1{
                    line += articles.choose(&mut rand::thread_rng()).unwrap();
                    state = State::Article_S;
                    
                } else if case == 2{
                    line += pronouns.choose(&mut rand::thread_rng()).unwrap();
                    state = State::Pronoun_S;

                } else if case == 3{
                    line += nouns.choose(&mut rand::thread_rng()).unwrap();
                    state = State::Noun_S;

                } 
            }
            State::Adjec_S => {
                let case = rand::thread_rng().gen_range(1..=2);

                if case == 1{
                    line += adjectives.choose(&mut rand::thread_rng()).unwrap();
                    state = State::Adjec_S;
                    
                } else if case == 2{
                    line += nouns.choose(&mut rand::thread_rng()).unwrap();
                    state = State::Noun_S;
                } 
            }
            State::Indic_S => {
                let case = rand::thread_rng().gen_range(1..=3);

                if case == 1{
                    line += adverbs.choose(&mut rand::thread_rng()).unwrap();
                    state = State::Adverb_S;
                    
                } else if case == 2{
                    line += conjunctions.choose(&mut rand::thread_rng()).unwrap();
                    state = State::Conj_S;

                } else if case == 3{
                    state = State::Done_S;

                } 
            }
            State::Adverb_S => {
                let case = rand::thread_rng().gen_range(1..=5);

                if case == 1{
                    line += conjunctions.choose(&mut rand::thread_rng()).unwrap();
                    state = State::Conj_S;
                    
                } else if case == 2{
                    line += prepositions.choose(&mut rand::thread_rng()).unwrap();
                    state = State::Prep_S;

                } else if case == 3{
                    line += transitive_verbs.choose(&mut rand::thread_rng()).unwrap();
                    state = State::Trans_S;
                    
                }
                else if case == 4{
                    line += indicative_verbs.choose(&mut rand::thread_rng()).unwrap();
                    state = State::Indic_S;

                } else if case == 5{
                    state = State::Done_S;
                
                } 
            }
            State::Future_S => {
                let case = rand::thread_rng().gen_range(1..=2);

                if case == 1{
                    line += indicative_verbs.choose(&mut rand::thread_rng()).unwrap();
                    state = State::Indic_S;
                    
                } else if case == 2{
                    line += transitive_verbs.choose(&mut rand::thread_rng()).unwrap();
                    state = State::Trans_S;
                } 
            }
            State::Conj_S => {
                let case = rand::thread_rng().gen_range(1..=2);

                if case == 1{
                    line += nouns.choose(&mut rand::thread_rng()).unwrap();
                    state = State::Noun_S;
                    
                } else if case == 2{
                    line += pronouns.choose(&mut rand::thread_rng()).unwrap();
                    state = State::Pronoun_S;
                } 
            }
            State::Trans_S => {
                let case = rand::thread_rng().gen_range(1..=3);

                if case == 1{
                    line += articles.choose(&mut rand::thread_rng()).unwrap();
                    state = State::Article_S;
                    
                } else if case == 2{
                    line += pronouns.choose(&mut rand::thread_rng()).unwrap();
                    state = State::Pronoun_S;
                } else if case == 3{
                    line += nouns.choose(&mut rand::thread_rng()).unwrap();
                    state = State::Noun_S;
                }
            }
            State::Done_S => {
                break;
            }

        }
    
        line += " ";
    }
    line

}