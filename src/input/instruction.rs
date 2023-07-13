pub struct Instruction {
    instruction_type: InstructionType,
    inner: String,
}

/// Serves as a type marker
pub enum InstructionType {
    Context,
    Purpose,
    Audience,
    Complexity,
    Avoid,
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

impl Instruction {
    pub fn new(instruction_type: InstructionType, instruction: &str) -> Self {
        Self {
            instruction_type,
            inner: instruction.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        let prelude = match self.instruction_type {
            InstructionType::Context => "\n- Context:",
            InstructionType::Purpose => "\n- Purpose:",
            InstructionType::Audience => "\n- Audience:",
            InstructionType::Complexity => "\n- Complexity:",
            InstructionType::Avoid => "\n- Avoid:",
            InstructionType::Principle => "\n- Principle:",
            InstructionType::ResponseScope => "\n- Response scope (open-ended vs. close-ended):",
            InstructionType::OutputSchema => "\n- Output schema/format:",
        };

        format!("{} {}", prelude, self.inner)
    }
}
