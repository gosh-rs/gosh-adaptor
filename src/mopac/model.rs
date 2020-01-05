// imports

// [[file:~/Workspace/Programming/gosh-rs/adaptors/adaptors.note::*imports][imports:1]]
use gosh_core::guts::fs::*;
use gosh_core::guts::prelude::*;
use gosh_models::ModelProperties;

use super::parse::*;
// imports:1 ends here

// pub

// [[file:~/Workspace/Programming/gosh-rs/adaptors/adaptors.note::*pub][pub:1]]
pub(crate) fn get_mopac_results<P: AsRef<Path>>(fout: P) -> Result<Vec<ModelProperties>> {
    let s = read_file(fout)?;
    let (_, mps) = get_results(&s).unwrap();
    Ok(mps)
}

#[test]
fn test_parse_mopac() {
    let f = "tests/files/mopac-multiple/mopac.out";
    let mps = get_mopac_results(f).unwrap();
    assert_eq!(mps.len(), 6);
    assert_eq!(mps[5].get_energy(), Some(-747.22443));
}
// pub:1 ends here
