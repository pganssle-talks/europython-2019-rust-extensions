from setuptools import setup
from setuptools_rust import RustExtension

setup(
    rust_extensions=[RustExtension('pomodule.backend', 'Cargo.toml'),
                     RustExtension('pomodule.date_ex', 'Cargo.toml'),
                     RustExtension('pomodule.classy', 'Cargo.toml'),
                     ],
)
