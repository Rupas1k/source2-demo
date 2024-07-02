pub mod chat;
pub mod game_state;
pub mod game_time;
pub mod players;
pub mod wards;

#[macro_export]
macro_rules! try_observers {
    ($self:ident, $method:ident ( $($arg:expr),* )) => {
        $self.observers
            .iter()
            .try_for_each(|obs| obs.borrow_mut().$method($($arg),*))
    };
}
