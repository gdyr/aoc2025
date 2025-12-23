use aoc2025::read_lines;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(i8)]
enum Direction {
    Left = -1,
    Right = 1,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => write!(f, "Left"),
            Self::Right => write!(f, "Right"),
        }
    }
}

#[derive(Debug)]
struct Dial {
    position: u8,
}

impl Dial {
    fn turn(&mut self, rotation: &Rotation) -> u32 {

        let steps: i32 = rotation.steps.cast_signed() * rotation.direction as i32;

        let mut zero_crossings = (steps / 100).unsigned_abs();
        let rem_steps = steps % 100;

        let mut new_position = i32::from(self.position) + rem_steps;

        // Correct out-of-bounds caused by <100 step rotation
        if new_position < 0 {
            new_position += 100;

            // Going negative means we crossed zero - unless we were already at zero.
            if self.position != 0 { zero_crossings += 1; }
        }
        if new_position > 99 {
            new_position -= 100;

            // If we landed on zero exactly, this crossing
            // will be captured by the new_position == 0 check
            if new_position != 0 { zero_crossings += 1; }
        }

        // Count landing on zero as a zero-crossing,
        // unless it was achieved by an exact rotation,
        // in which case this is captured already.
        if new_position == 0 && rem_steps != 0 { zero_crossings += 1; }

        self.position =
            u8::try_from(new_position).expect("New position should alwayas be in the range 0..99");

        zero_crossings
    }
}

#[derive(Debug, PartialEq)]
struct Rotation {
    direction: Direction,
    steps: u32,
}

#[derive(Debug, PartialEq)]
enum RotationParseError {
    IncorrectStartOfLineCharacter,
    #[allow(dead_code)]
    ParseIntError(ParseIntError),
}

impl TryFrom<&str> for Rotation {
    type Error = RotationParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let direction = match value.chars().next() {
            Some('L') => Ok(Direction::Left),
            Some('R') => Ok(Direction::Right),
            _ => Err(Self::Error::IncorrectStartOfLineCharacter),
        }?;
        let steps = value[1..].parse().map_err(Self::Error::ParseIntError)?;
        Ok(Self { direction, steps })
    }
}

impl Default for Dial {
    fn default() -> Self {
        Self { position: 50 }
    }
}

#[cfg(test)]
mod test {
    
    use crate::Rotation;
    use crate::Direction;
    use crate::Dial;

    #[test]
    fn parse_succeeds() {
        let line = "L50";
        assert_eq!(
            Rotation::try_from(line),
            Ok(Rotation {
                direction: Direction::Left,
                steps: 50
            })
        );

        let line = "R1220";
        assert_eq!(
            Rotation::try_from(line),
            Ok(Rotation {
                direction: Direction::Right,
                steps: 1220
            })
        );
    }

    #[test]
    fn parse_fails() {

        assert_eq!(
            Rotation::try_from("X90"),
            Err(crate::RotationParseError::IncorrectStartOfLineCharacter)
        );

        assert!(matches!(
            Rotation::try_from("LYY"),
            Err(crate::RotationParseError::ParseIntError(_))
        ));

    }

    #[test]
    fn test_zero_crossings() {

        let mut dial = Dial::default();

        assert_eq!(dial.turn(&Rotation::try_from("L50").unwrap()), 1); // 50 L50 = 0, one ZC
        assert_eq!(dial.turn(&Rotation::try_from("L100").unwrap()), 1); // 0 L100 = 0, one ZC
        assert_eq!(dial.turn(&Rotation::try_from("R100").unwrap()), 1); // 0 R100 = 0, one ZC
        assert_eq!(dial.turn(&Rotation::try_from("R200").unwrap()), 2); // 0 R100 = 0, one ZC
        assert_eq!(dial.turn(&Rotation::try_from("L200").unwrap()), 2); // 0 R100 = 0, one ZC

        assert_eq!(dial.turn(&Rotation::try_from("L1").unwrap()), 0); // 0 L1 = 99, no ZC
        assert_eq!(dial.turn(&Rotation::try_from("R1").unwrap()), 1); // 99 R1 = 0, one ZC
        assert_eq!(dial.turn(&Rotation::try_from("L1").unwrap()), 0); // 0 L1 = 99, no ZC
        assert_eq!(dial.turn(&Rotation::try_from("R101").unwrap()), 2); // 0 R101 = 0, two ZC

        assert_eq!(dial.turn(&Rotation::try_from("R1").unwrap()), 0); // 0 R1 = 1, no ZC
        assert_eq!(dial.turn(&Rotation::try_from("L1").unwrap()), 1); // 1 L1 = 0, one ZC
        assert_eq!(dial.turn(&Rotation::try_from("R1").unwrap()), 0); // 0 R1 = 1, no ZC
        assert_eq!(dial.turn(&Rotation::try_from("L101").unwrap()), 2); // 1 L101 = 0, two ZC

        // Given example
        let mut dial = Dial::default();
        assert_eq!(dial.turn(&Rotation::try_from("L68").unwrap()), 1);
        assert_eq!(dial.turn(&Rotation::try_from("L30").unwrap()), 0);
        assert_eq!(dial.turn(&Rotation::try_from("R48").unwrap()), 1);
        assert_eq!(dial.turn(&Rotation::try_from("L5").unwrap()), 0);
        assert_eq!(dial.turn(&Rotation::try_from("R60").unwrap()), 1);
        assert_eq!(dial.turn(&Rotation::try_from("L55").unwrap()), 1);
        assert_eq!(dial.turn(&Rotation::try_from("L1").unwrap()), 0);
        assert_eq!(dial.turn(&Rotation::try_from("L99").unwrap()), 1);
        assert_eq!(dial.turn(&Rotation::try_from("R14").unwrap()), 0);
        assert_eq!(dial.turn(&Rotation::try_from("L82").unwrap()), 1);

    }

}

fn main() {
    assert!(std::env::args().len() >= 2, "Filename must be supplied.");
    let filename = std::env::args().collect::<Vec<_>>()[1].clone();

    let mut dial = Dial::default();

    let mut zero_stops = 0u32;
    let mut zero_crossings = 0u32;

    for (i, line) in read_lines(filename)
        .expect("Failed to read file.")
        .enumerate()
    {
        let line = line.unwrap_or_else(|_| panic!("Failed to read line {i}."));
        let rotation = Rotation::try_from(line.as_str())
            .unwrap_or_else(|e| panic!("Failed to parse line {i}: {e:?}"));

        let starting_position = dial.position;
        let turn_zero_crossings = dial.turn(&rotation);
        zero_crossings += turn_zero_crossings;

        if dial.position == 0 {
            zero_stops += 1;
        }

        println!(
            "Step {}, turn dial from {} to the {} by {} clicks, ends up at {} crossing zero {} times.",
            i,
            starting_position,
            rotation.direction.to_string().to_lowercase(),
            rotation.steps,
            dial.position,
            turn_zero_crossings
        );
    }

    println!("Zero-stopping count was {zero_stops}");
    println!("Zero-crossing count was {zero_crossings}");
}
