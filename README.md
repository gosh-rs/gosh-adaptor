gosh-adaptor reads computed data such as energy, forces from common
computational chemistry codes, include VASP, SIESTA, MOPAC, GULP, and
more.

gosh-adaptor now supports write all collected data in parquet file

for example, for VASP opt/MD task:

    gosh-adaptor vasp collect OUTCAR -o opt.parq

read from bbm checkpoints db:

    gosh-adaptor ckpts collect opt.db -o opt.parq

opt.parq can be read and analyzed using polars or pandas.

The columns in opt.parq include energy, symbols, forces, positions, stress, lattice

