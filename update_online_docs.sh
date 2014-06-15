#!/bin/bash

echo -e "\033[0;32mDeploying new Floor documentation to Github...\033[0m"

# delete old gh-pages branch
git branch -D gh-pages

git checkout -b gh-pages

# Build the docs
make doc

# Add changes to git.
git add -A -f doc

# Commit changes.
msg="doc(*): rebuilding docs `date`"
if [ $# -eq 1 ]
  then msg="$1"
fi
git commit -m "$msg"

# Push source and build repos.
git push -f origin gh-pages
