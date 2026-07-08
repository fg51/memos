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
pixi run rviz2
```

### custom python node

```sh
ros2 pkg create --build-type ament_python --destination-directory src --node-name my_node my_pkg
pixi add colcon-common-extensions "setuptools<=58.2.0"
pixi run colcon build
```

```pixi.toml
[activation]
scripts = ["install/setup.sh"]
```

```sh
pixi run ros2 run my_pkg my_node
# Hi from my_pkg
```

### add task

```sh
pixi task add build "colcon build --symlink"
pixi run build

pixi task add hello "ros2 run my_pkg my_node"
pixi run hello
```
