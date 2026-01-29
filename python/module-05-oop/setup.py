from setuptools import setup

setup(
    name="module-01-basics",
    version="0.1.0",
    packages=["src"],
    install_requires=[],
    extras_require={
        "dev": ["pytest>=7.0", "pytest-cov>=4.0"],
    },
)
