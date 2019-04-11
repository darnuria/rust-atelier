// -----------------------------------------------------------------------------
// Atelier Rust Journée du Logiciel Libre
// 2019-04-06 - 13h -> 13h45
// Animateur: Axel Viala (darnuria)
//
// addition.rs
//
// Dans ce programme on va découvrir comment résoudre une erreur de typage,
// c'est une erreur classique qui vous arrivera souvent! Donc on va le découvrir ensemble.
//
// Objectifs pédagogiques:
//
// - Découvrir le garde de fou qu'apporte le typage
// - Résoudre une erreur de typage
// - Manipuler des nombres
// -----------------------------------------------------------------------------


// Typage 101:
// En Rust les expressions portent un type et comme l'expression populaire suivante
// le dit: On ne peux pas additionner les choux et les carottes.
//
// Jusque ici nous avons manipuler les types `&str`, `i32`, et les fonctions qui sont un
// type un peu avancé. ;)


fn main() {
    // L'objectif est de faire une addition entre deux entiers en Rust! Rien de bien terrible!
    //

    // Oh non! Une erreur s'est glisée dans notre programme! J'ai tapé '3' au lieu de 3.
    // Quelques explications pour comprendre:
    //
    //           '3' est du type `char`
    //           ⬇
    let somme = '3' + 3;
    //   ^          ^ ^
    //   |          | \ 3 est du type i32.
    //   |          |
    //   |          \ Ici + fait l'addition entre un i32 et un i32 et
    //   |            et le résultat sera un i32.
    //   |
    //   \ Rust n'arrive donc pas à trouver un bon type pour `somme`!

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
