# Clear all files from docs
rm -rf docs/*

# Create a file named CNAME and folders
cd docs
mkdir img
echo "zigit.io" > CNAME
cd ..

# Run trunk build
trunk build --release

# Copy files from dist to docs
cp -r dist/* docs/
cp -r img/* docs/img/
