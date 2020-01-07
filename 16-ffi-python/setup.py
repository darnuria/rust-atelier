from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="matrix64",
    version="1.0",
    rust_extensions=[RustExtension("pytrix.pytrix", binding=Binding.PyO3)],
    packages=["pytrix"],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)