// -----------------------------------------------------------------------------
// Atelier Rust
// 2019-05-23 - 13h -> 13h45
// Animateur: Axel Viala (darnuria)
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
// - Utilisation des vecteurs
// - Boucles `for`
// - Fonctions anonymes `closures`
//
// /!\ Quand vous verrez les symboles: `???`, il s'agit de code à
// completer soit-même c'est normal que Rust indique une erreur! :)
// -----------------------------------------------------------------------------

// Étapes:
//
// 0. On fait la somme avec une boucle
// 1. on fait la somme sur un vector!
// 2. On extrait dans une fonction
// 2.1 On utilise une slice plutôt qu'une reférence sur un vecteur
// 3. On refait avec une closure et une fonction d'ordre supérieur (fold)

fn main() {
    let sum;

    // Range incluant 0 jusque à 10 exclusif,
    // si on veut inclusif on utilise ..=
    for ??? in 0..10 {
        sum += i;
    }

    println!("Somme: {}", sum);
}















// indices: Vous pouvez tenter de faire avec un seul appel à println! !
