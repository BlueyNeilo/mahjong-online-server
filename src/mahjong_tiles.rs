//This Module has the data for the tile types

//These enums are treated like C enums
enum Dragons {
    Red,
    Green,
    White
}
enum Winds {
    North,
    South,
    East,
    West
}

enum SimpleSuit {
    Dot,
    Bamboo,
    Character,
}

enum HonourSuit {
    Wind,
    Dragon,
}

enum HonourValue {
    Wind(Winds),
    Dragon(Dragons),
}

struct FlowerSuit;

trait Tile {
    type TileSuit;
    type TileValue;
    fn new(s: Self::TileSuit, v: Self::TileValue) -> Self;
}

struct SimpleTile {
    suit : SimpleSuit,
    value : u8,
}

struct HonourTile {
    suit : HonourSuit,
    value : HonourValue,
}

struct FlowerTile {
    suit : FlowerSuit,
    value : u8,
}

impl Tile for SimpleTile {
    type TileSuit = SimpleSuit;
    type TileValue = u8;
    fn new(s: Self::TileSuit, v: Self::TileValue) -> Self {
        Self {
            suit: s,
            value: v,
        }
    }
}

impl Tile for HonourTile {
    type TileSuit = HonourSuit;
    type TileValue = HonourValue;
    fn new(s: Self::TileSuit, v: Self::TileValue) -> Self {
        Self {
            suit: s,
            value: v,
        }
    }
}

impl Tile for FlowerTile {
    type TileSuit = FlowerSuit;
    type TileValue = u8;
    fn new(s: Self::TileSuit, v: Self::TileValue) -> Self {
        Self {
            suit: s,
            value: v,
        }
    }
}