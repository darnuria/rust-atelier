// -----------------------------------------------------------------------------
<<<<<<< HEAD:exercices/12-lifetime-basics-borrow.rs
// Programation avancée - exercice 2020
// Intervenant: Axel Viala (darnuria) axel@darnuria.eu
=======
// Atelier Rust
// 2019-11-17 - 14h -> 16h
// Animateur: Axel (darnuria) && Aurelia
>>>>>>> 806f69d (mise a jour JDLL 2022 authors):10-lifetime-basics.rs
//
// 10-lifetimes-basic.rs
//
// Objectifs pédagogiques:
//
// - Découverte du concept de lifetime
//
// Inspiré par l'exemple de *programming rust* de Jim Blandy et
// Jason Orendorff.
//
// /!\ Quand vous verrez les symboles: `???`, il s'agit de code à
// completer soit-même c'est normal que Rust indique une erreur! :)
// -----------------------------------------------------------------------------

// Etapes:
// 
// 1. Observer l'erreur de compilation
// 2. Corriger autant que neccessaire le code pour faire compiler
// le programme.

fn main() {
    // `r` ici sera une référence sur x.
    let r = {
        // `x` est une valeur dans la pile
        let x = 1;
        x
    }; // <- fin de vie de `x`; après x est `dropped`
    let _is_equal = r == 1;
    // Rust vérifie à la compilation la validité
    // de la référence `r`.
}