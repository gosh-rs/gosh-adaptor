// [[file:../adaptors.note::7f4ff432][7f4ff432]]
use super::*;
use crate::common::*;
// 7f4ff432 ends here

// [[file:../adaptors.note::5c72d49a][5c72d49a]]
mod parse;
// 5c72d49a ends here

// [[file:../adaptors.note::343b6bbc][343b6bbc]]
/// VASP model adaptor
pub struct Vasp();
// 343b6bbc ends here

// [[file:../adaptors.note::43388644][43388644]]
use gosh_model::ModelProperties;

// FIXME: still looks hacky
fn guess_molecule_from_contcar(positions: Vec<[f64; 3]>, fout: &Path) -> Molecule {
    // recover element data from CONTCAR
    let contcar = fout.with_file_name("CONTCAR");
    if let Ok(mut mol) = Molecule::from_file(contcar).context("read molecule from CONTCAR") {
        mol.set_positions(positions);
        mol
    } else {
        warn!("CONTCAR not found, molecule data could be incomplete!");
        // FIXME: do not bother to parse element data
        let atoms = positions.into_iter().map(|p| ("C", p));
        Molecule::from_atoms(atoms)
    }
}

impl crate::ModelAdaptor for Vasp {
    fn parse_all<P: AsRef<Path>>(&self, outfile: P) -> Result<Vec<ModelProperties>> {
        let frames = self::parse::parse_from(outfile.as_ref())?;
        Ok(frames
            .into_iter()
            .map(|frame| {
                let mut mp = ModelProperties::default();
                mp.set_energy(frame.energy);
                mp.set_forces(frame.forces);
                mp.set_molecule(guess_molecule_from_contcar(frame.positions, outfile.as_ref()));
                mp
            })
            .collect())
    }

    fn parse_last<P: AsRef<Path>>(&self, outfile: P) -> Result<ModelProperties> {
        let all = self.parse_all(outfile)?;
        if let Some(last) = all.into_iter().last() {
            Ok(last)
        } else {
            bail!("parsed no result!");
        }
    }
}
// 43388644 ends here
