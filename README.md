# Training Platform
### CTFd plugin used to easily manage challenges

## How to start training platform:
```
git clone https://github.com/surge119/training_platform.git \
    && cd training_platform \
    && ./setup.sh
```
Logout of session and log back in to allow user group changes to apply
```
python3 platform_setup.py challenges \
    && docker compose up -d
```

## TODO:
