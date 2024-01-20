use super::utils::MathVec;
use flo_canvas::*;
use std::f64::consts::PI;

const WIDTH: f64 = 1000.0;
const HEIGHT: f64 = 1000.0;
const VELOCITY_UPPER_BOUND: f64 = 25.0;
pub const RADIUS_UPPER_BOUND: f64 = 50.0;

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    // Because id is used for SpriteId, great care must be taken
    // to ensure that id is unique amongst the particles, otherwise
    // the particles in the simulation will spontaneously swap places
    // without warning!!
    pub id: u64,
    pub radius: f64,
    //Here we will set mass to be proportional to
    // the area of the circle, it is not independent!
    pub mass: f64,
    pub position_x: f64,
    pub position_y: f64,
    pub velocity_x: f64,
    pub velocity_y: f64,
}

impl Particle {
    #[allow(dead_code)]
    pub fn new(
        id: u64,
        radius: f64,
        position_x: f64,
        position_y: f64,
        velocity_x: f64,
        velocity_y: f64,
    ) -> Self {
        //mass is dependent on radius and therefore should not
        // be set independently
        let mass = radius.powi(2) * PI;
        Particle {
            id,
            radius,
            mass,
            position_x,
            position_y,
            velocity_x,
            velocity_y,
        }
    }
    pub fn new_random() -> Self {
        //!Creates a Random particle whose
        //!velocity, and position are bounded by the constants
        //! `WIDTH`, `HEIGHT`, and `VELOCITY_UPPER_BOUND`
        let random_radius = rand::random::<f64>() * RADIUS_UPPER_BOUND;
        Particle {
            id: (rand::random::<u64>()),
            radius: random_radius,
            mass: random_radius.powi(2) * PI,
            position_x: (rand::random::<f64>() * WIDTH),
            position_y: (rand::random::<f64>() * HEIGHT),
            velocity_x: (rand::random::<f64>() * VELOCITY_UPPER_BOUND),
            velocity_y: (rand::random::<f64>() * VELOCITY_UPPER_BOUND),
        }
    }

    pub fn draw(&self, sprite_id: SpriteId, canvas: &DrawingTarget, color: Color) {
        //!Draw the circle onto the canvas.
        canvas.draw(|gc| {
            gc.sprite(sprite_id);
            gc.clear_sprite();

            gc.new_path();
            gc.circle(0.0, 0.0, self.radius as f32);
            gc.fill_color(color);
            gc.fill();
        })
    }

    pub fn particle_factory(count: usize) -> Vec<Particle> {
        //! Generate `count` randomly sized circles.
        //`` All random values are bounded by the constants
        //`` defined at the top of the file.
        (0..count).map(|_| Particle::new_random()).collect()
    }

    pub fn update(&mut self, dt: f64) {
        //! Update the positions and velocities of the particle.
        // First, update the position by applying the velocity times the dt
        self.position_x += self.velocity_x * dt;
        self.position_y += self.velocity_y * dt;

        //Second, check for particles reaching any boundaries, and reverse their velocity

        if self.position_x + self.radius >= WIDTH && self.velocity_x > 0.0
            || self.position_x - self.radius <= 0.0 && self.velocity_x < 0.0
        {
            self.velocity_x *= -1.0;
        }

        if self.position_y + self.radius >= HEIGHT && self.velocity_y > 0.0
            || self.position_y - self.radius <= 0.0 && self.velocity_y < 0.0
        {
            self.velocity_y *= -1.0;
        }
    }

    pub fn check_pairwise_collision(&self, other: &Particle) -> bool {
        // Returns a boolean depending on whether or not a collision has occurred
        // For circles, if the distance is less than the sum of the radii,
        // then the circles must overlap
        let circle_distance = f64::sqrt(
            (self.position_x - other.position_x).powi(2)
                + (self.position_y - other.position_y).powi(2),
        );
        circle_distance < self.radius + other.radius
    }

    pub fn collision_react(&self, other: &Particle) -> (MathVec, MathVec) {
        //! Given two particles that are determined to have collided,
        //! perform the physics calcs for an elastic collision.
        //! Returns a tuple of the new velocities for self and other.

        // If the particles have collided, apply the rules of
        // an elastic collision
        let v1 = MathVec(self.velocity_x, self.velocity_y);
        let x1 = MathVec(self.position_x, self.position_y);
        let v2 = MathVec(other.velocity_x, other.velocity_y);
        let x2 = MathVec(other.position_x, other.position_y);

        //There is a known bug where particles can get stuck together because
        //they fail to clear each others area before the next re-render after a
        //collision. In order to prevent this, do not change the velocity of particles
        // that are currently moving away from each other. Only particles moving towards each other
        //should be 'colliding', otherwise the particles are in the act of recoiling.
        let dt = 0.000001;
        if (x1 + dt * v1).distance(&(x2 + dt * v2)) - x1.distance(&x2) > 0.0 {
            return (v1, v2);
        }

        // Quite an ugly formula from wikipedia
        // https://en.wikipedia.org/wiki/Elastic_collision
        let v_self_new = v1
            - (2.0 * other.mass / (self.mass + other.mass))
                * (v1 - v2).inner_product(&(x1 - x2))
                * (1.0 / ((x1 - x2).inner_product(&(x1 - x2))))
                * (x1 - x2);

        let v_other_new = v2
            - (2.0 * self.mass / (self.mass + other.mass))
                * (v2 - v1).inner_product(&(x2 - x1))
                * (1.0 / ((x2 - x1).inner_product(&(x2 - x1))))
                * (x2 - x1);

        (v_self_new, v_other_new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_normal() {
        //First, test a normal particle not hitting a wall
        let mut test_particle = Particle::new(1, 1.0, 1.0, 1.0, 2.5, 3.5);

        test_particle.update(1.0);

        //After the move, particle position_x should be at
        // position_x + velocity_x * dt = 1.0 + 2.5 * 1 = 3.5
        assert_eq!(test_particle.position_x, 3.5);
        //particle position_y should be at
        // position_y + velocity_y * dt = 1.0 + 3.5 * 1 = 4.5
        assert_eq!(test_particle.position_y, 4.5);
    }

    #[test]
    fn test_move_past_boundary() {
        // This time velocity_y of -3.5 will cause the particle to go off
        // the map in the y direction during the next move
        let mut test_particle = Particle::new(1, 1.0, 1.0, 1.0, 2.5, -3.5);
        test_particle.update(1.0);
        assert_eq!(test_particle.position_x, 3.5);
        assert_eq!(test_particle.position_y, -2.5);

        //Since position_y is now outside of the boundary,
        // the velocity_y should have switched signs to 3.5
        // velocity_x should be unaffected
        assert_eq!(test_particle.velocity_x, 2.5);
        assert_eq!(test_particle.velocity_y, 3.5);
    }

    #[test]
    fn test_pairwise_collision_detection() {
        //These two particles are colliding
        let p1 = Particle::new(1, 1.5, 1.0, 2.0, 2.5, -3.5);
        let mut p2 = Particle::new(1, 1.0, 2.0, 1.0, 2.5, -3.5);
        assert!(p1.check_pairwise_collision(&mut p2));

        //These two particle are not colliding
        let p3 = Particle::new(1, 1.5, 1.0, 2.0, 2.5, -3.5);
        let mut p4 = Particle::new(1, 1.0, 4.0, 1.0, 2.5, -3.5);
        assert!(!p3.check_pairwise_collision(&mut p4));
    }
}
