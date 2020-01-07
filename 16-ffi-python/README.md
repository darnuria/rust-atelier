# Foreign Function Interface: les bases

⚠ Ce projet risque d'evoluer pensez à faire votre branche git et à
verifier les changements sur ma branche! ⚠

Le contexte: On souhaite pouvoir faire un calcul en Rust,
depuis Python, on dit qu'on crée une «interface de fonction étrangère» "Foreign Function Interface" FFI en Anglais.

Pour s'échauffer on va faire une première version très basique,
avec les mécanismes de FFI de base de Rust.

Puis dans un second temps dans l'exercice 17 on fera un exercice avec la crate Rust PyO3 qui vous rends la tache plus agréable.

## Produit matriciel parallèlle

On va faire un produit matriciel parallèlle en Rust pour profiter de
la compilation native.

Pour commencer on va s'aider du projet pyo3 qui facilite l'interfaçage python/rust.
