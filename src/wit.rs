wit_bindgen::generate!({
    world: "llm",
    exports: {
        world: Exports,
    },
});

struct Exports;

impl Guest for Exports {
    fn version() -> String {
        crate::version().to_string()
    }

    fn inference() -> String {
        crate::inference().to_string()
    }
}
