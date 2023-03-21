use crate::circle::Circle;
use crate::circle::RADIUS_UPPER_BOUND;
use ordered_float::OrderedFloat;

pub fn sweep_and_prune(list_of_particles: &mut Vec<Circle>) -> Vec<(usize, usize)> {
    //!Apply the sweep_and_prune algorithm to check for potential collisions
    //! Sort all the particles along the x-axis, and then check for a potential overlap
    //! Returns tuple pairs of the positions of possible collisions

    //First, we sort the list_of_particles along an axis (the x axis)
    // We cannot sort the vector using the build in method because f64 does not
    // implement Ord
    list_of_particles.sort_by_key(|particle| OrderedFloat(particle.position_x));

    // initialize an empty vectors that will store tuple pairs or the index of
    //potential collisions that we have to check more thoroughly
    let mut list_of_possible_collisions = Vec::new();

    //We now iterate over the list of particles,
    let mut outer_counter = 0;
    while outer_counter < list_of_particles.len() {
        let mut inner_counter = outer_counter + 1;
        while inner_counter < list_of_particles.len() {
            let particle_1 = list_of_particles.get(outer_counter).unwrap();
            let particle_2 = list_of_particles.get(inner_counter).unwrap();

            // If the two particles overlap on the a axis, then there may be a collision to check
            if particle_1.position_x + particle_1.radius > particle_2.position_x - particle_2.radius
            {
                list_of_possible_collisions.push((outer_counter, inner_counter));
            }

            //One important optimization is that if the farthest right point
            //of particle_a is further from the farthest left point of particle_b than the max radius
            // allowed for a particle, then because the particles are sorted, we know no particles
            //further in the list can possibly collide with the current particle, so we break early.
            if (particle_2.position_x - particle_2.radius)
                - (particle_1.position_x + particle_1.radius)
                > RADIUS_UPPER_BOUND
            {
                break;
            }

            inner_counter += 1;
        }
        outer_counter += 1;
    }

    return list_of_possible_collisions;
}

pub fn filter_actual_collisions<'a>(
    list_of_particles: &Vec<Circle>,
    possible_collisions: &'a Vec<(usize, usize)>,
) -> Vec<&'a (usize, usize)> {
    //!Given the list of possible_collisions, apply a full check
    //! to determine the real collisions. Filter and return the real collisions.

    //Iterate over the possible collision tuples,
    // filter out any collisions that are not actual collisions
    return possible_collisions
        .iter()
        .filter(|(particle_1, particle_2)| {
            let particle_1 = list_of_particles.get(*particle_1).unwrap();
            let particle_2 = list_of_particles.get(*particle_2).unwrap();
            return Circle::check_pairwise_collision(particle_1, particle_2);
        })
        .collect();
}

pub fn apply_collision_updates(
    particles: &mut Vec<Circle>,
    actual_collisions: Vec<&(usize, usize)>,
) {
    //!Due to borrowing rules, we take each particle mutably one at a time.
    //! There is a nightly method to mutably borrow multiple at a time, but that is not used
    //! here.

    // At this point, actual_collisions contains all of the index pairs of collisions
    // Now we just need to iterate one by one, and apply the collision updates

    for (index_a, index_b) in actual_collisions.iter() {
        //First, we immutably borrow both particles
        let particle_a = particles.get(*index_a).unwrap();
        let particle_b = particles.get(*index_b).unwrap();
        //Obtain the required updates to the two particles
        let (update_a, update_b) = particle_a.collision_react(&particle_b);

        // Now we can borrow mutably one at a time without issue.
        let particle_a = particles.get_mut(*index_a).unwrap();
        particle_a.velocity_x = update_a.0;
        particle_a.velocity_y = update_a.1;

        let particle_b = particles.get_mut(*index_b).unwrap();
        particle_b.velocity_x = update_b.0;
        particle_b.velocity_y = update_b.1;
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_sort() {
        // Test that radix sort does the job
        let mut t1 = vec![
            Circle::new(1, 1.0, 1.0, 1.0, 1.0, 1.0),
            Circle::new(1, 0.5, 0.5, 0.5, 0.5, 0.5),
            Circle::new(1, 3.2, 3.2, 3.2, 3.2, 3.2),
        ];
        t1.sort_by_key(|particle| OrderedFloat(particle.position_x));

        let expected_sort = vec![0.5, 1.0, 3.2];
        let actual_sort: Vec<f64> = t1.iter().map(|circle| circle.position_x).collect();

        assert_eq!(expected_sort, actual_sort);
    }
}
