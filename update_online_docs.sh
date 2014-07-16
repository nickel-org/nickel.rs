#!/bin/bash

echo -e "\033[0;32mDeploying new Nickel documentation to Github...\033[0m"

# delete old gh-pages branch
git branch -D docs

git checkout -b docs

# Build the docs
make doc

#echo docs.nickel.rs > doc/CNAME

# Add changes to git.
git add -A -f doc

# Commit changes.
msg="doc(*): rebuilding docs `date`"
if [ $# -eq 1 ]
  then msg="$1"
fi
git commit -m "$msg"

git subtree split -P doc -b docs-deploy

# Push source and build repos.
git push -f docs docs-deploy:master
git branch -D docs-deploy
git checkout master
git branch -D docs
