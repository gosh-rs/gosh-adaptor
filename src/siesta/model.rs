// imports

// [[file:~/Workspace/Programming/gosh-rs/adaptors/adaptors.note::*imports][imports:1]]
use gosh_core::guts;
use gosh_models::ModelProperties;

use guts::prelude::*;

use std::path::Path;
// imports:1 ends here

// core

// [[file:~/Workspace/Programming/gosh-rs/adaptors/adaptors.note::*core][core:1]]
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
    let s = guts::fs::read_file(&siesta_out_file)?;
    let (_, e) = super::parse::get_total_energy_many(&s)
        .map_err(|e| format_err!("parse siesta energy failed:\n{:?}", e))?;
    mp.set_energy(e.into_iter().last().unwrap());

    // get forces
    let s = guts::fs::read_file(&siesta_forces_file)?;
    let (_, f) = super::parse::get_forces(&s)
        .map_err(|e| format_err!("parse siesta forces failed:\n{:?}", e))?;
    mp.set_forces(f);

    // get structures
    let s = guts::fs::read_file(&siesta_struct_file)?;
    let (_, (cell, atoms)) = super::parse::get_structure(&s)
        .map_err(|e| format_err!("parse siesta structure failed:\n{:?}", e))?;
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
// core:1 ends here
