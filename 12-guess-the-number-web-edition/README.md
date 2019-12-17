
# Devine le nombre - WebEdition

## Objectif

On souhaite écrire un petit jeu web ou le but est de deviner un nombre,
mystère. L'utilisateur dois être informé de si il a reussi a trouver le nombre
ou l'inviter à retenter.

## Framework web

Vous pouvez choisir soit [Gotham](https://gotham.rs/) ou soit [Nickel](https://nickel-org.github.io/).

## Specification (minimale)

Il doit y avoir un `<form>` avec un `<input>` pour proposer a un utilisateur
de saisir un nombre et un `<button>` pour envoyer ce nombre.

On va utiliser une route web sur `/guess` pour traiter le contenu de ce formulaire 
encoder en format `urlencoded` on utilisera une bibliothèque pour gérer ce format de
donnée.

## Bonus

- Utiliser une crate pour gérer du logging suggestion de crate https://docs.rs/log/0.4.8/log/
- Envoyer en `JSON` les données, faire une mini API-REST pour gérer le traitement (vous pouvez changer de framework par exemple si vous voulez ex: https://rocket.rs/)
- MEGABonus: Utiliser rust dans le navigateur avec WebAssembly https://rustwasm.github.io/docs/book/