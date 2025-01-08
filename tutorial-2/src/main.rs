mod safety1;
mod safety2;
mod deadlockprevention;
fn main() {
    safety1::safety1();
    safety2::safety2();
    deadlockprevention::deadlockprevention();
}
