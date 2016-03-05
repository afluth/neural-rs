use scoped_threadpool::Pool;
use super::Individual;
use rand;
use rand::Rng;
use num_cpus;

pub struct Evolution<T: Individual> {
    generation: u32,
    generation_size: usize,
    survival_rate: f32,
    pub individuals: Vec<T>,
    thread_pool: Pool,
}

impl<T: Individual> Evolution<T> {

    /// Constructs a new `Evolution` with the given number of individuals per generation
    /// and populates the first generation with new individuals
    pub fn new(generation_size: usize) -> Evolution<T> {
        let mut individuals = Vec::with_capacity(generation_size);

        for _ in 0..generation_size {
            individuals.push(T::new());
        }

        Evolution {
            generation: 0,
            generation_size: generation_size,
            survival_rate: 0.5,
            individuals: individuals,
            thread_pool: Pool::new(num_cpus::get() as u32), // TODO replace with num_cpus
        }
    }
    
    /// Evolves for the given number of generations
    pub fn evolve(&mut self, generations: u32) {
        
        for _ in 0..generations {
            self.repopulate();

            // Make the individuals compete
            Evolution::bisect_individuals(&mut self.thread_pool, &mut self.individuals);   

			self.find_fittest();
			
            self.generation += 1;
            
            println!("Generation: {}", self.generation);
        }

    }

    fn bisect_individuals(pool: &mut Pool, individuals: &mut [T]) {
        let length = individuals.len();
        let bisect = length - (length / 2);
        
        let (group1, group2) = individuals.split_at_mut(bisect);
        
        Evolution::compete_groups(pool, group1, group2);
        
        if bisect > 1 {
            Evolution::bisect_individuals(pool, group1);
            Evolution::bisect_individuals(pool, group2);
        }
    }

    fn compete_groups(pool: &mut Pool, group1: &mut [T], group2: &mut [T]) {
        
        // This assumes group1 is larger if they aren't the same size
        assert!(group2.len() <= group1.len());
        for i in 0..group1.len() {
            let (front, back) = group1.split_at_mut(i);
            let group1_iter = back.iter_mut()
                                  .chain(front.iter_mut());

            let pairs = group1_iter.zip(group2.iter_mut());

            pool.scoped(|scope| {
                for (individual1, individual2) in pairs {
                    scope.execute(move || {
                            individual1.compete(individual2);
                    });
                }
            });

        }
    }

    fn find_fittest(&mut self) {
        // Sort by wins/loses/mistakes
        self.individuals.sort_by(|a, b| {
            b.get_rating().cmp(&a.get_rating())
        });

        // Keep only the best
        self.individuals.truncate((self.generation_size as f32 * self.survival_rate) as usize);
        
        {
        	let best = &self.individuals[0];
        	println!("Generation: {}, {:?}", self.generation, best);
        }
        
        // Reset the ratings
        for individual in self.individuals.iter_mut() {
            individual.reset();
        }
    }

    fn repopulate(&mut self) {
        let mut rng = rand::thread_rng();
        let num_survivors = self.individuals.len();

        // Repopulate any culled players
        for i in 0..(self.generation_size - num_survivors) {
            
            let child = self.individuals[i % num_survivors]
                        .reproduce(&self.individuals[rng.gen_range(0, num_survivors)]);

            self.individuals.push(child);
        }
    }

}
