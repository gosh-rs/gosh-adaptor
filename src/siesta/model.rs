// [[file:../../adaptors.note::dadf5967][dadf5967]]
use gchemol_parser::parsers::parse_error;
use winnow::Parser;

use gosh_core::gut;
use gosh_model::ModelProperties;

use gut::prelude::*;

use std::path::Path;
// dadf5967 ends here

// [[file:../../adaptors.note::17ce3c72][17ce3c72]]
/// Read SIESTA calculated results.
///
/// # Parameters
///
/// * siesta_out_file: SIESTA output file, e.g. siesta.log
pub(crate) fn get_siesta_results(siesta_out_file: &Path) -> Result<ModelProperties> {
    // guesses other files for reading forces and structure.
    let siesta_forces_file = siesta_out_file.with_extension("FA");
    let siesta_struct_file = siesta_out_file.with_extension("STRUCT_OUT");

    let mut mp = ModelProperties::default();

    // get energy
    let s = gut::fs::read_file(&siesta_out_file)?;
    let e = super::parse::get_total_energy_many
        .parse(&s)
        .map_err(|e| parse_error(e, &s))?;
    mp.set_energy(e.into_iter().last().unwrap());

    // get forces
    let s = gut::fs::read_file(&siesta_forces_file)?;
    let f = super::parse::get_forces.parse(&s).map_err(|e| parse_error(e, &s))?;
    mp.set_forces(f);

    // get structures
    let s = gut::fs::read_file(&siesta_struct_file)?;
    let (cell, atoms) = super::parse::get_structure.parse(&s).map_err(|e| parse_error(e, &s))?;

    // scaled fractional coordinates => Cartesian coordinates
    mp.set_structure(atoms, Some(cell), true);

    Ok(mp)
}

#[test]
fn test_model_properties() -> Result<()> {
    let f = "tests/files/siesta-opt/siesta.log";
    let siesta_out_file = Path::new(f);
    let mp = get_siesta_results(siesta_out_file)?;
    assert!(!mp.get_energy().is_none());
    assert!(!mp.get_forces().is_none());
    assert!(!mp.get_molecule().is_none());

    Ok(())
}
// 17ce3c72 ends here
