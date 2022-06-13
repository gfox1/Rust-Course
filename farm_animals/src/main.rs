#[derive(PartialEq, Debug)]
enum FarmAnimal {
    Worm,
    Cow,
    Bull,
    Chicken { num_eggs: usize },
    Dog { name: String },
}

fn what_does_the_animal_say(animal: &FarmAnimal) {

    /* TODO: fill in the match statement below to make this code compile */

    let noise = match animal {
        FarmAnimal::Cow | FarmAnimal::Bull => "moo".to_string(),
        FarmAnimal::Chicken { num_eggs:_ } => "cluck, cluck!".to_string(),
        FarmAnimal::Dog { ref name } if name == "Lassie" => format!("I am a talking dog named {}!", name),
        FarmAnimal::Dog { name } => format!("woof, woof! I am {}!", name),
        _ => "-- (silence)".to_string(),
    };

    println!("{:?} says: {:?}", animal, noise);
}

fn main() {
    what_does_the_animal_say(
        &FarmAnimal::Dog {
            name: "Lassie".to_string()
    });
    what_does_the_animal_say(
        &FarmAnimal::Dog {
            name: "Bob".to_string()
    });
    what_does_the_animal_say(&FarmAnimal::Cow);
    what_does_the_animal_say(&FarmAnimal::Bull);
    what_does_the_animal_say(&FarmAnimal::Chicken{num_eggs: 3});
    what_does_the_animal_say(&FarmAnimal::Worm);
}
