# Point Cloud Rust Library 

## Functionality 
- Need some vector of pretty 3d point structs
  - Contains x,y,z touple (or fields)
  - RGB data (touple?)
  - Timestamp (how do you get time?)
- Seems like we should use the realsense rust wrapper 

## Visualization Seperate Project 
- RVIZ (ROS1 or ROS2?)
  - roslibrust? 
    - How would I talk to a docker container with my realsense?
      - How would it talk back to me so I can see it in RVIZ?
- BEVY 3D? 
	- https://youtubetranscript.com/?v=MWIO-jP6pVo chris bacardi video where he rendered 1 million particles using a point cloud
	- Need to convert realsense data to BEVY particle, then maybe mesh
