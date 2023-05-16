pub mod aisaac_protocol;

#[cfg(test)]
mod tests {
    use crate::aisaac_protocol::StrategyPcCpmmand;

    use super::*;

    #[test]
    fn strategy_command_test() {
        let rx_command: [u8; 70] = [ 33, 161, 0, 0, 3, 232, 255, 255, 252, 24, 0, 0, 39, 16, 0, 0, 7, 208, 255, 255, 248, 48, 0, 0, 78, 32, 2, 0, 0, 11, 184, 0, 0, 15, 160, 255, 255, 240, 96, 0, 0, 156, 64, 0, 0, 19, 136, 3, 10, 0, 0, 23, 112, 0, 0, 27, 88, 255, 255, 228, 168, 0, 1, 17, 112, 0, 0, 31, 64, 2 ];
        let command:StrategyPcCpmmand = aisaac_protocol::StrategyPcCpmmand::from(rx_command);

        assert_eq!(command.protocol_version, 0b00100001);
        assert_eq!(command.data_type, 0b10100001);
        assert_eq!(command.goal_pose.x, 1000);
        assert_eq!(command.goal_pose.y, -1000);
        assert_eq!(command.goal_pose.theta, 10000);
        assert_eq!(command.middle_goal_pose.x, 2000);
        assert_eq!(command.middle_goal_pose.y, -2000);
        assert_eq!(command.middle_goal_pose.theta, 20000);
        assert_eq!(command.prohibited_zone_ignore, false);
        assert_eq!(command.middle_target_flag, true);
        assert_eq!(command.halt_flag, false);
        // Kick
        assert_eq!(command.kick_power, 3000);
        assert_eq!(command.ball_goal.x, 4000);
        assert_eq!(command.ball_goal.y, -4000);
        assert_eq!(command.ball_goal.theta, 40000);
        assert_eq!(command.ball_target_allowable_error, 5000);
        assert_eq!(command.kick_type, 3);
        assert_eq!(command.ball_kick_state, true);
        assert_eq!(command.ball_kick, false);
        assert_eq!(command.ball_kick_active, true);
        assert_eq!(command.free_kick_flag, false);
        // Dribble
        assert_eq!(command.dribble_power, 6000);
        assert_eq!(command.dribble_goal.x, 7000);
        assert_eq!(command.dribble_goal.y, -7000);
        assert_eq!(command.dribble_goal.theta, 70000);
        assert_eq!(command.dribble_complete_distance, 8000);
        assert_eq!(command.dribble_state, true);
        assert_eq!(command.dribbler_active, false);

        let tx_command = aisaac_protocol::StrategyPcCpmmand {
            protocol_version: 0b00100001,
            data_type: 0b10100001,

            goal_pose: aisaac_protocol::Position { x: 1000, y: -1000, theta: 10000 },
            middle_goal_pose: aisaac_protocol::Position { x: 2000, y: -2000, theta: 20000 },
            prohibited_zone_ignore: false,
            middle_target_flag: true,
            halt_flag: false,
            kick_power: 3000,
            ball_goal: aisaac_protocol::Position { x: 4000, y: -4000, theta: 40000 },
            ball_target_allowable_error: 5000,
            kick_type: 3,
            ball_kick_state: true,
            ball_kick: false,
            ball_kick_active: true,
            free_kick_flag: false,
            dribble_power: 6000,
            dribble_goal: aisaac_protocol::Position { x: 7000, y: -7000, theta: 70000 },
            dribble_complete_distance: 8000,
            dribble_state: true,
            dribbler_active: false
        };

        let encoded_command: [u8; 70] = tx_command.into();
        assert_eq!(encoded_command, rx_command);
    }

    #[test]
    fn dwa_result_test() {
        assert_eq!(0, 0);

        let dwa_result = aisaac_protocol::DwaResult { protocol_version: 0b00100001, data_type: 0b10100010 , dwa_position: aisaac_protocol::Position { x: 100, y: -10000, theta: 200 }};
        let encoded_dwa_result: [u8; 14] = dwa_result.into();

        let ans = [ 33, 162, 0, 0, 0, 100, 255, 255, 216, 240, 0, 0, 0, 200 ];

        assert_eq!(encoded_dwa_result, ans);
    }

    #[test]
    fn vision_data_test() {
        let rx_vision_data = [ 33, 163, 0, 0, 0, 100, 255, 255, 255, 156, 0, 0, 3, 232, 0, 0, 0, 200, 255, 255, 255, 56, 19, 250, 169, 232, 31, 0, 0, 7, 208, 255, 255, 248, 48, 0, 0, 78, 32, 0, 0, 11, 184, 255, 255, 244, 72, 0, 0, 7, 209, 255, 255, 248, 47, 0, 0, 78, 33, 0, 0, 11, 185, 255, 255, 244, 71, 0, 0, 7, 210, 255, 255, 248, 46, 0, 0, 78, 34, 0, 0, 11, 186, 255, 255, 244, 70, 0, 0, 7, 211, 255, 255, 248, 45, 0, 0, 78, 35, 0, 0, 11, 187, 255, 255, 244, 69, 0, 0, 7, 212, 255, 255, 248, 44, 0, 0, 78, 36, 0, 0, 11, 188, 255, 255, 244, 68, 0, 0, 7, 213, 255, 255, 248, 43, 0, 0, 78, 37, 0, 0, 11, 189, 255, 255, 244, 67, 0, 0, 7, 214, 255, 255, 248, 42, 0, 0, 78, 38, 0, 0, 11, 190, 255, 255, 244, 66, 0, 0, 7, 215, 255, 255, 248, 41, 0, 0, 78, 39, 0, 0, 11, 191, 255, 255, 244, 65, 0, 0, 7, 216, 255, 255, 248, 40, 0, 0, 78, 40, 0, 0, 11, 192, 255, 255, 244, 64, 0, 0, 7, 217, 255, 255, 248, 39, 0, 0, 78, 41, 0, 0, 11, 193, 255, 255, 244, 63, 0, 0, 7, 218, 255, 255, 248, 38, 0, 0, 78, 42, 0, 0, 11, 194, 255, 255, 244, 62, 0, 0, 7, 219, 255, 255, 248, 37, 0, 0, 78, 43, 0, 0, 11, 195, 255, 255, 244, 61, 0, 0, 7, 220, 255, 255, 248, 36, 0, 0, 78, 44, 0, 0, 11, 196, 255, 255, 244, 60, 0, 0, 7, 221, 255, 255, 248, 35, 0, 0, 78, 45, 0, 0, 11, 197, 255, 255, 244, 59, 0, 0, 7, 222, 255, 255, 248, 34, 0, 0, 78, 46, 0, 0, 11, 198, 255, 255, 244, 58, 0, 0, 7, 223, 255, 255, 248, 33, 0, 0, 78, 47, 0, 0, 11, 199, 255, 255, 244, 57, 0, 0, 7, 224, 255, 255, 248, 32, 0, 0, 78, 48, 0, 0, 11, 200, 255, 255, 244, 56, 0, 0, 7, 225, 255, 255, 248, 31, 0, 0, 78, 49, 0, 0, 11, 201, 255, 255, 244, 55, 0, 0, 7, 226, 255, 255, 248, 30, 0, 0, 78, 50, 0, 0, 11, 202, 255, 255, 244, 54, 0, 0, 7, 227, 255, 255, 248, 29, 0, 0, 78, 51, 0, 0, 11, 203, 255, 255, 244, 53, 0, 0, 7, 228, 255, 255, 248, 28, 0, 0, 78, 52, 0, 0, 11, 204, 255, 255, 244, 52, 0, 0, 7, 229, 255, 255, 248, 27, 0, 0, 78, 53, 0, 0, 11, 205, 255, 255, 244, 51, 0, 0, 7, 230, 255, 255, 248, 26, 0, 0, 78, 54, 0, 0, 11, 206, 255, 255, 244, 50, 0, 0, 7, 231, 255, 255, 248, 25, 0, 0, 78, 55, 0, 0, 11, 207, 255, 255, 244, 49, 0, 0, 7, 232, 255, 255, 248, 24, 0, 0, 78, 56, 0, 0, 11, 208, 255, 255, 244, 48, 0, 0, 7, 233, 255, 255, 248, 23, 0, 0, 78, 57, 0, 0, 11, 209, 255, 255, 244, 47, 0, 0, 7, 234, 255, 255, 248, 22, 0, 0, 78, 58, 0, 0, 11, 210, 255, 255, 244, 46, 0, 0, 7, 235, 255, 255, 248, 21, 0, 0, 78, 59, 0, 0, 11, 211, 255, 255, 244, 45, 0, 0, 7, 236, 255, 255, 248, 20, 0, 0, 78, 60, 0, 0, 11, 212, 255, 255, 244, 44, 0, 0, 7, 237, 255, 255, 248, 19, 0, 0, 78, 61, 0, 0, 11, 213, 255, 255, 244, 43, 0, 0, 7, 238, 255, 255, 248, 18, 0, 0, 78, 62, 0, 0, 11, 214, 255, 255, 244, 42 ];
        let vision_data = aisaac_protocol::VisionData::from(rx_vision_data);

        assert_eq!(vision_data.current_pose.x, 100);
        assert_eq!(vision_data.current_pose.y, -100);
        assert_eq!(vision_data.current_pose.theta, 1000);
        assert_eq!(vision_data.ball_position.x, 200);
        assert_eq!(vision_data.ball_position.y, -200);
        assert_eq!(vision_data.number_of_obstacles, 31);
        for i in 0..31 {
            assert_eq!(vision_data.obstacles[i].x, 2000 + i as i32);
            assert_eq!(vision_data.obstacles[i].y, -2000 - i as i32);
            assert_eq!(vision_data.obstacles[i].theta, 20000 + i as i32);
            assert_eq!(vision_data.obstacles[i].vx, 3000 + i as i32);
            assert_eq!(vision_data.obstacles[i].vy, -3000 -i as i32);
        }
    }
}
