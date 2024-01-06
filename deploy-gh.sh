# Clear all files from docs
rm -rf docs/*

# Create a file named CNAME
cd docs
echo "zigit.io" > CNAME
cd ..

# Run trunk build
trunk build --release

# Copy files from dist to docs
cp -r dist/* docs/
