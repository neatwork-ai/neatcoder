pub mod tasks;

struct GptOutput<O> {
    intro: Option<String>,
    objects: O,
    remarks: Option<String>,
    residual: Option<String>,
}
