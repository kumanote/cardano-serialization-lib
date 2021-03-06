from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name='cardano_serialization_lib',
    version='0.1',
    install_requires=["setuptools_rust==0.11.3"],
    rust_extensions=[RustExtension('cardano_serialization_lib', 'rust/Cargo.toml', binding=Binding.PyO3)],
    zip_safe=False
)
