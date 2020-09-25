# Bases de Rust 🦀

## Variables et mutabilité

### Déclaration de variables `let` et `mut`

Pour déclarer une variable on utilse le mot-clef `let` :

```rust
let x = 5;
```

Par défaut, en rust les variables sont **immutables**.

> Note: On dit qu'une variable est **immutable** car elle ne peux pas changer de
> **valeur** au cours de l'exécution.

Notre ami le compilateur 🤖 risque de vous sortir:
`cannot assign twice to immutable variable`.

```rust
let x = 5;
x = 7;
```

### Mutabilité sur une variable

Pour rendre une variable mutable vous pouvez utiliser le mot-clef `mut` :

```rust
let mut x = 5;
// Attention on change la valeur de x en mémoire!
x = 7;
```

**⚠ Attention ⚠:**
> Un grand pouvoir vient avec de grandes responsabilités,
>changer la valeur des choses peut apporter beaucoup de bugs mais parfois
>c'est néccessaire en Rust!


**Pour la culture:**

> Il existe des langages fonctionnels sans mutabilité comme
> [Haskell](https://en.wikipedia.org/wiki/Haskell_(programming_language))
> On laissera au compilateur le soin de transformer du code immutable en
> instructions et transformations d'état sur la mémoire. Ocaml est partiellement
> immutable.

### Expressions

On peut lier à un nom avec `let` une valeur qui contient n'importe quelle
expression.

**Par exemple :**

```rust
let c_pas_faux = (42 - 10) == 32;
```

En rust énormement de constructions sont des expressions! Le `if`, le `match`,
les appels de fonctions que nous verrons plus tard!

Et aussi les blocs de code entre `{ }`.

Par exemple le code suivant est une expression de type `i32` associée au nom
`super_calcul_complexe`.

```rust
let super_calcul_complexe = {
    let c = 20 + 2;
    c + 20
};
```

Ce qui n'est pas une expression est une instrution *statement* en anglais,
les déclarations avec `let` sont des instructions, on enchaîne les instructions
avec `;`.

> ⚠ Attention ⚠: Avec `;` on peut se retrouver à faire `{ 5 + 5; }` la valeur de
> ce block sera le tuple vide: `()` et son type `()` dit `unit` type!

### Shadowing

On peut déclarer une nouvelle variable avec un nom d'une variable déjà utilisée.

```rust
// on définit que x = 5 pour le reste du bloc de code
let x = 5;
// et en fait on se dit finalement x c'est 5 + 5;
// Mais en mémoire le premier x n'a pas changé!
let x = 5 + x;
```

Dans cet exemple: `x` n'est pas mutable, la mémoire ne peut pas être modifiée sans
le mot-clef `mut`.

Ce procédé est utile lorsqu'on à une variable qui au cours du programme,
représente quelque chose et que on ne s'intéresse pas à l'ancienne variable mais
que le sens est le même par exemple:

```rust
// Imaginez une application web
// Ou vous avez une fonction app.get('age') qui vous donne une string
// puis vous voulez transformer age en entier avec le code suivant:
let age = app.get("age"); // age est du type String
let age: i32 = age.parse().expect("Bebop impossible de convertir en i32");
// le premier age est un peu «une variable jetable».
```

Attention à ne pas faire la même chose avec `mut`.
Le compilateur va de vous dire : `mismatched types` si les types sont différents.

Si les types ne sont pas différents vous risquez d'avoir un bug! Attention mut
implique de grandes responsabilitées! Conclusion il faut utiliser `mut` avec
modération et préférer faire du `shadowing` en Rust quand on ne s'importe pas
de l'ancien état d'une variable.

```rust
let mut espaces = "    "
// on essaye d'affecter un entier à une variable qui est un string.
// len sert a récupérer la taille d'une chaine de caractères.
espaces = espaces.len();
```

Mais vous pouvez faire:

```rust
let espaces = "     ";
let espaces = espaces.len();
```

### `const`

On peut définir des [constantes](https://doc.rust-lang.org/rust-by-example/custom_types/constants.html), toujours nommées en majuscules.
Elles sont utiles pour partager une valeur avec l'ensemble d'un programme.
Nous y reviendrons plus tard.

```rust
const VITESSE_SON: u32 = 340;
```

## Essentiel à retenir sur les variables.

Avoir des variables immutables par défaut évite pas mal de bugs courants et
permet pleins d'optimisations. Et on peut se concentrer lors de la lecture
sur les variables mutables! Si vous devez partager une valeur avec tout le
programme faites une constante avec `const`.

A voir plus tard: On a pas encore vu les variables notées `static` c'est pour plus
tard! ;)

## Types de données

La plupart du temps, le compilateur peut inférer le type des données.

à venir
<!-- pas encore pret
types de bases:

* entiers
* caractères
* booleens
* tuples
* slices (tranches)
* references
* fonctions
-->


## Fonctions

À venir:

```rust
fn ma_fonction_qui_fait_rien() {

}
```

Une fonction qui n'accepte pas d'argument qui renvoie une valeur

```rust
fn good_number() -> i32 { 42 }
```

Une fonction qui prends deux arguments et renvoie un entiers sur 32 bits.

```rust
fn addition_en_plus_long(a: i32, b: i32) -> i32 {
    a + b
}
```

## Contrôle de flux d'execution

À venir:

```rust
let bonnes_conditions_d_etudes = if temperature <= 25 { true } else { false };
```
