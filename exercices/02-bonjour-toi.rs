// -----------------------------------------------------------------------------
// Atelier Rust Journée du Logiciel Libre
// 2022-04-03 - 11h
// Animateur: Axel (darnuria) && Aurelia
//
// 02-bonjour-rust.rs
//
// Dans cet exercice vous allez découvrir les chaines de caractères un *type*
// très utile, comment en afficher et comment les formater, aussi vous découvrirez
// les expressions et comment associer une expression à un «nom» de variable.
//
// Objectifs pédagogiques:
//
// - Première rencontre avec les types
// - Première rencontre avec les expressions
// - Déclaration de votre première chaine de caractère
// - Affichage dans la console de votre chaine de caratère
// - Association d'une expression à un nom
// - Notions sur les macro et fonctions
//
// /!\ Quand vous verrez les symboles: `???`, il s'agit de code à
// completer soit-même c'est normal que Rust indique une erreur! :)
// -----------------------------------------------------------------------------

// À faire: Modifier le programme pour qu'il affiche votre prénom et votre age.

fn main() {
    println!("Bienvenu dans l'atelier d'initiation à Rust!");

    // Ci-dessous je vous invite à faire «dire» à votre programme,
    // «Bonjour» suivit de votre prenom! Par defaut il affichera le prenom d'un
    // des intervenants.
    // Je vous invite à vous approprier ce code avec votre prénom! :D
    let prenom = "axel";
//  ^                  ^
//  |                  | Notez le `;` il sert à dire qu'on a
//  |                    fini de décrire notre «instruction» on reviendra dessus!
//  |
//  \ Le mot clé `let` sert à demander à Rust d'associer l'identifiant, ici
// `prenom`, à la valeur d'expression ici la chaine de caractères `"axel"`.

    // Ici on utilise la «macro» println
    println!("Bonjour {}!", prenom);
    // Et notre expression nommée «prenom».

    // ---------------------------------
    // Macros:
    // ---------------------------------
    // `println!`est une macro car son nom est suivi d'un `!`.
    // Une macro est un moyen de créer un sous-programme,
    // à la compilation pour résoudre un calcul, pour nous
    // à l'heure actuelle ce n'est pas très important.
    // On fera comme si c'était une fonction classique!

    // On peut faire la même chose avec des nombres:
    let age = 26;
    println!(
        "{} a {} ans, c'est un bon age pour programmer.",
        prenom, age
    );
}

















// indices: Vous pouvez tenter de faire avec un seul appel à println! !
