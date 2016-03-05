use std::fmt;

/// An evolutionary individual
pub trait Individual : Send + fmt::Debug {
    
    /// Creates a new individual
    fn new() -> Self;     // TODO: Replace with Rand?

	/// Produces a new individual by collaborating with a mate
    fn reproduce(&self, mate: &Self) -> Self; 
	
	/// Returns a rating used to compare this individual against
	/// others of the same type
	fn get_rating(&self) -> i32;    // TODO: Replace with Ord?
	
	/// Resets any internal counters that determine this individual's rating
	fn reset(&mut self);
	
	/// Compete with another individual of the same type
	fn compete(&mut self, other: &mut Self);
}
