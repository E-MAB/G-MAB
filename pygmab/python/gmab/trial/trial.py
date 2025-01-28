from gmab.utils.singleton import Singleton


class Trial(Singleton):
    def __init__(self):
        if not hasattr(self, "_names"):
            self._names: list = []
        if not hasattr(self, "low"):
            self._low: list = []
        if not hasattr(self, "_high"):
            self._high: list = []
        if not hasattr(self, "_bounds"):
            self._bounds: list[tuple] | None = None

    @property
    def bounds(self) -> list[tuple]:
        if not self._bounds:
            bounds = []
            for idx in range(len(self._names)):
                bounds.append((self._low[idx], self._high[idx]))
            self._bounds = bounds
        return self._bounds

    def suggest_int(self, name: str, low: int, high: int) -> None:
        self._names.append(name)
        self._low.append(low)
        self._high.append(high)


# Instantiate Trial in the module to replicate Optuna's behaviour
trial = Trial()
