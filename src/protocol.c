#include "protocol.h"
#include <stdint.h>
#include <arpa/inet.h>
#include <string.h>
#include <assert.h>
#include <stdbool.h>
#include <stdio.h>

#define RX_ASSERT(x) (assert(send.x == recv.x));

const uint8_t protocol_version = 0b00100010;    // Ver. 2.2
const uint8_t strategy_pc_command_data_type = 0b10100001;   // 5-1
const uint8_t dwa_result_data_type = 0b10100010;   // 5-2
const uint8_t vision_data_data_type = 0b10100011;   // 5-3
const uint8_t manual_controller_data_type = 0b10100100; // 5-4
const uint8_t robot_odometry_data_type = 0b10100101; // 5-5
const uint8_t robot_observed_ball_data_type = 0b10100110;    // 5-6
const uint8_t robot_debug_data_type = 0b10100111;    // 5-7

int encodeStrategyPcCommand(_strategy_pc_command *command, char *buffer) {
    uint8_t buffer_index = 0;
    uint16_t tmp_u16;

    buffer[buffer_index] = protocol_version;
    buffer_index += 1;
    buffer[buffer_index] = strategy_pc_command_data_type;
    buffer_index += 1;

    buffer[buffer_index] =
        (char)command->halt_flag << 7
        | (char)command->stop_game_flag << 6
        | (char)command->ball_placement_flag << 5
        | (char)command->ball_placement_team << 4
        | (char)command->in_game << 3
        | (char)command->robot_position_init << 2
        | (char)command->dribble_state << 1
        | (char)command->dribble_advance;
    buffer_index += 1;
    // Dribble
    tmp_u16= htons(command->dribble_enabble_error);
    memcpy(&buffer[buffer_index], &tmp_u16, 2);
    buffer_index += 2;
    tmp_u16= htons(command->dribble_target_ball_x);
    memcpy(&buffer[buffer_index], &tmp_u16, 2);
    buffer_index += 2;
    tmp_u16= htons(command->dribble_target_ball_y);
    memcpy(&buffer[buffer_index], &tmp_u16, 2);
    buffer_index += 2;
    buffer[buffer_index] = (char)(command->dribble_type);
    buffer_index += 1;
    // Kick
    buffer[buffer_index] =
        (char)(command->ball_kick_state) << 3
        | (char)(command->free_kick_flag) << 2
        | (char)(command->ball_kick) << 1
        | (char)(command->kick_straight);
    buffer_index += 1;
    tmp_u16 = htons(command->ball_target_allowable_error);
    memcpy(&buffer[buffer_index], &tmp_u16, 2);
    buffer_index += 2;
    tmp_u16 = htons(command->target_ball_x);
    memcpy(&buffer[buffer_index], &tmp_u16, 2);
    buffer_index += 2;
    tmp_u16 = htons(command->target_ball_y);
    memcpy(&buffer[buffer_index], &tmp_u16, 2);
    buffer_index += 2;
    buffer[buffer_index] = (char)(command->kick_type);
    buffer_index += 1;
    // Target position
    tmp_u16 = htons(command->robot_position_target_x);
    memcpy(&buffer[buffer_index], &tmp_u16, 2);
    buffer_index += 2;
    tmp_u16 = htons(command->robot_position_target_y);
    memcpy(&buffer[buffer_index], &tmp_u16, 2);
    buffer_index += 2;
    tmp_u16 = htons(command->robot_position_target_theta);
    memcpy(&buffer[buffer_index], &tmp_u16, 2);
    buffer_index += 2;

    return buffer_index;
}

int decodeStrategyPcCommand(_strategy_pc_command *command, char *buffer, uint8_t buffer_length) {
    uint8_t buffer_index = 0;
    uint16_t tmp_u16;
    // Check buffer length
    assert(buffer_length > 70);

    assert((uint8_t)buffer[buffer_index] == protocol_version);
    command->protocol_version = protocol_version;
    buffer_index += 1;
    assert((uint8_t)buffer[buffer_index] == strategy_pc_command_data_type);
    command->data_type = strategy_pc_command_data_type;
    buffer_index += 1;

    command->halt_flag = (buffer[buffer_index] & 0b10000000) == 0b10000000;
    command->stop_game_flag = (buffer[buffer_index] & 0b1000000) == 0b1000000;
    command->ball_placement_flag = (buffer[buffer_index] & 0b100000) == 0b100000;
    command->ball_placement_team = (buffer[buffer_index] & 0b10000) == 0b10000;
    command->in_game = (buffer[buffer_index] & 0b1000) == 0b1000;
    command->robot_position_init = (buffer[buffer_index] & 0b100) == 0b100;

    // Dribble
    command->dribble_state = (buffer[buffer_index] & 0b10) == 0b10;
    command->dribble_advance = (buffer[buffer_index] & 0b1) == 0b1;
    buffer_index += 1;
    memcpy(&tmp_u16, &buffer[buffer_index], 2);
    command->dribble_enabble_error = (uint16_t)ntohs(tmp_u16);
    buffer_index += 2;
    memcpy(&tmp_u16, &buffer[buffer_index], 2);
    command->dribble_target_ball_x = (int16_t)ntohs(tmp_u16);
    buffer_index += 2;
    memcpy(&tmp_u16, &buffer[buffer_index], 2);
    command->dribble_target_ball_y = (int16_t)ntohs(tmp_u16);
    buffer_index += 2;
    command->dribble_type = (uint8_t)buffer[buffer_index];
    buffer_index += 1;

    // Kick
    command->ball_kick_state = (buffer[buffer_index] & 0b1000) == 0b1000;
    command->free_kick_flag = (buffer[buffer_index] & 0b100) == 0b100;
    command->ball_kick = (buffer[buffer_index] & 0b10) == 0b10;
    command->kick_straight = (buffer[buffer_index] & 0b1) == 0b1;
    buffer_index += 1;
    memcpy(&tmp_u16, &buffer[buffer_index], 2);
    command->ball_target_allowable_error = (uint16_t)ntohs(tmp_u16);
    buffer_index += 2;
    memcpy(&tmp_u16, &buffer[buffer_index], 2);
    command->target_ball_x = (int16_t)ntohs(tmp_u16);
    buffer_index += 2;
    memcpy(&tmp_u16, &buffer[buffer_index], 2);
    command->target_ball_y = (int16_t)ntohs(tmp_u16);
    buffer_index += 2;
    command->kick_type = (uint8_t)buffer[buffer_index];
    buffer_index += 1;

    // Target position
    memcpy(&tmp_u16, &buffer[buffer_index], 2);
    command->robot_position_target_x = (int16_t)ntohs(tmp_u16);
    buffer_index += 2;
    memcpy(&tmp_u16, &buffer[buffer_index], 2);
    command->robot_position_target_y = (int16_t)ntohs(tmp_u16);
    buffer_index += 2;
    memcpy(&tmp_u16, &buffer[buffer_index], 2);
    command->robot_position_target_theta = (int16_t)ntohs(tmp_u16);
    buffer_index += 2;

    return buffer_index;
}

int encodeVisionData(_vision_data *vision_data, char *buffer) {
    uint16_t buffer_index = 0;
    uint16_t tmp_u16;

    buffer[buffer_index] = protocol_version;
    buffer_index += 1;
    buffer[buffer_index] = vision_data_data_type;
    buffer_index += 1;

    // Current Pose
    tmp_u16 = htons(vision_data->current_pose.x);
    memcpy(&buffer[buffer_index], &tmp_u16, 2);
    buffer_index += 2;
    tmp_u16 = htons(vision_data->current_pose.y);
    memcpy(&buffer[buffer_index], &tmp_u16, 2);
    buffer_index += 2;
    tmp_u16 = htons(vision_data->current_pose.theta);
    memcpy(&buffer[buffer_index], &tmp_u16, 2);
    buffer_index += 2;
    tmp_u16 = htons(vision_data->current_pose.vx);
    memcpy(&buffer[buffer_index], &tmp_u16, 2);
    buffer_index += 2;
    tmp_u16 = htons(vision_data->current_pose.vy);
    memcpy(&buffer[buffer_index], &tmp_u16, 2);
    buffer_index += 2;
    tmp_u16 = htons(vision_data->current_pose.omega);
    memcpy(&buffer[buffer_index], &tmp_u16, 2);
    buffer_index += 2;
    buffer[buffer_index] =
        (char) vision_data->current_pose.camera_valid << 1
        | (char) vision_data->current_pose.data_valid;
    buffer_index += 1;

    // Ball Position
    tmp_u16 = htons(vision_data->ball_position.x);
    memcpy(&buffer[buffer_index], &tmp_u16, 2);
    buffer_index += 2;
    tmp_u16 = htons(vision_data->ball_position.y);
    memcpy(&buffer[buffer_index], &tmp_u16, 2);
    buffer_index += 2;
    tmp_u16 = htons(vision_data->ball_position.vx);
    memcpy(&buffer[buffer_index], &tmp_u16, 2);
    buffer_index += 2;
    tmp_u16 = htons(vision_data->ball_position.vy);
    memcpy(&buffer[buffer_index], &tmp_u16, 2);
    buffer_index += 2;
    buffer[buffer_index] =
        (char) vision_data->ball_position.camera_valid << 1
        | (char) vision_data->ball_position.data_valid;
    buffer_index += 1;

    // Obstacles
    assert(vision_data->number_of_obstacles < 32);
    buffer[buffer_index] = vision_data->number_of_obstacles;
    buffer_index += 1;
    for (int obstacle_index = 0; obstacle_index < vision_data->number_of_obstacles; obstacle_index++) {
        tmp_u16 = htons(vision_data->obstacles[obstacle_index].x);
        memcpy(&buffer[buffer_index], &tmp_u16, 2);
        buffer_index += 2;
        tmp_u16 = htons(vision_data->obstacles[obstacle_index].y);
        memcpy(&buffer[buffer_index], &tmp_u16, 2);
        buffer_index += 2;
        tmp_u16 = htons(vision_data->obstacles[obstacle_index].theta);
        memcpy(&buffer[buffer_index], &tmp_u16, 2);
        buffer_index += 2;
        tmp_u16 = htons(vision_data->obstacles[obstacle_index].vx);
        memcpy(&buffer[buffer_index], &tmp_u16, 2);
        buffer_index += 2;
        tmp_u16 = htons(vision_data->obstacles[obstacle_index].vy);
        memcpy(&buffer[buffer_index], &tmp_u16, 2);
        buffer_index += 2;
        tmp_u16 = htons(vision_data->obstacles[obstacle_index].omega);
        memcpy(&buffer[buffer_index], &tmp_u16, 2);
        buffer_index += 2;
        buffer[buffer_index] =
            (char) vision_data->obstacles[obstacle_index].camera_valid << 1
            | (char) vision_data->obstacles[obstacle_index].data_valid;
        buffer_index += 1;
    }

    return buffer_index;
}

int decodeVisionData(_vision_data *vision_data, char *buffer, uint16_t buffer_length) {
    uint16_t buffer_index = 0;
    uint16_t tmp_u16;
    uint32_t tmp_u32;
    // Check buffer length
    assert(buffer_length > 647);    // Need update max length

    assert((uint8_t)buffer[buffer_index] == protocol_version);
    vision_data->protocol_version = protocol_version;
    buffer_index += 1;
    assert((uint8_t)buffer[buffer_index] == vision_data_data_type);
    vision_data->data_type = vision_data_data_type;
    buffer_index += 1;

    // Current Pose
    memcpy(&tmp_u16, &buffer[buffer_index], 2);
    vision_data->current_pose.x = (int16_t)ntohs(tmp_u16);
    buffer_index += 2;
    memcpy(&tmp_u16, &buffer[buffer_index], 2);
    vision_data->current_pose.y = (int16_t)ntohs(tmp_u16);
    buffer_index += 2;
    memcpy(&tmp_u16, &buffer[buffer_index], 2);
    vision_data->current_pose.theta = (int16_t)ntohs(tmp_u16);
    buffer_index += 2;
    memcpy(&tmp_u16, &buffer[buffer_index], 2);
    vision_data->current_pose.vx = (int16_t)ntohs(tmp_u16);
    buffer_index += 2;
    memcpy(&tmp_u16, &buffer[buffer_index], 2);
    vision_data->current_pose.vy = (int16_t)ntohs(tmp_u16);
    buffer_index += 2;
    memcpy(&tmp_u16, &buffer[buffer_index], 2);
    vision_data->current_pose.omega = (int16_t)ntohs(tmp_u16);
    buffer_index += 2;
    vision_data->current_pose.camera_valid = (buffer[buffer_index] & 0b10) == 0b10;
    vision_data->current_pose.data_valid = (buffer[buffer_index] & 0b1) == 0b1;
    buffer_index += 1;

    // Ball Position
    memcpy(&tmp_u16, &buffer[buffer_index], 2);
    vision_data->ball_position.x = (int16_t)ntohs(tmp_u16);
    buffer_index += 2;
    memcpy(&tmp_u16, &buffer[buffer_index], 2);
    vision_data->ball_position.y = (int16_t)ntohs(tmp_u16);
    buffer_index += 2;
    memcpy(&tmp_u16, &buffer[buffer_index], 2);
    vision_data->ball_position.vx = (int16_t)ntohs(tmp_u16);
    buffer_index += 2;
    memcpy(&tmp_u16, &buffer[buffer_index], 2);
    vision_data->ball_position.vy = (int16_t)ntohs(tmp_u16);
    buffer_index += 2;
    vision_data->ball_position.camera_valid = (buffer[buffer_index] & 0b10) == 0b10;
    vision_data->ball_position.data_valid = (buffer[buffer_index] & 0b1) == 0b1;
    buffer_index += 1;

    // Obstacles
    vision_data->number_of_obstacles = buffer[buffer_index];
    buffer_index += 1;
    for (int obstacle_index = 0; obstacle_index < vision_data->number_of_obstacles; obstacle_index++) {
        memcpy(&tmp_u16, &buffer[buffer_index], 2);
        vision_data->obstacles[obstacle_index].x = (int16_t)ntohs(tmp_u16);
        buffer_index += 2;
        memcpy(&tmp_u16, &buffer[buffer_index], 2);
        vision_data->obstacles[obstacle_index].y = (int16_t)ntohs(tmp_u16);
        buffer_index += 2;
        memcpy(&tmp_u16, &buffer[buffer_index], 2);
        vision_data->obstacles[obstacle_index].theta = (int16_t)ntohs(tmp_u16);
        buffer_index += 2;
        memcpy(&tmp_u16, &buffer[buffer_index], 2);
        vision_data->obstacles[obstacle_index].vx = (int16_t)ntohs(tmp_u16);
        buffer_index += 2;
        memcpy(&tmp_u16, &buffer[buffer_index], 2);
        vision_data->obstacles[obstacle_index].vy = (int16_t)ntohs(tmp_u16);
        buffer_index += 2;
        memcpy(&tmp_u16, &buffer[buffer_index], 2);
        vision_data->obstacles[obstacle_index].omega = (int16_t)ntohs(tmp_u16);
        buffer_index += 2;
        vision_data->obstacles[obstacle_index].camera_valid = (buffer[buffer_index] & 0b10) == 0b10;
        vision_data->obstacles[obstacle_index].data_valid = (buffer[buffer_index] & 0b1) == 0b1;
        buffer_index += 1;
    }

    return buffer_index;
}

int encodeManualContollerData(_manual_controller_data *controller_data, char *buffer) {
    uint8_t buffer_index = 0;
    uint32_t tmp_u32;

    buffer[buffer_index] = protocol_version;
    buffer_index += 1;
    buffer[buffer_index] = manual_controller_data_type;
    buffer_index += 1;

    buffer[buffer_index] =
        (char)controller_data->controller_start << 4
        | (char)controller_data->dribbler_on << 3
        | (char)controller_data->kick_straight << 2
        | (char)controller_data->kick_tip << 1
        | (char)controller_data->emergency_stop;
    buffer_index += 1;

    memcpy(&tmp_u32, &buffer[buffer_index], 4);
    controller_data->robot_vx = (int32_t)ntohl(tmp_u32);
    buffer_index += 4;
    memcpy(&tmp_u32, &buffer[buffer_index], 4);
    controller_data->robot_vy = (int32_t)ntohl(tmp_u32);
    buffer_index += 4;
    memcpy(&tmp_u32, &buffer[buffer_index], 4);
    controller_data->robot_vw = (int32_t)ntohl(tmp_u32);
    buffer_index += 4;

    return buffer_index;
}

int decodeManualContollerData(_manual_controller_data *controller_data, char *buffer, uint8_t buffer_length) {
    uint8_t buffer_index = 0;
    uint32_t tmp_u32;
    // Check buffer length
    assert(buffer_length > 14);

    assert((uint8_t)buffer[buffer_index] == protocol_version);
    controller_data->protocol_version = protocol_version;
    buffer_index += 1;
    assert((uint8_t)buffer[buffer_index] == manual_controller_data_type);
    controller_data->data_type = manual_controller_data_type;
    buffer_index += 1;

    controller_data->controller_start = (buffer[buffer_index] & 0b10000) == 0b10000;
    controller_data->dribbler_on = (buffer[buffer_index] & 0b1000) == 0b1000;
    controller_data->kick_straight = (buffer[buffer_index] & 0b100) == 0b100;
    controller_data->kick_tip = (buffer[buffer_index] & 0b10) == 0b10;
    controller_data->emergency_stop = (buffer[buffer_index] & 0b1) == 0b1;
    buffer_index += 1;

    memcpy(&tmp_u32, &buffer[buffer_index], 4);
    controller_data->robot_vx = (int32_t)ntohl(tmp_u32);
    buffer_index += 4;
    memcpy(&tmp_u32, &buffer[buffer_index], 4);
    controller_data->robot_vy = (int32_t)ntohl(tmp_u32);
    buffer_index += 4;
    memcpy(&tmp_u32, &buffer[buffer_index], 4);
    controller_data->robot_vw = (int32_t)ntohl(tmp_u32);
    buffer_index += 4;

    return buffer_index;
}

int encodeRobotOdometryData(_robot_odometry_data *odometry_data, char *buffer) {
    uint8_t buffer_index = 0;
    uint16_t tmp_u16;

    buffer[buffer_index] = protocol_version;
    buffer_index += 1;
    buffer[buffer_index] = robot_odometry_data_type;
    buffer_index += 1;

    tmp_u16 = htons(odometry_data->robot_position_x);
    memcpy(&buffer[buffer_index], &tmp_u16, 2);
    buffer_index += 2;
    tmp_u16 = htons(odometry_data->robot_position_y);
    memcpy(&buffer[buffer_index], &tmp_u16, 2);
    buffer_index += 2;
    tmp_u16 = htons(odometry_data->robot_position_theta);
    memcpy(&buffer[buffer_index], &tmp_u16, 2);
    buffer_index += 2;

    return buffer_index;
}

int decodeRobotOdometryData(_robot_odometry_data *odometry_data, char *buffer, uint8_t buffer_length) {
    uint8_t buffer_index = 0;
    uint16_t tmp_u16;
    // Check buffer length
    assert(buffer_length > 7);

    assert((uint8_t)buffer[buffer_index] == protocol_version);
    odometry_data->protocol_version = protocol_version;
    buffer_index += 1;
    assert((uint8_t)buffer[buffer_index] == robot_odometry_data_type);
    odometry_data->data_type = robot_odometry_data_type;
    buffer_index += 1;

    memcpy(&tmp_u16, &buffer[buffer_index], 2);
    odometry_data->robot_position_x = (int16_t)ntohs(tmp_u16);
    buffer_index += 2;
    memcpy(&tmp_u16, &buffer[buffer_index], 2);
    odometry_data->robot_position_y = (int16_t)ntohs(tmp_u16);
    buffer_index += 2;
    memcpy(&tmp_u16, &buffer[buffer_index], 2);
    odometry_data->robot_position_theta = (int16_t)ntohs(tmp_u16);
    buffer_index += 2;

    return buffer_index;
}

int encodeRobotObservedBallData(_robot_observed_ball_data *ball_data, char *buffer) {
    uint8_t buffer_index = 0;
    uint16_t tmp_u16;

    buffer[buffer_index] = protocol_version;
    buffer_index += 1;
    buffer[buffer_index] = robot_observed_ball_data_type;
    buffer_index += 1;

    tmp_u16 = htons(ball_data->ball_position_x);
    memcpy(&buffer[buffer_index], &tmp_u16, 2);
    buffer_index += 2;
    tmp_u16 = htons(ball_data->ball_position_y);
    memcpy(&buffer[buffer_index], &tmp_u16, 2);
    buffer_index += 2;

    return buffer_index;
}

int decodeRobotObservedBallData(_robot_observed_ball_data *ball_data, char *buffer, uint8_t buffer_length) {
    uint8_t buffer_index = 0;
    uint16_t tmp_u16;
    // Check buffer length
    assert(buffer_length > 5);

    assert((uint8_t)buffer[buffer_index] == protocol_version);
    ball_data->protocol_version = protocol_version;
    buffer_index += 1;
    assert((uint8_t)buffer[buffer_index] == robot_observed_ball_data_type);
    ball_data->data_type = robot_observed_ball_data_type;
    buffer_index += 1;

    memcpy(&tmp_u16, &buffer[buffer_index], 2);
    ball_data->ball_position_x = (int16_t)ntohs(tmp_u16);
    buffer_index += 2;
    memcpy(&tmp_u16, &buffer[buffer_index], 2);
    ball_data->ball_position_y = (int16_t)ntohs(tmp_u16);
    buffer_index += 2;

    return buffer_index;
}

int main() {
    // test code
    {
        _strategy_pc_command send, recv;
        send.halt_flag = false;
        send.stop_game_flag = true;
        send.ball_placement_flag = false;
        send.ball_placement_team = true;
        send.in_game = false;
        send.robot_position_init = true;
        // Dribble
        send.dribble_state = false;
        send.dribble_advance = true;
        send.dribble_enabble_error = 1000;
        send.dribble_target_ball_x = 2000;
        send.dribble_target_ball_y = -2000;
        send.dribble_type = 1;
        // Kick
        send.ball_kick_state = false;
        send.free_kick_flag = true;
        send.ball_kick = false;
        send.kick_straight = true;
        send.ball_target_allowable_error = 3000;
        send.target_ball_x = 4000;
        send.target_ball_y = -4000;
        send.kick_type = 2;
        // Target position
        send.robot_position_target_x = 5000;
        send.robot_position_target_y = -5000;
        send.robot_position_target_theta = 6000;

        char buffer[200];

        int length = encodeStrategyPcCommand(&send, &buffer[0]);
        decodeStrategyPcCommand(&recv, &buffer[0], 200);

        printf("len: %d\r\n", length);
        printf("[ ");
        for (int i = 0; i < length; i++) {
            printf("%d, ", (uint8_t)buffer[i]);
        }
        printf(" ];\r\n");

        // Verify
        RX_ASSERT(halt_flag)
        RX_ASSERT(stop_game_flag)
        RX_ASSERT(ball_placement_flag)
        RX_ASSERT(ball_placement_team)
        RX_ASSERT(in_game)
        RX_ASSERT(robot_position_init)
        // Dribble
        RX_ASSERT(dribble_state)
        RX_ASSERT(dribble_advance)
        RX_ASSERT(dribble_enabble_error)
        RX_ASSERT(dribble_target_ball_x)
        RX_ASSERT(dribble_target_ball_y)
        RX_ASSERT(dribble_type)
        // Kick
        RX_ASSERT(ball_kick_state)
        RX_ASSERT(free_kick_flag)
        RX_ASSERT(ball_kick)
        RX_ASSERT(kick_straight)
        RX_ASSERT(ball_target_allowable_error)
        RX_ASSERT(target_ball_x)
        RX_ASSERT(target_ball_y)
        RX_ASSERT(kick_type)
        // Target position
        RX_ASSERT(robot_position_target_x)
        RX_ASSERT(robot_position_target_y)
        RX_ASSERT(robot_position_target_theta)

        printf("StrategyPcCommand: OK\n");
    }

    {
        _vision_data send, recv;
        send.current_pose.x = 100;
        send.current_pose.y = -100;
        send.current_pose.theta = 1000;
        send.current_pose.vx = 150;
        send.current_pose.vy = -150;
        send.current_pose.omega = 1500;
        send.current_pose.camera_valid = true;
        send.current_pose.data_valid = false;

        send.ball_position.x = 200;
        send.ball_position.y = -200;
        send.ball_position.vx = 250;
        send.ball_position.vy = -250;
        send.ball_position.camera_valid = true;
        send.ball_position.data_valid = false;

        send.number_of_obstacles = 31;
        for (int i = 0; i < send.number_of_obstacles; i++) {
            send.obstacles[i].x = 2000 + i;
            send.obstacles[i].y = -2000 - i;
            send.obstacles[i].theta = 20000 + i;
            send.obstacles[i].vx = 3000 + i;
            send.obstacles[i].vy = -3000 - i;
            send.obstacles[i].omega = 30000 + i;
            send.obstacles[i].camera_valid = false;
            send.obstacles[i].data_valid = true;
        }

        char buffer[680];

        int length = encodeVisionData(&send, &buffer[0]);
        decodeVisionData(&recv, &buffer[0], 680);

        printf("len: %d\r\n", length);
        printf("[ ");
        for (int i = 0; i < length; i++) {
            printf("%d, ", (uint8_t)buffer[i]);
        }
        printf(" ];\r\n");

        // Verify
        // Robot
        RX_ASSERT(current_pose.x)
        RX_ASSERT(current_pose.y)
        RX_ASSERT(current_pose.theta)
        RX_ASSERT(current_pose.vx)
        RX_ASSERT(current_pose.vy)
        RX_ASSERT(current_pose.omega)
        RX_ASSERT(current_pose.camera_valid)
        RX_ASSERT(current_pose.data_valid)
        // Ball
        RX_ASSERT(ball_position.x)
        RX_ASSERT(ball_position.y)
        RX_ASSERT(ball_position.vx)
        RX_ASSERT(ball_position.vy)
        RX_ASSERT(ball_position.camera_valid)
        RX_ASSERT(ball_position.data_valid)
        for (int i = 0; i < send.number_of_obstacles; i++) {
            RX_ASSERT(obstacles[i].x)
            RX_ASSERT(obstacles[i].y)
            RX_ASSERT(obstacles[i].theta)
            RX_ASSERT(obstacles[i].vx)
            RX_ASSERT(obstacles[i].vy)
            RX_ASSERT(obstacles[i].omega)
            RX_ASSERT(obstacles[i].camera_valid)
            RX_ASSERT(obstacles[i].data_valid)
        }
    }

    {
        _manual_controller_data send, recv;
        send.controller_start = false;
        send.robot_vx = 100;
        send.robot_vy = -100;
        send.robot_vw = 1000;
        send.dribbler_on = true;
        send.kick_straight = false;
        send.kick_tip = true;
        send.emergency_stop = false;

        char buffer[15];

        int length = encodeManualContollerData(&send, &buffer[0]);
        decodeManualContollerData(&recv, &buffer[0], 15);

        printf("len: %d\r\n", length);
        printf("[ ");
        for (int i = 0; i < length; i++) {
            printf("%d, ", (uint8_t)buffer[i]);
        }
        printf(" ];\r\n");

        // Verify
        RX_ASSERT(controller_start)
        RX_ASSERT(robot_vx)
        RX_ASSERT(robot_vy)
        RX_ASSERT(robot_vw)
        RX_ASSERT(dribbler_on)
        RX_ASSERT(kick_straight)
        RX_ASSERT(kick_tip)
        RX_ASSERT(emergency_stop)
    }

    {
        _robot_odometry_data send, recv;
        send.robot_position_x = 1000;
        send.robot_position_y = -1000;
        send.robot_position_theta = 2000;

        char buffer[8];

        int length = encodeRobotOdometryData(&send, &buffer[0]);
        decodeRobotOdometryData(&recv, &buffer[0], 8);

        printf("len: %d\r\n", length);
        printf("[ ");
        for (int i = 0; i < length; i++) {
            printf("%d, ", (uint8_t)buffer[i]);
        }
        printf(" ];\r\n");

        // Verify
        RX_ASSERT(robot_position_x)
        RX_ASSERT(robot_position_y)
        RX_ASSERT(robot_position_theta)
    }

    {
        _robot_observed_ball_data send, recv;
        send.ball_position_x = 1000;
        send.ball_position_y = -1000;

        char buffer[6];

        int length = encodeRobotObservedBallData(&send, &buffer[0]);
        decodeRobotObservedBallData(&recv, &buffer[0], 6);

        printf("len: %d\r\n", length);
        printf("[ ");
        for (int i = 0; i < length; i++) {
            printf("%d, ", (uint8_t)buffer[i]);
        }
        printf(" ];\r\n");

        // Verify
        RX_ASSERT(ball_position_x)
        RX_ASSERT(ball_position_y)
    }

    return 0;
}
