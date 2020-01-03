// core

// [[file:~/Workspace/Programming/gosh-rs/adaptors/adaptors.note::*core][core:1]]
use gosh_models::ModelProperties;

#[test]
fn test_model_properties() {
    use std::path::Path;

    let d = "/home/ybyygu/siesta-opt/";
    let l = "siesta";
    let siesta_job_dir = Path::new(d);

    let mut mp = ModelProperties::default();

    // get energy
    let siesta_job_file = siesta_job_dir.join(format!("{}.log", l));
    let s = guts::fs::read_file(&siesta_job_file).unwrap();
    let (_, e) = super::parse::get_total_energy_many(&s).unwrap();
    mp.set_energy(e.into_iter().last().unwrap());

    // get forces
    let siesta_forces_file = siesta_job_file.with_extension("FA");
    let s = guts::fs::read_file(&siesta_forces_file).unwrap();
    let (_, f) = super::parse::get_forces(&s).unwrap();
    mp.set_forces(f);

    // get structures
    let siesta_struct_file = siesta_job_file.with_extension("STRUCT_OUT");
    let s = guts::fs::read_file(&siesta_struct_file).unwrap();
    let (_, (cell, atoms)) = super::parse::get_structure(&s).unwrap();
    // mp.set_molecule(mol);
    dbg!(cell);

    println!("{}", mp);
}
// core:1 ends here
