pub struct Instruction {
    instruction_type: InstructionType,
    inner: String,
}

/// Serves as a type marker
pub enum InstructionType {
    Purpose,
    Audience,
    Complexity,
    Context,
    Restriction,
    Principle,
    /// A term that could generalize open-endedness and close-endedness
    /// into one could be "Response Scope." This term communicates the
    /// range or breadth of potential responses to the prompt, from
    /// highly specific (close-ended) to broad or limitless (open-ended).
    /// It suggests the extent of freedom the responder has in
    /// formulating their answer.
    ResponseScope,
    /// TODO: e.g. JSON, Table, CSV, unstructured text, list, list of lists, etc.
    /// TODO: Output will most likely be a beast in it of itself..
    OutputSchema,
}
