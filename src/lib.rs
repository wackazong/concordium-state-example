use concordium_std::ops::Deref;
use concordium_std::*;

#[derive(Serial, DeserialWithState, StateClone)]
#[concordium(state_parameter = "S")]
struct State<S: HasStateApi> {
    collections: StateMap<u8, StateMap<u8, Vec<u8>, S>, S>,
}

#[init(contract = "contract")]
fn init<S: HasStateApi>(
    _: &impl HasInitContext,
    state_builder: &mut StateBuilder<S>,
) -> InitResult<State<S>> {
    Ok(State {
        collections: state_builder.new_map(),
    })
}
impl<S: HasStateApi> State<S> {
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = Vec<u8>> + 'a> {
        let collection = self.collections.get(&5).unwrap();
        // let iter = collection.iter();
        // let result = iter.map(|(_, value)| value.deref().clone());
        Box::new(Iter::new(collection.deref().clone()))
    }
}

struct Iter<'a, S: HasStateApi> {
    collection: &'a StateMap<u8, Vec<u8>, S>,
    iter: StateMapIter<'a, u8, Vec<u8>, S>,
}

impl<'a, S: HasStateApi> Iter<'a, S> {
    pub fn new(collection: &'a StateMap<u8, Vec<u8>, S>) -> Self {
        let iter = collection.iter();
        Self { collection, iter }
    }
}

impl<'a, S: HasStateApi> Iterator for Iter<'a, S> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next().map(|state_ref| state_ref.1.deref().clone());
        next
    }
}
