# ROS2

## setup with pixi

### create project

```sh
pixi init {project} -c robostack-humble -c conda-forge
```

### run sim

```sh
pixi add ros-humble-desktop ros-humble-turtlesim
pixi run ros2 run turtlesim turtlesim_node
```
