// -----------------------------------------------------------------------------
// Atelier Rust Journée du Logiciel Libre
// 2022-11-19 - 16h30h
// Animateur: Axel (darnuria) && Aurelia
//
// 01-premiers-pas.rs
//
// Bienvenu!
//
// Dans cet exercice vous allez découvrir les commentaires, faire votre premier
// programme il sera très simple, il ne fera rien!
//
// Le support sera les fichiers sources et les commentaires! Pas de diapo! ;)
//
// L'enjeux est de prendre en main votre éditeur de texte, d'apprendre à compiler
// et de découvrir les erreurs de compilation.
//
// Objectifs pédagogiques:
//
// - Commentaires mono-ligne et multi-lignes `\\` et `\* *\`
// - Point d'entrée du programme la fonction `main`
// - Découverte des blocs `{}`
// - Compilation de notre programme!
// - Execution du binaire
//
// -----------------------------------------------------------------------------

// En Rust pour écrire du texte ignoré par le compilateur, l'outil qui valide et transforme
// ce fichier en code machine on peut écrire des commentaires
// elles doivent commencer par // ou être englobée entre /* et */.

/* En Rust on structure notre code avec des fonctions, c'est comme des mini-programmes.
  Tout programme Rust contient une fonction `main`, c'est le *point-d'entrée* de notre programme.

/- `fn` est un mot clef permettant de déclarer une fonction.
|
|      /- Les parenthèses servent à indiquer si la fonction
|      |  accepte des paramètres on s'interessera à ça plus tard.
|      |                                                                        */
fn main() {
/* ^
    \ main est le nom de notre fonction.
/- Les accolades servent à découper des blocs en Rust ici le bloc de notre fonction!
| */
}

// Félicitation! Voici notre premier programme Rust. Il ne fait rien! :)
// Vous pouvez le compiler avec la commande : `rustc 01-premiers-pas.rs`
//
// Et l'executer avec la commande `./premiers-pas` ou le tester dans le playground rust
// https://play.rust-lang.org

// Attention d'ailleurs une erreur de compilation existe dans un fichier source,
// nous allons la résoudre en lisant le message d'erreur!


























// Indice: Un commentaire sur plusieurs lignes n'est pas fermé!!!
// Vous aurrez souvent des indices en pied de fichier
