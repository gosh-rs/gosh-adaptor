// [[file:../../adaptors.note::*imports][imports:1]]
use gosh_core::gut::fs::*;
use gosh_core::gut::prelude::*;
use gosh_model::ModelProperties;

use super::parse::*;
// imports:1 ends here

// [[file:../../adaptors.note::*pub][pub:1]]
pub(crate) fn get_mopac_results<P: AsRef<Path>>(fout: P) -> Result<Vec<ModelProperties>> {
    use gosh_core::text_parser::parsers::*;

    let s = read_file(fout)?;
    let (_, mps) = get_results(&s).nom_trace_err()?;

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

// [[file:../../adaptors.note::a2449942][a2449942]]
#[test]
fn test_mopac_parse() {
    let f = "./data/fc/d52f29-cb6c-4940-a6a1-d4b081e72ff4/mopac.out";

    let mps = get_mopac_results(f).unwrap();
   println!("{}", mps[0]);

    //     assert_relative_eq!(-720.18428, mr.energy.unwrap(), epsilon=1e-4);

    //     let dipole = mr.dipole.unwrap();
    //     assert_relative_eq!(0.10742828, dipole[0], epsilon=1e-4);

    //     let forces = mr.forces.unwrap();
    //     assert_eq!(13, forces.len());
    //     assert_relative_eq!(0.33629483, forces[0][0], epsilon=1e-4);

    //     let mol = mr.molecule.unwrap();
    //     assert_eq!(13, mol.natoms());

    //     // parsing single point energy calculations
    //     let fname = "tests/files/models/mopac/mopac-sp.out";

    //     let m = MOPAC();
    //     let mr = m.parse_last(fname).unwrap();
    //     assert_eq!(Some(-748.27010), mr.energy)
}
// a2449942 ends here
