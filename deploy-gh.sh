# Clear all files from docs
rm -rf docs/*

# Run trunk build
trunk build --release

# Copy files from dist to docs
cp -r dist/* docs/
