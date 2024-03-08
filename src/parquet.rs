// [[file:../adaptors.note::0597e8fd][0597e8fd]]
use super::ModelProperties;
use crate::common::*;
use gosh_dataset::SimpleParquetFileWriter;
// 0597e8fd ends here

// [[file:../adaptors.note::819fc289][819fc289]]
#[derive(Debug, Serialize, Clone, Default)]
struct Parsed {
    energy: Option<f64>,
    symbols: Option<Vec<String>>,
    // NOTE: do not [f64; 3], for easy to read out using polars
    positions: Option<Vec<Vec<f64>>>,
    forces: Option<Vec<Vec<f64>>>,
    lattice: Option<Vec<Vec<f64>>>,
    stress: Option<Vec<f64>>,
}

fn to_parsed(mp: ModelProperties) -> Parsed {
    let mol_opt = mp.get_molecule();
    Parsed {
        energy: mp.get_energy(),
        symbols: mol_opt.map(|mol| mol.symbols().into_iter().map(|x| x.to_owned()).collect_vec()),
        positions: mol_opt.map(|mol| mol.positions().collect_vec()).map(to_parquet_vector),
        forces: mp.get_forces().cloned().map(to_parquet_vector),
        stress: mol_opt.and_then(|mol| mol.properties.load::<Vec<f64>>("stress").ok()),
        lattice: mol_opt
            .and_then(|mol| mol.get_lattice())
            .map(|lat| lat.vectors())
            .map(to_parquet_vector),
        ..Default::default()
    }
}

fn to_parquet_vector(nested_array: impl IntoIterator<Item = impl Into<[f64; 3]>>) -> Vec<Vec<f64>> {
    nested_array.into_iter().map(|x| x.into().to_vec()).collect()
}
// 819fc289 ends here

// [[file:../adaptors.note::da2377d0][da2377d0]]
/// A trait for write `Computed` results in parquet format.
pub trait WriteParquet {
    /// Dump parsed results in parquet format to file `pqfile`.
    fn write_parquet(&self, pqfile: impl AsRef<Path>) -> Result<()>;
}

impl WriteParquet for [ModelProperties] {
    fn write_parquet(&self, pqfile: impl AsRef<Path>) -> Result<()> {
        let pqfile = pqfile.as_ref();
        let parsed: Vec<_> = self.iter().cloned().map(to_parsed).collect();
        println!("Parsed {} complete frames in total.", parsed.len());
        let mut writer = SimpleParquetFileWriter::new(pqfile);
        writer.write_row_group(&parsed)?;
        println!("Wrote into parquet file: {:?}", pqfile);
        writer.close();

        Ok(())
    }
}
// da2377d0 ends here

// [[file:../adaptors.note::c966bf00][c966bf00]]
/// A trait for write parsed results from computed outfile in parquet
/// format.
pub trait ParquetWrite {
    /// Dump parsed results in parquet format to file `pqfile`.
    fn dump(&self, outfile: impl AsRef<Path>, pqfile: impl AsRef<Path>) -> Result<()>;
}

impl<T: super::ModelAdaptor> ParquetWrite for T {
    /// Dump parsed results in parquet format to file `pqfile`.
    fn dump(&self, outfile: impl AsRef<Path>, pqfile: impl AsRef<Path>) -> Result<()> {
        let outfile = outfile.as_ref();
        let pqfile = pqfile.as_ref();
        println!("Parsing frames from {outfile:?}");
        let mps = self.parse_all(outfile)?;
        mps.write_parquet(pqfile)?;

        Ok(())
    }
}
// c966bf00 ends here
