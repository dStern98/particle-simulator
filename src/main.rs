mod circle;
mod sweep_prune;
mod utils;
use circle::Circle;
use sweep_prune::{apply_collision_updates, filter_actual_collisions, sweep_and_prune};

use flo_canvas::*;
use flo_draw::*;

use std::thread;
use std::time::Duration;

fn main() {
    let number_of_particles = utils::read_args();
    with_2d_graphics(move || {
        let canvas = create_drawing_window("Particles Simulator");

        //Clear the canvas to set a background colour
        canvas.draw(|gc| {
            gc.clear_canvas(Color::Rgba(0.0, 0.0, 0.0, 1.0));
        });

        let mut particles = Circle::particle_factory(number_of_particles);

        for particle in particles.iter() {
            particle.draw(SpriteId(particle.id), &canvas, utils::get_random_color())
        }

        loop {
            for particle in particles.iter_mut() {
                particle.update(1.0);
            }

            let possible_collisions = sweep_and_prune(&mut particles);
            let actual_collisions = filter_actual_collisions(&particles, &possible_collisions);
            apply_collision_updates(&mut particles, actual_collisions);

            // At this point, actual_collisions contains all of the index pairs of collisions
            // Now we just need to iterate one by one, and apply the collision updates

            canvas.draw(|gc| {
                gc.layer(LayerId(0));
                gc.clear_layer();

                gc.canvas_height(1000.0);
                gc.center_region(0.0, 0.0, 1000.0, 1000.0);

                for particle in particles.iter() {
                    // Render the ball's sprite at its location
                    gc.sprite_transform(SpriteTransform::Identity);
                    gc.sprite_transform(SpriteTransform::Translate(
                        particle.position_x as f32,
                        particle.position_y as f32,
                    ));
                    gc.draw_sprite(SpriteId(particle.id));
                }
            });
            // Wait for the next frame
            thread::sleep(Duration::from_nanos(1_000_000_123 / 15));
        }
    })
}
