// -----------------------------------------------------------------------------
// Atelier Rust Journée du Logiciel Libre
// 2022-11-19 - 16h30h
// Animateur: Axel (darnuria) && Aurelia
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

// Typage ça se mange ?:

// En Rust les expressions portent un type et comme l'expression populaire suivante
// le dit: On ne peut pas additionner les choux et les carottes.
//
// Jusqu'ici nous avons manipulé les types:
//
// - "Toto": `&str` (les chaînes de caractères)
// - 3: `u32` (nombre entiers) par exemple
// - main: les fonctions qui sont un type un peu avancé ;)
//
// Aller plus loin:
//
// En Rust les nombres sont les nombres que votre processeur peut supporter!
// Un u32 sera sur 32bits! Un u16 sur 16bits, u8 sur 8 bits.
// C'est important? Au tout debut pas trop, mais quand on est plus a l'aise
// en rust et prog systeme c'est utile comme distinction!

fn main() {
    // L'objectif est de faire une addition entre deux entiers en Rust ! Rien de bien terrible !

    //-------------------------------------------------------------------------
    // Quelques «commentaires gesticulés» pour comprendre:
    //
    //           '3' est du type `char`
    //           ⬇
    let somme = '3' + 3u32;
    //   ^          ^ ^
    //   |          | \ 3 est du type `u32` on a mis le petit `integer`
    //   |          |
    //   |          \ Ici `+` fait l'addition entre un u32 et un u32 et
    //   |            et le résultat sera un u32.
    //   |
    //   \ Rust n'arrive donc pas à trouver un bon type pour `somme`,
    //     car '3' n'est pas un `u32` !
    //-------------------------------------------------------------------------

    // Petite aide pour s'assurer qu'on a bien réussi a écrire 6 :)
    // `assert!` est une macro qui permet de vérifier une assertion pour
    // vérifier qu'on a le bon comportement :)
    assert!(somme == 6u32);

    // Oh non ! Une erreur s'est glissée dans notre programme à la ligne 38.
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
