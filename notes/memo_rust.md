# Rust 

## Variables et mutabilité

### `let` et `mut`
Pour déclarer une variable on utilse le mot clef `let` : 

```rust
let x = 5;
```

Par défaut les variables sont immutables, i.e. elles ne peuvent pas évoluer au cours de l'exécution. Le code suivant ne va pas compiler `cannot assign twice to immutable variable`.

```rust
let x = 5;
x = 7;
```

Pour rendre une variable mutable : `mut`. Le code suivant compile bien comme il faut.

```rust
let mut x = 5;
x = 7;
```

### shadowing

On peut déclarer une nouvelle variable avec un nom d'une variable déjà utilisée.

```rust
let x = 5;
let x = x + 1; // le x de droite vaut 5, et on assigne à x la valeur 6
```

Dans cet exemple, x n'est pas mutable, il ne peut pas être modifié sans le mot clef `let`.

Ce procédé est utile lorsqu'on veut transformer le type d'une variable :

```rust
let espaces = "    "; // ici espace est un String
let espaces = espaces.len(); // espace a été assigné à une autre valeur : un entier
```

Dans ce cas on ne peut pas utiliser `mut`. L'exemple suivant ne compile pas `mismatched types`
```rust
let mut espaces = "    "
espaces = espaces.len(); // on essaye d'affecter un entier à une variable qui est un string
```


### `const`

Il existe aussi les constantes, toujours en majuscules. Elles sont utiles pour partager une valeur avec l'ensemble d'un programme. Nous y reviendrons plus tard. 

```rust
let const VITESSE_SON u32 = 340;
```




## Types de données

La plupart du temps, le compilateur peut inférer le type des données. 

## Fonctions

## Control flow
