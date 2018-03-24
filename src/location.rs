#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Location {
    Hand,
    A7,
    A4,
    A1,
    B6,
    B4,
    B2,
    C5,
    C4,
    C3,
    D7,
    D6,
    D5,
    D3,
    D2,
    D1,
    E5,
    E4,
    E3,
    F6,
    F4,
    F2,
    G7,
    G4,
    G1,
    Captured
}

impl Location {
    pub fn from_str(code: &str) -> Location {
        use Location::*;
        match code {
            "Hand"      => Hand,
            "A7"        => A7,
            "A4"        => A4,
            "A1"        => A1,
            "B6"        => B6,
            "B4"        => B4,
            "B2"        => B2,
            "C5"        => C5,
            "C4"        => C4,
            "C3"        => C3,
            "D7"        => D7,
            "D6"        => D6,
            "D5"        => D5,
            "D3"        => D3,
            "D2"        => D2,
            "D1"        => D1,
            "E5"        => E5,
            "E4"        => E4,
            "E3"        => E3,
            "F6"        => F6,
            "F4"        => F4,
            "F2"        => F2,
            "G7"        => G7,
            "G4"        => G4,
            "G1"        => G1,
            "Captured"  => Captured,
            _           => panic!("argh")
        }
    }

    pub fn to_str(&self) -> &'static str {
        use Location::*;
        match *self {
            Hand      => "Hand",
            A7        => "A7",
            A4        => "A4",
            A1        => "A1",
            B6        => "B6",
            B4        => "B4",
            B2        => "B2",
            C5        => "C5",
            C4        => "C4",
            C3        => "C3",
            D7        => "D7",
            D6        => "D6",
            D5        => "D5",
            D3        => "D3",
            D2        => "D2",
            D1        => "D1",
            E5        => "E5",
            E4        => "E4",
            E3        => "E3",
            F6        => "F6",
            F4        => "F4",
            F2        => "F2",
            G7        => "G7",
            G4        => "G4",
            G1        => "G1",
            Captured  => "Captured"
        }

    }

    pub fn get_rows(location: Location) -> Result<Vec<(Location, Location)>, ()> {
        use Location::*;
        match location {
            A7          => Ok(vec!((A1, A4), (D7, G7))),
            A4          => Ok(vec!((A7, A1), (B4, C4))),
            A1          => Ok(vec!((A7, A4), (D1, G1))),
            B6          => Ok(vec!((B4, B2), (D6, F6))),
            B4          => Ok(vec!((B2, B6), (A4, C4))),
            B2          => Ok(vec!((B6, B4), (D2, F2))),
            C5          => Ok(vec!((C4, C3), (D5, E5))),
            C4          => Ok(vec!((C3, C5), (A4, B4))),
            C3          => Ok(vec!((C4, C5), (D3, E3))),
            D7          => Ok(vec!((D5, D6), (A7, G7))),
            D6          => Ok(vec!((D5, D7), (B6, F6))),
            D5          => Ok(vec!((D6, D7), (C5, E5))),
            D3          => Ok(vec!((D2, D1), (C3, E3))),
            D2          => Ok(vec!((D1, D3), (B2, F2))),
            D1          => Ok(vec!((D2, D3), (A1, G1))),
            E5          => Ok(vec!((E4, E3), (C5, D5))),
            E4          => Ok(vec!((E3, E5), (F4, G4))),
            E3          => Ok(vec!((E4, E5), (C3, D3))),
            F6          => Ok(vec!((F2, F4), (D6, B6))),
            F4          => Ok(vec!((F2, F6), (E4, G4))),
            F2          => Ok(vec!((F6, F4), (D2, B2))),
            G7          => Ok(vec!((G4, G1), (D7, A7))),
            G4          => Ok(vec!((G1, G7), (E4, F4))),
            G1          => Ok(vec!((G4, G7), (D1, A1))),
            Hand | Captured => Err(())
        }
    }
}
