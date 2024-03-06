// [[file:../../adaptors.note::*imports][imports:1]]
use gosh_core::gut::fs::*;
use gosh_core::gut::prelude::*;
use gosh_model::ModelProperties;

use super::parse::*;
// imports:1 ends here

// [[file:../../adaptors.note::75b5807d][75b5807d]]
pub(crate) fn get_mopac_results<P: AsRef<Path>>(fout: P) -> Result<Vec<ModelProperties>> {
    let s = read_file(fout)?;
    let mps = get_results(&s)?;

    Ok(mps)
}
// 75b5807d ends here

// [[file:../../adaptors.note::70ef6bab][70ef6bab]]
#[test]
fn test_parse_mopac() {
    let f = "tests/files/mopac-multiple/mopac.out";
    let mps = get_mopac_results(f).unwrap();
    assert_eq!(mps.len(), 6);
    assert_eq!(mps[5].get_energy(), Some(-747.22443));
}
// 70ef6bab ends here
