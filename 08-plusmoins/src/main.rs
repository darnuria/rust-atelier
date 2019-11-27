// -----------------------------------------------------------------------------
// Atelier Rust
// 2019-11-27 - 14h -> 16h
// Animateur: Axel Viala (darnuria)
//
// 08-plusmoins.rs
//
// Dans cet exercice on va écrire un programme pour jouer au plus ou moins
// On va partir d'un code qui fonctionne presque!
//
//
// Objectifs pédagogiques:
//
// - Manipulation des entrées sorties
// - Types Errors - découverte des enums
// - Découverte des imports
// - Découverte du if impératif
// - Découverte du match
// - boucle `loop`
//
// /!\ Quand vous verrez les symboles: `???`, il s'agit de code à
// completer soit-même c'est normal que Rust indique une erreur! :)
// -----------------------------------------------------------------------------

// Étapes:
//
// - Fixer le programme pour qu'il compile
// - Faire une boucle afficher un message succès echec
// - Convertir la lecture sur l'entrée standard en nombre
//   indices :
//   - std::string::String::trim() 
//   - std::string::String::parse()
// - Faire afficher plus ou moins selon que l'essai soit plus grand ou petit que le nombre
//   hint: lire la doc de std::cmp::Ordering + utiliser un match
// - Utiliser un générateur d'aleatoire disponnible dans la lib standard

use std::io;

fn main() {
    // On poeux utiliser un générateur de nombre aleatoire ici plutot que 42.
    let mystere = 42;
    let guess = String::new();
    
    // loop {

        // Utiliser expect ou faire un match ;)
        io::stdin().read_line(&guess);
        
        println!("Tu as saisi {}.", guess);
    // }
}