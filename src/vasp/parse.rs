// [[file:../../adaptors.note::3417889d][3417889d]]
use gosh_core::text_parser::parsers::*;
// 3417889d ends here

// [[file:../../adaptors.note::*energy][energy:1]]
fn get_total_energy(s: &str) -> IResult<&str, f64> {
    let mut token = "\n  free  energy   TOTEN  =";
    let mut jump = jump_to(token);
    let mut tag_ev = tag("eV");
    do_parse!(
        s,
        jump >> space0 >> e: double >> space0 >> tag_ev >> eol >> (e)
    )
}

#[test]
fn test_vasp_energy() {
    let txt = "  FREE ENERGIE OF THE ION-ELECTRON SYSTEM (eV)
  ---------------------------------------------------
  free  energy   TOTEN  =      -536.628381 eV

  energy  without entropy=     -536.775651  energy(sigma->0) =     -536.677471
";
    let (_, en) = get_total_energy(txt).unwrap();
    // assert!(r == "");
    assert_eq!(-536.628381, en);
}
// energy:1 ends here

// [[file:../../adaptors.note::*forces][forces:1]]
fn get_positions_and_forces(s: &str) -> IResult<&str, Vec<([f64; 3], [f64; 3])>> {
    let mut token = " POSITION                                       TOTAL-FORCE (eV/Angst)";
    let mut jump = jump_to(token);
    let mut read_data = many1(position_and_force);
    do_parse!(
        s,
        jump >> eol >>     // head line
        read_line   >>     // ignroe one line
        data: read_data >> // current coordinates and forces
        (data)
    )
}

fn position_and_force(s: &str) -> IResult<&str, ([f64; 3], [f64; 3])> {
    do_parse!(
        s,
        space0 >> p: xyz_array >>        // position
        space1 >> f: xyz_array >> eol >> // force
        ((p, f))
    )
}

#[test]
fn test_vasp_forces() {
    let txt = " POSITION                                       TOTAL-FORCE (eV/Angst)
 -----------------------------------------------------------------------------------
      3.13915      4.47145      7.05899        -0.051556     -0.016880     -0.033586
      5.48130      2.80880      7.05969         0.000184     -0.045212     -0.032849
      5.83773      5.79740      6.84087        -0.577800     -0.751742     -0.808742
      6.97326      7.41268      7.95898         0.339121      0.480393     -2.202270
      1.15573      1.63445     -0.00000        -0.012611      0.019147     -0.003608
      1.15573      4.08614     -0.00000         0.007421      0.011830      0.002581
      1.15573      6.53782     -0.00000         0.002656     -0.026602      0.000487
      1.15573      8.98950     -0.00000         0.001754     -0.000284     -0.051374
      3.46720      2.45168     -0.00000         0.007068      0.007910     -0.002521
      3.46720      4.90336     -0.00000         0.010139      0.010535      0.040481
      3.46720      7.35505     -0.00000         0.039958     -0.044033      0.015392
 -----------------------------------------------------------------------------------
    total drift:                               -0.004104      0.000869     -0.010144
";
    let (_, forces) = get_positions_and_forces(txt).unwrap();
    assert_eq!(11, forces.len());
}
// forces:1 ends here

// [[file:../../adaptors.note::*model][model:1]]
use gosh_core::gchemol::Molecule;
use gosh_core::gut;
use gosh_model::ModelProperties;

use gut::fs::*;
use gut::prelude::*;

fn get_results(s: &str) -> IResult<&str, ModelProperties> {
    let mut energy = context("Energy", get_total_energy);
    let mut positions_and_forces = context("Positions and forces", get_positions_and_forces);
    do_parse!(
        s,
        data: get_positions_and_forces >> // positions and forces
        energy: energy                 >> // energy
        ({
            let mut mp = ModelProperties::default();
            mp.set_energy(energy);
            let (positions, forces): (Vec<[f64; 3]>, Vec<[f64; 3]>) = data.into_iter().unzip();
            mp.set_forces(forces);
            // do not bother to parse element data
            let atoms = positions.into_iter().map(|p| ("C", p));
            let mol = Molecule::from_atoms(atoms);
            mp.set_molecule(mol);
            mp
        })
    )
}

/// Get all results for multiple structures.
pub(crate) fn parse_vasp_outcar<P: AsRef<Path>>(fout: P) -> Result<Vec<ModelProperties>> {
    use gosh_core::gchemol::prelude::*;

    let s = read_file(&fout)?;
    let (_, mut mps) = many1(get_results)(&s).nom_trace_err()?;

    // FIXME: still looks hacky
    // recover element data from CONTCAR
    let contcar = fout.as_ref().with_file_name("CONTCAR");
    if let Ok(parent_mol) = Molecule::from_file(contcar).context("read molecule from CONTCAR") {
        for mp in &mut mps {
            let positions = mp
                .get_molecule()
                .and_then(|mol| Some(mol.positions()))
                .expect("vasp no positions");
            let mut mol = parent_mol.clone();
            mol.set_positions(positions);
            mp.set_molecule(mol);
        }
    } else {
        warn!("CONTCAR not found, molecule data could be incomplete!");
    }
    Ok(mps)
}
// model:1 ends here

// [[file:../../adaptors.note::*test][test:1]]
#[test]
fn test_parse_vasp_outcar() {
    // vasp 5.3.5, single point
    let f = "./tests/files/vasp/OUTCAR-5.3.5";
    let mps = parse_vasp_outcar(f).unwrap();
    assert_eq!(mps.len(), 1);
}

// FIXME: make old vasp results parsing work
#[test]
#[ignore]
fn test_parse_vasp_outcar_old() {
    // test files from Jmol
    let f = "./tests/files/vasp/OUTCAR-5.2";
    let mps = parse_vasp_outcar(f).unwrap();
    assert_eq!(mps.len(), 1);

    // test files from Jmol
    let f = "./tests/files/vasp/AlH3_Vasp5.dat";
    let mps = parse_vasp_outcar(f).unwrap();
    assert_eq!(mps.len(), 7);

    // test files from Jmol
    let f = "./tests/files/vasp/OUTCAR_diamond.dat";
    let mps = parse_vasp_outcar(f).unwrap();
    assert_eq!(mps.len(), 1);
}
// test:1 ends here
