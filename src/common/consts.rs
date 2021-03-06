use core::f64::consts::PI;
use smash::lib::lua_const::*;

/// Hitbox Visualization
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HitboxVisualization {
    Off = 0,
    On = 1,
}

// DI
/*
 0, 0.785398, 1.570796, 2.356194, -3.14159, -2.356194,  -1.570796, -0.785398
 0, pi/4,     pi/2,     3pi/4,    pi,       5pi/4,      3pi/2,     7pi/4
*/

/// DI
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    None = 0,
    Right = 1,
    UpRight = 2,
    Up = 3,
    UpLeft = 4,
    Left = 5,
    DownLeft = 6,
    Down = 7,
    DownRight = 8,
    // lol what goes here jug smh my head
    Random = 9,
}

impl From<i32> for Direction {
    fn from(x: i32) -> Self {
        match x {
            0 => Direction::None,
            1 => Direction::Right,
            2 => Direction::UpRight,
            3 => Direction::Up,
            4 => Direction::UpLeft,
            5 => Direction::Left,
            6 => Direction::DownLeft,
            7 => Direction::Down,
            8 => Direction::DownRight,
            9 => Direction::Random,
            _ => panic!("Invalid direction {}", x),
        }
    }
}

//pub static FIGHTER_FACING_LEFT: f32 = 1.0;
pub static FIGHTER_FACING_RIGHT: f32 = -1.0;
pub static ANGLE_NONE: f64 = -69.0;
pub fn direction_to_angle(direction: Direction) -> f64 {
    match direction {
        Direction::None => ANGLE_NONE,
        Direction::Random => ANGLE_NONE, // Random Direction should be handled by the calling context
        _ => (direction as i32 - 1) as f64 * PI / 4.0,
    }
}

/// Mash Attack States
#[repr(i32)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Attack {
    Nair = 0,
    Fair = 1,
    Bair = 2,
    UpAir = 3,
    Dair = 4,
    NeutralB = 5,
    SideB = 6,
    UpB = 7,
    DownB = 8,
    UpSmash = 9,
    FSmash = 10,
    DSmash = 11,
    Grab = 12,
    Jab = 13,
    Ftilt = 14,
    Utilt = 15,
    Dtilt = 16,
    DashAttack = 17,
    Nothing = 9999,
}

impl From<i32> for Attack {
    fn from(x: i32) -> Self {
        use Attack::*;

        match x {
            0 => Nair,
            1 => Fair,
            2 => Bair,
            3 => UpAir,
            4 => Dair,
            5 => NeutralB,
            6 => SideB,
            7 => UpB,
            8 => DownB,
            9 => UpSmash,
            10 => FSmash,
            11 => DSmash,
            12 => Grab,
            13 => Jab,
            14 => Ftilt,
            15 => Utilt,
            16 => Dtilt,
            17 => DashAttack,
            _ => Nothing,
        }
    }
}

// Ledge Option
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LedgeOption {
    None = 0,
    Random = 1,
    Neutral = 2,
    Roll = 3,
    Jump = 4,
    Attack = 5,
}

impl From<i32> for LedgeOption {
    fn from(x: i32) -> Self {
        use LedgeOption::*;

        match x {
            0 => None,
            1 => Random,
            2 => Neutral,
            3 => Roll,
            4 => Jump,
            5 => Attack,
            _ => panic!("Invalid ledge option {}", x),
        }
    }
}

impl LedgeOption {
    pub fn into_status(&self) -> Option<i32> {
        Some(match self {
            LedgeOption::Neutral => *FIGHTER_STATUS_KIND_CLIFF_CLIMB,
            LedgeOption::Roll => *FIGHTER_STATUS_KIND_CLIFF_ESCAPE,
            LedgeOption::Jump => *FIGHTER_STATUS_KIND_CLIFF_JUMP1,
            LedgeOption::Attack => *FIGHTER_STATUS_KIND_CLIFF_ATTACK,
            _ => return None,
        })
    }
}

// Tech Option
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TechOption {
    None = 0,
    Random = 1,
    InPlace = 2,
    Roll = 3,
    Miss = 4,
}

impl From<i32> for TechOption {
    fn from(x: i32) -> Self {
        use TechOption::*;

        match x {
            0 => None,
            1 => Random,
            2 => InPlace,
            3 => Roll,
            4 => Miss,
            _ => panic!("Invalid tech option {}", x),
        }
    }
}

/// Mash States
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mash {
    None = 0,
    Airdodge = 1,
    Jump = 2,
    Attack = 3,
    Spotdodge = 4,
    RollForward = 5,
    RollBack = 6,
    Random = 7,
    Shield = 99,
}

impl From<i32> for Mash {
    fn from(x: i32) -> Self {
        match x {
            0 => Mash::None,
            1 => Mash::Airdodge,
            2 => Mash::Jump,
            3 => Mash::Attack,
            4 => Mash::Spotdodge,
            5 => Mash::RollForward,
            6 => Mash::RollBack,
            7 => Mash::Random,
            _ => panic!("Invalid mash state {}", x),
        }
    }
}

/// Shield States
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shield {
    None = 0,
    Infinite = 1,
    Hold = 2,
}

// Defensive States
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Defensive {
    None = 0,
    Random = 1,
    Spotdodge = 2,
    Roll = 3,
    Jab = 4,
    Shield = 5,
}

impl From<i32> for Defensive {
    fn from(x: i32) -> Self {
        use Defensive::*;

        match x {
            0 => None,
            1 => Random,
            2 => Spotdodge,
            3 => Roll,
            4 => Jab,
            5 => Shield,
            _ => panic!("Invalid mash state {}", x),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OnOff {
    Off = 0,
    On = 1,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Nothing = 0,
    Airdodge = 1,
    Jump = 2,
    Spotdodge = 3,
    RollForward = 4,
    RollBack = 5,
    Nair = 6,
    Fair = 7,
    Bair = 8,
    UpAir = 9,
    Dair = 10,
    NeutralB = 11,
    SideB = 12,
    UpB = 13,
    DownB = 14,
    UpSmash = 15,
    FSmash = 16,
    DSmash = 17,
    Grab = 18,
    Jab = 19,
    Ftilt = 20,
    Utilt = 21,
    Dtilt = 22,
    DashAttack = 23,
    Shield = 99,
}

impl Action {
    pub fn into_attack_air_kind(&self) -> Option<i32> {
        use Action::*;

        Some(match self {
            Nair => *FIGHTER_COMMAND_ATTACK_AIR_KIND_N,
            Fair => *FIGHTER_COMMAND_ATTACK_AIR_KIND_F,
            Bair => *FIGHTER_COMMAND_ATTACK_AIR_KIND_B,
            Dair => *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW,
            UpAir => *FIGHTER_COMMAND_ATTACK_AIR_KIND_HI,
            _ => return None,
        })
    }
}

// To satisfy the unused warning
impl From<i32> for Action {
    fn from(x: i32) -> Self {
        use Action::*;

        match x {
            0 => Nothing,
            1 => Airdodge,
            2 => Jump,
            3 => Spotdodge,
            4 => RollForward,
            5 => RollBack,
            6 => Nair,
            7 => Fair,
            8 => Bair,
            9 => UpAir,
            10 => Dair,
            11 => NeutralB,
            12 => SideB,
            13 => UpB,
            14 => DownB,
            15 => UpSmash,
            16 => FSmash,
            17 => DSmash,
            18 => Grab,
            19 => Jab,
            20 => Ftilt,
            21 => Utilt,
            22 => Dtilt,
            23 => DashAttack,
            99 => Shield,
            _ => Nothing,
        }
    }
}

#[repr(C)]
pub struct TrainingModpackMenu {
    pub hitbox_vis: HitboxVisualization,
    pub di_state: Direction,
    pub left_stick: Direction, // Currently only used for air dodge direction
    pub mash_attack_state: Attack,
    pub follow_up: Action,
    pub ledge_state: LedgeOption,
    pub tech_state: TechOption,
    pub mash_state: Mash,
    pub shield_state: Shield,
    pub defensive_state: Defensive,
    pub oos_offset: u32,
    pub reaction_time: u32,
    pub mash_in_neutral: OnOff,
    pub fast_fall: OnOff,
    pub fast_fall_delay: u32,
    pub falling_aerials: OnOff,
    pub full_hop: OnOff,
}

// Fighter Ids
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FighterId {
    Player = 0,
    CPU = 1,
}
