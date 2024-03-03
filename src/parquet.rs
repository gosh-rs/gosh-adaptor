// [[file:../adaptors.note::0597e8fd][0597e8fd]]
use super::ModelProperties;
use crate::common::*;
use gosh_dataset::SimpleParquetFileWriter;
// 0597e8fd ends here

// [[file:../adaptors.note::819fc289][819fc289]]
#[derive(Debug, Serialize, Clone, Default)]
struct Parsed {
    energy: Option<f64>,
    positions: Option<Vec<[f64; 3]>>,
    forces: Option<Vec<[f64; 3]>>,
}

fn to_parsed(mp: ModelProperties) -> Parsed {
    Parsed {
        energy: mp.get_energy(),
        forces: mp.get_forces().cloned(),
        ..Default::default()
    }
}
// 819fc289 ends here

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
        let mps = self.parse_all(outfile)?;
        let parsed: Vec<_> = mps.into_iter().map(to_parsed).collect();
        let mut writer = SimpleParquetFileWriter::new(pqfile.as_ref());
        writer.write_row_group(&parsed)?;
        writer.close();

        Ok(())
    }
}
// c966bf00 ends here
