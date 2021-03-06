use crate::{config::ArenaConfig, Paddle};
use amethyst::{
    core::{math::RealField, timing::Time, transform::Transform},
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
};
/// This system is responsible for moving all the paddles according to the user
/// provided input.
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        ReadStorage<'s, Paddle>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, ArenaConfig>,
    );

    fn run(&mut self, (paddles, mut transforms, time, input, arena_config): Self::SystemData) {
        use crate::Side;

        // Iterate over all planks and move them according to the input the user
        // provided.
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let opt_movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };

            if let Some(movement) = opt_movement {
                transform.prepend_translation_y(
                    paddle.velocity * time.delta_seconds() * movement as f32,
                );

                // We make sure the paddle remains in the arena.
                let paddle_y = transform.translation().y;
                transform.set_translation_y(
                    paddle_y
                        .max((0.0).into())
                        .min((arena_config.height - paddle.height).into()),
                );
            }
        }
    }
}
