// -----------------------------------------------------------------------------
// Atelier Rust Journée du Logiciel Libre
// 2019-04-06 - 13h -> 13h45
// Participant: Charles Gueunet
//
// formatteur.rs
//
// On a vu dans les exercices précédents
// qu'il est possible d'afficher le contenu
// d'une variable ou d'une référence grâce à la
// macro println! et du formatteur {}
// Nous allons ici explorer quelques autres formatteur

fn main() {

    let entier = 12;
    let reference = &entier;
    // Ici, on peut voir que le formatteur {} est capable
    // d'identifier que la variable reference contient une adresse
    // et que ce que l'on veut afficher doit être déréférencé.
    println!("voici un entier: {} et une reference vers ceui-c: {}.", entier, reference);

    // Pour ne pas déréférencer la variable mais afficher l'adresse
    // vers laquelle elle point on peut utiliser le formatteur {:p}
    println!("Notre entier est à l'adresse:  {:p}", reference);

    // Déclarons un tableau:
    // Ceux-ci se déclarent grâce à la macro vect![]
    let tableau = vec![1,2,3];

    // Si on essaye d'en afficher le contenus grâce au formatteur {}
    // On obtien une erreur de compilation:
    println!("Voici le tableau: {}", tableau);
    //                          ^
    //                          | Ce formatteur ne convient pas pour un vect!
    //                          | Erreur de compilation

    // A la place, il nous faut utiliser un formatteur de débug,
    // tel que {:?} ou {:#?}
    println!("Voici le tableau: {:#?}", tableau);
}

// Il existe en rust bien d'autres formatteur, permettant de choisir
// le nombre de décimal à afficher ou bien d'afficher en octal / binaire...
// Pour plus d'informations, la documentation est disponible ici:
// https://doc.rust-lang.org/std/fmt/
