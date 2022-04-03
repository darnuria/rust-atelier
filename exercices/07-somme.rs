// -----------------------------------------------------------------------------
// Atelier Rust
// 2022-04-03 - 11h
// Animateur: Axel (darnuria) && Aurelia
//
// Somme.rs
//
// Dans cet exercice on va écrire notre première fonction pour manipuler un
// vecteur et ensuite on découvrira que l'on peut le faire avec une fonction
// anonyme.
//
//
// Objectifs pédagogiques:
//
// - Utilisation des vecteurs (tableaux aggrandissables)
//   https://doc.rust-lang.org/stable/book/ch08-01-vectors.html?#creating-a-new-vector
// - Boucles `for`:
//   https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#looping-through-a-collection-with-for
// - Fonctions anonymes `closures`:
//   Doc sur les fonctions annonymes
//   https://doc.rust-lang.org/stable/book/ch13-01-closures.html
//
// /!\ Quand vous verrez les symboles: `???`, il s'agit de code à
// completer soit-même c'est normal que Rust indique une erreur! :)
// -----------------------------------------------------------------------------

// Étapes:
//
// 0. On fait la somme avec une boucle
// 1. On extrait dans une fonction
//    https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html#functions
// 2. On fait la somme sur un vec!
// 2.1 On utilise une slice plutôt qu'une reférence sur un vecteur
//    https://doc.rust-lang.org/stable/book/ch04-03-slices.html#the-slice-type
// 3. On refait avec une closure et une fonction d'ordre supérieur (fold)

fn sum(start: u32, u32) -> i32 {
    unimplemented!("Etape 1. A faire!");
}

fn sum_vec(vec: Vec<u32>) -> u32 {
    unimplemented!("Etape 2. A faire!");
}

fn main() {
    let sum;

    // Range incluant 0 jusque à 10 exclusif,
    // si on veut inclusif on utilise ..=
    for ??? in 0..10 {
        sum += i;
    }

    println!("Somme: {}", sum);
}
