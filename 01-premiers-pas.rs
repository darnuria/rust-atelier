// -----------------------------------------------------------------------------
// Atelier Rust Journée du Logiciel Libre
// 2019-04-06 - 13h -> 13h45
// Animateur: Axel Viala (darnuria)
//
// premiers-pas.rs
//
// Dans cet exercice vous allez découvrir les commentaires, faire votre premier
// programme il sera très simple, il ne fera rien!
// L'enjeux est de prendre en main votre éditeur de texte, d'apprendre à compiler
// et de découvrir les erreurs de compilation.
//
// Objectifs pédagogiques:
//
// - Commentaires mono-ligne et multi-lignes
// - Point d'entrée: fonction main
// - Découverte des blocs
// - Compilation
//
// -----------------------------------------------------------------------------

// Bienvenu à cet atelier! L'objectif est de découvrir ensemble la programmation
// à l'aide du langage Rust. L'atelier sera centré sur la pratique donc trève de
// discussions! Le format principal sera le fichier source hautement documenté.

// En Rust pour écrire des choses ignorées par le «compilateur» qui transforme
// ce fichier en code machine on peut écrire des commentaires (comme toutes les lignes
// çi-dessus) elles doivent commencer par // ou être englobée entre /* et */.

/* <- Ceci est aussi un commentaire.
   mais sur plusieurs lignes */

// Notre premier programme Rust va être très simple, il ne va rien faire!

/* En Rust on structure notre code avec des fonctions, c'est comme des mini-programmes.
  Tout programme Rust contient une fonction `main`, c'est le *point-d'entrée* de notre programme.

/- `fn` est un mot clef permettant de déclarer une fonction.
|
|      /- Les parenthèses servent à indiquer si la fonction
|      |  accepte des paramètres on s'interessera à ça plus tard.
|      |                                                                        */
fn main() {
/* ^
   \ main est le nom de notre fonction. */
/*

/- Les accolades servent à découper des blocs en Rust ici le bloc de notre fonction!
| */
}


// Félicitation! Voici notre premier programme Rust. Il ne fait rien! :)
// Vous pouvez le compiler avec la commande : `rustc premiers-pas.rs`
// Et l'executer avec la commande `./premiers-pas` ou le tester dans le playground rust
// https://play.rust-lang.org

// Attention d'ailleurs une erreur de compilation existe dans un fichier source,
// nous allons la résoudre en lisant le message d'erreur!


























// Indice: Un commentaire sur plusieurs lignes n'est pas fermé!!!
