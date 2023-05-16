#[allow(dead_code)]
fn htons(u: u16) -> u16 {
    u.to_be()
}

#[allow(dead_code)]
fn ntohs(u: u16) -> u16 {
    u16::from_be(u)
}

#[allow(dead_code)]
fn htonl(u: u32) -> u32 {
    u.to_be()
}

#[allow(dead_code)]
fn ntohl(u: u32) -> u32 {
    u32::from_be(u)
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub theta: i32,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Obstacle {
    pub x: i32,
    pub y: i32,
    pub theta: i32,
    pub vx: i32,
    pub vy: i32,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct StrategyPcCpmmand {
    pub protocol_version: u8,
    pub data_type: u8,

    pub goal_pose: Position,
    pub middle_goal_pose: Position,
    pub prohibited_zone_ignore: bool,
    pub middle_target_flag: bool,
    pub halt_flag: bool,

    pub kick_power: u32,
    pub ball_goal: Position,
    pub ball_target_allowable_error: i32,
    pub kick_type: u8,
    pub ball_kick_state: bool,
    pub ball_kick: bool,
    pub ball_kick_active:bool,
    pub free_kick_flag: bool,

    pub dribble_power: u32,
    pub dribble_goal: Position,
    pub dribble_complete_distance: i32,
    pub dribble_state: bool,
    pub dribbler_active: bool,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct DwaResult {
    pub protocol_version: u8,
    pub data_type: u8,
    pub dwa_position: Position,
}

const MAX_OBSTACLE_NUM: usize = 31;
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct VisionData {
    pub protocol_version: u8,
    pub data_type: u8,
    pub current_pose: Position,
    pub ball_position: Position,
    pub number_of_obstacles: u8,
    pub obstacles: [Obstacle; MAX_OBSTACLE_NUM],
}

pub const PROTOCOL_VERSION: u8 = 0b00100001;
pub const STRATEGY_PC_COMMAND_DATA_TYPE: u8 = 0b10100001;
pub const DWA_RESULT_DATA_TYPE: u8 = 0b10100010;
pub const VISION_DATA_DATA_TYPE: u8 = 0b10100011;

impl From<&[u8]> for StrategyPcCpmmand {
    fn from(rx: &[u8]) -> Self {
        let mut _buffer_index: usize = 0;
        assert_eq!(rx[_buffer_index], PROTOCOL_VERSION);
        _buffer_index += 1;
        assert_eq!(rx[_buffer_index], STRATEGY_PC_COMMAND_DATA_TYPE);
        _buffer_index += 1;

        let mut command: StrategyPcCpmmand = StrategyPcCpmmand { protocol_version: PROTOCOL_VERSION, data_type: STRATEGY_PC_COMMAND_DATA_TYPE, goal_pose: Position { x: 0, y: 0, theta: 0 }, middle_goal_pose: Position { x: 0, y: 0, theta: 0 }, prohibited_zone_ignore: false, middle_target_flag: false, halt_flag: false, kick_power: 0, ball_goal: Position { x: 0, y: 0, theta: 0 }, ball_target_allowable_error: 0, kick_type: 0, ball_kick_state: false, ball_kick: false, ball_kick_active: false, free_kick_flag: false, dribble_power: 0, dribble_goal: Position { x: 0, y: 0, theta: 0 }, dribble_complete_distance: 0, dribble_state: false, dribbler_active: false };

        command.goal_pose.x = u32::from_be_bytes(rx[_buffer_index..(_buffer_index+4)].try_into().unwrap()) as i32;
        _buffer_index += 4;
        command.goal_pose.y = u32::from_be_bytes(rx[_buffer_index..(_buffer_index+4)].try_into().unwrap()) as i32;
        _buffer_index += 4;
        command.goal_pose.theta = u32::from_be_bytes(rx[_buffer_index..(_buffer_index+4)].try_into().unwrap()) as i32;
        _buffer_index += 4;
        command.middle_goal_pose.x = u32::from_be_bytes(rx[_buffer_index..(_buffer_index+4)].try_into().unwrap()) as i32;
        _buffer_index += 4;
        command.middle_goal_pose.y = u32::from_be_bytes(rx[_buffer_index..(_buffer_index+4)].try_into().unwrap()) as i32;
        _buffer_index += 4;
        command.middle_goal_pose.theta = u32::from_be_bytes(rx[_buffer_index..(_buffer_index+4)].try_into().unwrap()) as i32;
        _buffer_index += 4;
        command.prohibited_zone_ignore = (rx[_buffer_index] & 0b100) == 0b100;
        command.middle_target_flag = (rx[_buffer_index] & 0b10) == 0b10;
        command.halt_flag = (rx[_buffer_index] & 0b1) == 0b1;
        _buffer_index += 1;
        // Kick
        command.kick_power = u32::from_be_bytes(rx[_buffer_index..(_buffer_index+4)].try_into().unwrap());
        _buffer_index += 4;
        command.ball_goal.x = u32::from_be_bytes(rx[_buffer_index..(_buffer_index+4)].try_into().unwrap()) as i32;
        _buffer_index += 4;
        command.ball_goal.y = u32::from_be_bytes(rx[_buffer_index..(_buffer_index+4)].try_into().unwrap()) as i32;
        _buffer_index += 4;
        command.ball_goal.theta = u32::from_be_bytes(rx[_buffer_index..(_buffer_index+4)].try_into().unwrap()) as i32;
        _buffer_index += 4;
        command.ball_target_allowable_error = u32::from_be_bytes(rx[_buffer_index..(_buffer_index+4)].try_into().unwrap()) as i32;
        _buffer_index += 4;
        command.kick_type = rx[_buffer_index];
        _buffer_index += 1;
        command.ball_kick_state = (rx[_buffer_index] & 0b1000) == 0b1000;
        command.ball_kick = (rx[_buffer_index] & 0b100) == 0b100;
        command.ball_kick_active = (rx[_buffer_index] & 0b10) == 0b10;
        command.free_kick_flag = (rx[_buffer_index] & 0b1) == 0b1;
        _buffer_index += 1;
        // Dribble
        command.dribble_power = u32::from_be_bytes(rx[_buffer_index..(_buffer_index+4)].try_into().unwrap());
        _buffer_index += 4;
        command.dribble_goal.x = u32::from_be_bytes(rx[_buffer_index..(_buffer_index+4)].try_into().unwrap()) as i32;
        _buffer_index += 4;
        command.dribble_goal.y = u32::from_be_bytes(rx[_buffer_index..(_buffer_index+4)].try_into().unwrap()) as i32;
        _buffer_index += 4;
        command.dribble_goal.theta = u32::from_be_bytes(rx[_buffer_index..(_buffer_index+4)].try_into().unwrap()) as i32;
        _buffer_index += 4;
        command.dribble_complete_distance = u32::from_be_bytes(rx[_buffer_index..(_buffer_index+4)].try_into().unwrap()) as i32;
        _buffer_index += 4;
        command.dribble_state = (rx[_buffer_index] & 0b10) == 0b10;
        command.dribbler_active = (rx[_buffer_index] & 0b1) == 0b1;
        _buffer_index += 1;

        command
    }
}

impl From<StrategyPcCpmmand> for [u8; 70] {
    fn from(command: StrategyPcCpmmand) -> Self {
        let mut _buffer = Vec::new();
        let mut _buffer_index: usize = 0;

        _buffer.push(PROTOCOL_VERSION);
        _buffer.push(STRATEGY_PC_COMMAND_DATA_TYPE);

        _buffer.extend(command.goal_pose.x.to_be_bytes());
        _buffer.extend(command.goal_pose.y.to_be_bytes());
        _buffer.extend(command.goal_pose.theta.to_be_bytes());
        _buffer.extend(command.middle_goal_pose.x.to_be_bytes());
        _buffer.extend(command.middle_goal_pose.y.to_be_bytes());
        _buffer.extend(command.middle_goal_pose.theta.to_be_bytes());
        _buffer.push((command.prohibited_zone_ignore as u8) << 2 |
            (command.middle_target_flag as u8) << 1 |
            command.halt_flag as u8);
        // Kick
        _buffer.extend(command.kick_power.to_be_bytes());
        _buffer.extend(command.ball_goal.x.to_be_bytes());
        _buffer.extend(command.ball_goal.y.to_be_bytes());
        _buffer.extend(command.ball_goal.theta.to_be_bytes());
        _buffer.extend(command.ball_target_allowable_error.to_be_bytes());
        _buffer.push(command.kick_type);
        _buffer.push((command.ball_kick_state as u8) << 3 |
            (command.ball_kick as u8) << 2 |
            (command.ball_kick_active as u8) << 1 |
            command.free_kick_flag as u8);
        // Dribble
        _buffer.extend(command.dribble_power.to_be_bytes());
        _buffer.extend(command.dribble_goal.x.to_be_bytes());
        _buffer.extend(command.dribble_goal.y.to_be_bytes());
        _buffer.extend(command.dribble_goal.theta.to_be_bytes());
        _buffer.extend(command.dribble_complete_distance.to_be_bytes());
        _buffer.push((command.dribble_state as u8) << 1 |
            command.dribbler_active as u8);

        _buffer.try_into().unwrap()
    }
}

impl From<DwaResult> for [u8; 14] {
    fn from(dwa: DwaResult) -> Self {
        let mut _buffer_index: usize = 0;
        
        let dwa_x: [u8; 4] = (dwa.dwa_position.x as u32).to_be_bytes();
        let dwa_y: [u8; 4] = (dwa.dwa_position.y as u32).to_be_bytes();
        let dwa_theta: [u8; 4] = (dwa.dwa_position.theta as u32).to_be_bytes();
        {
            [PROTOCOL_VERSION, DWA_RESULT_DATA_TYPE, dwa_x[0], dwa_x[1], dwa_x[2], dwa_x[3], dwa_y[0], dwa_y[1], dwa_y[2], dwa_y[3], dwa_theta[0], dwa_theta[1], dwa_theta[2], dwa_theta[3]]
        }
    }
}
impl From<&[u8]> for VisionData {
    fn from(rx: &[u8]) -> Self {
        let mut _buffer_index: usize = 0;
        assert_eq!(rx[_buffer_index], PROTOCOL_VERSION);
        _buffer_index += 1;
        assert_eq!(rx[_buffer_index], VISION_DATA_DATA_TYPE);
        _buffer_index += 1;

        // Current Pose
        let mut _current_pose = Position { x: 0, y: 0, theta: 0 };
        _current_pose.x = u32::from_be_bytes(rx[_buffer_index..(_buffer_index+4)].try_into().unwrap()) as i32;
        _buffer_index += 4;
        _current_pose.y = u32::from_be_bytes(rx[_buffer_index..(_buffer_index+4)].try_into().unwrap()) as i32;
        _buffer_index += 4;
        _current_pose.theta = u32::from_be_bytes(rx[_buffer_index..(_buffer_index+4)].try_into().unwrap()) as i32;
        _buffer_index += 4;
        // Ball Position
        let mut _ball_position = Position { x: 0, y: 0, theta: 0 };
        _ball_position.x = u32::from_be_bytes(rx[_buffer_index..(_buffer_index+4)].try_into().unwrap()) as i32;
        _buffer_index += 4;
        _ball_position.y = u32::from_be_bytes(rx[_buffer_index..(_buffer_index+4)].try_into().unwrap()) as i32;
        _buffer_index += 4;
        _ball_position.theta = u32::from_be_bytes(rx[_buffer_index..(_buffer_index+4)].try_into().unwrap()) as i32;
        _buffer_index += 4;
        // Obstacles
        let _number_of_obstacles = rx[_buffer_index] as usize;
        _buffer_index += 1;
        let mut _obstacles: [Obstacle; 31] = [Obstacle { x: 0, y: 0, theta: 0, vx: 0, vy: 0 }; 31];
        for obstacle_index in 0.._number_of_obstacles {
            _obstacles[obstacle_index].x = u32::from_be_bytes(rx[_buffer_index..(_buffer_index+4)].try_into().unwrap()) as i32;
            _buffer_index += 4;
            _obstacles[obstacle_index].y = u32::from_be_bytes(rx[_buffer_index..(_buffer_index+4)].try_into().unwrap()) as i32;
            _buffer_index += 4;
            _obstacles[obstacle_index].theta = u32::from_be_bytes(rx[_buffer_index..(_buffer_index+4)].try_into().unwrap()) as i32;
            _buffer_index += 4;
            _obstacles[obstacle_index].vx = u32::from_be_bytes(rx[_buffer_index..(_buffer_index+4)].try_into().unwrap()) as i32;
            _buffer_index += 4;
            _obstacles[obstacle_index].vy = u32::from_be_bytes(rx[_buffer_index..(_buffer_index+4)].try_into().unwrap()) as i32;
            _buffer_index += 4;
        }
        
        VisionData { protocol_version: PROTOCOL_VERSION, data_type: VISION_DATA_DATA_TYPE, current_pose: _current_pose, ball_position: _ball_position, number_of_obstacles: _number_of_obstacles as u8, obstacles: _obstacles }
    }
}
