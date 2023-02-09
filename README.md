## RocketSim-Py

Python bindings for [RocketSim](https://github.com/ZealanL/RocketSim)! Compatible with Python 3.7+, Windows and Linux.

Produces just 2 wheels, and includes IDE type hints!

# Using these bindings

Everything you want is in the [latest release](https://github.com/VirxEC/rocketsim-py/releases)!

Here's what the files are:
 - `rocketsim.zip` - Unzip this into your project and have Windows & Linux support for Python 3.7+!
 - `rocketsim-X.X.X-cp37-abi3-win_amd64.whl` - The Windows-only wheel file for the bindings. You can `pip install file_name.whl` to try out RLUtilities in Python 3.7+ on Windows.
 - `rocketsim-X.X.X-cp37-abi3-manylinux_2_28_x86_64.whl` - The Linux-only wheel file for the bindings. You can `pip install file_name.whl` to try out RLUtilities in Python 3.7+ on Linux.
 - `rocketsim-X.X.X.tar.gz` - The minimal source code of this project. Unzip and it contains all the files you need to build the bindings yourself, for your platform and architecture. Beware that this doesn't included any of the required tools/dependencies to build the bindings!
