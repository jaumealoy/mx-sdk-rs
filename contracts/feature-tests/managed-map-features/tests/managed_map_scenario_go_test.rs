#[test]
fn mmap_get_go() {
    multiversx_sc_scenario::run_go("scenarios/mmap_get.scen.json");
}

#[test]
fn mmap_remove_go() {
    multiversx_sc_scenario::run_go("scenarios/mmap_remove.scen.json");
}
