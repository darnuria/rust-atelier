# Premier projet

Jeu du plus ou moins inspiré du projet 1 du livre "The Rust
Programming language".

<https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html>

Dans ce travail pratique on va implementer un jeu ou il faut deviner
un nombre choisi aléatoirement par l'ordinateur.

L'ordinateur répondra «moins» si le nombre est plus petit que celui
que l'ordinateur a choisi, «plus» si il est plus grand sinon
bravo.

Tous les bonus vous venant a l'esprit sont pertinants.

## Compilation et dépendances.

Pour gérer le projet on va utiliser `cargo`, un gestionnaire de paquet
et de dépendances pour Rust il viens avec Rustc le compilateur.

Pour commencer un projet Rust la prochaine fois vous pourrez faire
`cargo init` pour faire une bibliothèque `cargo init --lib`.

Ça vous fait un dépot minimal avec un fichier très important le `Cargo.toml`.
C'est le manifeste de configuration du projet qui contiens les dépendances
et toutes les informations pour construire le projet!

Pour compiler vous pouvez faire `cargo build` pour juste verifier votre code
`cargo check`.
