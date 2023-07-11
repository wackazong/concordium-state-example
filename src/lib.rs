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
        let iter = collection.iter();
        let result = iter.map(|(_,value)|value.deref().clone());
        Box::new(result)
    }
}
