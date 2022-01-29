// [[file:../adaptors.note::*imports][imports:1]]
use super::*;
// imports:1 ends here

// [[file:../adaptors.note::3894d75f][3894d75f]]
#[test]
fn test_xtb_calc() -> Result<()> {
    use std::convert::TryInto;
    use vecfx::*;
    use xtb_model::{XtbModel, XtbParameters};

    use gchemol::units::{Bohr, Hartree};

    let mol = Molecule::from_file("./tests/files/C4O8.cif")?;

    let positions = mol
        .positions()
        .flat_map(|x| [x[0] / Bohr, x[1] / Bohr, x[2] / Bohr])
        .collect_vec();
    let coord = &positions;
    let numbers = mol.atomic_numbers().map(|x| x as i32).collect_vec();
    let lattice = mol
        .get_lattice()
        .unwrap()
        .matrix()
        .iter()
        .map(|x| x / Bohr)
        .collect_vec();
    let lattice: [f64; 9] = lattice.try_into().unwrap();
    let mut params = XtbParameters::default();
    params.output_muted().method("GFN1-xTB").lattice(lattice);
    let mut xtb = XtbModel::create(&numbers, coord, params)?;
    let mut gradient = coord.to_vec();
    let energy = xtb.calculate_energy_and_gradient(&mut gradient)?;
    dbg!(energy * Hartree - -1256.768280010025);

    Ok(())
}
// 3894d75f ends here
