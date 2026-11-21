use super::step::Step;


// Work running in a background thread.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq
)]
pub(crate) enum Task {
    Shoot(Step),
    OpenShot(Step),
    Compare,
    OpenDiff,
}
