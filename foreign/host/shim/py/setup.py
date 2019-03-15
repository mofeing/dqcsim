from setuptools import setup, Extension
import os

try:
    os.environ['DQCSIM_RUST_RELEASE']
    libdir = '../../../../target/release'
except KeyError:
    libdir = '../../../../target/debug'

setup(
    name = 'dqcsim',
    version = '1.0', # TODO: retrieve from cargo
    description = 'Python bindings for DQCsim, the Delft Quantum & Classical Simulator',
    author = 'TU Delft Quantum & Computer Architecture, QuTech',
    packages = [
        'dqcsim',
        'dqcsim.host',
    ],
    package_dir = {'dqcsim': 'src'},
    ext_modules = [Extension(
        'dqcsim.host._dqcshost',
        ['gen/dqcshost.c'],
        library_dirs = [libdir],
        runtime_library_dirs = [os.environ['DQCSIM_HOME'] + '/lib'],
        libraries = ['dqcshost'],
        include_dirs = ['../c/gen']
    )],
    data_files = [libdir + '/libdqcshost.so'] # FIXME: file is copied to egg, but rpath points to DQCSIM_HOME.
)
