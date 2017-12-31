//This Module has the data for the tile types

enum Dragon {
    Red,
    Green,
    White
}
enum Wind {
    North,
    South,
    East,
    West
}

enum Simple {
    Dot(u8),
    Bamboo(u8),
    Character(u8)
}

enum Honour {
    W(Wind),
    D(Dragon)
}

enum Tile {
    S(Simple),
    H(Honour),
    Flower(u8)
}
