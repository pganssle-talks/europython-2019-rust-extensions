from setuptools import setup

def build_native(spec):
    # build an example rust library
    build = spec.add_external_build(
        cmd=['cargo', 'build', '--release'],
        path='./rust'
    )

    spec.add_cffi_module(
        module_path='msmodule._native',
        dylib=lambda: build.find_dylib('msmodule', in_path='target/release'),
        header_filename=lambda: build.find_header('msmodule.h', in_path='target'),
        rtld_flags=['NOW', 'NODELETE']
    )

setup(
    milksnake_tasks=[
        build_native
    ]
)
