// imports

// [[file:~/Workspace/Programming/gosh-rs/adaptors/adaptors.note::*imports][imports:1]]
use std::path::Path;

use gosh_core::guts;
use gosh_models::ModelProperties;

use guts::fs::read_file;
use guts::prelude::*;

use super::parse::*;
// imports:1 ends here

// pub

// [[file:~/Workspace/Programming/gosh-rs/adaptors/adaptors.note::*pub][pub:1]]
pub struct MOPAC();

use crate::ModelAdaptor;

impl ModelAdaptor for MOPAC {
    fn parse_all<P: AsRef<Path>>(&self, outfile: P) -> Result<Vec<ModelProperties>> {
        get_mopac_results(outfile)
    }

    fn parse_last<P: AsRef<Path>>(&self, outfile: P) -> Result<ModelProperties> {
        let all = self.parse_all(outfile)?;
        let last = all.into_iter().last().unwrap();
        Ok(last)
    }
}
fn get_mopac_results<P: AsRef<Path>>(fout: P) -> Result<Vec<ModelProperties>> {
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
