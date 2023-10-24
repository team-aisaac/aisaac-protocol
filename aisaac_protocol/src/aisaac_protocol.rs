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
pub struct StrategyPcCpmmand {
    pub protocol_version: u8,
    pub data_type: u8,


    pub halt_flag: bool,
    pub stop_game_flag: bool,
    pub ball_placement_flag: bool,
    pub ball_placement_team: bool,
    pub in_game: bool,
    pub robot_position_init: bool,

    // Dribble
    pub dribble_state: bool,
    pub dribble_advance: bool,
    pub dribble_enabble_error: u16,
    pub dribble_target_ball_x: i16,
    pub dribble_target_ball_y: i16,
    pub dribble_type: u8,
    // Kick
    pub ball_kick_state: bool,
    pub free_kick_flag: bool,
    pub ball_kick: bool,
    pub kick_straight: bool,
    pub ball_target_allowable_error: u16,
    pub target_ball_x: i16,
    pub target_ball_y: i16,
    pub kick_type: u8,
    // Target position
    pub robot_position_target_x: i16,
    pub robot_position_target_y: i16,
    pub robot_position_target_theta: i16,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct SslVisionRobotData {
    pub x: i16,
    pub y: i16,
    pub theta: i16,
    pub vx: i16,
    pub vy: i16,
    pub omega: i16,
    pub camera_valid: bool,
    pub data_valid: bool,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct SslVisionBallData {
    pub x: i16,
    pub y: i16,
    pub vx: i16,
    pub vy: i16,
    pub camera_valid: bool,
    pub data_valid: bool,
}

const MAX_OBSTACLE_NUM: usize = 31;
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct VisionData {
    pub protocol_version: u8,
    pub data_type: u8,
    pub current_pose: SslVisionRobotData,
    pub ball_position: SslVisionBallData,
    pub number_of_obstacles: u8,
    pub obstacles: [SslVisionRobotData; MAX_OBSTACLE_NUM],
}


#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct ManualControllerData {
    pub protocol_version: u8,
    pub data_type: u8,
    pub controller_start: bool,
    pub robot_vx: i32,
    pub robot_vy: i32,
    pub robot_vw: i32,
    pub dribbler_on: bool,
    pub kick_straight: bool,
    pub kick_tip: bool,
    pub emergency_stop: bool,
}


#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct RobotOdometryData {
    pub protocol_version: u8,
    pub data_type: u8,
    pub robot_position_x: i16,
    pub robot_position_y: i16,
    pub robot_position_theta: i16,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct RobotObservedBallData {
    pub protocol_version: u8,
    pub data_type: u8,
    pub ball_position_x: i16,
    pub ball_position_y: i16,
}

pub const PROTOCOL_VERSION: u8 = 0b00100010;
pub const STRATEGY_PC_COMMAND_DATA_TYPE: u8 = 0b10100001;
pub const DWA_RESULT_DATA_TYPE: u8 = 0b10100010;
pub const VISION_DATA_DATA_TYPE: u8 = 0b10100011;
pub const MANUAL_CONTROLLER_DATA_TYPE: u8 = 0b10100100;
pub const ROBOT_ODOMETRY_DATA_TYPE: u8 = 0b10100101;
pub const ROBOT_OBSERVED_BALL_DATA_TYPE: u8 = 0b10100110;
pub const ROBOT_DEBUG_DATA_TYPE: u8 = 0b10100111;

impl From<&[u8]> for StrategyPcCpmmand {
    fn from(rx: &[u8]) -> Self {
        let mut _buffer_index: usize = 0;
        assert_eq!(rx[_buffer_index], PROTOCOL_VERSION);
        _buffer_index += 1;
        assert_eq!(rx[_buffer_index], STRATEGY_PC_COMMAND_DATA_TYPE);
        _buffer_index += 1;

        // Init struct
        let mut command: StrategyPcCpmmand = StrategyPcCpmmand {
            protocol_version: PROTOCOL_VERSION,
            data_type: STRATEGY_PC_COMMAND_DATA_TYPE,
            halt_flag: false,
            stop_game_flag: false,
            ball_placement_flag: false,
            ball_placement_team: false,
            in_game: false,
            robot_position_init: false,
            dribble_state: false,
            dribble_advance: false,
            dribble_enabble_error: 0,
            dribble_target_ball_x: 0,
            dribble_target_ball_y: 0,
            dribble_type: 0,
            ball_kick_state: false,
            free_kick_flag: false,
            ball_kick: false,
            kick_straight: false,
            ball_target_allowable_error: 0,
            target_ball_x: 0,
            target_ball_y: 0,
            kick_type: 0,
            robot_position_target_x: 0,
            robot_position_target_y: 0,
            robot_position_target_theta: 0,
        };

        command.halt_flag = (rx[_buffer_index] & 0b10000000) == 0b10000000;
        command.stop_game_flag = (rx[_buffer_index] & 0b1000000) == 0b1000000;
        command.ball_placement_flag = (rx[_buffer_index] & 0b100000) == 0b100000;
        command.ball_placement_team = (rx[_buffer_index] & 0b10000) == 0b10000;
        command.in_game = (rx[_buffer_index] & 0b1000) == 0b1000;
        command.robot_position_init = (rx[_buffer_index] & 0b100) == 0b100;
        // Dribble
        command.dribble_state = (rx[_buffer_index] & 0b10) == 0b10;
        command.dribble_advance = (rx[_buffer_index] & 0b1) == 0b1;
        _buffer_index += 1;
        command.dribble_enabble_error = u16::from_be_bytes(rx[_buffer_index..(_buffer_index+2)].try_into().unwrap());
        _buffer_index += 2;
        command.dribble_target_ball_x = u16::from_be_bytes(rx[_buffer_index..(_buffer_index+2)].try_into().unwrap()) as i16;
        _buffer_index += 2;
        command.dribble_target_ball_y = u16::from_be_bytes(rx[_buffer_index..(_buffer_index+2)].try_into().unwrap()) as i16;
        _buffer_index += 2;
        command.dribble_type = rx[_buffer_index];
        _buffer_index += 1;
        // Kick
        command.ball_kick_state = (rx[_buffer_index] & 0b1000) == 0b1000;
        command.free_kick_flag = (rx[_buffer_index] & 0b100) == 0b100;
        command.ball_kick = (rx[_buffer_index] & 0b10) == 0b10;
        command.kick_straight = (rx[_buffer_index] & 0b1) == 0b1;
        _buffer_index += 1;
        command.ball_target_allowable_error = u16::from_be_bytes(rx[_buffer_index..(_buffer_index+2)].try_into().unwrap());
        _buffer_index += 2;
        command.target_ball_x = u16::from_be_bytes(rx[_buffer_index..(_buffer_index+2)].try_into().unwrap()) as i16;
        _buffer_index += 2;
        command.target_ball_y = u16::from_be_bytes(rx[_buffer_index..(_buffer_index+2)].try_into().unwrap()) as i16;
        _buffer_index += 2;
        command.kick_type = rx[_buffer_index];
        _buffer_index += 1;
        // Target position
        command.robot_position_target_x = u16::from_be_bytes(rx[_buffer_index..(_buffer_index+2)].try_into().unwrap()) as i16;
        _buffer_index += 2;
        command.robot_position_target_y = u16::from_be_bytes(rx[_buffer_index..(_buffer_index+2)].try_into().unwrap()) as i16;
        _buffer_index += 2;
        command.robot_position_target_theta = u16::from_be_bytes(rx[_buffer_index..(_buffer_index+2)].try_into().unwrap()) as i16;
        _buffer_index += 2;

        command
    }
}

impl From<StrategyPcCpmmand> for [u8; 24] {
    fn from(command: StrategyPcCpmmand) -> Self {
        let mut _buffer = Vec::new();
        let mut _buffer_index: usize = 0;

        _buffer.push(PROTOCOL_VERSION);
        _buffer.push(STRATEGY_PC_COMMAND_DATA_TYPE);

        _buffer.push(
            (command.halt_flag as u8) << 7 |
            (command.stop_game_flag as u8) << 6 |
            (command.ball_placement_flag as u8) << 5 |
            (command.ball_placement_team as u8) << 4 |
            (command.in_game as u8) << 3 |
            (command.robot_position_init as u8) << 2|
            (command.dribble_state as u8) << 1 |
            command.dribble_advance as u8);
        // Dribble
        _buffer.extend(command.dribble_enabble_error.to_be_bytes());
        _buffer.extend(command.dribble_target_ball_x.to_be_bytes());
        _buffer.extend(command.dribble_target_ball_y.to_be_bytes());
        _buffer.push(command.dribble_type);
        // Kick
        _buffer.push((command.ball_kick_state as u8) << 3 |
            (command.free_kick_flag as u8) << 2 |
            (command.ball_kick as u8) << 1 |
            command.kick_straight as u8);
        _buffer.extend(command.ball_target_allowable_error.to_be_bytes());
        _buffer.extend(command.target_ball_x.to_be_bytes());
        _buffer.extend(command.target_ball_y.to_be_bytes());
        _buffer.push(command.kick_type);
        // Target position
        _buffer.extend(command.robot_position_target_x.to_be_bytes());
        _buffer.extend(command.robot_position_target_y.to_be_bytes());
        _buffer.extend(command.robot_position_target_theta.to_be_bytes());

        _buffer.try_into().unwrap()
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
        let mut _current_pose = SslVisionRobotData { x: 0, y: 0, theta: 0, vx: 0, vy: 0, omega: 0, camera_valid: false, data_valid: false };
        _current_pose.x = u16::from_be_bytes(rx[_buffer_index..(_buffer_index+2)].try_into().unwrap()) as i16;
        _buffer_index += 2;
        _current_pose.y = u16::from_be_bytes(rx[_buffer_index..(_buffer_index+2)].try_into().unwrap()) as i16;
        _buffer_index += 2;
        _current_pose.theta = u16::from_be_bytes(rx[_buffer_index..(_buffer_index+2)].try_into().unwrap()) as i16;
        _buffer_index += 2;
        _current_pose.vx = u16::from_be_bytes(rx[_buffer_index..(_buffer_index+2)].try_into().unwrap()) as i16;
        _buffer_index += 2;
        _current_pose.vy = u16::from_be_bytes(rx[_buffer_index..(_buffer_index+2)].try_into().unwrap()) as i16;
        _buffer_index += 2;
        _current_pose.omega = u16::from_be_bytes(rx[_buffer_index..(_buffer_index+2)].try_into().unwrap()) as i16;
        _buffer_index += 2;
        _current_pose.camera_valid = (rx[_buffer_index] & 0b10) == 0b10;
        _current_pose.data_valid = (rx[_buffer_index] & 0b1) == 0b1;
        _buffer_index += 1;

        // Ball Position
        let mut _ball_position = SslVisionBallData { x: 0, y: 0, vx: 0, vy: 0, camera_valid: false, data_valid: false };
        _ball_position.x = u16::from_be_bytes(rx[_buffer_index..(_buffer_index+2)].try_into().unwrap()) as i16;
        _buffer_index += 2;
        _ball_position.y = u16::from_be_bytes(rx[_buffer_index..(_buffer_index+2)].try_into().unwrap()) as i16;
        _buffer_index += 2;
        _ball_position.vx = u16::from_be_bytes(rx[_buffer_index..(_buffer_index+2)].try_into().unwrap()) as i16;
        _buffer_index += 2;
        _ball_position.vy = u16::from_be_bytes(rx[_buffer_index..(_buffer_index+2)].try_into().unwrap()) as i16;
        _buffer_index += 2;
        _ball_position.camera_valid = (rx[_buffer_index] & 0b10) == 0b10;
        _ball_position.data_valid = (rx[_buffer_index] & 0b1) == 0b1;
        _buffer_index += 1;

        // Obstacles
        let _number_of_obstacles = rx[_buffer_index] as usize;
        _buffer_index += 1;
        let mut _obstacles: [SslVisionRobotData; 31] = [ SslVisionRobotData { x: 0, y: 0, theta: 0, vx: 0, vy: 0, omega: 0, camera_valid: false, data_valid: false }; 31];
        for obstacle_index in 0.._number_of_obstacles {
            _obstacles[obstacle_index].x = u16::from_be_bytes(rx[_buffer_index..(_buffer_index+2)].try_into().unwrap()) as i16;
            _buffer_index += 2;
            _obstacles[obstacle_index].y = u16::from_be_bytes(rx[_buffer_index..(_buffer_index+2)].try_into().unwrap()) as i16;
            _buffer_index += 2;
            _obstacles[obstacle_index].theta = u16::from_be_bytes(rx[_buffer_index..(_buffer_index+2)].try_into().unwrap()) as i16;
            _buffer_index += 2;
            _obstacles[obstacle_index].vx = u16::from_be_bytes(rx[_buffer_index..(_buffer_index+2)].try_into().unwrap()) as i16;
            _buffer_index += 2;
            _obstacles[obstacle_index].vy = u16::from_be_bytes(rx[_buffer_index..(_buffer_index+2)].try_into().unwrap()) as i16;
            _buffer_index += 2;
            _obstacles[obstacle_index].omega = u16::from_be_bytes(rx[_buffer_index..(_buffer_index+2)].try_into().unwrap()) as i16;
            _buffer_index += 2;
            _obstacles[obstacle_index].camera_valid = (rx[_buffer_index] & 0b10) == 0b10;
            _obstacles[obstacle_index].data_valid = (rx[_buffer_index] & 0b1) == 0b1;
            _buffer_index += 1;
        }
        
        VisionData { protocol_version: PROTOCOL_VERSION, data_type: VISION_DATA_DATA_TYPE, current_pose: _current_pose, ball_position: _ball_position, number_of_obstacles: _number_of_obstacles as u8, obstacles: _obstacles }
    }
}
