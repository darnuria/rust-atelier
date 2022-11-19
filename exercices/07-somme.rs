// -----------------------------------------------------------------------------
// Atelier Rust
// 2022-11-19 - 16h30h
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
// 2.1 On va eviter de deplacer le vecteur c'est dommage de le detruire si on veux
//     veux juste la somme!
// 2.2 On utilise une slice plutôt qu'une reférence sur un vecteur
//    https://doc.rust-lang.org/stable/book/ch04-03-slices.html#the-slice-type
// 3. On refait avec une closure et une fonction d'ordre supérieur (fold)
// 4. Bonus: Il existe une formule mathematique pour faire la somme sans boucle! ;)
//    https://fr.wikipedia.org/wiki/Somme_(arithm%C3%A9tique)
// 4.1 Bonus: Faire sans boucle mais avec un range et la fonction mathematique!

/// Calcule la somme de `start` a `end_excluded`.
fn sum(start: u32, end_excluded: u32) -> i32 {
    unimplemented!("Etape 1. Avec une fonction");
}

/// Calcule la somme des elements d'un `vec` de `u32`.
fn sum_vec(vec: Vec<u32>) -> u32 {
    unimplemented!("Etape 2. A faire avec un vec!");
}

/// Calcule la somme d'un `vec` sans la reference.
fn sum_ref_vec(vec: &Vec<u32>) -> u32 {
    // On vera plus loin les references en detail on demande avec une reference avec & devant une variable!
    unimplemented!("Etape 2.1 A faire avec un vec mais avec une reference!");
}

/// Calcule la somme depuis une slice une vue sur la memoire de taille connue.
fn sum_slice(vec: &[u32]) -> u32 {
    unimplemented!("Etape 2.1. A faire avec une slice");
}

/// Calcul la somme des nombres d'un range exclusif de 0 a 10
fn sum_range(range: std::ops::Range<u32>) -> u32 {
    let begin = range.start;
    let end = range.end;
    let sum = 0;
    // Ici la petite fonction mathematique voir au dessus pour le lien wikipedia
    // C'est aussi possible de le faire avec une boucle ;)
    0
}

fn main() {
    let sum;
    // let numbers

    // 0. une boucle et un range!
    // Range incluant 0 jusque à 10 exclusif,
    // si on veut inclusif on utilise `..=`
    for ??? in 0..10 {
        sum += i;
    }

    println!("Somme: {}", sum);

    // 1.0 Avec une fonction et en parametre le debut et la fin

    // 2.0 Somme mais sur un vec
    let my_vector = vec![1,2,3,4,5];

    // 2.1 Somme sur un vec mais on prends par reference:
    // Pour avoir une reference ecrivez `&my_vector` ;)

    // 2.2 Pour obtenir une slice on a plusieurs facons!
    // Juste `&my_vector` vous donnera une slice c'est un peu magique
    // ou comme l'exemple en dessous pour obteir un "sous bout" de notre vecteur
    let just_the_two_first_numbers = &my_vector[0..=1];

    ////////////////////////////////////////////////////////////////////////////////
    // C'est plus avancer apres hesitez pas a passer aux autres et revenir apres! //
    ////////////////////////////////////////////////////////////////////////////////

    // 3.0 Ici ecrire une nouvelle fonction mais en utilisant un iterateur. :)
    // Help: On peut ecrire `[0,1,2,3].iter().fold(0, |n| { ??? })`.
    // ??? c'est la ou c'est a vous de jouer! :)

    // 4.0 Bonus: Implementer avec une fonction sans boucle ni vecteur via
    // une fonction :)

    // 4.1 Ici on reprends le range du debut mais on le passe a une fonction et on fait comme dans
    // 4.0!

    // let range = 0..10; // On peut avec le last avec .start et .end sur range :
}
