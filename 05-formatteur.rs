// -----------------------------------------------------------------------------
// Atelier Rust Journée du Logiciel Libre
// 2019-04-06 - 13h -> 13h45
// Participant: Charles Gueunet
//
// formatteur.rs
//
// On a vu dans les exercices précédents qu'il est possible d'afficher dans
// le terminal la valeur d'une variable ou la valeur pointé par une
// référence grace à la macro `println!` et du formateur par defaut `{}`.
//
// Nous allons ici explorer quelques autres formatteur

fn main() {

    let entier = 12; // La valeur de entier est 12
    let reference = &entier;
    //  ↑↑ La valeur de reference est une adresse.

    //                      Ici `{}` insert directement `entier`
    //                      ↓↓
    println!("Voici entier: {} et une reference vers celui-çi: {}.",
             entier, reference); //                            ↑↑
                                 // Ici le formateur auto-déréférence
                                 // `reference` vers `entier`


    // Pour ne pas déréférencer la variable mais afficher l'adresse
    // vers laquelle elle point on peut utiliser le formatteur {:p}
    println!("Notre entier est à l'adresse:  {:p}", reference);
    //                                        ↑↑
    // Si souhaite afficher l'adresse de notre référence
    // On peut utiliser le formateur `{:p}`

    // Déclarons un vecteur:
    // Ceux-ci se déclarent grâce à la macro vec![]
    let vector = vec![1,2,3];

    // Si on essaye d'en afficher le contenus grâce au formatteur `{}`
    // On obtien une erreur de compilation:
    println!("Voici le vector: {}", vector);
    //                          ^
    //                          | Ce formatteur ne convient pas pour un `vec`

    // A la place, il nous faut utiliser un formatteur de debug,
    // tel que {:?} ou {:#?}
    println!("Voici le vector: {:#?}", vector);
}


// Il existe en rust bien d'autres formatteur, permettant de choisir
// le nombre de décimal à afficher ou bien d'afficher en octal / binaire...
// Pour plus d'informations, la documentation est disponible ici:
// https://doc.rust-lang.org/std/fmt/
