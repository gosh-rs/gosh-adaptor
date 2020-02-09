// imports

// [[file:~/Workspace/Programming/gosh-rs/adaptors/adaptors.note::*imports][imports:1]]
use crate::parser::*;
// imports:1 ends here

// energy
//  Free energy of the ion-electron system (eV)
//   ---------------------------------------------------
//   alpha Z        PSCENC =         0.07795844
//   Ewald energy   TEWEN  =         2.98949878
//   -1/2 Hartree   DENC   =       -19.88646085
//   -exchange  EXHF       =         0.00000000
//   -V(xc)+E(xc)   XCENC  =         5.30101276
//   PAW double counting   =         8.54250292       -8.59278258
//   entropy T*S    EENTRO =         0.00000000
//   eigenvalues    EBANDS =       -20.18312919
//   atomic energy  EATOM  =        24.97678373
//   ---------------------------------------------------
//   free energy    TOTEN  =        -6.77461598 eV

//   energy without entropy =       -6.77461598  energy(sigma->0) =       -6.77461598

// [[file:~/Workspace/Programming/gosh-rs/adaptors/adaptors.note::*energy][energy:1]]
fn get_total_energy(s: &str) -> IResult<&str, f64> {
    let token = "\n  free  energy   TOTEN  =";
    let jump = jump_to(token);
    let tag_ev = tag("eV");
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

// forces
//  POSITION                                       TOTAL-FORCE (eV/Angst)
//  -----------------------------------------------------------------------------------
//       4.73330      5.58579      5.33333         0.004230      0.007496      0.000000
//       1.43848      6.45843      5.33333         0.004652     -0.004046      0.000000
//       2.27731      4.51308      6.54283        -0.005354      0.004441     -0.004168
//       2.27731      4.51308      4.12384        -0.005354      0.004441      0.004168
//       6.61677      4.43569      5.33333        -0.003625      0.008090      0.000000
//       3.93261      6.14206      5.33333         0.005874     -0.010314      0.000000
//       7.62868      3.90899      5.33333         0.007598     -0.004560      0.000000
//       2.46155      5.37666      5.33333        -0.008022     -0.005549      0.000000
//  -----------------------------------------------------------------------------------
//     total drift:                               -0.003238     -0.001265     -0.000178

// [[file:~/Workspace/Programming/gosh-rs/adaptors/adaptors.note::*forces][forces:1]]
fn get_positions_and_forces(s: &str) -> IResult<&str, Vec<([f64; 3], [f64; 3])>> {
    let token = " POSITION                                       TOTAL-FORCE (eV/Angst)";
    let jump = jump_to(token);
    let read_data = many1(position_and_force);
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

// model

// [[file:~/Workspace/Programming/gosh-rs/adaptors/adaptors.note::*model][model:1]]
use gosh_core::gchemol::Molecule;
use gosh_core::gut;
use gosh_models::ModelProperties;

use gut::fs::*;
use gut::prelude::*;

fn get_results(s: &str) -> IResult<&str, ModelProperties> {
    do_parse!(
        s,
        energy: get_total_energy >>       // force consistent energy
        data: get_positions_and_forces >> // forces
        ({
            let mut mp = ModelProperties::default();
            mp.set_energy(energy);
            let (_, forces): (Vec<[f64; 3]>, Vec<[f64; 3]>) = data.into_iter().unzip();
            mp.set_forces(forces);
            mp
        })
    )
}

/// Get all results for multiple structures.
pub(crate) fn parse_vasp_outcar<P: AsRef<Path>>(fout: P) -> Result<Vec<ModelProperties>> {
    let s = read_file(fout)?;
    let (_, mps) = many1(get_results)(&s)
        .map_err(|e| format_err!("Failed to parse gulp results:\n{:?}", e))?;
    Ok(mps)
}

#[test]
fn test_parse_vasp_outcar() {
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
// model:1 ends here
