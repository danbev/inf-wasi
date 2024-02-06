wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "llm",

    // For all exported worlds, interfaces, and resources, this specifies what
    // type they're corresponding to in this module. In this case the `MyHost`
    // struct defined below is going to define the exports of the `world`,
    // namely the `run` function.
    exports: {
        world: Exports,
    },
});

struct Exports;

impl Guest for Exports {
    fn version() -> String {
        crate::version().to_string()
    }
}
