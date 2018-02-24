Holoviews Bokeh
====

[Deploying Bokeh Apps — HoloViews ](http://holoviews.org/user_guide/Deploying_Bokeh_Apps.html)


## matplotlib

```python
from holoviews import Curve, Store, extension


extension("matplotlib")

curve = Curve(([1, 2, 3], [1, 2, 4]))

renderer = Store.renderers['matplotlib']
renderer.show(curve)
renderer.save(curve, "image", fmt="png")
```


## Bokeh

```python
from holoviews import Curve, Store, extension


extension("bokeh")

curve = Curve(([1, 2, 3], [1, 2, 4]))

renderer = Store.renderers['bokeh']
renderer.save(curve, "graph")
```
