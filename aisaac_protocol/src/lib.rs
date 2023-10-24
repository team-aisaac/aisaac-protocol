pub mod aisaac_protocol;

#[cfg(test)]
mod tests {
    use crate::aisaac_protocol::StrategyPcCpmmand;

    use super::*;

    #[test]
    fn strategy_command_test() {
        let rx_command: [u8; 24] = [
            34, 161, 85, 3, 232, 7, 208, 248, 48, 1, 5, 11, 184, 15, 160, 240, 96, 2, 19, 136, 236, 120, 23, 112
            ];
        let command:StrategyPcCpmmand = aisaac_protocol::StrategyPcCpmmand::from(rx_command.as_slice());

        assert_eq!(command.protocol_version, 0b00100010);
        assert_eq!(command.data_type, 0b10100001);

        assert_eq!(command.halt_flag, false);
        assert_eq!(command.stop_game_flag, true);
        assert_eq!(command.ball_placement_flag, false);
        assert_eq!(command.ball_placement_team, true);
        assert_eq!(command.in_game, false);
        assert_eq!(command.robot_position_init, true);
        // Dribble
        assert_eq!(command.dribble_state, false);
        assert_eq!(command.dribble_advance, true);
        assert_eq!(command.dribble_enabble_error, 1000);
        assert_eq!(command.dribble_target_ball_x, 2000);
        assert_eq!(command.dribble_target_ball_y, -2000);
        assert_eq!(command.dribble_type, 1);
        // Kick
        assert_eq!(command.ball_kick_state, false);
        assert_eq!(command.free_kick_flag, true);
        assert_eq!(command.ball_kick, false);
        assert_eq!(command.kick_straight, true);
        assert_eq!(command.ball_target_allowable_error, 3000);
        assert_eq!(command.target_ball_x, 4000);
        assert_eq!(command.target_ball_y, -4000);
        assert_eq!(command.kick_type, 2);
        // Target position
        assert_eq!(command.robot_position_target_x, 5000);
        assert_eq!(command.robot_position_target_y, -5000);
        assert_eq!(command.robot_position_target_theta, 6000);

        let tx_command = aisaac_protocol::StrategyPcCpmmand {
            protocol_version: 0b00100010,
            data_type: 0b10100001,

            halt_flag: false,
            stop_game_flag: true,
            ball_placement_flag: false,
            ball_placement_team: true,
            in_game: false,
            robot_position_init: true,

            dribble_state: false,
            dribble_advance: true,
            dribble_enabble_error: 1000,
            dribble_target_ball_x: 2000,
            dribble_target_ball_y: -2000,
            dribble_type: 1,

            ball_kick_state: false,
            free_kick_flag: true,
            ball_kick: false,
            kick_straight: true,
            ball_target_allowable_error: 3000,
            target_ball_x: 4000,
            target_ball_y: -4000,
            kick_type: 2,

            robot_position_target_x: 5000,
            robot_position_target_y: -5000,
            robot_position_target_theta: 6000
        };

        let encoded_command: [u8; 24] = tx_command.into();
        assert_eq!(encoded_command, rx_command);
    }

    #[test]
    fn vision_data_test() {
        let rx_vision_data: [u8; 428] = [
            34, 163, 0, 100, 255, 156, 3, 232, 0, 150, 255, 106, 5, 220, 2, 0, 200, 255, 56, 0, 250, 255, 6, 2, 31, 7, 208, 248, 48, 78, 32, 11, 184, 244, 72, 117, 48, 1, 7, 209, 248, 47, 78, 33, 11, 185, 244, 71, 117, 49, 1, 7, 210, 248, 46, 78, 34, 11, 186, 244, 70, 117, 50, 1, 7, 211, 248, 45, 78, 35, 11, 187, 244, 69, 117, 51, 1, 7, 212, 248, 44, 78, 36, 11, 188, 244, 68, 117, 52, 1, 7, 213, 248, 43, 78, 37, 11, 189, 244, 67, 117, 53, 1, 7, 214, 248, 42, 78, 38, 11, 190, 244, 66, 117, 54, 1, 7, 215, 248, 41, 78, 39, 11, 191, 244, 65, 117, 55, 1, 7, 216, 248, 40, 78, 40, 11, 192, 244, 64, 117, 56, 1, 7, 217, 248, 39, 78, 41, 11, 193, 244, 63, 117, 57, 1, 7, 218, 248, 38, 78, 42, 11, 194, 244, 62, 117, 58, 1, 7, 219, 248, 37, 78, 43, 11, 195, 244, 61, 117, 59, 1, 7, 220, 248, 36, 78, 44, 11, 196, 244, 60, 117, 60, 1, 7, 221, 248, 35, 78, 45, 11, 197, 244, 59, 117, 61, 1, 7, 222, 248, 34, 78, 46, 11, 198, 244, 58, 117, 62, 1, 7, 223, 248, 33, 78, 47, 11, 199, 244, 57, 117, 63, 1, 7, 224, 248, 32, 78, 48, 11, 200, 244, 56, 117, 64, 1, 7, 225, 248, 31, 78, 49, 11, 201, 244, 55, 117, 65, 1, 7, 226, 248, 30, 78, 50, 11, 202, 244, 54, 117, 66, 1, 7, 227, 248, 29, 78, 51, 11, 203, 244, 53, 117, 67, 1, 7, 228, 248, 28, 78, 52, 11, 204, 244, 52, 117, 68, 1, 7, 229, 248, 27, 78, 53, 11, 205, 244, 51, 117, 69, 1, 7, 230, 248, 26, 78, 54, 11, 206, 244, 50, 117, 70, 1, 7, 231, 248, 25, 78, 55, 11, 207, 244, 49, 117, 71, 1, 7, 232, 248, 24, 78, 56, 11, 208, 244, 48, 117, 72, 1, 7, 233, 248, 23, 78, 57, 11, 209, 244, 47, 117, 73, 1, 7, 234, 248, 22, 78, 58, 11, 210, 244, 46, 117, 74, 1, 7, 235, 248, 21, 78, 59, 11, 211, 244, 45, 117, 75, 1, 7, 236, 248, 20, 78, 60, 11, 212, 244, 44, 117, 76, 1, 7, 237, 248, 19, 78, 61, 11, 213, 244, 43, 117, 77, 1, 7, 238, 248, 18, 78, 62, 11, 214, 244, 42, 117, 78, 1
            ];
        let vision_data = aisaac_protocol::VisionData::from(rx_vision_data.as_slice());

        assert_eq!(vision_data.current_pose.x, 100);
        assert_eq!(vision_data.current_pose.y, -100);
        assert_eq!(vision_data.current_pose.theta, 1000);
        assert_eq!(vision_data.current_pose.vx, 150);
        assert_eq!(vision_data.current_pose.vy, -150);
        assert_eq!(vision_data.current_pose.omega, 1500);
        assert_eq!(vision_data.current_pose.camera_valid, true);
        assert_eq!(vision_data.current_pose.data_valid, false);

        assert_eq!(vision_data.ball_position.x, 200);
        assert_eq!(vision_data.ball_position.y, -200);
        assert_eq!(vision_data.ball_position.vx, 250);
        assert_eq!(vision_data.ball_position.vy, -250);
        assert_eq!(vision_data.ball_position.camera_valid, true);
        assert_eq!(vision_data.ball_position.data_valid, false);

        assert_eq!(vision_data.number_of_obstacles, 31);
        for i in 0..31 {
            assert_eq!(vision_data.obstacles[i].x, 2000 + i as i16);
            assert_eq!(vision_data.obstacles[i].y, -2000 - i as i16);
            assert_eq!(vision_data.obstacles[i].theta, 20000 + i as i16);
            assert_eq!(vision_data.obstacles[i].vx, 3000 + i as i16);
            assert_eq!(vision_data.obstacles[i].vy, -3000 -i as i16);
            assert_eq!(vision_data.obstacles[i].omega, 30000 + i as i16);
            assert_eq!(vision_data.obstacles[i].camera_valid, false);
            assert_eq!(vision_data.obstacles[i].data_valid, true);
        }
    }
}
