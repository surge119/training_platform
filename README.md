# Training Platform
### CTFd plugin used to easily manage challenges

## How to start training platform:
```
git clone https://github.com/surge119/training_platform.git
cd training_platform
./setup.sh
python3 platform_setup.py
```
CTFd should now be accessible on port 4000. Docker containers will need to be manually routed/exposed.

## TODO:
Add functions to admin panel to start/stop/reset CTFd docker challenges - done\
Add option in admin panel to allow users to start/stop/reset CTFd docker challenges - WIP\
Integrate with GCP - WIP
