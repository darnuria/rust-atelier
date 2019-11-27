// -----------------------------------------------------------------------------
// Atelier Rust
// 2019-11-17 - 14h -> 16h
// Animateur: Axel Viala (darnuria)
//
// 09-options.rs
//
// Dans cet exercice on découvre la joie des types options et match! ;p
//
// Objectifs pédagogiques:
//
// - Rappels sur les structures
// - Type Option
// - Manipulation avec match basique
//
// /!\ Quand vous verrez les symboles: `???`, il s'agit de code à
// completer soit-même c'est normal que Rust indique une erreur! :)
// -----------------------------------------------------------------------------

// Etapes:
// 
// 1. Faire compiler le code ;)
// 2. S'inspirer de l'exo 08 pour construire un user depuis une saisie clavier
//   depuis la stdin

/// Structure pour representer un utilisateur et optionnellement le nom de son animal.
struct User {
    firstname: String,
    lastname: String,
    petname: Option<String>,
}

// TODO Corriger la ligne çi dessous
impl ??? {

    /// Construct a User with a optional pet.
    /// Take by move for simplicity.
    fn new(firstname: String, lastname: String, petname: Option<???>) -> User { // <- Fix me
        User { ???, lastname, ??? }
    }
}

fn main() {
    // Alyx form half-life games
    let alyx = User::new("Alyx".to_string(), "Vance".to_string(), Some("Dog".to_string()));
    //                    ⬆
    //                    `to_string()` sert a transformer une &str
    //                    en String on vera plus tard pourquoi

    let petname = match alyx.petname {
        None => ", I don't own a pet".to_string(),
        Some(???) => format!(", my pet name is: {}", ???),
    };

    println!("Hi I am {} {}{}.", alyx.firstname, alyx.lastname, petname);
}
