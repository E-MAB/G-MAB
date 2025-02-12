import gmab
import pytest


@pytest.mark.parametrize(
    "low, high, kwargs, exp_internal",
    [
        pytest.param(0, 1, {}, [(0, 1)]),
        pytest.param(0, 5, {"size": 1}, [(0, 5)]),
        pytest.param(0, 5, {"size": 2}, [(0, 5), (0, 5)]),
        pytest.param(0.0, 1, {}, [(0, 1)], marks=pytest.mark.xfail(raises=TypeError)),
        pytest.param(0, 1.0, {}, [(0, 1)], marks=pytest.mark.xfail(raises=TypeError)),
        pytest.param(0, 1, {"size": 0.0}, [(0, 1)], marks=pytest.mark.xfail(raises=TypeError)),
        pytest.param(0, 0, {}, [(0, 0)], marks=pytest.mark.xfail(raises=ValueError)),
        pytest.param(0, 1, {"size": 0}, [], marks=pytest.mark.xfail(raises=ValueError)),
    ],
)
def test_suggest_int(low, high, kwargs, exp_internal):
    bounds = gmab.Bounds()
    bounds.suggest_int(low=low, high=high, **kwargs)
    assert bounds.internal == exp_internal
