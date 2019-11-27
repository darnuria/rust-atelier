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
//
// /!\ Quand vous verrez les symboles: `???`, il s'agit de code à
// completer soit-même c'est normal que Rust indique une erreur! :)
// -----------------------------------------------------------------------------

// Étapes:
//
// 0. On complête la structure Point2D pour la faire compiler
// 1. On ajoute une fonction `new` pour creer un Point2D
// 2. On associe la fonction `new` avec un bloc `impl Point2D {}`
// 2. On crée une fonction `add` qui fait l'addition de 2 `Points2D`, `self` et  `p`
// 2.0 Observer ce qui se passe si on tente de réutiliser un des deux arguments de `add`!
// 2.1 On reprend `add` pour prendre par référence `self` et `p`
// 2.2 Écrire quelques tests pour nos fonctions

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
fn new(???) -> ??? { // <- Fix me
    ???
}
}

fn main() {
    // Alyx form half-life games
    let alyx = User::new("Alyx", "Vance", "Dog");

    let petname = match alyx.petname {
        ??? => ", I don't own a pet",
        Some(???) => format!(", my pet name is: {}", ???)
    }

    println!("Hi I am {} {}{}.", alyx.firstname, alyx.lastname, petname);
}

// Pour commencer à écrire les tests il faudra décomenter le bloc ci-dessous:
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let a = Point::new(1, 2);
        let b = Point::new(3, 4);
    }
}
*/