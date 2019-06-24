from setuptools import setup, Extension


setup(
    ext_modules=[
        Extension('cmod.ext', ['src/ext.c']),
    ]
)
