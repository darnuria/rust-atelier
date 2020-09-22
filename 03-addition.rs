// -----------------------------------------------------------------------------
// Programation avancée - exercice 2020
// Intervenant: Axel Viala (darnuria) axel@darnuria.eu
//
// 03-addition.rs
//
// Dans ce programme on va découvrir comment résoudre une erreur de typage,
// c'est une erreur classique qui vous arrivera souvent! Donc on va le découvrir ensemble.
//
// Objectifs pédagogiques:
//
// - Découvrir le garde-fou qu'apporte le typage
// - Résoudre une erreur de typage
// - Manipuler des nombres
// -----------------------------------------------------------------------------


// Typage ça se mange?:

// En Rust les expressions portent un type et comme l'expression populaire suivante
// le dit: On ne peux pas additionner les choux et les carottes.
//
// Jusqu'ici nous avons manipulé les types:
//
// - "Toto": `&str` (les chaînes de caractères)
// - 3: `i32` (nombre entiers) par exemple
// - main: les fonctions qui sont un type un peu avancé ;)


fn main() {
    // L'objectif est de faire une addition entre deux entiers en Rust! Rien de bien terrible!


    //-------------------------------------------------------------------------
    // Quelques «commentaires gesticulés» pour comprendre:
    //
    //           '3' est du type `char`
    //           ⬇
    let somme = '3' + 3;
    //   ^          ^ ^
    //   |          | \ 3 est du type i32.
    //   |          |
    //   |          \ Ici `+` fait l'addition entre un i32 et un i32 et
    //   |            et le résultat sera un i32.
    //   |
    //   \ Rust n'arrive donc pas à trouver un bon type pour `somme`,
    //     car '3' n'est pas un `i32` !
    //-------------------------------------------------------------------------


    // Oh non ! Une erreur s'est glisée dans notre programme à la ligne 38.
    // Par inadvertance, j'ai tapé le caractère '3' au lieu de 3.

    // Compilons ce programme et voyons si rustc peut nous aider!
    println!("{}", somme);
}




























// indice: rustc addition.rs

/*
error[E0369]: binary operation `+` cannot be applied to type `char`
  --> 03-addition.rs:32:17
   |
32 |     let somme = '3' + 3;
   |                 ^^^^^^^
   |
   = note: an implementation of `std::ops::Add` might be missing for `char`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0369`.
*/

// On peut résoudre notre problème en changeant '3' par juste 3.
