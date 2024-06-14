pub(crate) enum Command {
    LEFT,
    RIGHT,
    DOWN,
    ROTATE,
    SMASH,
    NONE,
    EXIT,
}

impl Command {
    pub(crate) fn from_bytes(bytes: [u8; 4]) -> Self {
        if bytes[0] == 27 {
            if bytes[2] == 66 {
                Self::DOWN
            } else if bytes[2] == 68 {
                Self::LEFT
            } else if bytes[2] == 67 {
                Self::RIGHT
            } else if bytes[2] == 65 {
                Self::ROTATE
            } else {
                Self::NONE
            }
        } else if bytes[0] == 4 {
            Self::EXIT
        } else if bytes[0] == 32 {
            Self::SMASH
        } else {
            Self::NONE
        }
    }

    pub(crate) fn to_string(&self) -> String {
        match self {
            Command::LEFT => { String::from("LEFT") }
            Command::RIGHT => { String::from("RIGHT") }
            Command::DOWN => { String::from("DOWN") }
            Command::SMASH => { String::from("SMASH") }
            Command::NONE => { String::from("None") }
            Command::EXIT => { String::from("Exit") }
            Command::ROTATE => { String::from("ROTATE") }
        }
    }
}
