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
//
// /!\ Quand vous verrez les symboles: `???`, il s'agit de code à
// complêter soit même c'est normal que rust indique une erreur! :)
// -----------------------------------------------------------------------------


// Typage 101:
// En Rust les expressions portent un type et comme l'expression populaire suivante
// le dit: On ne peux pas additionner les choux et les carottes.
//
// Jusque ici nous avons manipuler les types &str, i32, et les fonctions qui sont un
// type un peyu avancé. ;)


fn main() {
    // L'objectif est de faire une addition entre deux entiers en Rust! Rien de bien terrible!
    let somme = "3" + 3;
    // Oh non! Une erreur s'est glisée dans notre programme!
    // Par inavertance j'ai taper la chaine de caractère "3" et non le nombre 3!
    // Compilons ce programme et voyons si rustc peut nous aider!
    println!("{}", somme);
}




























// indice: rustc addition.rs

/*
error[E0369]: binary operation `+` cannot be applied to type `&str`
--> addition.rs:23:17
|
23 |     let somme = "3" + 3;
|                 ^^^^^^^
|
= note: an implementation of `std::ops::Add` might be missing for `&str`
*/

// On peut résoudre notre problème en changeant "3" par juste 3.
