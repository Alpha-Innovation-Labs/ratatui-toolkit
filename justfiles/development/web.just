# Serve the web demo locally
# Usage: just web-serve
web-serve:
    cd web-demo && trunk serve

# Build the web demo for production
# Usage: just web-build
web-build:
    cd web-demo && trunk build --release
