import sys

from .rocketsim import *
from .rocketsim import sim

__doc__ = rocketsim.__doc__
if hasattr(rocketsim, "__all__"):
    __all__ = rocketsim.__all__

sys.modules["rocketsim.sim"] = sim
