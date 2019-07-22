// pub mod random;
// pub mod rotate;

// pub fn rainbow(
//     connection: &ApiConnection,
//     time_per_loop: &Duration,
// ) -> Result<Animation, failure::Error> {
//     let minimum_time_per_loop = Duration::from_secs(3);
//     let transition_time = Duration::from_millis(500);

//     let lights = get_active_lights(connection)?.0;

//     let mut frames = Vec::new();

//     let time_per_loop = if *time_per_loop < minimum_time_per_loop {
//         minimum_time_per_loop
//     } else {
//         *time_per_loop
//     };

//     let time_millis = (time_per_loop.as_secs() * 1000) + u64::from(time_per_loop.subsec_millis());
//     let transition_time_millis =
//         (transition_time.as_secs() * 1000) + u64::from(transition_time.subsec_millis());

//     let number_of_steps = time_millis / transition_time_millis;
//     let transition_time = Duration::from_millis(time_millis / number_of_steps);
//     let number_of_lights = lights.len();
//     let hue_step_size = std::u16::MAX / number_of_steps as u16;
//     let next_light_step_size = std::u16::MAX / number_of_lights as u16;

//     for i in 0..number_of_steps {
//         let mut states = Vec::new();

//         let mut current_hue: u16 = (i as u16 * hue_step_size) as u16;

//         for light_num in lights.keys() {
//             let state = LightState::new().hue(current_hue).sat(255);
//             states.push((*light_num, state));

//             current_hue = current_hue.wrapping_add(next_light_step_size);
//         }

//         let frame = AnimationFrame {
//             hold_time: Duration::from_secs(0),
//             transition_time,
//             states,
//         };

//         frames.push(frame);
//     }

//     Ok(Animation::new().with_frames(frames))
// }
