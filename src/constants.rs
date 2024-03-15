pub static APPLICATION_NAME: &str = "galactic";

#[cfg(debug_assertions)]
#[allow(dead_code)]
pub static APPLICATION_ID: &str = "io.github.danielwolbach.Galactic.Devel";

#[cfg(not(debug_assertions))]
#[allow(dead_code)]
pub static APPLICATION_ID: &str = "io.github.danielwolbach.Galactic";
