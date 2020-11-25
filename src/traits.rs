// collection of traits implemented in the code

// naive csv preparation of available entry types -> will be handled by serde later in the code
// development. Structs implementing this trait are: MedicationForm, Activity
pub trait SerializeInput {
    fn to_vec(&self) -> Vec<String>;
}
