// -----------------------------------------------------------------------------
// Atelier Rust Journée du Logiciel Libre
// 2019-04-06 - 13h -> 13h45
// Animateur: Axel Viala (darnuria)
//
// bonjour-rust.rs
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
// - notions sur les macro et fonctions
//
// /!\ Quand vous verrez les symboles: `???`, il s'agit de code à
// complêter soit même c'est normal que rust indique une erreur! :)
// -----------------------------------------------------------------------------


// À faire: Modifier le programme pour qu'il affiche votre prénom et votre age.


fn main() {
    println!("Bienvenu dans l'atelier d'initiation à Rust!");


    // Çi dessous je vous invite à faire «dire» à votre programme,
    // «Bonjour» suivit de votre prenom! Par defaut il affichera mon prénom
    // Je vous invite à vous approprier ce code avec votre prénom! :D
    let prenom = "axel";
//  ^                  ^
//  |                  | Notez le `;` il sert à dire qu'on a
//  |                    fini de décrire notre «instrction» on reviendra dessus!
//  |
//  \ Le mot clé `let` sert à dire à Rust associe l'identifiant ici
// `prenom` à la valeur d'expression ici la chaine de caractères `"axel"`.

    // Ici on utilise la «macro» println
    println!("Bonjour {}!", prenom);
    // Et notre expression nommée «prenom».

    // ---------------------------------
    // Macros:
    // ---------------------------------
    // `println!`est une macro car son nom est suivi d'un `!`.
    // Une macro est un moyen de créer un sous-programme,
    // à la compilation pour résoudre un calcul, pour nous
    // à l'heure actuel c'est pas très important.
    // On fera comme çi c'était une fonction classique!

    // On peut faire la même chose avec des nombres:
    let age = 26;
    println!("{} a {} c'est un bon age pour programmer.", prenom, age);
    // Tout age est bon pour programmer. ;)
}
